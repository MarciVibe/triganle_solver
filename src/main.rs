use std::io;
fn main() {
    println!("Enter the length side a");
    let mut a = String::new();

    io::stdin()
    .read_line(&mut a).expect("Failed to read line");

    let _a: u32 = match a.trim().parse() {
        Ok(num) => num,
        Err(_) => todo!(),
        };

    println!("Enter the length side b");    
    let mut b = String::new();

    io::stdin()
    .read_line(&mut b).expect("Failed to read line");
    
    let _b: u32 = match b.trim().parse() {
        Ok(num) => num,
        Err(_) => todo!(),
        };
    
    if _a>_b{
    println!("CUM");
    }   
    
}
