#[derive(Debug)]
enum Cell{
    Empty,
    Filled,
    X,
    O
}

impl std::fmt::Display for Cell{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self{
            Self::Empty => ' ',
            Self::Filled => '#',
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
                let (first,rest) = coord.split_first().expect("todo");
                match first{
                    1 => top.set(rest,item),
                    0 => mid.set(rest,item),
                    -1=> bot.set(rest,item),
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
    
    fn generate_coordinate(list_of_coordinates: &mut Vec<Vec<i8>>, current_coordinate: &mut Vec<i8>, dimensions_left: u8){
        if dimensions_left == 0{
            list_of_coordinates.push(current_coordinate.to_owned());
            return
        }
        
        for n in -1..=1{
            current_coordinate.push(n);
            Self::generate_coordinate(list_of_coordinates, current_coordinate, dimensions_left - 1);
            current_coordinate.pop();
        }
    }
    
    fn print(&self){
        for odd_coordinate in self.generate_list_of_odd_coordinates(){
            for even_coordinate in self.generate_list_of_even_coordinates(){
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
                
                print!("[{cell}]");
                
                if even_coordinate.last() == Some(&1i8){
                    print!("  ")
                }
            }
            
            if odd_coordinate.last() == Some(&-1i8){
                println!()
            }
            println!()
        }
    }
}

fn main(){
    let mut board = Board::new(3);
    
    board.set(&[0,0,0],Cell::Filled);
    board.set(&[0,1,0],Cell::X);
    board.set(&[1,0,1],Cell::O);
    board.set(&[0,1,-1],Cell::X);
    
    board.print();
}
