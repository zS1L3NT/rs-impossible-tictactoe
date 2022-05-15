use std::{
    cmp::{max, min},
    io::{stdin, stdout, Write},
    thread::sleep,
    time::Duration,
};

use prettytable::{Cell, Row, Table};

#[derive(Clone, PartialEq)]
enum Tile {
    O,
    X,
    Empty,
}

impl Tile {
    fn to_string(&self) -> &'static str {
        match self {
            Tile::O => "O",
            Tile::X => "X",
            Tile::Empty => " ",
        }
    }
}

#[derive(Clone)]
struct Game {
    first: String,
    tiles: Vec<Tile>,
}

impl Game {
    fn new() -> Self {
        Self {
            first: String::from("u"),
            tiles: vec![Tile::Empty; 9],
        }
    }

    fn get_mut(&mut self, index: i32) -> Option<&mut Tile> {
        self.tiles.get_mut(index as usize)
    }

    fn print(&self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!(
            "{}",
            Table::init(
                self.tiles
                    .chunks(3)
                    .into_iter()
                    .map(|row| {
                        Row::new(
                            row.iter()
                                .map(|tile| Cell::new(tile.to_string()))
                                .collect::<Vec<_>>(),
                        )
                    })
                    .collect::<Vec<_>>(),
            ),
        );
    }

    fn evaluate(&self) -> i32 {
        let tc = self
            .tiles
            .chunks(3)
            .map(|row| row.to_vec())
            .collect::<Vec<_>>();

        // Horizontal Wins
        if tc.iter().any(|row| row.iter().all(|tile| tile == &Tile::O)) {
            return -1;
        }

        if tc.iter().any(|row| row.iter().all(|tile| tile == &Tile::X)) {
            return 1;
        }

        // Vertical Wins
        if (0..3).any(|index| tc.iter().all(|row| row[index] == Tile::O)) {
            return -1;
        }

        if (0..3).any(|index| tc.iter().all(|row| row[index] == Tile::X)) {
            return 1;
        }

        // Diagonal Wins
        if tc[0][0] == Tile::O && tc[1][1] == Tile::O && tc[2][2] == Tile::O {
            return -1;
        }

        if tc[0][2] == Tile::O && tc[1][1] == Tile::O && tc[2][0] == Tile::O {
            return -1;
        }

        if tc[0][0] == Tile::X && tc[1][1] == Tile::X && tc[2][2] == Tile::X {
            return 1;
        }

        if tc[0][2] == Tile::X && tc[1][1] == Tile::X && tc[2][0] == Tile::X {
            return 1;
        }

        return 0;
    }

    fn minmax(&self, is_computer: bool) -> i32 {
        let score = self.evaluate();
        if score != 0 || self.tiles.iter().all(|tile| *tile != Tile::Empty) {
            return score;
        }

        if is_computer {
            let mut best_score = -1;
            for (index, tile) in self.tiles.iter().enumerate() {
                if let Tile::Empty = tile {
                    let mut game = self.clone();
                    *game.get_mut(index as i32).unwrap() = Tile::X;

                    best_score = max(best_score, game.minmax(false));
                }
            }

            return best_score;
        } else {
            let mut best_score = 1;
            for (index, tile) in self.tiles.iter().enumerate() {
                if let Tile::Empty = tile {
                    let mut game = self.clone();
                    *game.get_mut(index as i32).unwrap() = Tile::O;

                    best_score = min(best_score, game.minmax(true));
                }
            }

            return best_score;
        }
    }

    fn get_best_move_index(&self) -> i32 {
        let mut best_score = -1;
        let mut best_move_index = -1;

        for (index, tile) in self.tiles.iter().enumerate() {
            if let Tile::Empty = tile {
                let mut game = self.clone();
                *game.get_mut(index as i32).unwrap() = Tile::X;

                let score = game.minmax(false);
                if score > best_score {
                    best_score = score;
                    best_move_index = index as i32;
                }
            }
        }

        return best_move_index;
    }
}

fn main() {
    let mut game = Game::new();

    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        print!("Who starts first, user or computer? (u/c): ");
        stdout().flush().expect("Failed to flush input");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read input");

        match input.to_ascii_lowercase().trim() {
            "c" => {
                game.print();
                println!("Computer is thinking...");

                let best_move_index = game.get_best_move_index();
                *game.get_mut(best_move_index).unwrap() = Tile::X;
                game.first = String::from("c");

                break;
            }
            "u" => break,
            _ => {
                println!("Input was not u or c");
                continue;
            }
        }
    }

    game.print();

    loop {
        print!("Enter tile number (1-9): ");
        stdout().flush().expect("Failed to flush input");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read input");

        let result = input.trim().parse::<i32>();

        if let Err(_) = &result {
            println!("Input was not a number!");
            continue;
        }

        let number = result.unwrap() - 1;
        if number < 0 || number > 8 {
            println!("Input was not a valid tile!");
            continue;
        }

        let tile = game.get_mut(number).unwrap();
        if let Tile::Empty = tile {
            *tile = Tile::O;
        } else {
            println!("Tile is already taken!");
            continue;
        }

        game.print();
        if game.evaluate() == -1 {
            // LOL THIS WILL NEVER HAPPEN
            println!("You won!");
            break;
        }

        let best_move_index = game.get_best_move_index();
        if best_move_index != -1 {
            *game.get_mut(best_move_index).unwrap() = Tile::X;
            println!("Computer is thinking...");
            sleep(Duration::from_secs(1));
        } else {
            game.print();
            println!("Tie!");
            break;
        }

        game.print();

        if game.evaluate() == 1 {
            println!("You lost!");
            break;
        }

        if game.evaluate() == 0 && game.tiles.iter().all(|tile| *tile != Tile::Empty) {
            println!("Tie!");
            break;
        }
    }
}
