use std::{
	fs::read_to_string,
	env::{args, current_dir, var},
	path::Path,
	process::{Command, Child, ChildStdin, ChildStdout, Stdio}
};

use configparser::ini::Ini;

fn main() {
	// Prepare config file
	let i_dir = current_dir().unwrap();
	let mut dir = i_dir.as_path();
	let mut i_path = dir.join(".open");
	let mut path = i_path.as_path();
	let root = Path::new("/");
	while !path.exists() {
		if dir == root {
			// If we hit root while propagating, default to the
			// user config.
			let i_dir = String::from(var("HOME").ok().unwrap());
			dir = Path::new(i_dir.as_str());
			i_path = dir.join(".config/open.conf");
			path = i_path.as_path();
			if path.exists() {
				break;
			} else {
				println!("No configuration found.");
				return;
			}
		}
		dir = dir.parent().unwrap();
		i_path = dir.join(".open");
		path = i_path.as_path();
	}
	let ini_str = read_to_string(path).unwrap();
	let mut config = Ini::new();
	config.read(ini_str).ok();
	dir = i_dir.as_path();

	let args: Vec<String> = args().collect();

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
