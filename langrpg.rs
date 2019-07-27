use std::collections::HashMap;
use std::io::*;
use std::fs;
use std::convert::TryInto;
use std::str::FromStr;
use quick_xml::Reader;
use quick_xml::events::Event;
use textwrap::{Wrapper, termwidth, fill};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::color;

pub struct Player {
    name: String,
    hp: u32,
    max_hp: u32,
    exp: u32,
    exp_to_next: u32,
    items: Vec<Item>,
    pub books: Vec<Book>,
    decks: Vec<Deck>,
    current_map: String,
    x_pos: usize,
    y_pos: usize,
    defense: u32,
    attack: u32,
    last_tile: u32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            hp: 20,
            max_hp: 20,
            exp: 0,
            exp_to_next: 20,
            current_map: String::from("Market"),
            x_pos: 8,
            y_pos: 1,
            name: String::from(""),
            items: Vec::new(),
            books: Vec::new(),
            decks: Vec::new(),
            defense: 0,
            attack: 0,
            last_tile: _EMPTY,
        }
    }

    pub fn load_data() {
        // load data from save file
    }

    pub fn get_current_map_info(&mut self, world: &mut World) {
        for (mapName, mapStruct) in &mut world.maps {
            if &mut self.current_map == mapName {
                mapStruct.load_events(self);
            }
        }
    }


    pub fn update_map(&mut self, s: String) {
        self.current_map = s[..].to_string();
    }

    pub fn take_damage(&mut self, amount: u32) {
        if (self.hp - amount) < 0 {
            self.hp = 0;
        }

        else {
            self.hp -= amount;
        }
    }

    pub fn heal(&mut self, amount: u32) {
        if (self.hp + amount) > self.max_hp {
            self.hp = self.max_hp;
        }

        else {
            self.hp += amount;
        }
    }

    pub fn print_health(&self) {
        print!("HP: {} / {}", self.hp, self.max_hp);
    }

    pub fn print_exp(&self) {
        print!("EXP: {} / {}", self.exp, self.exp_to_next);
    }

    pub fn print_coords(&self) {
        println!(" ---------------------------");
        println!(" You are at {}, {} in {}", self.x_pos, self.y_pos, self.current_map);
        println!(" ---------------------------");
    }

    pub fn attack(&mut self, enemy: &Enemy) {
        // attack an enemy type
        println!("A wild {} attacked!", enemy.name);
    }

    pub fn open_book(&mut self, name: &String) {
        let mut found = false;
        for i in &self.books {
            if i.name.to_ascii_lowercase() == name.to_string().to_ascii_lowercase() {
                i.open_book();
                found = true;
                break;
            }
            
        }

        if found == false {
            println!("Could not find book {}", name);
        }
    }

    pub fn has_book(&self, name: &String) -> bool {
        let mut found = false;
        for i in &self.books {
            if i.name == name.to_string() {
                found = true;
            }
        }

        found
    }

    pub fn print_books(&mut self) {
        for i in &mut self.books {
            println!("* {}", i.get_name());
        }
    }

    pub fn use_item(&mut self, name: &String) {
        for i in &self.items {
            if i.name == name.to_string() {
                // use item implementation
            }
        }
    }

    pub fn open_deck(&mut self, name: &String) {
        for i in &self.decks {
            if i.name == name.to_string() {
                // use deck implementation
            }
        }
    }

    pub fn get_x(&self) -> usize {
        self.x_pos
    }

    pub fn get_y(&self) -> usize {
        self.y_pos
    }

    pub fn move_player(&mut self, x_pos: usize, y_pos: usize) {
        self.x_pos = x_pos;
        self.y_pos = y_pos;
    }
}

struct Item {
    name: String,
    quantity: u32,
    capacity: u32,
    effects: Vec<Effect>,
    desc: String,
    kind: String
}

impl Item {
    pub fn new() -> Item {
        Item {
            name: String::from(""),
            quantity: 0,
            capacity: 0,
            effects: Vec::new(),
            desc: String::from(""),
            kind: String::from("Item")
        }
    }

    pub fn create(&mut self, name: String, qty: u32, cap: u32, desc: String, kind: String) {
        self.name = name[..].to_string();
        self.quantity = qty;
        self.capacity = cap;
        self.desc = desc[..].to_string();
        self.kind = kind[..].to_string();
    }
}

pub trait Useable {
    fn use_obj(&mut self);
}

pub trait Findable<T> {
    fn add_to_inventory (&self, player: &mut Player);
}

impl Useable for Item {
    fn use_obj(&mut self) {
        if self.kind == "Potion".to_string() {
            self.quantity -= 1;
        }
    }
}

impl Findable<Item> for Item {
    fn add_to_inventory(&self, player: &mut Player) {
        for i in &mut player.items {
            if i.name == self.name && i.quantity < i.capacity {
                i.quantity += 1;
                break;
            }
        }

        let newItem = Item::new();
        player.items.push(newItem);
    }
}

