use std::io;

//No 1

fn main(){  
    let mut param1 = String::new();
    let mut param2 = String::new();
    println!("Enter Parameter 1");
    io::stdin().read_line(&mut param1);
    println!("Enter Parameter 2");
    io::stdin().read_line(&mut param2);
    print!("{} {}", param1, param2);
 }

