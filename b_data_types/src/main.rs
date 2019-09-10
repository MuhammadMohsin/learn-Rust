fn main() {
    // Remember, always all variables in rust contain some type.	

    let a = 1; // by default integer is a 32 bit i.e i32 (signed) 
        
    // By default all variables are immutable.
    // We can mutate variables by adding a keyword "mut"
    let mut b = 2;
    b = 3;
    
    // We can also annotate type in variables
    let c: i8 = 10;
    let d: bool = true;
    let e: f32 = 3.15;
    let f = "Mohsin";

    // We use single quote while defining char
    let g:char = 'A'; 


    // We can also declare constants as well
    const TOTAL_MARKS: u8 = 100; // This is a convention to declare all constants in upper case
    
  
    println!("value of a is: {}", a); // "!" means macro function
    println!("value of b is: {}", b);
    println!("value of c is: {}", c);
    println!("value of d is: {}", d);
    println!("value of e is: {}", e);
    println!("value of TOTAL_MARKS is: {}", TOTAL_MARKS);
    println!("value of f is: {}", f);
    println!("value of g is: {}", g);
}
