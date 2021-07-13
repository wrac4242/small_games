use std::io;

mod games;

fn main() {
    println!("Welcome to Wrac's set of games");
    // A basic terminal menu, press numbers to go to the different games

    
    #[allow(unused_assignments)]
    let mut game = 0;
    
    loop {
        println!("Press 1 for nim, 2 for blackjack");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                let choice = input.trim();

                match choice {
                    "1" => {
                        game = 1;
                        break;
                    },
                    "2" => {
                        game = 2;
                        break;
                    },
                    _ => {
                        println!("Error, this is not a valid selection\nplease reenter");
                    }
                }
            }
            Err(error) => {
                println!("error: {}", error);
                return;
            },
        }
    }
    

    match game {
        1 => games::nim::main_game(),
        2 => games::blackjack::main_game(),
        _ => {
            println!("Critical error in game selection, {}", game);
            return;
        }
    }
}
