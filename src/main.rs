use std::{thread::sleep, time::Duration};

use rand::{SeedableRng, Rng};
use rand_xoshiro::Xoshiro256PlusPlus;
use tlennis_data::TlennisData;
use set_data::Set;

mod tlennis_data;
mod player_data;
mod match_data;
mod game_data;
mod set_data;



fn main() {
	let mut rng = Xoshiro256PlusPlus::from_entropy();
	
	let mut data = TlennisData::new_from_file();
	let p1 = data.player_order[rng.gen_range(0..data.player_order.len())];
	let p2 = data.player_order[rng.gen_range(0..data.player_order.len())];
	let mut s1 = Set::new(p1, p2);
	loop {
		s1.process(&mut data);
		sleep(Duration::from_millis(10));
		if s1.current_game >= 3 && s1.commentary.len() == 0 {
			break;
		}
	}

	data.save_to_file();
}
