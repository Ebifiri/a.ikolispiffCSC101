use std::io;

fn FacPub(){
    let mut name = String::new();
    println!("What is your faculty name");
    io::stdin().read_line(&mut name).expect("Not valid");

    let mut paper = String::new();
    println!("How many papers have you published?");
    io::stdin().read_line(&mut paper).expect("Not valid");
    let papnum:i32 = paper.trim().parse().expect("Not valid");

    println!("Faculty Name: {}",name);
    if papnum 
}

fn main(){
    println!("The Faculty Publication Incentive System");
    println!("--------------------------------");
    FacPub();
}