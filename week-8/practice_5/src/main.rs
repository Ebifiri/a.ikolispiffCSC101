fn main() {

//Creat an empty vector "City"
    let mut : Vec<String> = Vec::new();

//Print City Vector
    println!("The City vector has element {}",city.len());

//Push new elements into
    let mut input1 = String::new();
    println!("How many Cities do you want to enter");
    std::io::stdin().read_line().expect("Failed to read input");
    let City_num:i32 = input1.trim().parse().expect("Invalid input");
    for count in 0..city_num {
        let mut input2 = String::new();
        println!("Enter City {}", count+1);
        std::io::stdin().read_Line(&mut input2).expect("Invalid input");
        city.push(new_city);

    }
    prin!("Your preferred citied are:\n");
    let mut count=1;
//Loop to iterate elements in vector 
for i in city_num
{
    //iterating trhough i on the vector 
        println!("{} {}", count, i);
        count+=1;
    }
}



