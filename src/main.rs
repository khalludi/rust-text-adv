use std::io;
use rand::Rng;

struct Player {
    health: i32,
    attack_dmg: i32,
}

struct State {
    location: i32, // 0 = home, 1 = dungeon,
    // 2 = treasure room
    prev_location: i32,
    big_chungus_count: i32,
    hagrid_count: i32,
    treasure_room_chance: i32,
}

fn main() {
    let mut player: Player = Player {
        health: 23,
        attack_dmg: 8
    };
    
    let mut state: State = State {
        location: 0,
        prev_location: 1,
        big_chungus_count: 0,
        hagrid_count: 0,
        treasure_room_chance: 3,
    };

    intro();

    loop {
        if state.location == 0 {
            home_screen(&mut player, state.location == state.prev_location);
        } else {
            dungeon_screen(&mut player, &mut state);

            if player.health <= 0 || state.location == 2 {
                break;
            }
        }

        println!("What do you choose to do? ");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("You have chosen to: {}", input);
                input = input[0..n-1].to_string();
            }
            Err(error) => println!("Error: {}", error),
        }

        if state.location == 0 {
            state.prev_location = state.location;
            if input == "rest" {
                player.health = 58;
            } else if input == "enter" { 
                state.location = 1;
                
                println!("\n\n");
                print_center("With a great shout, you barrel into the dungeon!");
                println!("\n\n");
            } else {
                println!("Are you for real, Do something gargoyle!\n");
            }
        } else if state.location == 1 {
            state.prev_location = state.location;
            if input == "exit" {
                state.location = 0;
                state.treasure_room_chance = 3;
            }
        }
    }

    if state.location != 2 {
        println!("You have failed. Try again next time.\n");
    } else {
        println!("Congratulations, you made it. Go show that feudal lord who's boss!");
    }

    println!("");

    print_center("STATISTICS");
    print_center(&format!("# of Wild Big Chungus: {}", state.big_chungus_count)[..]);
    print_center(&format!("# of Wild Hagrids: {}", state.hagrid_count)[..]);
    
}

fn dungeon_screen(player: &mut Player, state: &mut State) {
    print_center("Dungeon");

    let num = rand::thread_rng().gen_range(0..100);
    
    if num < state.treasure_room_chance {
        println!("Congratulations, you found the treasure room.");
        state.location = 2;
    } else if num < state.treasure_room_chance + 20 {
        println!("You have encountered a big chungus. He trips and falls on you");
        println!("for 20 damage!");

        player.health -= 20;
        state.big_chungus_count += 1;
    } else {
        println!("You encounter Hagrid. He says hi, but accidentally breaks ur hand.");
        println!("You take 5 damage.");

        player.health -= 5;
        state.hagrid_count += 1;
    }

    if state.treasure_room_chance < 15 {
        state.treasure_room_chance += 2;
    }

    println!("\nRemaining Health: {}", player.health);
}

fn home_screen(player: &mut Player, skip_text: bool) {
    print_center("Home");

    if !skip_text {
        print_center("You ready yourself for battle with your trusty fork and knife, ");
        print_center("just as your forefathers have done before you.");
    }

    println!("\nHealth: {}\n", player.health);
}

fn intro() {
    println!("");
    print_center("WELCOME TO RTA");
    println!("");

    print_center("As a humble farmer, you feel immense hate against the feudal lord.");
    print_center("How dare he take away your prized peanuts that have been aging");
    print_center("for the past 20 years! To get back at him, you've heard rumors");
    print_center("of the legendary XXX. Against all odds, you decide to YOLO (as");
    print_center("the young'uns say) and embark on a heroic quest for revenge.");

    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
}

fn print_center(text: &str) {
    if let Some((w, h)) = term_size::dimensions() {
        let num_spaces = (w - text.len())/2;
        let mut output: String = (0..num_spaces)
            .map(|_| " ").collect::<String>();
        println!("{}{}", output, text);
    } else {
        println!("Unable to get term size :(");
    }
}

