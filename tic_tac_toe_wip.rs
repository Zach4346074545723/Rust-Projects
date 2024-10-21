#[derive(Debug)]
enum Board{
    Terminal(u8),
    NonTerminal{
        top: Box<Self>,
        mid: Box<Self>,
        bot: Box<Self>,
    }
}

impl Board{
    fn new(dimension: u8) -> Self{
        if dimension == 0{
            return Self::Terminal(0)
        }
        
        Self::NonTerminal{
            top: Box::new(Self::new(dimension-1)),
            mid: Box::new(Self::new(dimension-1)),
            bot: Box::new(Self::new(dimension-1)),
        }
    }
    
    fn get(&self, coord: &[i8]) -> &u8{
        match self{
            Self::Terminal(n) => n,
            Self::NonTerminal{
                top,
                mid,
                bot,
            } => {
                let (first,rest) = coord.split_first().expect("todo");
                match first{
                    1 => top.get(rest),
                    0 => mid.get(rest),
                    -1=> bot.get(rest),
                    _ => todo!(),
                }
            }
        }
    }
    
    fn set(&mut self, coord: &[i8], item: u8){
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
}

impl std::fmt::Display for Board{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        match self{
            Self::Terminal(n) => write!(f,"{n}"),
            Self::NonTerminal{top,mid,bot} => write!(f, "[{}][{}][{}]", top, mid, bot)
        }
        
    }
}

fn main(){
    let mut board = Board::new(3);
    
    println!("{board}");
    
    println!("{}",board.get(&[0,0,0]));
    
    board.set(&[0,0,0],1);
    
    println!("{board}");
    
    println!("{}",board.get(&[0,0,0]))
}
