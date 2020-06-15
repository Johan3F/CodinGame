use std::io;
use std::fmt;
use std::cmp;

use std::f32::consts::PI;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

// #########################################
// ##               Vector                ##
// #########################################
#[derive(Clone)]
struct Vector{
    x: f32,
    y: f32,
}
impl Vector{
    fn new(pos: &Position) -> Vector {
        Vector{
            x: pos.x.clone() as f32,
            y: pos.y.clone() as f32,
        }
    }
    fn zero() -> Vector {
        Vector{
            x: 0.0,
            y: 0.0,
        }
    }
    fn is_zero(&self) -> bool {
        (self.x == 0.0) && (self.y == 0.0)
    }
    fn add(&self, other: &Vector) -> Vector {
        Vector{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    fn substract(&self, other: &Vector) -> Vector {
        Vector{
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
    fn dot_product(&self, other: &Vector) -> f32{
        (self.x*other.x) + (self.y*other.y)
    }
    fn determinant(&self, other: &Vector) -> f32 {
        (self.x*other.y) - (self.y*other.x)
    }
    fn module(&self) -> f32{
        (self.x.powf(2.0)+ self.y.powf(2.0)).sqrt()
    }
    fn rotate(&self, angle: f32) -> Vector {
        let cos = angle.cos();
        let sin = angle.sin();
        Vector{
            x: (self.x*cos) - (self.y*sin),
            y: (self.x*sin) + (self.y*cos),
        }
    }
    fn get_angle(&self, other: & Vector) -> f32 {
        self.determinant(other).atan2(self.dot_product(other))
    }
    fn get_unitary(&self) -> Vector {
        Vector{
            x: self.x / self.module(),
            y: self.y / self.module(),
        }
    }
    fn multiply(&self, factor: f32) -> Vector{
        Vector {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}
impl fmt::Debug for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vector")
         .field("x", &self.x)
         .field("y", &self.y)
         .finish()
    }
}

fn intersect_lines(a: &Vector, b: &Vector, c: &Vector, d: &Vector) -> Vector {
    // A = a+t*b
    // V = c+u*d
    // u=(bx(cy-ay) +by(ax-cx))/(dx.by-dy.bx)
    // t=(dx(ay-cy) +dy(cx-ax))/(bx.dy-by.dx)
    let ax = a.x;
    let ay = a.y;
    let bx = b.x;
    let by = b.y;
    let cx = c.x;
    let cy = c.y;
    let dx = d.x;
    let dy = d.y;

    let u = (bx*(cy-ay) +by*(ax-cx))/(dx*by-dy*bx);
    let t = (dx*(ay-cy) +dy*(cx-ax))/(bx*dy-by*dx);

    // Apply to line formula, to get the intersection point
    Vector {
        x: ax - bx*t,
        y: ay - by*t,
    }
}

// #########################################
// ##              Position               ##
// #########################################
#[derive(Clone)]
struct Position {
    x: i32,
    y: i32,
}
impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position{
            x, 
            y,
        }
    }
}
impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Position")
         .field("x", &self.x)
         .field("y", &self.y)
         .finish()
    }
}
impl cmp::PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}

// #########################################
// ##               PodInfo               ##
// #########################################
struct PodInfo{
    last_position: Option<Position>,
    position: Option<Position>,
    speed: Vector,
}
impl PodInfo {
    fn new() -> PodInfo {
        PodInfo {
            last_position: None,
            position: None,
            speed: Vector::zero(),
        }
    }
    fn new_position(&mut self, x: i32, y: i32) {
        self.last_position = self.position.take();
        self.position = Some(Position::new(x,y));

        self.speed = match &self.last_position {
            Some(position) => Vector{ x: (self.position.as_ref().unwrap().x-position.x) as f32,
                                      y: (self.position.as_ref().unwrap().y-position.y) as f32},
            None => Vector::zero(),
        };


    }
}
impl fmt::Debug for PodInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PodInfo")
         .field("previous position", &self.position)
         .field("current position", &self.position)
         .field("speed", &self.speed)
         .finish()
    }
}

