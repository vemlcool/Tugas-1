use std::io;
//No 2
fn main() {

    let mut param1 = String::new();
    let mut param2 = String::new();
    let mut param3 = 0;
    println!("Input Parameter 1:");
    io::stdin()
        .read_line(&mut param1)
        .expect("Gagal");
   let param1: u32 = param1.trim().parse().expect("Tolong masukan angka");

   println!("Input Parameter 2:");
   io::stdin()
       .read_line(&mut param2)
       .expect("Gagal");
  let param2: u32 = param2.trim().parse().expect("Tolong masukan angka");

  param3 = param1%param2;

  println!("Output: {}",param3);

    

 
}