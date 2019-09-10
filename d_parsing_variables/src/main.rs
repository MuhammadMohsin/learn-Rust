fn main() {

    let a: u32 = "42".parse().expect("Not a number!");
    println!("value of a is: {}", a);	

    let b = "100";
    let c:u8 = b.parse().expect("This is not a number!");
    println!("value of c is: {}", c);	
    
}
