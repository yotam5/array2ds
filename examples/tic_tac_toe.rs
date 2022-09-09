use array2ds::array2d::*;
use std::io::stdin;

struct Square {
    data: char,
}

impl Default for Square {
    fn default() -> Self {
        Square { data: '#' }
    }
}

fn main() {
    run_game();
}

fn run_game() {
    let mut board: Array2d<Square> = Array2d::filled_with_default(3, 3);
    println!("enter location as: row col, for example: `0 1`, row: 0 column 1");
    print_board(&board);

    let mut winner = false;
    let mut current_player = 'X';

    while !winner {
        let (row, column) = read_grid_location();

        if board[[row, column]].data == '#' {
            board[[row, column]].data = current_player;

            match current_player {
                'X' => current_player = 'O',
                'O' => current_player = 'X',
                _ => panic!("invalid data for square"),
            }
            println!();
            print_board(&board);
            winner = check_horizontal(&board)  || check_vertical(&board) || check_diagnol(&board);
        } else {
            println!("invalid location, try another");
        }
    }
}

fn read_grid_location() -> (usize, usize) {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let buffer = buffer.trim();
    let splitted: Vec<&str> = buffer.split(' ').collect();
    if splitted.len() < 2 {
        panic!("enter data as `row col` ");
    }
    let r = splitted[0].parse::<usize>().unwrap();
    let c = splitted[1].parse::<usize>().unwrap();
    (r, c)
}

fn check_diagnol(board: &Array2d<Square>) -> bool{
    
    let mid = &board[(1,1)];
    if mid.data == '#'{
        return false
    }
    if board[[0,0]].data == mid.data && mid.data == board[(2,2)].data{
        return true
    }
    if board[[2,0]].data == mid.data && mid.data == board[(0,2)].data{
        return true
    }
    false
}

fn check_vertical(board: &Array2d<Square>) -> bool {
    for col in 0..3 {
        let mut x_count = 0;
        let mut o_count = 0;
        for row in 0..3 {
            match board[[row, col]].data {
                'X' => x_count += 1,
                'O' => o_count += 1,
                _ => {}
            }
        }

        if x_count == 3 || o_count == 3 {
            return true;
        }
    }
    false
}

fn check_horizontal(board: &Array2d<Square>) -> bool {
    for row in board.iter_rows() {
        let mut o_count = 0;
        let mut x_count = 0;
        for player in row {
            match player.data {
                'O' => o_count += 1,
                'X' => x_count += 1,
                _ => {}
            }
        }
        if o_count == 3 || x_count == 3 {
            return true;
        }
    }
    false
}

fn print_board(board: &Array2d<Square>) {
    for row in board.iter_rows() {
        for square in row {
            print!("{}", square.data);
        }
        println!();
    }
}
