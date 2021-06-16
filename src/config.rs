use std::{
	fs::read_to_string,
	env::{current_dir, var},
	path::Path
};

use configparser::ini::Ini;

pub struct Config {
	pub local: Option<Ini>,
	pub global: Option<Ini>
}

impl Config {
	pub fn new() -> Config {
		// Instantiate global config
		let i_dir_global = String::from(var("HOME").ok().unwrap());
		let dir_global = Path::new(i_dir_global.as_str());
		let i_path_global = dir_global.join(".config/open.conf");
		let path_global = i_path_global.as_path();
		let mut global: Option<Ini> = None;
		if path_global.exists() {
			let i_global = read_to_string(path_global).unwrap();
			let mut tmp = Ini::new();
			tmp.read(i_global).ok();
			global = Some(tmp);
		}

		// Instantiate local config, if it exists.
		let i_dir_local = current_dir().unwrap();
		let mut dir_local = i_dir_local.as_path();
		let mut i_path_local = dir_local.join(".open");
		let mut path_local = i_path_local.as_path();
		let root = Path::new("/");
		let mut local: Option<Ini> = None;
		loop {
			if dir_local == root {
				break;
			}
			if path_local.exists() {
				let i_local = read_to_string(path_local).unwrap();
				let mut tmp = Ini::new();
				tmp.read(i_local).ok();
				local = Some(tmp);
				break;
			}
			dir_local = dir_local.parent().unwrap();
			i_path_local = dir_local.join(".open");
			path_local = i_path_local.as_path();
		}

		if global.is_none() && local.is_none() {
			panic!("No configuration found.");
		}

		// Prepare loop condition
		let output = Config {
			global,
			local
		};
		return output;
	}
	pub fn get(&self, section: &str, key: &str) -> Option<String> {
		let mut output: Option<String> = None;
		if self.local.is_some() {
			output = self.local.clone().unwrap().get(section, key);
		}
		if output.is_none() && self.global.is_some() {
			output = self.global.clone().unwrap().get(section, key);
		}
		return output;
	}
	pub fn getbool(&self, section: &str, key: &str) -> Result<Option<bool>, String> {
		let mut output = Ok(None);
		if self.local.is_some() {
			output = self.local.clone().unwrap().getbool(section, key);
		}
		if output.clone().ok().is_none() && self.global.is_some() {
			output = self.global.clone().unwrap().getbool(section, key);
		}
		return output;
	}
}
