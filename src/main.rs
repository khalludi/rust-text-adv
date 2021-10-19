use std::io;

struct Player {
    health: i32,
    attack_dmg: i32,
}

struct State {
    location: i32, // 0 = home, 1 = dungeon
    prev_location: i32,
}

fn main() {
    let mut player: Player = Player {
        health: 23,
        attack_dmg: 8
    };
    
    let mut state: State = State {
        location: 0,
        prev_location: 1,
    };

    intro();

    while true {
        if state.location == 0 {
            home_screen(&player, state.location == state.prev_location);
        } else {
            dungeon_screen(&player);
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
        }
    }
    
}

fn dungeon_screen(player: &Player) {
    print_center("Dungeon");

    

    println!("Health: {}", player.health);
}

fn home_screen(player: &Player, skip_text: bool) {
    print_center("Home");

    if (!skip_text) {
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

fn isValid() {
    let home_commands: Vec<&str> = vec!["enter"];
    let dungeon_commands: Vec<&str> = vec!["1"];
}

