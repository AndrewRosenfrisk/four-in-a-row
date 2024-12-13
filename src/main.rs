use core::fmt;
use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{
        Clear,
        ClearType::{All, Purge},
        DisableLineWrap,
    },
};
use std::{
    collections::HashMap,
    io::{stdin, stdout},
};

const PIECE: &str = "●";
const WIDTH: u16 = 7;
const HEIGHT: u16 = 6;
const HEADER: (&str, Color) = ("+1234567+", Color::Blue);
const FOOTER: (&str, Color) = ("+-------+", Color::Blue);
const MARGIN: (&str, Color) = ("|", Color::Blue);

fn main() {
    let neighbors = vec![
        vec![(-3, -3), (-2, -2), (-1, -1), (0, 0), (1, 1), (2, 2), (3, 3)],
        vec![(-3, 3), (-2, 2), (-1, 1), (0, 0), (1, -1), (2, -2), (3, -3)],
        vec![(-3, 0), (-2, 0), (-1, 0), (0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(0, 3), (0, 2), (0, 1), (0, 0)],
    ];

    let mut board: HashMap<Point, Player> = HashMap::new();
    let mut current_player = Player::ONE;
    'turns: loop {
        execute!(stdout(), Clear(All), Clear(Purge), DisableLineWrap, Hide).unwrap();
        display_board(&board);

        if board.len() == (WIDTH * HEIGHT) as usize {
            display_board(&board);
            print_msg(0, HEIGHT + 3, Color::White, "It's a tie!\n");
            break 'turns;
        }
        let player_move = get_player_input(&board, &current_player);

        if player_move.is_none() {
            println!("Thanks for playing.");
            break 'turns;
        } else if player_move.is_some() {
            let point = player_move.unwrap();
            board.insert(point, current_player);

            for list in &neighbors {
                if game_over(&board, current_player, point, &list) {
                    display_board(&board);
                    println!("Player {} has won!", current_player);
                    break 'turns;
                }
            }
        }

        current_player = match current_player {
            Player::ONE => Player::TWO,
            Player::TWO => Player::ONE,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: u16,
    y: u16,
}

#[derive(PartialEq, Clone, Copy)]
enum Player {
    ONE,
    TWO,
}
impl fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::ONE => write!(f, "one"),
            Player::TWO => write!(f, "two"),
        }
    }
}

fn display_board(board: &HashMap<Point, Player>) {
    print_msg(0, 0, HEADER.1, HEADER.0);

    for y in 1..=HEIGHT {
        for x in 0..=(WIDTH + 1) {
            let value;
            let color;

            if x == 0 || x == WIDTH + 1 {
                value = MARGIN.0;
                color = MARGIN.1;
            } else {
                value = PIECE;
                color = match board.get(&Point { x, y }) {
                    Some(player) => {
                        if *player == Player::ONE {
                            Color::Green
                        } else {
                            Color::Magenta
                        }
                    }
                    None => Color::Black,
                }
            }
            print_msg(x, y, color, value);
        }
    }
    print_msg(0, HEIGHT + 1, FOOTER.1, FOOTER.0);
}

fn get_player_input(board: &HashMap<Point, Player>, player: &Player) -> Option<Point> {
    print_msg(0, HEIGHT + 3, Color::White, "");
    println!("Player {}, enter a column number or [Q]uit: ", player);
    let mut x;

    'input: loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input = input.trim().to_uppercase();

        if input == "Q" {
            return None;
        }
        if let Ok(n) = input.parse::<u16>() {
            if n > 0 && n <= 7 {
                x = n;
            } else {
                println!("Please enter a positive whole number from 1 - 7:");
                continue 'input;
            }
        } else {
            println!("Please enter a positive whole number from 1 - 7:");
            continue 'input;
        }

        let count = (0..=HEIGHT)
            .filter(|y| board.contains_key(&Point { x, y: *y }))
            .count();

        if count == HEIGHT as usize {
            println!("Column is full. Please try again:");
            continue 'input;
        } else {
            let y = HEIGHT - count as u16;
            return Some(Point { x, y });
        }
    }
}

fn game_over(
    board: &HashMap<Point, Player>,
    current_player: Player,
    last_point: Point,
    neighbors: &Vec<(i16, i16)>,
) -> bool {
    let mut count = 0;

    for pair in neighbors {
        if let (Some(x), Some(y)) = (
            last_point.x.checked_add_signed(pair.0),
            last_point.y.checked_add_signed(pair.1),
        ) {
            if x > 0 && y > 0 {
                if let Some(neighbor) = board.get(&Point { x, y }) {
                    if *neighbor == current_player {
                        count += 1;
                    } else {
                        count = 0;
                    }
                    if count == 4 {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn print_msg(x: u16, y: u16, color: Color, msg: &str) {
    execute!(
        stdout(),
        MoveTo(x, y),
        SetForegroundColor(color),
        Print(msg)
    )
    .unwrap();
}
