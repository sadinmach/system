use crate::system::Society;
use crate::population::Citizen;
use crate::population::Dob;

pub mod system;
pub mod population;

fn main() {
    let society = Society::new();
    let sadin = Citizen::new(
        String::from("Sadin"), 
        String::from("Mach"), 
        String::from("sadinmach"), 
        Dob::new(
            1990, 
            String::from("January"), 
            01), 
        33, 
        100_000);

    println!("{:?}", society);
    println!("{:?}", sadin);
}

