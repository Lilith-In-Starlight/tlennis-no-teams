use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::tlennis_data::{TlennisData};

pub enum GameStates {
	Serving,
	Playing,
	Ended,
}

enum HitResults {
	Hit,
	NotHit,
	Beyond,
}

pub struct Game {
	pub home_id: usize,
	pub away_id: usize,
	pub home_pos: u64,
	pub away_pos: u64,
	pub ball_pos: u64,
	pub ball_in_home: bool,
	pub state: GameStates,
	pub away_score: u64,
	pub home_score: u64,
	pub commentary: Vec<String>,
}

const SPEED_MULT: f32 = 12.0;
const ACC_MULT: f32 = 1.8;


impl Game {
	pub fn new(away: usize, home: usize) -> Self {
		Self {
			home_id: home,
			away_id: away,
			state: GameStates::Serving,
			..Default::default()
		}
	}
	pub fn tennisfy(s: u64) -> String {
		match s {
			0 => "Love".to_string(),
			1 => "15".to_string(),
			2 => "30".to_string(),
			3 | 4 => "40".to_string(),
			_ => s.to_string(),
		}
	}
	pub fn tennis_score(&self) -> String {
		if self.home_score != self.away_score {
			format!("{}-{}", Game::tennisfy(self.home_score), Game::tennisfy(self.away_score))
		} else {
			if self.home_score == 3 {
				"Deuce".to_string()
			} else {
				format!("{}-All", Game::tennisfy(self.home_score))
			}
		}
	}
	pub fn process(&mut self, tlennis_data: &TlennisData) {
		match self.state {
			GameStates::Serving => {
				let hit = self.hit_ball(tlennis_data, true);
				self.manage_hit_result(tlennis_data, hit);
			},
			GameStates::Playing => {
				let hit = self.hit_ball(tlennis_data, false);
				self.manage_hit_result(tlennis_data, hit);

				if self.away_score > 3 || self.home_score > 3 {
					let winner_id: usize = if self.home_score > self.away_score { self.home_id } else { self.away_id };
					let winner_name = tlennis_data.players[&winner_id].fullname();
					self.commentary.push(format!("Game end! {} wins!", winner_name));
					self.state = GameStates::Ended;
				}
			},
			GameStates::Ended => todo!("Ending State"),
		}
	}


	fn hit_ball(&mut self, tlennis_data: &TlennisData, serve:bool) -> HitResults {
		/* This function processes a hit; be it a serve or not. Its result is then passed to
		a function that handles scoring and other things. Variable names are written from the perspective
		of the player that is about to hit it; they are the hitting player. The other plyer sent them the ball */

		let mut rng = Xoshiro256PlusPlus::from_entropy();
		let hitting_player_id: usize = if self.ball_in_home { self.home_id } else { self.away_id };
		let sending_player_id: usize = if self.ball_in_home { self.away_id } else { self.home_id };
		let hitting_player_pos = if self.ball_in_home { self.home_pos } else { self.away_pos };
		let sending_player_pos = if self.ball_in_home { self.away_pos } else { self.home_pos };

		let hitting_player_speed: f32 = tlennis_data.players[&hitting_player_id].speed;
		let hitting_player_accuracy: f32 = tlennis_data.players[&hitting_player_id].accuracy;
		let sending_player_power: f32 = tlennis_data.players[&sending_player_id].power;
		let sending_player_accuracy: f32 = tlennis_data.players[&sending_player_id].accuracy;

		let hp_b_dist = (hitting_player_pos as f32 - self.ball_pos as f32).abs();
		let p_space = hitting_player_speed * SPEED_MULT + sending_player_power + hp_b_dist * 3.0;
		let p = rng.gen_range(0.0..p_space);

		let hitting_player_name = tlennis_data.players[&hitting_player_id].fullname();

		// The player hits the ball.
		// This must always happen if the hit is a serve
		if p < hitting_player_speed * SPEED_MULT || serve {
			if serve {
				self.commentary.push(format!("{} serves!", hitting_player_name));
			} else {
				self.commentary.push(format!("{} hits the ball!", hitting_player_name));
			}
			self.ball_in_home = !self.ball_in_home;
			if rng.gen_range(0.0..6.0) < hitting_player_accuracy {
				self.ball_pos = { 
					match sending_player_pos {
						0 => 2,
						2 => 0,
						_ => rng.gen_range(0..=2),
					}
				};
			} else {
				match rng.gen_range(0..=1) {
					0 => self.ball_pos = (self.ball_pos + 1) % 3,
					_ => (),
				}
			}
			HitResults::Hit
		} else { // The player failed to hit the ball
			let p_space = sending_player_power + sending_player_accuracy * ACC_MULT;
			let p = rng.gen_range(0.0..p_space);
			// Did they fail because the other player hit it too hard?
			if p < sending_player_power {
				self.commentary.push(format!("The ball flies into the great beyond!!!"));
				self.ball_in_home = !self.ball_in_home;
				HitResults::Beyond
			// Or was it because they simply couldn't get to it?
			} else {
				self.commentary.push(format!("{} fails to hit the ball!", hitting_player_name));
				self.ball_in_home = !self.ball_in_home;
				HitResults::NotHit
			}
		}
	}

	fn manage_hit_result(&mut self, tlennis_data: &TlennisData, result: HitResults) {
		/* This function manages what happens after the ball is hit. Scoring and stuff. It is written from
		the perspective of the person who hits the ball. They hit it, the other receives it */
		let hitting_player_id: usize = if self.ball_in_home { self.away_id } else { self.home_id };
		let receiving_player_id: usize = if self.ball_in_home { self.home_id } else { self.away_id };
		let hitting_player_name = tlennis_data.players[&hitting_player_id].fullname();
		let receiving_player_name = tlennis_data.players[&receiving_player_id].fullname();

		let hitter_score;
		let receiver_score;
		if self.ball_in_home { 
			hitter_score = &mut self.away_score;
			receiver_score = &mut self.home_score;
		} else { 
			hitter_score = &mut self.home_score;
			receiver_score = &mut self.away_score;
		}
		
		match result {
			HitResults::Hit => self.state = GameStates::Playing,
			HitResults::NotHit => {
				*hitter_score += 1;
				self.state = GameStates::Serving;
				self.commentary.push(format!("{} scores! ({}) scores!", hitting_player_name, self.tennis_score()));
				self.commentary.push("".to_string());
			},
			HitResults::Beyond => {
				*receiver_score += 1;
				self.state = GameStates::Serving;
				self.commentary.push(format!("{} scores! ({}) scores!", receiving_player_name, self.tennis_score()));
				self.commentary.push("".to_string());
			},
		}
	}
}


impl Default for Game {
	fn default() -> Self {
		Self {
			home_id: 0,
			away_id: 1,
			home_pos: 1,
			away_pos: 1,
			ball_pos: 1,
			state: GameStates::Ended,
			ball_in_home: true,
			home_score: 0,
			away_score: 0,
			commentary: Vec::new(),
		}
	}
}