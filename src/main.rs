use std::io;
use rand::Rng;

fn main() {
	
	/*-------------Init--------------*/
	
	let mut player = Player	{ Default::default() };
	print_line_break();
	println!("Welcome to RPG!");
	println!("by KMR");
	print_line_break();

	//Promts user for name
	println!("Please type a name for your Warrior: ");
	
	let temp_name = input_name();
	
	player.name = temp_name.trim().to_string();
	
	print_line_break();
	
	//Init player stats
	let mut stats = Stats { strength: 0, dex: 0, con: 0, int: 0, wis: 0, cha: 0 };
	//Bool to break loop
	let mut keep_stats = false;
	
	
	while keep_stats == false {
		stats = build_stats();
		
		println!("{}'s stats are: ", player.name);
		print_stats(&stats);
		
		//Asks player if they wish to keep stats, if not, rerolls
		print_line_break();
		
		println!("Keep these stats? (Y) = Yes, keep them (N) = No, reroll them");
		
		keep_stats = yes_or_no();
		
		//If player answers no, rerolls and prompts again
	}
	
	//Converts and applies stat modifiers to player
	level_up(&mut stats, &mut player);
	
	println!("Player stats: {:#?}", player);
	

	
	/*-----------------Gameplay starts-------------------*/
	
	
	
	print_line_break();
	print_line_break();
	println!("You arrive in town");
	
	//Main game loop
	loop {
		println!("What do you want to do?");
		//action_menu(&player);
		
		break;
		
	}
	
	
		
	
	
	
	
}


/*-----------------Structs----------------*/
#[derive(Debug)]
struct Player {
	name: String,
	hit_points: i16,
	damage: i16,
	armor_class: i16,
	speed: i16,
	gold: i32,
	xp: i32,
	level: i16,
	stats: Stats,
	inventory: Inventory
}

impl Default for Player {
	fn default() -> Player {
		Player { 	
			name: "Default", 
			hit_points:		12, 
			damage: 		0, 
			armor_class:	0, 
			speed:			0, 
			gold: 			10,
			xp:				0,
			level:			0,
			stats: 			Stats { Default::default() },
			inventory:		Inventory { Default::default() },
			}
		}
	}
}

impl Player {
	fn level_up(&self) {
		self.level += 1; 
		self.damage += (stats_to_mods(self.stats.strength)) *  self.level;
		self.speed += (stats_to_mods(self.stats.dex)) * self.level;
		self.armor_class += self.speed;
		self.hit_points += (stats_to_mods(self.stats.con)) * self.level;
		self.gold += (stats_to_mods(self.stats.cha) * self.level * 5) as i32;
	}
}


struct Stats {
	strength: u8,
	dex: u8,
	con: u8,
	int: u8,
	wis: u8,
	cha: u8,
}

impl Default for Stats {
	fn default() -> Stats {
		Stats { 
			strength: 0,
			dex: 0,
			con: 0,
			int: 0,
			wis: 0,
			cha: 0,
		}
	}
}
	

impl Stats {
	fn build_stats(&self) {
		self.strength: roll_stat(),
		self.dex: roll_stat(),
		self.con: roll_stat(),
		self.int: roll_stat(),
		self.wis: roll_stat(),
		self.cha: roll_stat(),
	}
}

	
struct Inventory {
	armor: Item,
	weapon: Item,
	magic_item: Item,
	potion1: Item,
	potion2: Item,
	Potion3: Item
	}
	
impl Default for Inventory {
	fn default() -> Inventory {
		Inventory {
			armor: Item { "Old Rags", 0 },
			weapon: Item { "Bare Hands", 0},
			magic_Item: Item { "Empty", 0},
			potion1: Item {"Empty", 0},
			potion2: Item {"Empty", 0},
			potion3: Item {"Empty", 0},
			}
		}
	}
}
			

impl Inventory {
	fn equip_armor(&self, player: &Player, item: &Item) {
		player.armor_class -= self.armor.effect;
		self.armor.name = item.name;
		player.armor_class += item.effect;
		}
		
	fn equip_weapon(&self, player: &Player, item: &Item) {
		player.damage -= self.weapon.effect;
		self.weapon.name = item.name;
		player.damage += item.effect;
		}
		
	fn equip_magic(&self, player: &Player, item: &Item) {
		player.magic -= self.magic_item.effect;
		self.magic_item.name = item.name;
		player.magic += item.effect;
		}
	}

struct Item {
	name: String,
	effect: i16
}


struct Creature {
	name: String,
	hit_points: i16,
	damage: i16,
	armor_class: i8,
	speed: i8,
}

/*------------------Functions----------------*/

fn loc_town(player: &mut Player) {
}

fn print_line_break() {
	println!(" ");
	println!("--------------------");
	println!(" ");
}

fn input_name() -> String {
	let mut name = String::new();
	
	io::stdin().read_line(&mut name).expect("Failed to read line");
		
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


fn print_stats(stats: &Stats) {
	println!("STR: {}", stats.strength);
	println!("DEX: {}", stats.dex);
	println!("CON: {}", stats.con);
	println!("INT: {}", stats.int);
	println!("WIS: {}", stats.wis);
	println!("CHA: {}", stats.cha);
}
	
fn yes_or_no() -> bool {

		let mut input = String::new(); 
		
		io::stdin().read_line(&mut input).expect("Failed to read line");
		
		input.trim() == "Y" || input.trim() == "y" 			
	}
	
fn stats_to_mods(stat: u8) -> i16 {
	
	let modifier = match stat {
		 3 | 4  => -3,
		 5 | 6  => -2,
		 7 | 8  => -1,
		 9 | 10 =>  0,
		11 | 12 =>  1,
		13 | 14 =>  2, 
		15 | 16 =>  3,
		17 | 18 =>  4,
		19 | 20 =>  5,
		_       =>  0,
		};
		
		modifier
}




