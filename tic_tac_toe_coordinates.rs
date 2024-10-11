fn main(){
    let mut init_v_coord = Vec::new();
    for _ in 0..(D/2){
        init_v_coord.push(0)
    }
    
    print_complete(D,D,init_v_coord.as_slice())
}

const D: u32 = 3;

fn print_row(horizontal_dimension: u32, horizontal_coordinates: &[i8], vertical_coordinates: &[i8]){
    if horizontal_dimension.div_ceil(2) == 0{
        print!("[");
        for index in 0..horizontal_coordinates.len(){ // h_coord.len() will always be greater than v_coord.len()
            print!("{} ",horizontal_coordinates[index]);
            
            match vertical_coordinates.get(index){
                Some(c) => print!("{c} "),
                None => ()
            }
        }
        print!("]");
        return
    }
    
    for x in -1..=1{
        let mut new_coords = horizontal_coordinates.to_owned();
        
        new_coords[((horizontal_dimension.div_ceil(2))) as usize - 1] = x;
        
        let h_dim = match horizontal_dimension.checked_sub(2){
            Some(d) => d,
            None => 0
        };
        
        print_row(h_dim,&new_coords,vertical_coordinates)
    }
    
    print!("  ")
}

fn print_complete(total_dimensions: u32, dimensions_left: u32, vertical_coordinates: &[i8]){
    if dimensions_left/2 == 0{
        let mut init_h_coord = Vec::new();
        for _ in 0..(total_dimensions.div_ceil(2)){
            init_h_coord.push(0)
        }
        print_row(total_dimensions,init_h_coord.as_slice(),vertical_coordinates);
        println!();
        return
    }
    
    for y in -1..=1{
        let mut new_coords = vertical_coordinates.to_owned();
        
        new_coords[(dimensions_left/2) as usize - 1] = -y;
        
        print_complete(total_dimensions,dimensions_left-2,&new_coords)
    }
    
    println!()
}
