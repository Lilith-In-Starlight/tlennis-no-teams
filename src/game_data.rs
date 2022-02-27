use crate::vectors::{Vec2, Vec3};

pub enum GameStates {
	Serving,
	Playing,
	Ended,
}

pub struct Game {
	pub home_id: usize,
	pub away_id: usize,
	pub home_pos: Vec2,
	pub away_pos: Vec2,
	pub ball_pos: Vec3,
	pub state: GameStates,
}


impl Game {
	pub fn new() -> Self {
		Self {
			state: GameStates::Serving,
			..Default::default()
		}
	}
	pub fn process(&self) {
		match self.state {
			GameStates::Serving => todo!("Serving State"),
			GameStates::Playing => todo!("Playing State"),
			GameStates::Ended => todo!("Ending State"),
		}
	}
}

impl Default for Game {
	fn default() -> Self {
		Self {
			home_id: 0,
			away_id: 1,
			home_pos: Vec2::new(-6.0, 0.0),
			away_pos: Vec2::new(6.0, 0.0),
			ball_pos: Vec3::new(-6.0, 0.0, 0.0),
			state: GameStates::Ended,
		}
	}
}