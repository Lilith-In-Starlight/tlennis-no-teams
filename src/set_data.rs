use crate::{game_data::Game, tlennis_data::TlennisData, game_data::GameStates};

pub struct Set {
	pub game: Game,
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
			game: Game::default(),
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
			game: Game::new(home, away),
			..Default::default()
		}
	}

	pub fn process(&mut self, tlennis_data: &mut TlennisData) {
		if self.commentary.len() == 0 {
			if self.current_game < 3 {
				self.game.process(tlennis_data);
				self.commentary.append(&mut self.game.commentary);
				match self.game.state {
					GameStates::Ended => {
						self.current_game += 1;
						if self.home_wins < 6 && self.away_wins < 6 {
							self.commentary.push(format!("Starting game {}!", self.current_game + 1));
							if self.game.home_score > self.game.away_score {
								self.home_wins += 1;
							} else {
								self.away_wins += 1;
							}
							self.game = Game::new(self.away_id, self.home_id);
						} else {
							let winner_id: usize = if self.home_wins >= 6 { self.home_id } else { self.away_id };
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