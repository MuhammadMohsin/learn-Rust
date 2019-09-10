fn main() {
    
    // Shadow variables can change data type on next declaration
    // We cannot change a data type even after making variable
    // a "mutable" variable. Mutable variable can only change
    // a value stored in it but cannot the data type.

    let a = true;
    println!("value of a is: {}", a);

    let a = "Mohsin Khalid";
    println!("value of a is: {}", a);

    // We can also optionally annotate type as well
    let a: f32 = 3.15;
    println!("value of a is: {}", a);

    let a = false;
    println!("value of a is: {}", a);


    // This is not allowed
    let mut b = true;
    println!("value of b is: {}", b);

    // b = "Mohsin"; // Error!
}
