use std::io;

fn main() {
    let mut input_line = String::new();
    
    println!("Enter your weight (kg):");
    io::stdin().read_line(& mut input_line).unwrap();
    let weight: f32 = input_line.trim().parse().unwrap();
    println!("weight would be {}kg on mars",mars_weight_calc(weight));
}

fn mars_weight_calc(weight: f32)-> f32{
    (weight/9.81)*3.711
} 
