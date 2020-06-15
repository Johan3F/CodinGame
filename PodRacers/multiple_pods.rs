use std::io;
use std::fmt;
use std::cmp;

use std::f32::consts::PI;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

const BOOSTS_PER_GAME: i32 = 1;

// #########################################
// ##               Vector                ##
// #########################################
#[derive(Clone)]
struct Vector{
    x: f32,
    y: f32,
}
impl Vector{
    fn new(x: f32, y: f32) -> Vector {
        Vector{x, y}
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
impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} x {})", self.x, self.y)
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
// ##               PodInfo               ##
// #########################################
struct PodInfo{
    position: Vector,
    speed: Vector,
    angle: f32,
    next_checkpoint: usize,
    remaining_boosts: i32,
}
impl PodInfo {
    fn new(x: f32, y: f32, speed_x: f32, speed_y: f32, angle: f32, next_checkpoint: usize) -> PodInfo {
        PodInfo {
            position: Vector::new(x, y),
            speed: Vector::new(speed_x, speed_y),
            angle: angle,
            next_checkpoint: next_checkpoint,
            remaining_boosts: BOOSTS_PER_GAME,
        }
    }
}
impl fmt::Display for PodInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "position: {}, speed: {}, angle: {}, next_checkpoint: {}, remaining_boosts: {}",
               self.position,
               self.speed,
               self.angle,
               self.next_checkpoint,
               self.remaining_boosts
               )
    }
}