struct Effect {
    name: String,
    desc: String,
    duration: u32,
}

pub struct Enemy {
    name: String,
    hp: u32,
    max_hp: u32
}

impl Enemy {
    pub fn new(name: &String) -> Enemy {
        let mut hp = 0;
        let mut max_hp = 0;
        if name == "Green Blob" {
            hp = 5;
            max_hp = 5;
        }

        Enemy {
            name: name[..].to_string(),
            hp: hp,
            max_hp: max_hp
        }
    }
}

pub struct NPC {
    pub name: String,
    x_pos: usize,
    y_pos: usize,
    pub dialogs: Vec<Dialog>,
    flags: HashMap<String, bool>
}


impl NPC {
    pub fn new(x_pos: usize, y_pos: usize, name: &String) -> NPC {
        NPC {
            name: name[..].to_string(),
            x_pos: x_pos,
            y_pos: y_pos,
            dialogs: Vec::new(),
            flags: HashMap::new()
        }
    }

    pub fn talk(&self) {
        println!("You approach {}[{}]{}.", color::Fg(color::Red), self.name, color::Fg(color::Reset));
        for i in &self.dialogs {
            println!("* {}", i.text);
        }

        println!("---------------------------");
    }
}

#[derive(Clone)]
pub struct Dialog {
    pub text: String
}

impl Dialog {
    pub fn new(text: String) -> Dialog {
        Dialog {
            text: text
        }
    }
}

struct PlainItem {

}

pub struct Book {
    name: String,
    desc: String,
    pages: Vec<Page>,
    page_count: usize
}


/* @TODO: Add support for loading this info from a file? */
impl Book {
    pub fn new_from_name(name: &String) -> Book {
        let data = fs::read_to_string("data/".to_string() + &name.to_string() + ".txt").expect("Sorry, that book doesn't exist.");

        let mut reader = Reader::from_str(data.as_str());
        let mut count = 0;
        let mut text = Vec::new();
        let mut buf = Vec::new();

        reader.trim_text(true);
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    
                },
                Ok(Event::Text(e)) => {
                    text.push(e.unescape_and_decode(&reader).unwrap());
                },
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break,
                _ => (),
            }
            buf.clear();
        }


        let mut pages: Vec<Page> = Vec::with_capacity(5);
        let mut count: usize = 0;
        for (i, page) in text[2..].iter().enumerate() {
            pages.push(Page::new(page, i));
            count += 1;
        }

        Book {
            name: text[0].to_string(),
            desc: text[1].to_string(),
            pages: pages,
            page_count: count
        }
    }

    pub fn open_book(&self) {
        println!("Opened \"{}\" ({} pages).", self.name, self.pages.len());
        
        let mut reading = true;
        while reading == true {
            print!("Enter a page num or \"close\" to quit: ");
            let mut s = String::new();
            let _ = stdout().flush();
            stdin().read_line(&mut s).expect("The string you entered was invalid.");

            let input: Vec<&str> = s.split('\n').collect();

            if input[0] == "close" {
                reading = false;
                break;
            }

            let page_id = input[0].parse::<u32>().unwrap();

            if page_id > 0 && page_id <= self.pages.len().try_into().unwrap() {
                println!("----------------------------");
                self.open_page(page_id.try_into().unwrap());
            }
        }
        
    }

    pub fn open_page(&self, page_num: usize) {
        let wrapper = Wrapper::new(50).initial_indent("| ").subsequent_indent("| ");
        println!("|        {}Page {}{}            |", color::Fg(color::Red), page_num, color::Fg(color::Reset));
        println!("----------------------------");
        println!("{}", wrapper.fill(&self.pages[page_num-1].contents));
        println!("----------------------------");
    }

    pub fn get_name(&mut self) -> &str {
        &self.name
    }
}

pub struct Deck {
    name: String,
    desc: String,
    cards: Vec<Card>
}

pub struct Page {
    contents: String,
    page_num: usize
}

impl Page {
    pub fn new(contents: &String, page_num: usize) -> Page {
        Page {
            contents: contents[..].to_string(),
            page_num: page_num
        }
    }
}

pub struct Card {
    term: String,
    definition: String,
    part_of_speech: String
}

pub struct World {
    pub maps: HashMap<String, Map>
}

impl World {
    pub fn new() -> World { 
        World { 
            maps: HashMap::new()
        }
    }

    pub fn init(&mut self) {
        self.maps.insert("Market".to_string(), Map::new("Market", 10, 10));
        self.maps.insert("Fields".to_string(), Map::new("Fields", 10, 10));
        self.maps.insert("Castle".to_string(), Map::new("Castle", 10, 10));
        self.maps.insert("Desert".to_string(), Map::new("Desert", 10, 10));

        for (key, value) in &mut self.maps {
            if *key == "Market".to_string() {
                value.tiles[0][0] = _EMPTY;
                value.tiles[1][0] = _EMPTY;
                value.tiles[2][0] = _EMPTY;
                value.tiles[3][0] = _EMPTY;
                value.tiles[4][0] = _EMPTY;
                value.tiles[5][0] = _NPC;
                value.tiles[6][0] = _EMPTY;
                value.tiles[7][0] = _EMPTY;
                value.tiles[8][0] = _NPC;
                value.tiles[9][0] = _ENEMY;
                value.tiles[0][1] = _EXIT;
            }

            else if *key == "Desert".to_string() {
                value.tiles[8][1] = _NPC;
                value.tiles[9][1] = _EXIT;
            }
        }
    }
}

