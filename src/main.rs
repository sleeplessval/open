use std::{
	env::{args, current_dir, var},
	path::Path,
	process::{Command, exit, Stdio}
};

mod config;
use config::Config;

fn main() {
	// Prepare config file
	let i_dir = current_dir().unwrap();
	let dir = i_dir.as_path();
	let mut config = Config::new();

	// Parse arguments and handle them.
	let args: Vec<String> = args().collect();

	let mut error: Option<String> = None;
	let mut file_operand = false;
	for arg in &args[1..] {
		match arg.as_str() {
			"-h" |
			"--help" => {
				println!("open
Valerie Wolfe <sleeplessval@gmail.com>
A Linux implementation of the \"open\" command on Mac OS written in Rust and easily configurable.

USAGE:
		open [FLAGS] [OPERAND]

FLAGS:
		-h, --help		Prints this help text
		-a, --add		Add a handler for a operand type
		-p, --path		Prints the config path used
");
				return;
			},
			"-a" |
			"--add" => {
				if args.len() < 4 {
					println!("open: too few arguments.");
					exit(1);
				}
				let ext = args.get(2).unwrap();
				let exe = args.get(3).unwrap();
				let tmp = args.get(4);
				let shell = tmp.is_some() && tmp.unwrap() == "shell";
				println!("{} {} {}", ext, exe, shell);
				config.add(ext, "command", exe.to_string());
				if shell {
					config.add(ext, "shell", "true".to_string());
				}
				config.write().ok();
				return;
			},
			"-p" |
			"--path" => {
				let local = config.local_path;
				if local.is_some() {
					println!("{}", local.unwrap());
				} else {
					println!("{}", config.global_path.unwrap());
				}
				return;
			},
			_ => {
				if file_operand {
					error = Some("open: too many file operands.".to_string());
				} else {
					file_operand = true;
				}
			}
		}
	}
	if error.is_some() {
		println!("{}", error.unwrap());
		exit(1);
	}
	let default = ".".to_string();
	let arg_target = args.get(1);
	let i_target = arg_target.unwrap_or(&default);
	let target = Path::new(i_target);
	if !target.exists() {
		println!("open: \"{}\" does not exist.", i_target);
		exit(1);
	}
	let i_ext = target.extension();
	let i_filename: String;
	let ext: &str;
	if target.is_dir() {
		if arg_target.is_none() {
			ext = "open";
		} else {
			ext = "dir";
		}
	} else {
		if i_ext.is_none() {
			i_filename = ["filename", target.file_name().unwrap().to_str().unwrap()].join(":");
		} else {
			i_filename = [".", i_ext.unwrap().to_str().unwrap()].join("");
		}
		ext = i_filename.as_str();
	}
	let i_exe = config.get(ext, "command");
	if i_exe.is_none() {
		let use_editor = config.getbool("open", "use_editor");
		if use_editor.is_ok() && use_editor.ok().unwrap().unwrap() {
			let i_editor = var("EDITOR");
			if i_editor.is_err() {
				println!("open: encountered an error trying to access $EDITOR");
				exit(1);
			}
			let editor = i_editor.ok().unwrap();
			if editor.is_empty() {
				println!("open: $EDITOR is not defined.");
				exit(1);
			}
			let mut command = Command::new(editor);
			command.args(vec![i_target]);
			command.stdout(Stdio::inherit())
			.stderr(Stdio::inherit())
			.stdin(Stdio::inherit());
			command.output().ok();
			exit(0);
		} else {
			match ext {
				"dir" => println!("open: no command specified for directories."),
				_ => println!("open: no command specified for \"{}\" files.", ext)
			}
			exit(1);
		}
	}
	let exe = i_exe.unwrap();
	let mut parts = exe.split(" ");
	let mut command = Command::new(parts.next().unwrap());
	let mut param: Vec<&str> = vec![];
	for part in parts {
		param.push(part);
	}
	param.push(i_target);
	command.args(param)
	.current_dir(dir);

	let is_sh = config.getbool(ext, "shell").ok().unwrap_or(Some(false)).unwrap_or(false);
	if is_sh {
		command.stdout(Stdio::inherit())
		.stderr(Stdio::inherit())
		.stdin(Stdio::inherit());
		command.output().ok();
	} else {
		command.stdout(Stdio::null())
		.stderr(Stdio::null());
		command.spawn().ok();
	}
}