/**
 * This code automatically collects game data in an infinite loop.
 * It uses the standard input to place data into the game variables such as x and y.
 * YOU DO NOT NEED TO MODIFY THE INITIALIZATION OF THE GAME VARIABLES.
 **/
fn main() {
    // Constants
    let width: f32 = 16000.0;
    let heigh: f32 = 9000.0;
    let diagonal: f32 = (width.powf(2.0)+ heigh.powf(2.0)).powf(1.0/2.0);
    let pod_bubble_size: f32 = 400.0;
    let checkpoint_bubble_size: f32 = 600.0;

    let mut checkpoints = Vec::new();
    let mut first_lap = true;

    let mut used_boost = false;

    let mut player_info = PodInfo::new();
    let mut opponents_info = PodInfo::new();

    let mut message = "";

    // game loop
    loop {
        // +--------------------------+
        // |      Input processing    |
        // +--------------------------+
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs_second = input_line.split(" ").collect::<Vec<_>>();

        player_info.new_position(parse_input!(inputs[0], i32), parse_input!(inputs[1], i32));
        opponents_info.new_position(parse_input!(inputs_second[0], i32), parse_input!(inputs_second[1], i32));

        let checkpoint = Position::new(parse_input!(inputs[2], i32), parse_input!(inputs[3], i32));
        let checkpoint_dist = parse_input!(inputs[4], f32);
        let checkpoint_angle = parse_input!(inputs[5], f32);
        
        // +--------------------------+
        // |  Checkpoints processing  |
        // +--------------------------+
        // Adding next checkpoint to known checkpoints. Getting index of current checkpoint        
        if ! checkpoints.contains(&checkpoint) {
            checkpoints.push(checkpoint.clone());
        }

        let checkpoint_index = checkpoints.iter().position(|r| *r == checkpoint).unwrap();

        // Updating if this is the first lap or not, and getting next checkpoint in case it's possible
        if checkpoint_index < checkpoints.len()-1 && first_lap {
            first_lap = false;
        }
        let next_checkpoint: Option<Position>;
        if first_lap{
            next_checkpoint = None;
        } else {
            if checkpoint_index == checkpoints.len()-1 {
                next_checkpoint = Some(checkpoints[0].clone());
            }else{
                next_checkpoint = Some(checkpoints[checkpoint_index + 1].clone());
            }
        }

        let close_to_checkpoint_threshold = (diagonal/7.0);

        // +--------------------------+
        // |        Debug output      |
        // +--------------------------+
        // eprintln!("Player info        : {:?}", player_info);
        // eprintln!("Player speed       : {:?}", player_info.speed.module());
        // eprintln!("Opponents info     : {:?}", opponents_info);
        // eprintln!("Checkpoint         : {:?}", checkpoint);
        // eprintln!("Next checkpoint    : {:?}", next_checkpoint);
        // eprintln!("Checkpoint_dist    : {}", checkpoint_dist);
        // eprintln!("Checkpoint_angle   : {}", checkpoint_angle);
        // eprintln!("Checkpoints stored : total {}", checkpoints.len());
        // for checkpoint in &checkpoints {
        //     eprintln!(" - checkpoint  : {:?}", checkpoint);
        // }

        // +--------------------------+
        // |     Target calculation   |
        // +--------------------------+
        let target_x;
        let target_y;
        if player_info.speed.is_zero() {
            target_x = checkpoint.x;
            target_y = checkpoint.y;
        }else{
            let player_position = player_info.position.as_ref().unwrap();
            let vector_speed = &player_info.speed;

            let vector_pod_ch = Vector::new(&checkpoint).substract(&Vector::new(&player_position));
            let vector_perpendicular = vector_pod_ch.rotate(PI/2.0);
            let angle = vector_pod_ch.get_angle(&vector_speed);

            
            if angle.abs() < (PI/2.0) {
                let vector_v = vector_speed.clone();
                let vector_v_angle = vector_pod_ch.get_angle(&vector_v);

                // Intersect perpendicular line with vector_v line using the perpendicular vector and the checkpoint
                if  ((vector_perpendicular.x-vector_v.x) == 0.0)
                || ((vector_perpendicular.y-vector_v.y) == 0.0)
                {
                    target_x = checkpoint.x;
                    target_y = checkpoint.y;
                }else{
                    // Intersect:
                    // - (A) Perpendicular line passing by the checkpoint
                    // - (V) The velocity line passing by the pod
                    // A = checkpoint+t*vector_perpendicular
                    // V = player_position+u*vector_v
                    let vector_intersect = intersect_lines(&Vector::new(&checkpoint), 
                                                           &vector_perpendicular,
                                                           &Vector::new(&player_position),
                                                           &vector_v);

                    // Allow only a max of checkpoint_bubble deviation from checkpoint
                    let vector_ch_intersect = vector_intersect.substract(&Vector::new(&checkpoint));
                    let vector_longest_distance_allowed = vector_ch_intersect
                                                                .get_unitary()
                                                                .multiply(checkpoint_bubble_size);

                    eprintln!("vector_longest_distance_allowed : {:?}", vector_longest_distance_allowed.module());
                    eprintln!("vector_intersect                : {:?}", vector_intersect.module());
                    if vector_longest_distance_allowed.module() < vector_ch_intersect.module() {
                        let checkpoint_edge = Vector::new(&checkpoint).add(&vector_longest_distance_allowed);
                        target_x = checkpoint_edge.x as i32;
                        target_y = checkpoint_edge.y as i32;
                    }else{
                        target_x = vector_intersect.x as i32;
                        target_y = vector_intersect.y as i32;
                    }

                    // target_x = vector_intersect.x as i32;
                    // target_y = vector_intersect.y as i32;
                }

                eprintln!("Target              : Vector {{ x: {}, y: {} }}", target_x, target_y);
                eprintln!("vector_pod_ch       : {:?}", vector_pod_ch);
                eprintln!("vector_perpendicular: {:?}", vector_perpendicular);
                eprintln!("Angle (radians)     : {:?}", angle);
                eprintln!("Angle (degrees)     : {:?}", (angle*180.0)/PI);
                eprintln!("Speed vector        : {:?}", vector_speed);
                eprintln!("v vector            : {:?}", vector_v);
            }else{
                target_x = checkpoint.x;
                target_y = checkpoint.y;
            }
        }
        
        // +--------------------------+
        // |     Thrust calculation   |
        // +--------------------------+
        let critical_angle = 70.0;
        let minimun_thrust = 15;
        let thrust;
        // If the angle is above critical, reduce thrust
        if checkpoint_angle.abs() > critical_angle {
            thrust = minimun_thrust;
        }else{
            // If we are too close to the checkpoint...
            if checkpoint_dist < close_to_checkpoint_threshold {
                // ... and too fast! go slow!
                match next_checkpoint {
                    None => {
                        if player_info.speed.module() > 500.0 {
                            thrust = minimun_thrust;
                        }else{
                            thrust = 100;
                        }
                    },
                    Some(next_ch) => {
                        let player_position = player_info.position.as_ref().unwrap();

                        let vector_ch_pod = Vector::new(&checkpoint).substract(&Vector::new(&player_position));
                        let vector_ch_next = Vector::new(&checkpoint).substract(&Vector::new(&next_ch));

                        let angle = (vector_ch_pod.get_angle(&vector_ch_next)*180.0)/PI;
                        eprintln!("curve angle          : {:?}", angle);
                        
                        if      (angle.abs() < (180.0 - critical_angle)) 
                            &&  (player_info.speed.module() > 400.0) {
                            thrust = (angle.abs() / (180.0 - critical_angle)) as i32 + minimun_thrust;
                        }else{
                            thrust = 100;
                        }
                    },
                }
            }else{
                thrust = 100;
            }
        }
        

        // +--------------------------+
        // |      BOOS calculation    |
        // +--------------------------+
        // If we can still use the BOOST and we have distant enough to the next checkpoint, use it
        let use_boost: bool;
        if      (! used_boost) 
            &&  (checkpoint_dist > (diagonal/2.7))
            && (checkpoint_angle.abs() < 15.0)
        {
            use_boost = true;
            used_boost = true;
        }else{
            use_boost = false;
        }
        
        match use_boost{
            true => println!("{} {} {}{}", target_x, target_y, "BOOST", message),
            false => println!("{} {} {}{}", target_x, target_y, thrust, message),
        }
    }
}
