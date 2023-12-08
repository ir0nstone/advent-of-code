use std::fs::File;
use std::io::{BufRead, BufReader};

struct Set {
    red: i8,
    blue: i8,
    green: i8
}

impl Set {
    fn new(red: i8, green: i8, blue: i8) -> Set {
        Set {
            red, green, blue
        }
    }

    fn can_play(&self, red_cnt: i8, blue_cnt: i8, green_cnt: i8) -> bool {
        (red_cnt >= self.red) & (blue_cnt >= self.blue) & (green_cnt >= self.green)
    }

    fn power(&self) -> i32 {
        return self.red as i32 * self.green as i32 * self.blue as i32;
    }
}

struct Game {
    id: i8,
    sets: Vec<Set>
}

impl Game {
    fn new(id: i8, sets: Vec<Set>) -> Game {
        return Game {
            id, sets
        }
    }

    fn can_play(&self, red_cnt: i8, blue_cnt: i8, green_cnt: i8) -> bool {
        for set in &self.sets {
            if !set.can_play(red_cnt, blue_cnt, green_cnt) {
                return false;
            }
        }

        return true;
    }

    fn minimum_set(&self) -> Set {
        let mut min_red: i8 = 0;
        let mut min_green: i8 = 0;
        let mut min_blue: i8 = 0;

        for s in &self.sets {
            if s.red > min_red {
                min_red = s.red
            }

            if s.green > min_green {
                min_green = s.green
            }

            if s.blue > min_blue {
                min_blue = s.blue
            }
        }

        return Set::new(min_red, min_green, min_blue);
    }

    fn power_of_minimum(&self) -> i32 {
        self.minimum_set().power()
    }
}

pub fn do_task() {
    let games: Vec<Game> = parse_file();

    let mut total_id: i32 = 0;
    let mut total_power: i32 = 0;

    for g in games {
        if g.can_play(12, 13, 14) {
            total_id += g.id as i32;
        }

        total_power += g.power_of_minimum();
    }

    println!("{}", total_id);
    println!("{}", total_power);
}

fn parse_file() -> Vec<Game> {
    let f = File::open("src/inputs/day2").expect("Unable to open file");
    let reader = BufReader::new(f);

    let mut games: Vec<Game> = Vec::new();

    for (i, l) in reader.lines().enumerate() {
        let line = l.unwrap();

        let game_data = line.split(": ").nth(1);

        match game_data {
            Some(game_data) => {
                games.push(
                    parse_game((i+1) as i8, game_data)
                );
            },
            None => println!("Data does not fit the pattern!")
        }
    }

    return games;
}

fn parse_game(id: i8, game_data: &str) -> Game {
    let parts: Vec<&str> = game_data.split("; ").collect();
    let mut sets: Vec<Set> = Vec::new();

    for p in parts {
        sets.push(
            parse_set(p)
        )
    }

    return Game::new(id, sets);
}

fn parse_set(set_data: &str) -> Set {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;


    for colour in set_data.split(", ") {
        let parts: Vec<&str> = colour.split_whitespace().collect();

        if let Some(count_str) = parts.get(0) {
            if let Some(color) = parts.get(1) {
                if let Ok(count) = count_str.parse::<i8>() {
                    match *color {
                        "red" => red = count,
                        "green" => green = count,
                        "blue" => blue = count,
                        _ => println!("Unknown color: {}", color),
                    }
                } else {
                    println!("Invalid count: {}", count_str);
                }
            }
        }
    }

    return Set::new(red, green, blue);
}
