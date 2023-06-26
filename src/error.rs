use std::{
	path::Path,
	process::exit
};

pub fn no_configs() {
	println!("open: no configurations found");
	exit(1);
}

pub fn many_args() {
	println!("open: too many arguments supplied");
	exit(2);
}

pub fn editor_unset() {
	println!("open: $EDITOR is not set");
	exit(3);
}

pub fn not_found(path: &Path) {
	println!("open: {path:?} does not exist");
	exit(4);
}

pub fn no_section(path: &Path) {
	println!("open: no appropriate sections for {path:?}");
	exit(5);
}

