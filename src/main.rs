use std::io;
use rand::Rng;

fn main() {
	
	println!("Welcome to RPG!");
	println!("by KMR");
	println!("--------------------");

	//Promts user for name
	println!("Please type a name for your Warrior: ");
	let temp_name = input_name();
	
	//Instantiates player
	let mut player = build_player(temp_name, 0, 0, 0, 0);
	
	println!("{}'s stats are:", player.name);
	
	//Creates a stats list
	let mut stats = build_stats();
	print_stats(&stats);
		
	
	
	
}

struct Stats {
	strength: u8,
	dex: u8,
	con: u8,
	int: u8,
	wis: u8,
	cha: u8,
}
	

struct ItemSlot {
	name: String,
	amount: u8,
}

struct Player {
	name: String,
	hit_points: i16,
	damage: i16,
	armor_class: i8,
	speed: i8,
}

struct Creature {
	name: String,
	hit_points: i16,
	damage: i16,
	armor_class: i8,
	speed: i8,
}

fn input_name() -> String {
	let mut name = String::new();
	
	io::stdin().read_line(&mut name).expect("Failed to read line");
	
	//Truncates the endline character
	let len = name.len();
	name.truncate(len - 1);
	
	name
}

fn roll_stat() -> u8 {
	let r1 = rand::thread_rng().gen_range(1, 6);
	let r2 = rand::thread_rng().gen_range(1, 6);
	let r3 = rand::thread_rng().gen_range(1, 6);
	let r4 = rand::thread_rng().gen_range(1, 6);
	
	let mut rolls = [r1, r2, r3, r4];
	
	rolls.sort_unstable();
	
	rolls[1] + rolls[2] + rolls[3]
}

fn build_stats() -> Stats {
	Stats {
		strength: roll_stat(),
		dex: roll_stat(),
		con: roll_stat(),
		int: roll_stat(),
		wis: roll_stat(),
		cha: roll_stat(),
	}
}

fn print_stats(stats: &Stats) {
	println!("STR: {}", stats.strength);
	println!("DEX: {}", stats.dex);
	println!("CON: {}", stats.con);
	println!("INT: {}", stats.int);
	println!("WIS: {}", stats.wis);
	println!("CHA: {}", stats.cha);
}

fn build_player(name: String, hit_points: i16, damage: i16, armor_class: i8, speed: i8)
	-> Player {
		Player {
			name,
			hit_points,
			damage, 
			armor_class,
			speed,
		}
	}
	
	




