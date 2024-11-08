use rand::prelude::*;

#[derive(Debug,PartialEq)]
enum Cell{
    Empty,
    Filled,
    X,
    O
}

impl std::fmt::Display for Cell{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self{
            Self::Empty => '.',
            Self::Filled => '=',
            Self::X => 'X',
            Self::O => 'O'
        };
        write!(f, "{}",symbol)
    }
}

#[derive(Debug)]
enum Board{
    Terminal(Cell),
    NonTerminal{
        top: Box<Self>,
        mid: Box<Self>,
        bot: Box<Self>,
    }
}

impl Board{
    fn new(dimension: u8) -> Self{
        if dimension == 0{
            return Self::Terminal(Cell::Empty)
        }
        
        Self::NonTerminal{
            top: Box::new(Self::new(dimension-1)),
            mid: Box::new(Self::new(dimension-1)),
            bot: Box::new(Self::new(dimension-1)),
        }
    }
    
    fn get(&self, coord: &[i8]) -> &Cell{
        match self{
            Self::Terminal(n) => n,
            Self::NonTerminal{
                top,
                mid,
                bot,
            } => {
                let (first,rest) = coord.split_first().expect("coord to small");
                match first{
                    1 => top.get(rest),
                    0 => mid.get(rest),
                    -1=> bot.get(rest),
                    _ => todo!(),
                }
            }
        }
    }
    
    fn set(&mut self, coord: &[i8], item: Cell){
        match self{
            Self::Terminal(n) => *n=item,
            Self::NonTerminal{
                top,
                mid,
                bot,
            } => {
                match coord.get(0).expect("todo"){
                    1 => top.set(&coord[1..],item),
                    0 => mid.set(&coord[1..],item),
                    -1=> bot.set(&coord[1..],item),
                    _ => todo!(),
                }
            }
        }
    }
    fn get_dimension(&self) -> u8{
        self.get_recursive_dimension(0)
    }
    fn get_recursive_dimension(&self,previous_dimension: u8) -> u8{
        match self{
            Self::Terminal(_) => previous_dimension,
            Self::NonTerminal{
                top, ..
            } => top.get_recursive_dimension(previous_dimension + 1)
        }
    }
    
    fn generate_list_of_odd_coordinates(&self) -> Vec<Vec<i8>>{
        let mut list_of_coords = Vec::new();
        let mut changing_coordinate = Vec::new();
        Self::generate_coordinate(&mut list_of_coords, &mut changing_coordinate, self.get_dimension()/2);
        list_of_coords.reverse();
        list_of_coords
    }
    
    fn generate_list_of_even_coordinates(&self) -> Vec<Vec<i8>>{
        let mut list_of_coords = Vec::new();
        let mut changing_coordinate = Vec::new();
        Self::generate_coordinate(&mut list_of_coords, &mut changing_coordinate, self.get_dimension().div_ceil(2));
        list_of_coords
    }

    fn generate_list_of_coordinates(&self, even: bool) -> Vec<Vec<i8>>{
        let mut list_of_coords = Vec::new();
        let mut changing_coordinate = Vec::new();
        let dimensions = (self.get_dimension() + if even{1}else{0}) / 2;
        Self::generate_coordinate(&mut list_of_coords, &mut changing_coordinate, dimensions);
        if !even{list_of_coords.reverse();}
        list_of_coords
    }
    
    fn generate_coordinate(list_of_coordinates: &mut Vec<Vec<i8>>, current_coordinate: &mut Vec<i8>, dimensions_left: u8){
        if dimensions_left == 0{
            list_of_coordinates.push(current_coordinate.to_owned());
        }else{
            for n in -1..=1{
                current_coordinate.push(n);
                Self::generate_coordinate(list_of_coordinates, current_coordinate, dimensions_left - 1);
                current_coordinate.pop();
            }
        }
    }
    
    fn print(&self){
        for odd_coordinate in self.generate_list_of_coordinates(false){
            for even_coordinate in self.generate_list_of_coordinates(true){
                let mut final_coordinates = Vec::new();
                
                let mut current_odd_coordinate = odd_coordinate.clone();
                let mut current_even_coordinate = even_coordinate.clone();
                
                for _ in 0..self.get_dimension(){
                    let even = current_even_coordinate.pop();
                    let odd = current_odd_coordinate.pop();
                    match even{
                        Some(n) => final_coordinates.push(n),
                        None => ()
                    }
                    
                    match odd {
                        Some(n) => final_coordinates.push(n),
                        None => ()
                    }
                }
                
                let cell = self.get(final_coordinates.as_slice());
                
                print!("{cell}");
                
                print!(" ");
                for even in even_coordinate.iter().rev(){
                    if *even == 1{
                        print!("  ")
                    }else{
                        break;
                    }
                }
            }
            println!();
            for odd in odd_coordinate.iter().rev(){
                if *odd == -1{
                    println!()
                }else{
                    break;
                }
            }
        }
    }
}

fn main(){
    let mut board = Board::new(4);
    
    let dimension = board.get_dimension() as usize;
    
    let mut generator = rand::thread_rng();
    
    for _ in 0..(3usize).pow(dimension as u32 - 1){
        let mut coord = vec![0i8;dimension];
        
        loop{
            for mut n in coord.iter_mut(){
                *n = generator.gen_range(-1..=1);
            }
            if board.get(coord.as_slice()) != &Cell::Filled{
                break
            }
        }
        let cell = Cell::Filled;
        board.set(coord.as_slice(),cell);
    }
    
    board.print();
}

