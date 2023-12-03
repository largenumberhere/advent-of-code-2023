use std::io::BufRead;
use std::panic::set_hook;

fn main() {
    let arg2 = std::env::args()
        .skip(1)
        .next()
        .expect("Usage: ./main path-to-file.txt");



    let file_contents = std::io::BufReader::new(
        std::fs::File::open(arg2).expect("Failed to open file")
    );

    // let file_contents = std::io::read_to_string(
    //     std::fs::File::open(arg2)
    //         .expect("Failed to open file")
    //     )
    //     .expect("Io erorr");

    let lines = file_contents.lines();

    // parse all the games from the file
    let games = {
        let mut games = Games::new();
        let mut game = Game::new();
        let mut set = Set::new();

        for game_string in lines {
            let game_string = game_string.expect("IO error");

            let mut line_chars = game_string.chars().peekable();

            // get game number
            let game_number: u32 = {
                let mut game_digits = Vec::new();

                assert_eq!(line_chars.next(), Some('G'));
                assert_eq!(line_chars.next(), Some('a'));
                assert_eq!(line_chars.next(), Some('m'));
                assert_eq!(line_chars.next(), Some('e'));
                assert_eq!(line_chars.next(), Some(' '));


                loop {
                    let line_letter = line_chars.next().expect("Missing game number");
                    if line_letter == ':' {
                        break;
                    }

                    game_digits.push(line_letter);

                }

                // for line_letter in line_chars.next()  {
                //     println!("line char: {}", line_letter);
                //     if line_letter != ':' {
                //         game_digits.push(line_letter);
                //     }
                //     else {
                //         break;
                //     }
                // }

                let string: String = game_digits.iter().collect();
                let game_num = string.parse::<u32>()
                    .unwrap_or_else(|e|
                        panic!("Error: '{:?}'.\nRest of string: {:?}", e, line_chars.clone().collect::<Vec<_>>())
                    );

                //assert_eq!(line_chars.next(), Some(':'), "game digits: {:?}",game_digits.clone().iter().collect::<Vec<_>>());


                // let mut game_num = None;
                // let mut last = None;
                // let mut current = None;
                // while let Some(c) = line_chars.next() {
                //     // update last
                //     last = current;
                //
                //     // update current
                //     current = Some(c);
                //
                //     if c == ':' {
                //         // end of game declaration
                //         game_num = last;
                //         break;
                //     }
                // }

                game_num
            };

            // parse game number to number
            // let game_number: char = game_number.expect("No game number found");
            // let game_number: u32 = char::to_digit(game_number, 10)
            //     .expect("Game was not a number");
            //println!("parsing game '{:?}'", game_number.clone());

            // check for separator
            let space = line_chars.next();
            assert_eq!(space, Some(' '), "Error. Remaining: {:?}", line_chars.clone().collect::<Vec<_>>());


            game.clear();

            // parse sets
            'outer: loop {
                // reuse the vec to avoid memory reallocation
                set.clear();

                //println!("parsing set");

                // parse colors
                'inner: loop {
                    // check for number
                    let number = line_chars.next().expect("Sudden end of input");
                    let mut number = number.to_digit(10).expect("Not a number");

                    // check for 2nd number
                    if line_chars.peek().is_some_and(|c| c.is_digit(10)) {
                        let num2 = line_chars.next()
                            .unwrap()
                            .to_digit(10)
                            .unwrap();

                        number *= 10;
                        number += num2;
                    }


                    // check for separator
                    let space = line_chars.next();
                    assert_eq!(space, Some(' '), "rem: {:?}", line_chars.collect::<Vec<_>>());

                    // check the begining of a color name
                    let mut expected_color;
                    match line_chars.next().expect("Sudden end of input while expecting color name") {
                        'r' => {
                            expected_color = CubeColor::Red;
                        }

                        'g' => {
                            expected_color = CubeColor::Green;
                        }

                        'b' => {
                            expected_color = CubeColor::Blue;
                        }

                        _ => {
                            panic!("Invalid color name")
                        }
                    }

                    // check for rest of color name
                    match expected_color {
                        CubeColor::Red => {
                            assert_eq!(line_chars.next(), Some('e'));
                            assert_eq!(line_chars.next(), Some('d'));
                        }
                        CubeColor::Green => {
                            assert_eq!(line_chars.next(), Some('r'));
                            assert_eq!(line_chars.next(), Some('e'));
                            assert_eq!(line_chars.next(), Some('e'));
                            assert_eq!(line_chars.next(), Some('n'));
                        }
                        CubeColor::Blue => {
                            assert_eq!(line_chars.next(), Some('l'));
                            assert_eq!(line_chars.next(), Some('u'));
                            assert_eq!(line_chars.next(), Some('e'));
                        }
                    }

                    // print!("color: {:?} ", expected_color);
                    // println!("count: {:?}", number);

                    // check for more colors
                    match line_chars.next() {
                        Some(',') => {
                            // new color expected
                            // finish current color
                            set.push((expected_color, number));

                            // check for separator
                            let space = line_chars.next();
                            assert_eq!(space, Some(' '));

                            continue;
                        }
                        Some(';') => {
                            // new set
                            // finish current color
                            set.push((expected_color, number));
                            // finish current set
                            game.push(set.clone());

                            // check for separator
                            let space = line_chars.next();
                            assert_eq!(space, Some(' '));

                            break 'inner;
                        }
                        None => {
                            // end of line.

                            // finish current color
                            set.push((expected_color, number));

                            // finish current set
                            game.push(set.clone());

                            // Finish current game
                            games.push((game_number, game.clone()));

                            break 'outer;
                        }
                        _ => {
                            // Should be the end of the line or comma, semicolon
                            panic!("Expected end of line, comma or semicolon. remaining: {:?}", line_chars.collect::<Vec<_>>());
                        }
                    }
                }
            }
        }

        games
    };

    for (i, g) in games.iter() {
        println!("game {}: {:?}", i, g);
    }

    // example consts
    const red_limit: u32 = 12;
    const green_limit: u32 = 13;
    const blue_limit: u32 = 14;


    let games_possible = {
        let mut games_possible = Vec::new();
        for (game_number, game) in games {
            if is_possible(red_limit, green_limit, blue_limit, game) {
                games_possible.push(game_number);
            }
        }

        games_possible
    };

    let sum: u32 = games_possible.iter().sum();

    println!("possible games sum: {}", sum);

}

fn is_possible(red_max: u32, green_max: u32, blue_max: u32, game: Game) -> bool {
    for set in game {
        // color totals for set
        let (counted_red, counted_green, counted_blue) = {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for (color, count) in set {
                match color {
                    CubeColor::Red => {
                        red += count;
                    }
                    CubeColor::Green => {
                        green += count;
                    }
                    CubeColor::Blue => {
                        blue += count;
                    }
                }
            }
            (red, green, blue)
        };

        if counted_red > red_max || counted_blue > blue_max || counted_green > green_max {
            return  false;
        }
    }

    return  true;
}

#[derive(Debug, Clone)]
enum CubeColor {
    Red,
    Green,
    Blue
}


type Set = Vec<(CubeColor, u32)>;
type Game = Vec<Set>;
type Games = Vec<(u32, Game)>;