use std::{thread::sleep, time::Duration};

use tlennis_data::TlennisData;
use game_data::Game;

mod tlennis_data;
mod player_data;
mod game_data;



fn main() {
	let mut data = TlennisData::new_from_file();
	let p1 = data.player_order[1];
	let p2 = data.player_order[5];

	let mut g1 = Game::new(p1, p2);
	loop {
		g1.process(&mut data);
		sleep(Duration::from_millis(500));
		match g1.state {
			game_data::GameStates::Ended => break,
			_ => (),
		}
	}

	data.save_to_file();
}
