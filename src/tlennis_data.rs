use std::{collections::HashMap, fs, path::Path};
use serde::{Serialize, Deserialize};

use crate::player_data::Player;

const TLENNIS_DATA_PATH: &str = "tlennnis_data.txt";


#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct TlennisData {
	pub players: HashMap<usize, Player>,
	pub player_order: Vec<usize>,
	pub dead_players: Vec<usize>,
}

impl TlennisData {
	pub fn new() -> Self {
		let mut p: HashMap<usize, Player> = HashMap::new();
		let mut p_o: Vec<usize> = Vec::new();
		for _ in 0..24 {
			let player = Player::new();
			p.insert(player.id, player.clone());
			p_o.push(player.id);
		}
		Self {
			players: p,
			player_order: p_o,
			dead_players: Vec::new(),
		}
	}

	pub fn new_from_file() -> Self {
		if Path::new(TLENNIS_DATA_PATH).exists() {
			let file = fs::read_to_string(TLENNIS_DATA_PATH).unwrap();
			let decoded: Self = serde_json::from_str(&file).unwrap();
			decoded
		} else {
			Self::new()
		}
	}

	pub fn save_to_file(&self) {
        let encoded = serde_json::to_string(&self).unwrap();
        fs::write("league_data.txt", &encoded).unwrap();
	}
}

impl Default for TlennisData {
	fn default() -> Self {
		Self::new()
	}
}