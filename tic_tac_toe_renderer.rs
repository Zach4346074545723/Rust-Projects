#![allow(unused)]
use rand::prelude::*;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Cell {
    Empty,
    Filled,
    X,
    O,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Self::Empty => ".",
            Self::Filled => "=",
            Self::X => "X",
            Self::O => "O",
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Debug)]
enum Board {
    Terminal(Cell),
    NonTerminal([Box<Self>;3]),
}

impl Board {
    fn new(dimension: u8) -> Self {
        if dimension == 0 {
            Self::Terminal(Cell::Empty)
        } else {
            let sub_boards = [
                Box::new(Self::new(dimension-1)),
                Box::new(Self::new(dimension-1)),
                Box::new(Self::new(dimension-1)),
            ];
            Self::NonTerminal(sub_boards)
        }
    }

    fn get(&self, coord: &[u8]) -> &Cell {
        match self {
            Self::Terminal(n) => n,
            Self::NonTerminal(sub_boards) => sub_boards
                .get(*coord.first().expect("coord length too small") as usize)
                .expect("coord values too big")
                .get(&coord[1..]),
        }
    }

    fn set(&mut self, coord: &[u8], item: Cell) {
        match self {
            Self::Terminal(n) => *n = item,
            Self::NonTerminal(sub_boards) => sub_boards
                .get_mut(*coord.first().expect("coord length too small") as usize)
                .expect("coord values too big")
                .set(&coord[1..],item),
        }
    }

    fn get_dimension(&self) -> u8 {
        self.get_recursive_dimension(0)
    }
    fn get_recursive_dimension(&self, previous_dimension: u8) -> u8 {
        match self {
            Self::Terminal(_) => previous_dimension,
            Self::NonTerminal(sub_boards) => sub_boards
                .get(0)
                .expect("unreachable")
                .get_recursive_dimension(previous_dimension + 1),
        }
    }
    
    fn generate_list_of_coordinates(&self, even: bool) -> Vec<Vec<u8>> {
        let mut list_of_coords = Vec::new();
        let mut changing_coordinate = Vec::new();
        let dimensions = (self.get_dimension() + if even { 1 } else { 0 }) / 2;
        Self::generate_coordinate(&mut list_of_coords, &mut changing_coordinate, dimensions);
        if !even {
            list_of_coords.reverse()
        }
        list_of_coords
    }
    fn generate_coordinate(
        list_of_coordinates: &mut Vec<Vec<u8>>,
        current_coordinate: &mut Vec<u8>,
        dimensions_left: u8,
    ) {
        if dimensions_left == 0 {
            list_of_coordinates.push(current_coordinate.to_owned());
        } else {
            for n in 0..3{
                current_coordinate.push(n);
                Self::generate_coordinate(
                    list_of_coordinates,
                    current_coordinate,
                    dimensions_left - 1,
                );
                current_coordinate.pop();
            }
        }
    }
    
    fn display_move(&mut self,move_position: &[u8],symbol: Cell){
        self.set(move_position,symbol);
        let centered_move_position = {
            let mut a = Vec::new();
            
            for n in move_position{
                a.push(*n as i8 -1)
            }
            
            a
        };
        println!("{symbol} was placed at {centered_move_position:?}");
        println!("{self}");
    }
}

impl std::fmt::Display for Board {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for odd_indexed_coordinate in self.generate_list_of_coordinates(false) {
            for even_indexed_coordinate in self.generate_list_of_coordinates(true) {
                let mut final_coordinates = Vec::new();

                let mut current_odd_indexed_coordinate = odd_indexed_coordinate.clone();
                let mut current_even_indexed_coordinate = even_indexed_coordinate.clone();

                for _ in 0..self.get_dimension() {
                    let even = current_even_indexed_coordinate.pop();
                    let odd = current_odd_indexed_coordinate.pop();
                    if let Some(n) = even {
                        final_coordinates.push(n)
                    }
                    if let Some(n) = odd {
                        final_coordinates.push(n)
                    }
                }

                write!(f, "{} ", self.get(final_coordinates.as_slice()))?;
                for even in even_indexed_coordinate.iter().rev() {
                    if *even == 2 {
                        write!(f, "  ")?
                    } else {
                        break;
                    }
                }
            }
            writeln!(f)?;
            for odd in odd_indexed_coordinate.iter().rev() {
                if *odd == 0 {
                    writeln!(f)?
                } else {
                    break;
                }
            }
        }
        Ok(())
    }
}

// fn get_coordinate_input(board: &mut Board) -> Vec<i8>{
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).expect("could not read input");
//     let first = input.first().expect("empty input");
//     match first {
//         'x' => board.display
//     }
// }

fn main() {
    let mut board = Board::new(3);

    let dimension = board.get_dimension() as usize;

    let mut generator = rand::thread_rng();

    let amount = 3usize.pow(dimension as u32) - (2 * dimension * dimension) - 1;

    for _ in 0..amount {
        let mut coord = vec![0u8; dimension];

        loop {
            for n in coord.iter_mut() {
                *n = generator.gen_range(0..3);
            }
            if board.get(coord.as_slice()) != &Cell::Filled {
                break;
            }
        }
        let cell = Cell::Filled;
        board.set(coord.as_slice(), cell);
    }
    
    board.display_move(&[1,1,1],Cell::Filled);
    board.display_move(&[2,2,2],Cell::X);
    board.display_move(&[2,2,1],Cell::O);
    board.display_move(&[1,1,2],Cell::X);
    board.display_move(&[0,0,2],Cell::O);
    board.display_move(&[2,1,2],Cell::X);
    board.display_move(&[0,1,1],Cell::X);
    board.display_move(&[0,2,2],Cell::O);
    board.display_move(&[0,1,0],Cell::X);
    board.display_move(&[0,0,1],Cell::O);
    board.display_move(&[1,2,1],Cell::X);

    print!("{board}")
}
