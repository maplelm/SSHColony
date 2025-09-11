#![allow(dead_code, unused_variables)]

use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use libc::{localtime, strftime, tm};
use std::ffi::CStr;

pub type Time = Duration;
pub type Size = u64;

pub const DAY: Time = std::time::Duration::new(86400, 0);
pub const WEEK: Time = std::time::Duration::from_secs(DAY.as_secs()*7);
pub const MONTH: Time = std::time::Duration::from_secs(WEEK.as_secs()*4);
pub const YEAR: Time = std::time::Duration::from_secs(DAY.as_secs()*365);

pub const KB: Size = 1024;
pub const MB: Size = KB * 1024;
pub const GB: Size = MB * 102;
pub const TB: Size = GB * 1024;

#[repr(u8)]
pub enum LogLevel {
	None,
	Debug,
	Info,
	Warn,
	Err,
}

pub struct Logger {
	max_age: Option<Duration>,
	max_size: Option<u64>,
	path : String,
	base_name : String,
	file : Option<File>,
	current_name: String,
	level: LogLevel
}

impl Logger {
	pub fn init(age: Option<Duration>, size: Option<Size>, path: String, name: String, lvl: LogLevel) -> Self {
		Self {
			max_age: age,
			max_size: size,
			path: path,
			base_name: name,
			file : None,
			current_name: String::new(),
			level: lvl,
		}
	}

	pub fn write(&mut self, msg: String) -> Result<usize, std::io::Error> {

		match self.rotate_check() {
			Ok(b) => {
				if b {
					if let Err(e) = self.rotate_file() {
						return Err(e);
					}
				}
				return self.file.as_ref().unwrap().write(msg.as_bytes());
			},
			Err(e) => Err(e)
		}
	}

	fn rotate_check(&self) -> Result<bool, io::Error> {
		// No file has been opened so you would call the rotate function to open a file 
		if self.current_name.len() == 0 {
			return Ok(true);
		}
		match std::fs::metadata(&self.current_name) {
			Ok(md) => {
				if !self.max_age.is_none() {
					match md.created() {
						Ok(c) => {
							if c.elapsed().unwrap() >= self.max_age.unwrap() {
								return Ok(true);
							}
						},
						Err(e) => {return Err(e);}
					}
				}
				if !self.max_size.is_none() {
					if self.max_size.unwrap() >= md.size() {
						return Ok(true);
					}
				}
				Ok(false)
			}
			Err(e) => Err(e)
		}
	}

	fn rotate_file(&mut self) -> Result<(), io::Error> {
		let fp: String = format!("{}/{} {}.log", self.path, current_date_str(), self.base_name);

		match std::fs::OpenOptions::new().append(true).create(true).open(&fp) {
			Ok(f) => {
				self.file = Some(f);
				self.current_name = fp;
				return Ok(());
			}
			Err(e) => Err(e)
		}
	}

	pub fn set_level(&mut self, lvl: LogLevel) {
			self.level = lvl;
	}

	pub fn debug(&mut self, msg: String) -> Result<usize, std::io::Error> {
		match self.level {
			LogLevel::Err | LogLevel::Warn | LogLevel::Info | LogLevel::None => Ok(0),
			_ => {
				let s = current_date_str() + String::from(":Info: ").as_str() + msg.as_str();
				self.write(s)
			}
		}
	}

	pub fn info (&mut self, msg: String) -> Result<usize, std::io::Error> {
		match self.level {
			LogLevel::Err | LogLevel::Warn | LogLevel::None => Ok(0),
			_ => {
				let s = current_date_str() + String::from(":Info: ").as_str() + msg.as_str();
				self.write(s)
			}
		}
	}

	pub fn warn (&mut self, msg: String) -> Result<usize, std::io::Error> {
		match self.level {
			LogLevel::Err | LogLevel::None => Ok(0),
			_ => {
				let s = current_date_str() + String::from(":Warning: ").as_str() + msg.as_str();
				self.write(s)
			}
		}	
	}

	pub fn err (&mut self, msg: String) -> Result<usize, std::io::Error> {
		match self.level {
			LogLevel::None => Ok(0),
			_ => {
				let s = current_date_str() + String::from(":Error: ").as_str() + msg.as_str();
				self.write(s)
			}
		}
	}
}

fn current_date_str() -> String {
	let mut time_buffer: [u8; 20] = [0; 20];
	let format: &'static str = "%d-%m-%Y %H:%M:%S\0";
	let t_now: SystemTime = SystemTime::now();
	let epoch: i64 = (t_now.duration_since(UNIX_EPOCH).unwrap().as_secs()) as i64;
	unsafe {
		let time_ptr: *mut tm = localtime(&epoch);
		let tm_struct: tm = *time_ptr;
		strftime(time_buffer.as_mut_ptr() as *mut i8, time_buffer.len(),
			format.as_ptr() as *const i8,
			&tm_struct);
		String::from(CStr::from_ptr(time_buffer.as_ptr() as *const i8).to_str().unwrap())
	}
}