// #########################################
// ##                 Main                ##
// #########################################
fn main() {
    // Constants
    let width: f32 = 16000.0;
    let heigh: f32 = 9000.0;
    let diagonal: f32 = (width.powf(2.0)+ heigh.powf(2.0)).powf(1.0/2.0);
    let pod_bubble_size: f32 = 400.0;
    let checkpoint_bubble_radius: f32 = 600.0;

    // +--------------------------+
    // |  Initializaytion input   |
    // +--------------------------+
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let laps = parse_input!(input_line, i32);

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let checkpoint_count = parse_input!(input_line, i32);

    let mut checkpoints: Vec<Vector> = Vec::with_capacity(checkpoint_count as usize);

    for index in 0..checkpoint_count{
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.split(" ").collect::<Vec<_>>();

        checkpoints.push(Vector::new(parse_input!(line[0], i32) as f32, 
                                     parse_input!(line[1], i32) as f32));
    }


    let mut player_pods: Vec<PodInfo> = Vec::with_capacity(2);
    let mut enemy_pods: Vec<PodInfo> = Vec::with_capacity(2);


    // game loop
    loop {
        // +--------------------------+
        // |      Input processing    |
        // +--------------------------+
        let first = player_pods.is_empty();
        // Player's pods info
        for index in 0..2 {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let line = input_line.split(" ").collect::<Vec<_>>();

            if first {
                player_pods.push(PodInfo::new(parse_input!(line[0], i32) as f32, 
                                              parse_input!(line[1], i32) as f32,
                                              parse_input!(line[2], i32) as f32,
                                              parse_input!(line[3], i32) as f32,
                                              parse_input!(line[4], i32) as f32,
                                              parse_input!(line[5], i32) as usize));
            }else{
                player_pods[index] = PodInfo::new(parse_input!(line[0], i32) as f32, 
                                                  parse_input!(line[1], i32) as f32,
                                                  parse_input!(line[2], i32) as f32,
                                                  parse_input!(line[3], i32) as f32,
                                                  parse_input!(line[4], i32) as f32,
                                                  parse_input!(line[5], i32) as usize);
            }

        }

        // Enemy's pods info
        for index in 0..2 {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let line = input_line.split(" ").collect::<Vec<_>>();

            if first {
                enemy_pods.push(PodInfo::new(parse_input!(line[0], i32) as f32, 
                                              parse_input!(line[1], i32) as f32,
                                              parse_input!(line[2], i32) as f32,
                                              parse_input!(line[3], i32) as f32,
                                              parse_input!(line[4], i32) as f32,
                                              parse_input!(line[5], i32) as usize));
            }else{
                enemy_pods[index] = PodInfo::new(parse_input!(line[0], i32) as f32, 
                                                  parse_input!(line[1], i32) as f32,
                                                  parse_input!(line[2], i32) as f32,
                                                  parse_input!(line[3], i32) as f32,
                                                  parse_input!(line[4], i32) as f32,
                                                  parse_input!(line[5], i32) as usize);
            }

        }

        for index in 0..2 {
            // +--------------------------+
            // |   Initial calculations   |
            // +--------------------------+
            let player_info = &mut player_pods[index];
            let player_position = &player_info.position;
            let checkpoint = &checkpoints[player_info.next_checkpoint];
            let checkpoint_next = &checkpoints[player_info.next_checkpoint];
            let checkpoint_dist = player_position.substract(&checkpoint).module();

            let vector_pod_ch = checkpoint.substract(&player_position);
            let vector_perpendicular = vector_pod_ch.rotate(PI/2.0);
            let angle = vector_pod_ch.get_angle(&player_info.speed);

            let message = format!(" {}", index);
            eprintln!("Pod {}:", index);
            eprintln!("player_info: {}", player_info);
            
            // +--------------------------+
            // |     Target calculation   |
            // +--------------------------+
            let target_x;
            let target_y;
            if player_info.speed.is_zero() {
                target_x = checkpoint.x as i32;
                target_y = checkpoint.y as i32;
            }else{
                let vector_speed = &player_info.speed;
                
                if angle.abs() < (PI/2.0) {
                    let vector_v = vector_speed.clone();
                    let vector_v_angle = vector_pod_ch.get_angle(&vector_v);

                    // Intersect perpendicular line with vector_v line using the perpendicular vector and the checkpoint
                    if  ((vector_perpendicular.x-vector_v.x) == 0.0)
                    || ((vector_perpendicular.y-vector_v.y) == 0.0)
                    {
                        target_x = checkpoint.x as i32;
                        target_y = checkpoint.y as i32;
                    }else{
                        // Intersect:
                        // - (A) Perpendicular line passing by the checkpoint
                        // - (V) The velocity line passing by the pod
                        // A = checkpoint+t*vector_perpendicular
                        // V = player_position+u*vector_v
                        let vector_intersect = intersect_lines(&checkpoint, 
                                                            &vector_perpendicular,
                                                            &player_position,
                                                            &vector_v);

                        // Allow only a max of checkpoint_bubble deviation from checkpoint
                        let vector_ch_intersect = vector_intersect.substract(&checkpoint);
                        let vector_longest_distance_allowed = vector_ch_intersect
                                                                    .get_unitary()
                                                                    .multiply(checkpoint_bubble_radius);
                        if vector_longest_distance_allowed.module() < vector_ch_intersect.module() {
                            let checkpoint_edge = checkpoint.add(&vector_longest_distance_allowed);
                            target_x = checkpoint_edge.x as i32;
                            target_y = checkpoint_edge.y as i32;
                        }else{
                            target_x = vector_intersect.x as i32;
                            target_y = vector_intersect.y as i32;
                        }
                    }
                }else{
                    target_x = checkpoint.x as i32;
                    target_y = checkpoint.y as i32;
                }
            }
        
            // +--------------------------+
            // |     Thrust calculation   |
            // +--------------------------+
            let critical_angle = 70.0;
            let minimun_thrust = 15;
            let thrust;
            let close_to_checkpoint_threshold = (checkpoint_bubble_radius*2.0)*2.0;
            // If the angle is above critical, reduce thrust
            eprintln!("angle                        : {:?}", angle); 
            eprintln!("checkpoint_dist              : {:?}", checkpoint_dist); 
            eprintln!("close_to_checkpoint_threshold: {:?}", close_to_checkpoint_threshold); 
            if angle.abs() > critical_angle {
                thrust = minimun_thrust;
            } else {
                // If we are too close to the checkpoint...
                if checkpoint_dist < close_to_checkpoint_threshold {
                    let vector_ch_pod = checkpoint.substract(&player_position);
                    let vector_ch_next = checkpoint.substract(&checkpoint_next);

                    let curve_angle = (vector_ch_pod.get_angle(&vector_ch_next)*180.0)/PI;
                    eprintln!("curve_angle          : {:?}", curve_angle);
                    
                    if      (curve_angle.abs() < (180.0 - critical_angle)) 
                        &&  (player_info.speed.module() > 400.0)
                    {
                        // thrust = (curve_angle.abs() / (180.0 - critical_angle)) as i32 + minimun_thrust;
                        thrust = 5;
                    } else {
                        thrust = 100;
                    }
                } else {
                    thrust = 100;
                }
            }
        

            // +--------------------------+
            // |      BOOS calculation    |
            // +--------------------------+
            // If we can still use the BOOST and we have distant enough to the next checkpoint, use it
            let use_boost: bool;
            let used_boost = player_info.remaining_boosts > 0;
            if     (! used_boost) 
                && (checkpoint_dist > (checkpoint_bubble_radius*5.0))
                && (player_info.angle.abs() < 15.0)
            {
                use_boost = true;
                player_info.remaining_boosts -= 1;
            }else{
                use_boost = false;
            }
            
            match use_boost{
                true => println!("{} {} {}{}", target_x, target_y, "BOOST", message),
                false => println!("{} {} {}{}", target_x, target_y, thrust, message),
            }
        }
    }
}
