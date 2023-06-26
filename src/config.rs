use std::{
	fs::read_to_string,
	env::{current_dir, var},
	path::Path
};

use toml::{
	Value,
	map::Map
};

pub struct Config {
	pub local: Option<Map<String, Value>>,
	pub global: Option<Map<String, Value>>,
	pub local_path: Option<String>,
	pub global_path: Option<String>
}

impl Config {
	pub fn new() -> Config {
		// Instantiate global config
		let i_dir_global = String::from(var("HOME").ok().unwrap());
		let dir_global = Path::new(i_dir_global.as_str());
		let i_path_global = dir_global.join(".config/open.toml");
		let path_global = i_path_global.as_path();
		let mut global = None;
		if path_global.exists() {
			let raw_conf = read_to_string(path_global).unwrap();
			let toml_conf: Value = toml::from_str(raw_conf.as_str()).unwrap();
			let toml = toml_conf.as_table().unwrap();
			global = Some(toml.to_owned());
		}

		// Instantiate local config, if it exists.
		let i_dir_local = current_dir().unwrap();
		let mut dir_local = i_dir_local.as_path();
		let mut i_path_local = dir_local.join(".open");
		let mut path_local = i_path_local.as_path();
		let root = Path::new("/");
		let mut local = None;
		loop {
			if dir_local == root {
				break;
			}
			if path_local.exists() {
				let raw_conf = read_to_string(path_local).unwrap();
				let toml_conf: Value = toml::from_str(raw_conf.as_str()).unwrap();
				let toml = toml_conf.as_table().unwrap();
				local = Some(toml.to_owned());
				break;
			}
			dir_local = dir_local.parent().unwrap();
			i_path_local = dir_local.join(".open");
			path_local = i_path_local.as_path();
		}

		if global.is_none() && local.is_none() {
			panic!("No configuration found.");
		}

		// prepare path vars
		let global_path: Option<String>;
		if global.is_some() {
			global_path = Some(path_global.to_str().unwrap().to_string());
		} else {
			global_path = None
		}
		let local_path: Option<String>;
		if local.is_some() {
			local_path = Some(dir_local.join(".open").to_str().unwrap().to_string());
		} else {
			local_path = None;
		}
		let output = Config {
			global,
			local,
			local_path,
			global_path
		};
		return output;
	}
	pub fn get(&self, key: &str) -> Option<Value> {
		let mut output: Option<Value> = None;
		if self.local.is_some() {
			let result = self.local.as_ref().unwrap().get(key);
			if result.is_some() {
				output = Some(result.unwrap().to_owned());
			}
		}
		if output.is_none() && self.global.is_some() {
			let result = self.global.as_ref().unwrap().get(key);
			if result.is_some() {
				output = Some(result.unwrap().to_owned());
			}
		}
		return output;
	}
}
