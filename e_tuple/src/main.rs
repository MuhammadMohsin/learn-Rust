fn main() {

    // Tuple is a compound data type which ca store multiple
    // values with a variety of different data types. Tuple 
    // awlays have a fixed length.
     
    let tup1: (i32, f64, u8, bool) = (100, 3.14, 1, true); // Optionally added data types

    println!("value of tup1 at first index is: {}", tup1.0);
    println!("value of tup1 at second index is: {}", tup1.1);
    println!("value of tup1 at third index is: {}", tup1.2);
    println!("value of tup1 at fourth index is: {}", tup1.3);


    let tup2 = (200, true, "Mohsin", 'A');

    println!("value of tup2 at first index is: {}", tup2.0);
    println!("value of tup2 at second index is: {}", tup2.1);
    println!("value of tup2 at third index is: {}", tup2.2);
    println!("value of tup2 at fourth index is: {}", tup2.3);


    // We can also destructure tuple values as well
    // It is always required to do full destructuring of a tuple
    let (a, b, c, d) = tup2;
     
    println!("values of a, b, c and d are: {}, {}, {}, {}", a, b, c, d);

}
