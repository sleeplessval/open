use std::{
	env::current_dir,
	io::{ stdout, IsTerminal },
	path::Path,
	process::{ exit, Command, Stdio }
};

use pico_args::Arguments;
use toml::value::{ Array, Value };

mod config;
mod error;
use config::Config;

fn main() {
	let mut args = Arguments::from_env();

	//	help flag		(-h / --help)
	if args.contains(["-h", "--help"]) {
		help_text();
		return;
	}
	//	version flag	(-v / --version)
	if args.contains(["-v", "--version"]) {
		println!("{}", env!("CARGO_PKG_VERSION"));
		return;
	}

	//	prepare configs
	let i_dir = current_dir().unwrap();
	let dir = i_dir.as_path();
	let config = Config::new();

	//	path flag		(-p / --path)
	if args.contains(["-p", "--path"]) {
		let local = config.local_path;
		let global = config.global_path;
		if local.is_some() {
			println!("{}", local.unwrap());
			return;
		}
		if global.is_some() {
			println!("{}", global.unwrap());
			return;
		}
		error::no_configs();
		return;
	}

	//	get target
	let arg_target = args.subcommand().unwrap();
	let i_target = arg_target.unwrap_or(String::from("."));
	let target = Path::new(&i_target);
	if !target.exists() { error::not_found(&target); }

	//	get section
	//	ordering: filename -> type (ext/dir)
	let mut section = None;

	//	by exact filename
	let filename = target.file_name();
	if filename.is_some() {
		let filename_section = config.get(filename.unwrap().to_str().unwrap());
		if filename_section.is_some() {
			section = filename_section;
		}
	}

	//	handle types; dir first
	if section.is_none() && target.is_dir() {
		let dir_section = config.get("dir");
		if dir_section.is_some() {
			section = dir_section;
		}
	}

	//	handle types; extensions second
	if section.is_none() {
		let extension = target.extension();
		if extension.is_some() {
			let extension = extension.unwrap().to_str();

			//	pull extension array and filter matches
			let i_macrosection: Option<Value> = config.get("extension");
			let macrosection: Array = i_macrosection.unwrap().as_array().unwrap().to_owned();
			let matches = macrosection.iter().filter(|value| {
				let table = value.as_table().unwrap();
				let i_target = table.get("match").unwrap();
				let target = i_target.as_str();
				target == extension
			}).map(|value| value.to_owned() );

			let sections: Vec<Value> = matches.collect();
			if sections.len() > 0 {
				section = sections.get(0).cloned();
			}
		}
	}

	//	default or fail on missing session
	if section.is_none() {
		let default = config.get("default");
		if default.is_some() { section = default; }
		else { error::no_section(target); }
	}

	//	unwrap our section
	let properties = section.unwrap();

	//	collect properties
	let command = properties.get("command").unwrap().as_str().unwrap().to_string();
	let shell = properties.get("shell").unwrap_or(&Value::Boolean(false)).as_bool().unwrap();

	if !stdout().is_terminal() {
		println!("{command} {i_target}");
		exit(0);
	}

	//	build child
	let mut parts = command.split(" ");
	let mut child = Command::new(parts.next().unwrap());
	let mut param: Vec<&str> = parts.collect();
	param.push(&i_target);
	child
		.args(param)
		.current_dir(dir);

	//	shell processes inherit stdio (and use `output()`)
	if shell {
		child
			.stdin(Stdio::inherit())
			.stdout(Stdio::inherit())
			.stderr(Stdio::inherit());
		child.output().ok();
	}
	//	non-shell processes should redirect to dev/null (and use `spawn()`)
	else {
		child
			.stdin(Stdio::null())
			.stdout(Stdio::null())
			.stderr(Stdio::null());
		child.spawn().ok();
	}
}

pub fn help_text() {
	println!("open v{}
Valerie Wolfe <sleeplessval@gmail.com>
A Rust reimagining of \"xdg-open\" configurable with an toml file.

usage: open [flags] <target>

flags:
   -h, --help      Prints this help text
   -p, --path      Prints the path to the config file
   -v, --version   Prints the version number
",
	env!("CARGO_PKG_VERSION")
	);
}

