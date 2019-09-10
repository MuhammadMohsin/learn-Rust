fn main() {

    // Unlike tuple, array can store only same data type values.
    // Array is of fixed length similar to tuple

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("values in array are: {} {} {} {} {}", a[0], a[1], a[2], a[3], a[4]);

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    println!("few values in months are: {} {} {} {}", months[0],months[2],months[4],months[6]);

    // This is create array of length 10 contains integer "6" 
    // at its all indexes.
 
    let b = [6; 10];
    println!("value of b is: {:?}", b);

}
