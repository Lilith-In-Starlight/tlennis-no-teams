use crate::{game_data::Game, tlennis_data::TlennisData, game_data::GameStates};

pub struct Set {
	pub games: Vec<Game>,
	pub current_game: usize,
	pub away_id: usize,
	pub home_id: usize,
	pub away_wins: isize,
	pub home_wins: isize,
	pub commentary: Vec<String>,
}

impl Default for Set {
	fn default() -> Self {
		Self {
			games: Vec::new(),
			current_game: 0,
			away_id: 0,
			home_id: 0,
			away_wins: 0,
			home_wins: 0,
			commentary: Vec::from(["Starting game 1!".to_string()]),
		}
	}
}

impl Set {
	pub fn new(home: usize, away: usize) -> Self {
		Self {
			home_id: home,
			away_id: away,
			games: Vec::from([Game::new(away, home), Game::new(away, home), Game::new(away, home)]),
			..Default::default()
		}
	}

	pub fn process(&mut self, tlennis_data: &mut TlennisData) {
		if self.commentary.len() == 0 {
			if self.current_game < 3 {
				let c_game = &mut self.games[self.current_game];
				c_game.process(tlennis_data);
				self.commentary.append(&mut c_game.commentary);
				match c_game.state {
					GameStates::Ended => {
						self.current_game += 1;
						if self.current_game < 3 {
							self.commentary.push(format!("Starting game {}!", self.current_game + 1));
						} else {
							let winner_id: usize = if self.home_wins > self.away_wins { self.home_id } else { self.away_id };
							let winner_name = tlennis_data.players[&winner_id].fullname();
							self.commentary.push(format!("Set end! {} wins!", winner_name));
						}
					},
					_ => (),
				}
			}
		} else {
			println!("{}", self.commentary[0]);
			self.commentary.remove(0);
		}
	}
}