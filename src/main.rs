use std::{
	env::{args, current_dir},
	path::Path,
	process::{Command, Stdio}
};

mod config;
use config::Config;

fn main() {
	// Prepare config file
	let i_dir = current_dir().unwrap();
	let dir = i_dir.as_path();
	let config = Config::new();

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
		open [FLAGS] [FILE]

FLAGS:
		-h, --help			Prints this help text
");
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
		return;
	}

	let default = ".".to_string();
	let arg_target = args.get(1);
	let i_target = arg_target.unwrap_or(&default);
	let target = Path::new(i_target);
	if !target.exists() {
		println!("\"{}\" does not exist.", i_target);
		return;
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
			"open" => {},
			"dir" => println!("No command specified for directories."),
			_ => println!("No command specified for \"{}\" files.", ext)
		}
		return;
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
