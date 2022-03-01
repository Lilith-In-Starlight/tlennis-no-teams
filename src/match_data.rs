use crate::{set_data::Set, tlennis_data::TlennisData};

pub struct Match {
	pub away_id: usize,
	pub home_id: usize,
	pub set: Set,
	pub current_set: usize,
	pub away_wins: isize,
	pub home_wins: isize,
	pub commentary: Vec<String>,
	pub state: MatchState,
	pub queue_for_deletion: bool,
}

pub enum MatchState {
	StartingSet,
	OngoingSet,
	EndedMatch,
}


impl Default for Match {
	fn default() -> Self {
		Self {
			away_id: 0,
			home_id: 0,
			set: Set::default(),
			current_set: 0,
			away_wins: 0,
			home_wins: 0,
			commentary: Vec::from([String::from("Match start!"), String::from("Set 1 start!")]),
			state: MatchState::StartingSet,
			queue_for_deletion: false,
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
		match self.state {
			MatchState::StartingSet => {
				self.commentary.push(format!("== STARTING SET {}! ({}-{}) ==", self.current_set + 1, self.home_wins, self.away_wins));
				self.state = MatchState::OngoingSet;
			},
			MatchState::OngoingSet => {
				self.set.process(tlennis_data);
				self.commentary.append(&mut self.set.commentary);
				if self.set.queue_for_deletion {
					self.current_set += 1;
					if self.set.home_wins > self.set.away_wins {
						self.home_wins += 1;
					} else {
						self.away_wins += 1
					}
					if self.home_wins < 2 && self.away_wins < 2 {
						self.set = Set::new(self.home_id, self.away_id);
						self.state = MatchState::StartingSet;
					} else {
						let winner_id: usize = if self.home_wins >= 2 { self.home_id } else { self.away_id };
						let winner_name = tlennis_data.players[&winner_id].fullname();
						self.commentary.push(format!("Match end! {} wins! ({}-{})", winner_name, self.home_wins, self.away_wins));
						self.state = MatchState::EndedMatch;
					}
				}
			},
			MatchState::EndedMatch => {
				if self.commentary.len() == 0 {
					self.queue_for_deletion = true;
				}
			}
		}
		if self.commentary.len() > 0 {
			println!("{}", self.commentary[0]);
			self.commentary.remove(0);
		}
	}
}