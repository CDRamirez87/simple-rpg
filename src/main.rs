use std::io;
use rand::Rng;

fn main() {
	
	/*-------------Init--------------*/
	
	let mut player: Player = Default::default();
	print_line_break();
	println!("Welcome to RPG!");
	println!("by KMR");
	print_line_break();

	//Promts user for name
	println!("Please type a name for your Warrior: ");
	
	let temp_name = input_name();
	player.name = temp_name.trim().to_string();
	
	print_line_break();
	
	//Bool to break loop
	let mut keep_stats = false;
	
	
	while keep_stats == false {
		player.stats.build();
		
		println!("{}'s stats are: ", player.name);
		player.stats.print();
		
		//Asks player if they wish to keep stats, if not, rerolls
		print_line_break();
		
		println!("Keep these stats? (Y) = Yes, keep them (N) = No, reroll them");
		
		keep_stats = yes_or_no();
		
		//If player answers no, rerolls and prompts again
	}
	
	//Converts and applies stat modifiers to player
	player.level_up();
	
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
	magic: i16,
	stats: Stats,
	inventory: Inventory
}

impl Default for Player {
	fn default() -> Player {
		Player { 	
			name: "Default".to_string(), 
			hit_points:		12, 
			damage: 		0, 
			armor_class:	0, 
			speed:			0, 
			gold: 			10,
			xp:				0,
			level:			0,
			magic:			0,
			stats: 			Stats {	
								strength: 0,
								dex: 0,
								con: 0,
								int: 0,
								wis: 0,
								cha: 0,
							},
							
			inventory:		Inventory {
								armor: Item { name: "Old Rags".to_string(), effect: 0 },
								weapon: Item { name: "Bare Hands".to_string(), effect: 0},
								magic_item: Item { name: "Empty".to_string(), effect: 0},
								potion1: Item { name: "Empty".to_string(), effect: 0},
								potion2: Item { name:"Empty".to_string(), effect: 0},
								potion3: Item { name:"Empty".to_string(), effect: 0},
							},
		}
	}
}


impl Player {
	fn level_up(&mut self) {
		self.level += 1; 
		self.damage += (Stats::to_mods(self.stats.strength)) *  self.level;
		self.speed += (Stats::to_mods(self.stats.dex)) * self.level;
		self.armor_class += self.speed;
		self.hit_points += (Stats::to_mods(self.stats.con)) * self.level;
		self.gold += (Stats::to_mods(self.stats.cha) * self.level * 5) as i32;
	}
}

#[derive(Debug)]
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

	fn roll() -> u8 {
		let r1 = rand::thread_rng().gen_range(1, 6);
		let r2 = rand::thread_rng().gen_range(1, 6);
		let r3 = rand::thread_rng().gen_range(1, 6);
		let r4 = rand::thread_rng().gen_range(1, 6);
	
		let mut rolls = [r1, r2, r3, r4];
	
		rolls.sort_unstable();
	
		rolls[1] + rolls[2] + rolls[3]
	}
	
	fn build(&mut self) {
		self.strength = Stats::roll();
		self.dex = Stats::roll();
		self.con = Stats::roll();
		self.int = Stats::roll();
		self.wis = Stats::roll();
		self.cha = Stats::roll();
	}
	
	fn to_mods(stat: u8) -> i16 {
	
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
	
	fn print(&self) {
	println!("STR: {}", self.strength);
	println!("DEX: {}", self.dex);
	println!("CON: {}", self.con);
	println!("INT: {}", self.int);
	println!("WIS: {}", self.wis);
	println!("CHA: {}", self.cha);
	}
}

#[derive(Debug)]
struct Inventory {
	armor: Item,
	weapon: Item,
	magic_item: Item,
	potion1: Item,
	potion2: Item,
	potion3: Item
	}
	
impl Default for Inventory {
	fn default() -> Inventory {
		Inventory {
			armor: Item { name: "Old Rags".to_string(), effect: 0 },
			weapon: Item { name: "Bare Hands".to_string(), effect: 0},
			magic_item: Item { name: "Empty".to_string(), effect: 0},
			potion1: Item { name: "Empty".to_string(), effect: 0},
			potion2: Item { name:"Empty".to_string(), effect: 0},
			potion3: Item { name:"Empty".to_string(), effect: 0},
		}
	}
}

			

impl Inventory {
	fn equip_armor(&mut self, player: &mut Player, item: &Item) {
		player.armor_class -= self.armor.effect;
		self.armor.name = item.name.clone();
		player.armor_class += item.effect;
		}
		
	fn equip_weapon(&mut self, player: &mut Player, item: &Item) {
		player.damage -= self.weapon.effect;
		self.weapon.name = item.name.clone();
		player.damage += item.effect;
		}
		
	fn equip_magic(&mut self, player: &mut Player, item: &Item) {
		player.magic -= self.magic_item.effect;
		self.magic_item.name = item.name.clone();
		player.magic += item.effect;
		}
	}

#[derive(Debug)]
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

//TODO: Complete this function
//fn loc_town(player: &mut Player) {
//}

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

	
fn yes_or_no() -> bool {

		let mut input = String::new(); 
		
		io::stdin().read_line(&mut input).expect("Failed to read line");
		
		input.trim() == "Y" || input.trim() == "y" 			
	}
	





