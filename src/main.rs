extern crate time;
extern crate regex;
extern crate itertools;

use time::PreciseTime;
use time::Duration;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use regex::Regex;
use itertools::*;

fn tc<T, F: FnOnce() -> T>(f: F) -> (T, Duration) {
	let start = PreciseTime::now();
	let res = f();
	(res, start.to(PreciseTime::now()))
}

#[derive(Debug)]
enum Error {
	Io(std::io::Error),
	Regex(regex::Error)
}

impl From<std::io::Error> for Error {
	fn from(e: std::io::Error) -> Error {
		Error::Io(e)
	}
}

impl From<regex::Error> for Error {
	fn from(e: regex::Error) -> Error {
		Error::Regex(e)
	}
}

trait Iter<T> {
	fn iter<F: FnOnce(T) -> ()>(self, f: F);
}

impl<T> Iter<T> for Option<T> {
	fn iter<F: FnOnce(T) -> ()>(self, f: F) {
		match self {
			Some(x) => f(x),
			None => ()
		}
	}
}

fn search(file: &str, pattern: &str) -> Result<Vec<String>, Error> {
	let r = try!(Regex::new(pattern));
	let f = try!(File::open(file));
	let buff = BufReader::new(f);
	buff
	.lines()
	.take(10_000_000)
	.fold_results(Vec::new(), |mut acc, line| {
		r
		.captures(&line)
		.and_then(|caps| caps.name("name"))
		.iter(|name| {
			let name =
				name
				.split(|c| c == ' ' || c == ';' || c == '"')
				.filter(|x| !x.is_empty())
				.join(".");
			acc.push(name.to_string())
		});
		acc })
	.map_err(Error::Io)
}

fn main() {
    let (res, elapsed) = tc(|| {
		search("d:\\big.txt", r"\{.*(?P<name>Microsoft.*)\|\]").unwrap()
    });
	//println!("Res: {:?}", res);
    println!("Res count = {}, Elapsed {}", res.len(), elapsed);
}
