mod langrpg;

use std::io::{stdin, stdout, Write};
use crate::langrpg::Useable;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::color;

fn main() {
    

    let mut PLAYER = langrpg::Player::new();
    let mut WORLD = langrpg::World::new();

    WORLD.init();
    PLAYER.update_map("Market".to_string());
    PLAYER.get_current_map_info(&mut WORLD);

    let mut get_commands = true; 

    while get_commands == true {
        print!("Enter a command: {}", color::Fg(color::Red));
        let mut s = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("The string you entered was invalid.");
        print!("{}", color::Fg(color::Reset));
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }

        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }

        get_commands = handle_input(s, &mut PLAYER, &mut WORLD);
    }
}

fn handle_input(command: String, player: &mut langrpg::Player, world: &mut langrpg::World) -> bool {
    let input: Vec<&str> = command.split(' ').collect();
    if(input[0] == "exit".to_string()) {
        println!("exiting...");
        false
    }

    else if (input[0] == "open".to_string()) {
        if input.len() > 1 {
            if input[1] == "book".to_string() {
                if input.len() > 2 {
                    if input[2] == "called".to_string() {
                        player.open_book(&input[3].to_string());
                    }
                }

                else {
                    println!("open book syntax: open book called <bookname>");
                }
            }

            else if input[1] == "books".to_string() {
                if input.len() < 3 {
                    player.print_books();
                }

                else {
                    println!("open books does not have more arguments!");
                }
            }

            else if input[1] == "deck".to_string() {
                if input.len() > 3 {
                    if input[2] == "called".to_string() {
                        //player.open_book(&input[3].to_string());
                    }
                }

                else {
                    println!("open deck syntax: open deck called <deckname>");
                }
            }

            else if input[1] == "decks".to_string() {
                if input.len() < 3 {
                    //player.print_books();
                }

                else {
                    println!("open decks does not have more arguments!");
                }
            }

            else {
                println!("open syntax: \n1. open book | deck called <name>\n2. open books | decks | inventory");   
            }
        }

        else {
            println!("open syntax: \n1. open book | deck called <name>\n2. open books | decks | inventory");   
        }
        true
    }

    else if input[0] == "move".to_string() {
        if input[1] == "left".to_string() {
            if player.get_x() > 0 {
                player.move_player(player.get_x() - 1, player.get_y());
                player.print_coords();
                player.get_current_map_info(world);
            }
        }

        if input[1] == "right".to_string() {
            if player.get_x() < 9 {
                player.move_player(player.get_x() + 1, player.get_y());
                player.print_coords();
                player.get_current_map_info(world);
            }
        }

        if input[1] == "up".to_string() {
            if player.get_y() > 0 {
                player.move_player(player.get_x(), player.get_y() - 1);
                player.print_coords();
                player.get_current_map_info(world);
            }
        }

        if input[1] == "down".to_string() {
            if player.get_y() < 9 {
                player.move_player(player.get_x(), player.get_y() + 1);
                player.print_coords();
                player.get_current_map_info(world);
            }
        }

        true
    }

    else if input[0] == "help".to_string() {
        println!("{}COMMANDS{}", color::Fg(color::Red), color::Fg(color::Reset));
        println!("* open <book|deck> called <name> -- open a book or deck by name");
        println!("* open inventory -- show a list of your items");
        println!("* open books -- show your list of books");
        println!("* open decks -- show your list of decks");
        println!("* use <item> -- use an item, if you have it");
        println!("* move <up|left|down|right> -- move in a direction 1 tile");
        println!("* exit -- quit this program");
        println!("* help -- show this information");
        println!("* status -- show your health, exp, location");
        println!("* clear -- clear the terminal screen");

        true
    }

    else if input[0] == "status".to_string() {
        println!(" ---------------------------");
        print!("| ");
        player.print_health();
        print!(" | ");
        player.print_exp();
        println!(" | ");
        player.print_coords();

        true
    }

    else if input[0] == "clear".to_string() {
        print!("{}[2J", 27 as char);
        true
    }

    else {
        println!("Please enter a valid command.");
        true
    }
}

fn get_dialogs(world: &langrpg::World, map_name: &String, NPC_name: &String) {
    match world.maps.get(&map_name.to_string()) {
        Some(x) => {
            for i in &x.NPCs {
                if i.name == NPC_name.to_string() {
                    i.talk();
                }
            }
        },
        None => {
            println!("DEBUG: No dialog found for map {} with NPC named {}", map_name, NPC_name);
        },
    }
}
