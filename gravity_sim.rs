use rand;

fn main(){
    let mut universe = Vec::new();
    for _ in 0..10{
        universe.push(
            Particle{
                position: Point(rand::random(),rand::random())*100.0,
                velocity: Point(rand::random(),rand::random()),
                mass: rand::random().powi(4)*10.0
            }
        )
    }
    
    
    for _ in 0..100{
        let mut new_universe = Vec::new();
        for particle in universe{
            new_universe.push(particle.step(universe.as_slice()))
        }
        universe = new_universe;
    }
}

fn print_universe(universe: &[Particle]){
    let screen: Vec<Vec<char>> = vec![vec![' ';50];50];
    
    for particle in universe{
        
    }
}

const ASCII_GRADIENT: Vec<char> = vec!['.',',','~','o','@'];


#[derive(Clone,Copy)]
struct Particle{
    position: Point,
    velocity: Point,
    mass: f64
}

const TIME_STEP: f64 = 0.01;

impl Particle{
    fn gravitational_acceleration(&self, other: &Self) -> Point{
        let position_difference = other.position - self.position;
        let acceleration_amount = other.mass/(position_difference.0.powi(2) + position_difference.1.powi(2));
        let accel_direction = position_difference.0.atan2(position_difference.1);
        
        Point(accel_direction.cos(), accel_direction.sin()) * acceleration_amount
    }
    
    fn simple_gravitational_acceleration(position_difference: Point, mass_of_other_body: f64) -> Point{
        let acceleration_amount = mass_of_other_body/(position_difference.0.powi(2) + position_difference.1.powi(2));
        position_difference.normalize() * acceleration_amount
    }
    
    fn position_step(&self, universe: &[Self]) -> Point{
        let mut accumulated_acceleration = Point(0.0,0.0);
        for particle in universe{
            let acceleration = self.gravitational_acceleration(&particle);
            accumulated_acceleration = accumulated_acceleration + acceleration
        }
        
        self.position + self.velocity * TIME_STEP + accumulated_acceleration * (TIME_STEP.powi(2) / 2.0)
    }
    
    fn velocity_step(&self, universe: &[Self]) -> Point{
        let mut accumulated_acceleration = Point(0.0,0.0);
        for particle in universe{
            let acceleration = self.gravitational_acceleration(&particle);
            accumulated_acceleration = accumulated_acceleration + acceleration
        }
        let mut accumulated_acceleration_after_position_step = Point(0.0,0.0);
        for (index,particle) in universe.iter().enumerate(){
            let new_pos = particle.position_step(&universe);
            let acceleration = Self::simple_gravitational_acceleration(new_pos - self.position,universe[index].mass);
            accumulated_acceleration_after_position_step = accumulated_acceleration_after_position_step + acceleration
        }
        
        self.velocity + (accumulated_acceleration + accumulated_acceleration_after_position_step) * (TIME_STEP/2.0)
    }
    
    fn step(&self,universe: &[Self]) -> Self{
        let new_pos = self.position_step(universe);
        let new_vel = self.velocity_step(universe);
        
        Self{
            position: new_pos,
            velocity: new_vel,
            mass: self.mass
        }
    }
}

#[derive(Clone,Copy)]
struct Point(f64,f64);

impl Point{
    fn magnitude(&self) -> f64{
        self.0.hypot(self.1)
    }
    fn normalize(self) -> Self{
        self/self.magnitude()
    }
}

impl std::ops::Add for Point{
    type Output = Self;
    fn add(self,other:Self) -> Self::Output{
        Self(
            self.0 + other.0,
            self.1 + other.1
        )
    }
}
impl std::ops::Neg for Point{
    type Output = Self;
    fn neg(self) -> Self::Output{
        Self(-self.0,-self.1)
    }
}
impl std::ops::Sub for Point{
    type Output = Self;
    fn sub(self,other:Self) -> Self::Output{
        self + -other
    }
}
impl std::ops::Mul<f64> for Point{
    type Output = Self;
    fn mul(self,scalar:f64) -> Self::Output{
        Self(
            self.0 * scalar,
            self.1 * scalar
        )
    }
}
impl std::ops::Div<f64> for Point{
    type Output = Self;
    fn div(self,divisor:f64) -> Self::Output{
        Self(
            self.0 / divisor,
            self.1 / divisor
        )
    }
}
