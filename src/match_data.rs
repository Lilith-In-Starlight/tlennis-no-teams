use crate::{set_data::Set, tlennis_data::TlennisData};

pub struct Match {
	pub away_id: usize,
	pub home_id: usize,
	pub set: Set,
	pub away_wins: isize,
	pub home_wins: isize,
	pub commentary: Vec<String>,
}


impl Default for Match {
	fn default() -> Self {
		Self {
			away_id: 0,
			home_id: 0,
			set: Set::default(),
			away_wins: 0,
			home_wins: 0,
			commentary: Vec::from([String::from("Match start!"), String::from("Set 1 start!")]),
		}
	}
}

impl Match {
	pub fn new(home: usize, away: usize) -> Self {
		Self {
			home_id: home,
			away_id: away,
			set: Set::new(home, away),
			..Default::default()
		}
	}

	pub fn process(&mut self, tlennis_data: &mut TlennisData) {
		
	}
}