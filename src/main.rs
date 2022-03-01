//use std::{thread::sleep, time::Duration};

use std::{thread::sleep, time::Duration};

use rand_xoshiro::Xoshiro256PlusPlus;
use tlennis_data::TlennisData;
use match_data::{Match};
use rand::{SeedableRng, Rng};

mod tlennis_data;
mod player_data;
mod match_data;
mod game_data;
mod set_data;



fn main() {
	let mut rng = Xoshiro256PlusPlus::from_entropy();
	
	let mut tlennis_data = TlennisData::new_from_file();
	let a1 = rng.gen_range(0..tlennis_data.player_order.len());
	let a2 = rng.gen_range(0..tlennis_data.player_order.len());
	let p1 = tlennis_data.player_order[a1];
	let  p2 = tlennis_data.player_order[a2];
	let mut s1 = Match::new(p1, p2);
	let mut home_wins = 0;
	let mut away_wins = 0;
	for _ in 0..500 {
		loop {
			s1.process(&mut tlennis_data);
			if s1.queue_for_deletion {
				if s1.home_wins >= 2 { 
					home_wins += 1;
				} else {
					away_wins += 1;
				}
				s1 = Match::new(p1, p2);
				break;
			}
			sleep(Duration::from_millis(800));
		}
	}
	let p = &tlennis_data.players[&p1];
	println!("{}: Speed: {}, Power: {}, Acc: {}", p.fullname(), p.speed, p.power, p.accuracy);
	/*let p = &tlennis_data.players[&p2];
	println!("{}: Speed: {}, Power: {}, Acc: {}", p.fullname(), p.speed, p.power, p.accuracy);*/
	println!("{}-{}", home_wins, away_wins);
	tlennis_data.save_to_file();
}
