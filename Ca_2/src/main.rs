use std::io;

fn StudentCouncil_VoteX(){
    let mut count:i32 = 0;
    loop {
        
    
    let mut rep = String::new();
    println!("Are you a current class rep?");
    println!("Yes(1) or No(2)");
    io::stdin().read_line(&mut rep).expect("Can't understand");
    let rvalue:u32 = rep.trim().parse().expect("Can't understand");

    let mut level = String::new();
    println!("What level are you in?");
    println!("100, 200, 300 or 400");
    io::stdin().read_line(&mut level).expect("Can't understand");
    let lvalue:u32 = level.trim().parse().expect("Not familiar with that");

    let mut cgpa = String::new();
    println!("What is your CGPA");
    io::stdin().read_line(&mut cgpa).expect("Can't understand");
    let gpaval:f32 = cgpa.trim().parse().expect("Don't have the facilities for that");

    if rvalue == 1 && lvalue >= 200 && gpaval > 4.0 {
        println!("Name");
        println!("------------");
        println!("Email");
        println!("------------");
        println!("Department");
        println!("------------");
        println!("State of Origin");
        println!("------------");
        println!("You can vote");
    }
    else {
        println!("Sorry, you are not eligible to vote");
    }
    count+=1;
    if count == 15{
        println!("Maximum number of students reached");
        break;
    }
}
}

fn FacPub(){
    let mut name = String::new();
    println!("What is your faculty name");
    io::stdin().read_line(&mut name).expect("Not valid");

    let mut paper = String::new();
    println!("How many papers have you published?");
    io::stdin().read_line(&mut paper).expect("Not valid");
    let papnum:i32 = paper.trim().parse().expect("Not valid");

    println!("Faculty Name: {}",name);
    if papnum <= 5 && papnum >= 3 {
        println!("Your incentive is N500,000");
    }
    if papnum > 5 && papnum < 10 {
        println!("Your incentive is N800,000");
    }
    if papnum >= 10 {
        println!("Your incentive is N1,000,000");
    }
    if papnum < 3{
        println!("Your incentive is N100,000");
    }
}

fn main(){
    println!("The Student Council Voter System");
    println!("--------------------------------");
    StudentCouncil_VoteX();

    println!("The Faculty Publication Incentive System");
    println!("--------------------------------");
    FacPub();
}