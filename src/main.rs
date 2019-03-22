use std::io;
use rand::Rng;

fn main() {
	
	print_line_break();
	println!("Welcome to RPG!");
	println!("by KMR");
	print_line_break();

	//Promts user for name
	println!("Please type a name for your Warrior: ");
	let mut temp_name = input_name();
	
	//Instantiates player
	//(name: String, hit_points: i16, damage: i16, armor_class: i8, speed: i8, gold: u16)
	let mut player = build_player(temp_name.trim().to_string(), 12, 0, 0, 0, 10);
	
	
	print_line_break();
	
	//Builds and displays stats
	let mut stats = build_stats();
	println!("{}'s stats are: ", player.name);
	print_stats(&stats);
	
	//Asks player if they wish to keep stats, if not, rerolls
	print_line_break();
	
	let mut keep_stats = false;
	
	println!("Keep these stats? (Y) = Yes, keep them (N) = No, reroll them");
	keep_stats = yes_or_no();
	
	//If player answers no, rerolls and prompts again
	while keep_stats == false {
		stats = build_stats();
		
		print_stats(&stats);
		println!("Keep these stats? (Y) = Yes, keep them (N) = No, reroll them");
		
		keep_stats = yes_or_no();
	}
	
	//Converts and applies stat modifiers to player
	apply_stat_mods(&mut stats, &mut player);
	
	//debug_player(&mut player);
	
	//Player Init Complete------------------------------------
	
	//Gameplay starts-----------------------------------------
	print_line_break();
	print_line_break();
	println!("You arrive in town");
	
	//loc_town();
	
	
		
	
	
	
	
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
	gold: i32,
}

struct Creature {
	name: String,
	hit_points: i16,
	damage: i16,
	armor_class: i8,
	speed: i8,
}



fn print_line_break() {
	println!(" ");
	println!("--------------------");
	println!(" ");
}

fn input_name() -> String {
	let mut name = String::new();
	
	io::stdin().read_line(&mut name).expect("Failed to read line");
	//assert_eq! (name.pop(), Some('\n'));	
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


fn build_player(name: String, hit_points: i16, damage: i16, armor_class: i8, speed: i8, gold: i32)
	-> Player {
		Player {
			name,
			hit_points,
			damage, 
			armor_class,
			speed,
			gold,
		}
	}
	
fn yes_or_no() -> bool {
		
		let mut input = String::new(); 
		
		io::stdin().read_line(&mut input);
		
		if input.trim() == "Y" || input.trim() == "y" {
			true
		}
		else {
			false
		}
		
	}
	
fn stats_to_mods(stat: u8) -> i8 {
	
	let modifier = match stat {
		2 | 3   => -4,
		3 | 4   => -3,
		5 | 6   => -2,
		7 | 8   => -1,
		9 | 10  => 0,
		11 | 12 => 1,
		13 | 14 => 2, 
		15 | 16 => 3,
		17 | 18 => 4,
		19 | 20 => 5,
		_        => 0,
		};
		
		modifier
	
}

fn apply_stat_mods(stats: &mut Stats, player: &mut Player) {
	//(name: String, hit_points: i16, damage: i16, armor_class: i8, speed: i8, gold: i32)
	player.damage += stats_to_mods(stats.strength) as i16;
	player.speed += stats_to_mods(stats.dex) as i8;
	player.armor_class += player.speed;
	player.hit_points += stats_to_mods(stats.con) as i16;
	player.gold += (stats_to_mods(stats.cha) * 5) as i32;
}

fn debug_player(player: &mut Player) {
	println!("Name: {}", player.name);
	println!("HP: {}", player.hit_points);
	println!("Damage: {}", player.damage);
	println!("AC: {}", player.armor_class);
	println!("Speed: {}", player.speed);
	println!("Gold: {}", player.gold);
}


