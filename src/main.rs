use std::{
	env::{args, current_dir},
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
		-h, --help			Prints this help text
		-a, --add			Add a handler for a operand type
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
				config.add(ext, "command", exe.to_string());
				if shell {
					config.add(ext, "shell", "true".to_string());
				}
				config.write();
				return;
			}
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
		match ext {
			"open" => println!("open: no zero-operand command specified."),
			"dir" => println!("open: no command specified for directories."),
			_ => println!("open: no command specified for \"{}\" files.", ext)
		}
		exit(1);
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

fn get_ext() {

}
