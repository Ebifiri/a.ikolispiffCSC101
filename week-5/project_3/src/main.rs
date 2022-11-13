use std::io;

fn main() {
    println!("This is our menu\nWith different varieties of your liking you can choose from");
    println!("Poundo Yam / Edinkaiko Soup (P)");
    println!("Fried Rice & Chicken (F)");
    println!("Amala & Ewedu Soup (A)");
    println!("Eba & Egusi Soup (E)");
    println!("White Rice & Stew (W)");

    let P:u32 = 3_200;
    let F:u32 = 3_000;
    let A:u32 = 2_500;
    let E:u32 = 2_000;
    let W:u32 = 2_500;
    let q:u32;

    println!();
    println!("Which will you be having today?");
    io::stdin().read_line().expect("Excuse me?");
}
