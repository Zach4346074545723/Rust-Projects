fn main() {
    println!("enter the dimension you would like to play in: ");

    let mut board = loop {
        let input = read_input();
        let input = input.trim();
        match input.parse::<u32>() {
            Ok(val) => break Board::new(val),
            Err(err) => {
                println!("{err}");
                continue;
            }
        }
    };
    let mut x_turn = true;
    loop {
        board.print();
        
        if x_turn {
            println!("x's move: ")
        } else {
            println!("o's move: ")
        }
        
        let input = read_input();

        if input.trim() == "exit" {
            return;
        }

        let coordinate = match parse_into_coordinate(input.as_str()) {
            Ok(val) => val,
            Err(err) => {
                println!("{err}");
                continue;
            }
        };
        
        match board.get_cell_mut(coordinate.as_slice()){
            Ok(cell) => {
                if let Cell::Empty = cell{
                    
                }else{
                    println!("Invalid move");
                    continue
                }
                
                *cell = if x_turn { Cell::X } else { Cell::O };
            },
            Err(err) => {
                println!("{err}");
                continue
            }
        }
        
        x_turn = !x_turn
    }
}

fn read_input() -> String {
    use std::io::{stdin, stdout, Write};
    let mut input = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("failed to get input");

    input
}

fn parse_into_coordinate(input: &str) -> Result<Vec<i8>, std::num::ParseIntError> {
    let mut coordinate = Vec::new();

    let input = input.replace(" ", "\n");
    let input = input.trim();

    for number in input.lines() {
        coordinate.push(number.parse::<i8>()?)
    }

    Ok(coordinate)
}

enum Board {
    Terminal(Cell),
    NonTerminal {
        top: Box<Self>,
        middle: Box<Self>,
        bottom: Box<Self>,
    },
}

enum Cell {
    Empty,
    Filled,
    X,
    O,
}

impl Board {
    fn new(dimension: u32) -> Self {
        if dimension - 1 == 0 {
            return Self::Terminal(Cell::Empty);
        }

        Self::NonTerminal {
            top: Box::new(Self::new(dimension - 1)),
            middle: Box::new(Self::new(dimension - 1)),
            bottom: Box::new(Self::new(dimension - 1)),
        }
    }
    
    fn get_cell(&self, coordinate: &[i8]) -> Result<&Cell, &str> {
        let (&first, rest) = coordinate.split_first().ok_or("incorrect coordinate size")?;

        match self {
            Self::Terminal(cell) => Ok(cell),
            Self::NonTerminal {
                top,
                middle,
                bottom,
            } => match first {
                1 => top.get_cell(rest),
                0 => middle.get_cell(rest),
                -1 => bottom.get_cell(rest),

                _ => Err("coordinate out of bounds"),
            },
        }
    }

    fn get_cell_mut(&mut self, coordinate: &[i8]) -> Result<&mut Cell, &str> {
        let (&first, rest) = coordinate.split_first().ok_or("incorrect coordinate size")?;

        match self {
            Self::Terminal(cell) => Ok(cell),
            Self::NonTerminal {
                top,
                middle,
                bottom,
            } => match first {
                1 => top.get_cell_mut(rest),
                0 => middle.get_cell_mut(rest),
                -1 => bottom.get_cell_mut(rest),

                _ => Err("coordinate out of bounds"),
            },
        }
    }
    
    fn get_dimension(&self) -> u32{
        for dimension in 0..=std::u32::MAX{
            match self.get_cell(vec![0i8;dimension as usize].as_slice()) {
                Ok(_) => return dimension,
                Err(_) => continue
            }
        }
        
        0
    }
    
    fn print(&self){
        println!("board dimension: {}",self.get_dimension());
        
        let grid_height = 3u32.pow((self.get_dimension() + 1)/2);
        let grid_width = 3u32.pow(self.get_dimension()/2);
        
        
        let mut output = String::new();
        
        for boards_on_y in 0..=((self.get_dimension()+1)/2){
            
        }
        
        for y in 0..grid_height{
            output.push_str("|");
            for x in 0..grid_width{
                output.push_str(" |");
            }
            output.push_str("\n")
        }
        
        println!("{output}")
    }
    
    fn append_row(display: &mut String){
        
    }
}
