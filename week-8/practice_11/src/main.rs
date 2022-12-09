fn main() {
    //Mutable array
    let mut colors = ["red", "green", "yellow", "white"];

    println!("\nOriginal array = {:?}", colors);

    //Mutable slice
    let sliced_colors = &mut colors[1..3];

    println!("First slice = {:?}", sliced_colors);

    //Chanhge the value of the original slice at the 1st index
    sliced_colors[1] = "purple";
    println!("Changed slice = {:?}", sliced_colors);
}
