use std::fs;
use std::io::Result;
use std::time::Instant;

const FILE_NAME: &str = "resources/input.txt";

const TOTAL_RED: i32 = 12;
const TOTAL_GREEN: i32 = 13;
const TOTAL_BLUE: i32 = 14;

struct Game {
    game_number: i32,
    hands: Vec<Hand>
}

struct Hand {
    colors: Vec<Color>
}

struct Color {
    count: i32,
    color: String
}

fn get_game(val: &str) -> (i32, &str) {
    let game: Vec<&str> = val.split(":").collect();

    let parse_game: Vec<&str> = game[0].split(" ").collect();

    let game_num: i32 = parse_game[1].parse::<i32>().unwrap();

    return (game_num, game[1]) 
}
fn split_game(val: &str) -> Vec<&str> {
    return val.split(";").collect();
}

fn split_hand(val: &str) -> Vec<&str> {
    return val.split(",").collect();
}

fn get_color(val: &str) -> Color {
    
    let c: Vec<&str> = val.split(" ").collect();
    
    return Color {
        count: c[1].parse::<i32>().unwrap(),
        color: c[2].to_owned()
    };
}

fn parse_game(val: &str) -> Game {

    let (game_num, val) = get_game(val);

    let mut game = Game {
        game_number: game_num,
        hands: Vec::new()
    };

    for hand in split_game(val) {
        
        let mut colors: Vec<Color> = Vec::new();

        for color in split_hand(hand) {
            
            colors.push(get_color(color));
        }

        game.hands.push(Hand {colors: colors});
    }

    return game;
}

fn is_game_possible(game: &Game) -> bool {
    for hand in &game.hands {
        for color in &hand.colors {
            let possible_val = match color.color.as_str() {
                "red" => TOTAL_RED,
                "green" => TOTAL_GREEN,
                "blue" => TOTAL_BLUE,
                _ => panic!("UNKNOWN COLOR")
            };

            if color.count > possible_val {
                return false;
            }
        }
    }

    return true;
}

fn minimum_possible_colors(game: &Game) -> (i32, i32, i32) {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for hand in &game.hands {
        for color in &hand.colors {
            match color.color.as_str() {
                "red" => {
                    if color.count > red {
                        red = color.count;
                    }
                },
                "green" => {
                    if color.count > green {
                        green = color.count;
                    }
                },
                "blue" => {
                    if color.count > blue {
                        blue = color.count;
                    }
                },
                _ => panic!("UNKNOWN COLOR")
            };
        }
    }

    return (red, green, blue);
}

fn part1(content: &Vec<String>) -> Result<()> {
    println!("\nPart 1");
    
    let mut total = 0;

    for line in content {

        let game = parse_game(line);        


        let is_possible = is_game_possible(&game);
        println!("Game {}: {}", game.game_number, is_possible);

        if is_possible {
            total += game.game_number;
        }


    }

    println!("Total {}", total);

    return Ok(());
}

fn part2(content: &Vec<String>) -> Result<()> {
    println!("\nPart 2");

    let mut total = 0;

    for line in content {

        let game = parse_game(line);        


        let (red, green, blue) = minimum_possible_colors(&game);

        let power = red*green*blue;
        println!("Game {}: red {}, green {}, blue {}, power {}", game.game_number, red, green, blue, power);

        total += power;
    }

    println!("Total {}", total);

    return Ok(());
}


fn main() -> Result<()> {
    let content = fs::read_to_string(FILE_NAME)?.lines().map(String::from).collect();
    let mut now = Instant::now();

    part1(&content)?;
    println!("process took {:?}", now.elapsed());

    now = Instant::now();
    part2(&content)?;
    println!("process took {:?}", now.elapsed());

    return Ok(());
}
