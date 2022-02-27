use rand_xoshiro::Xoshiro256PlusPlus;
use serde::{Serialize, Deserialize};
use rand::{Rng, SeedableRng};
use once_cell::sync::Lazy;
use std::fs;

static NAME_LIST: Lazy<Vec<&str>> = Lazy::new(||{
	let contents: &'static str = Box::leak(fs::read_to_string("names.txt").expect("Something went wrong with names.txt").into_boxed_str());
	let result: Vec<&str> = contents.split("\r\n").collect();
	result
});

static SURNAME_LIST: Lazy<Vec<&str>> = Lazy::new(||{
	let contents: &'static str = Box::leak(fs::read_to_string("surnames.txt").expect("Something went wrong with surnames.txt").into_boxed_str());
	let result: Vec<&str> = contents.split("\r\n").collect();
	result
});

static GENERAL_NAME_LIST: Lazy<Vec<&str>> = Lazy::new(||{
	let contents: &'static str = Box::leak(fs::read_to_string("general.txt").expect("Something went wrong with general.txt").into_boxed_str());
	let result: Vec<&str> = contents.split("\r\n").collect();
	result
});


#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
	pub id: usize,
	pub firstname: String,
	pub middlename: String,
	pub lastname: String,
	pub speed: f32,
	pub strength: f32,
	pub wins: f32,
	pub losses: f32,
	pub mods: Vec<String>,
	pub location: String,
}

impl Default for Player {
	fn default() -> Self {
		let name = generate_name();
		let mut rng = Xoshiro256PlusPlus::from_entropy();
		Self {
			id: rng.gen(),
			firstname: name.0,
			middlename: name.1,
			lastname: name.2,
			speed: rng.gen_range(0.0..6.0),
			strength: rng.gen_range(0.0..6.0),
			..Default::default()
		}
	}
}

impl Player {
	pub fn new() -> Player {
		Player::default()
	}
}

fn generate_name() -> (String, String, String) {
	(pick_name(&NAME_LIST).to_string(), pick_midname(), pick_surname())
}


fn pick_name<'a>(list: &'a Vec<&str>) -> &'a str {
	let full_len = list.len() + GENERAL_NAME_LIST.len();
	let mut rng = Xoshiro256PlusPlus::from_entropy();
	let n = rng.gen_range(0..full_len);
	if n < list.len() {
		list[n]
	} else {
		GENERAL_NAME_LIST[n - list.len()]
	}
	
}
fn pick_surname() -> String {
	let mut rng = Xoshiro256PlusPlus::from_entropy();
	let mut base = pick_name(&SURNAME_LIST).to_string();
	base = {
		match rng.gen_range(0..20) {
			0 => format!("Mc{}", base),
			1 => format!("O'{}", base),
			_ => base,
		}
	};
	base = {
		match rng.gen_range(0..20) {
			0 => format!("{}son", base),
			_ => base,
		}
	};

	base
}
fn pick_midname() -> String {
	let mut rng = Xoshiro256PlusPlus::from_entropy();
	let mut base = pick_name(&NAME_LIST).to_string();
	base = {
		match rng.gen_range(0..20) {
			0 => format!("{}man", base),
			1 => format!("{}boi", base),
			2 => format!("{}boy", base),
			_ => base,
		}
	};

	base
}