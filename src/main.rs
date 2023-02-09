use rand::Rng;

pub struct Player {
    pub name: String,
    pub health: i32,
    pub level: i32,
}

pub struct Game {
    pub num : i32,
    pub players : Vec<Player>,
}

fn main() {

    // We are going to build a game
    // This game for now will be a text based game
    // The story of the game will be about a legendary space pirate, known for your cunning and bravery. Your latest target is a mysterious treasure, said to be hidden on an abandoned planet in a remote corner of the galaxy. The treasure is said to contain artifacts from ancient civilizations, including the mythical world of the Greek gods.
    // The game will be a text based game, where the player will be able to choose between different options, and the game will respond accordingly.


    // Key concepts:
    // 1. The story will be randomly generated, so that the player can play the game multiple times and get a different story each time.
    // 2. The player will be able to choose between different options, and the game will respond accordingly.
    // 3. The game will be able to keep track of the player's progress, and will be able to respond accordingly.

    // Start the loop for the game
    'main: loop {
        // Create a new game
        let mut game = Game {
            num : rand::thread_rng().gen_range(1..100),
            players : Vec::new(),
        };

        // Create a new player
        let mut player = Player {
            name : String::new(),
            health : 100,
            level : 1,
        };

        // Print the welcome message
        println!("Welcome to the game!");
        // Getting the players name
        println!("What is your name?");
        std::io::stdin().read_line(&mut player.name).expect("Failed to read line");

        // Adding the player to the game
        game.players.push(player);

        println!("Hello {}Welcome to the game!", game.players[(&game.players).len()-1].name);

        println!("There are {} players in the game", (&game.players).len());

        'game: loop {
            println!("What do you want to do?");
            println!("1. Go to the next room");
            println!("2. Go to the previous room");
            println!("3. Quit the game");

            let mut choice = String::new();

            std::io::stdin().read_line(&mut choice).expect("Failed to read line");

            let choice : i32 = match choice.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            if choice == 1 {
                println!("You are in the next room");
            } else if choice == 2 {
                println!("You are in the previous room");
            } else if choice == 3 {
                println!("You have quit the game");
                break 'game;
            } else {
                println!("Please enter a valid choice");
            }
        }

        println!("Do you want to play again? (y/n)");
        let mut play_again = String::new();
        std::io::stdin().read_line(&mut play_again).expect("Failed to read line");

        if play_again.trim() == "n" {
            println!("Thank you for playing the game!");
            break 'main;
        }

    }

}

