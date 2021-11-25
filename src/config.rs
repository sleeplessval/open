use std::{
	fs::read_to_string,
	env::{current_dir, var},
	path::Path
};

use configparser::ini::Ini;

pub struct Config {
	pub local: Option<Ini>,
	pub global: Option<Ini>,
	pub local_path: Option<String>,
	pub global_path: Option<String>
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
	pub fn get(&self, section: &str, key: &str) -> Option<String> {
		let mut output: Option<String> = None;
		if self.local.is_some() {
			output = self.local.as_ref().unwrap().get(section, key);
		}
		if output.is_none() && self.global.is_some() {
			output = self.global.as_ref().unwrap().get(section, key);
		}
		return output;
	}
	pub fn getbool(&self, section: &str, key: &str) -> Option<bool> {
		let mut output = None;
		if self.local.is_some() {
			let i_out = self.local.as_ref().unwrap().getbool(section, key);
			output = i_out.unwrap_or(None);
		}
		if output.is_none() && self.global.is_some() {
			let i_out = self.global.as_ref().unwrap().getbool(section, key);
			output = i_out.unwrap_or(None);
		}
		return output;
	}
	pub fn add(&mut self, section: &str, key: &str, value: String) {
		let mut ini: Ini;
		let local = self.local.is_some();
		if local {
			ini = self.local.clone().unwrap();
		} else {
			ini = self.global.clone().unwrap();
		}
		ini.set(section, key, Some(value));
		if local {
			self.local = Some(ini);
		} else {
			self.global = Some(ini);
		}
	}
	pub fn add_global(&mut self, section: &str, key: &str, value: String) {
		let mut global = self.global.clone().unwrap();
		global.set(section, key, Some(value));
		self.global = Some(global);
	}
	pub fn write(&self) -> std::io::Result<()> {
		let mut path = self.local_path.as_ref();
		if path.is_some() {
			let result = self.local.as_ref().unwrap().write(path.unwrap().as_str());
			return result;
		}
		path = self.global_path.as_ref();
		self.global.as_ref().unwrap().write(path.unwrap().as_str())
	}
}
