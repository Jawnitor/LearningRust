use std::io;

pub fn run() 
{
	println!("Enter Weight (kg):");
	let mut s_input: String = String::new();
	
	io::stdin().read_line(&mut s_input).unwrap();
	let f_weight: f32 = s_input.trim().parse().unwrap();
	
	let f_mars_weight: f32 = calculate_weight_on_mars(f_weight); 
	println!("weight on mars: {}", f_mars_weight);
}

fn calculate_weight_on_mars(f_weight: f32) -> f32 
{ 
	return (f_weight / 9.81) * 3.711;
}
 
/* //INFO\\ */ 
//1) each value in rust is owned by a variable
//2) When the owner goes out of scope, the value will be deallocated
//3) There can only be ONE owner at a time