pub struct Map {
    width: u32,
    height: u32,
    name: String,
    pub tiles: [[u32; 10]; 10],
    pub events: Vec<String>,
    pub NPCs: Vec<NPC>,
    pub books: Vec<Book>,
    pub decks: Vec<Deck>,
    pub enemies: Vec<Enemy>
}

impl Map {
    pub fn new(name: &str, width: u32, height: u32) -> Map {
        let empty: &str = "";
        Map {
            name: String::from(name.to_string()),
            width: width,
            height: height,
            tiles: [[0; 10]; 10],
            events: Vec::new(),
            NPCs: Vec::new(),
            books: Vec::new(),
            decks: Vec::new(),
            enemies: Vec::new(),
        }
    }

    pub fn load_events(&mut self, player: &mut Player) {

        self.NPCs.clear();
        self.enemies.clear();
        self.decks.clear();
        self.books.clear();

        if player.current_map == "Market" {
            
            // NPC events
            if self.tiles[player.x_pos][player.y_pos] == _NPC {
                /* NPC at 5, 0 */
                if (player.x_pos == 5 && player.y_pos == 0) {
                    self.NPCs.push(NPC::new(player.x_pos, player.y_pos, &"Old Man".to_string()));
                    self.NPCs[0].dialogs.push(Dialog::new(String::from("It's nice to meet you.")));

                    self.NPCs[0].talk();
                }

                /* NPC at 8, 0 */
                else if (player.x_pos == 8 && player.y_pos == 0) {
                    self.NPCs.push(NPC::new(player.x_pos, player.y_pos, &"Martha".to_string()));
                    self.NPCs[0].dialogs.push(Dialog::new(String::from("Good heavens! You should have this book.")));
                    
                    self.NPCs[0].talk();

                    let mut new_book = Book::new_from_name(&"Hiragana".to_string());
                    let book_name = &new_book.name;
                    
                    if player.has_book(&book_name) == false {
                        println!("You received a book: {}{}{}!", color::Fg(color::Blue), book_name, color::Fg(color::Reset));
                        println!("Type {}open book called {}{} to open it.", color::Fg(color::Red), book_name, color::Fg(color::Reset));
                        println!("---------------------------");
                        player.books.push(new_book);
                    }
                }

                player.last_tile = _NPC;
            }

            else if self.tiles[player.x_pos][player.y_pos] == _ENEMY {
                if (player.x_pos == 9 && player.y_pos == 0) {
                    self.enemies.push(Enemy::new(&"Green Blob".to_string()));
                    player.attack(&self.enemies[0]);
                }

                player.last_tile = _ENEMY;
            } 

            else if self.tiles[player.x_pos][player.y_pos] == _EXIT && player.last_tile != _EXIT {
                if (player.x_pos == 0 && player.y_pos == 1) {
                    player.update_map("Desert".to_string());
                    player.move_player(9, 1);
                    println!(" NOW ENTERING: DESERT ");
                    player.print_coords();
                }

                player.last_tile = _EXIT;
            }

            else {
                player.last_tile = _EMPTY;
            }
        }

        else if player.current_map == "Desert" {
            if self.tiles[player.x_pos][player.y_pos] == _NPC {
                if player.x_pos == 8 && player.y_pos == 1 {
                    self.NPCs.push(NPC::new(player.x_pos, player.y_pos, &"Mysterious Man".to_string()));
                    self.NPCs[0].dialogs.push(Dialog::new(String::from("This desert... there are many enemies out here. They'll want to fight you, but not in a way you'd think. They'll test your vocabulary knowledge. Answer correctly to do damage!")));
                
                    self.NPCs[0].talk();
                }

                player.last_tile = _NPC;
            }

            else if self.tiles[player.x_pos][player.y_pos] == _EXIT && player.last_tile != _EXIT {
                if player.x_pos == 9 && player.y_pos == 1 {
                    player.update_map("Market".to_string());
                    player.move_player(0, 1);
                    println!(" NOW ENTERING: MARKET ");
                    player.print_coords();
                }

                player.last_tile = _EXIT;
            }

            else {
                player.last_tile = _EMPTY;
            }
        }
    }

}


// CONSTANTS
const _EMPTY: u32 = 0;
const _NPC: u32 = 1;
const _BOOK: u32 = 2;
const _DECK: u32 = 3;
const _ITEM: u32 = 4;
const _ENEMY: u32 = 5;
const _BOSS: u32 = 6;
const _EXIT: u32 = 7;