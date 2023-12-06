use std::time::Instant;

use regex::Regex;
use tokio::{
	fs::{read_to_string, write},
	process::Command,
};

use crate::utils::into_lines;

pub async fn build_bin(day: u64, input: String) {
	let fn_name = get_fn_name(day).await;

	new_project(day).await;
	write_main_file(&fn_name, input).await;
	build(day).await;
	cleanup().await;
	execute(day).await;
}

async fn get_fn_name(day: u64) -> String {
	let this_file = read_to_string("main.rs").await.unwrap();
	let regex = Regex::new(r"^\s*(\d+)\s*=>\s*(\w+)\(input\)\s*,\s*$").unwrap();

	let fn_name = into_lines(this_file).iter().find_map(|line| match regex.captures(&line) {
		Some(capture) => {
			if capture.get(1).unwrap().as_str() == &day.to_string() {
				Some(capture.get(2).unwrap().as_str().to_owned())
			} else {
				None
			}
		}
		None => None,
	});

	if fn_name.is_none() {
		panic!("Couldn't find a function name for day {day}")
	}

	fn_name.unwrap()
}

async fn new_project(day: u64) {
	Command::new("cargo")
		.args(&["new", &format!("day_{day}_bin")])
		.spawn()
		.unwrap()
		.wait()
		.await
		.unwrap();
	Command::new("mv")
		.args(&["day_5_bin", "build"])
		.spawn()
		.unwrap()
		.wait()
		.await
		.unwrap();
	Command::new("cargo")
		.args(&["add", "--path", "../"])
		.current_dir("build")
		.spawn()
		.unwrap()
		.wait()
		.await
		.unwrap();
}

async fn write_main_file(fn_name: &str, input: String) {
	let import = format!("use advent_of_code_2023::{{{fn_name}}};");
	let input_code = format!("let input = \"{input}\".to_owned();");
	let call = format!("{fn_name}(input);");
	let main = format!("{import}\n\nfn main() {{\n\t{input_code}\n\t{call}\n}}\n");

	write("build/src/main.rs", main).await.unwrap();
}

async fn build(day: u64) {
	Command::new("cargo")
		.args(&["build", "--release"])
		.current_dir("build")
		.spawn()
		.unwrap()
		.wait()
		.await
		.unwrap();

	Command::new("mv")
		.args(&[format!("build/target/release/day_{day}_bin"), format!("target/day_{day}_bin")])
		.spawn()
		.unwrap()
		.wait()
		.await
		.unwrap();
}

async fn cleanup() {
	Command::new("rm").args(&["-rf", "build"]).spawn().unwrap().wait().await.unwrap();
}

async fn execute(day: u64) {
	let file = format!("target/day_{day}_bin");
	let mut child = Command::new(&file).spawn().unwrap();

	let start = Instant::now();
	child.wait().await.unwrap();

	let time = start.elapsed().as_millis();
	println!("Executed {file} in {time}ms");
}
