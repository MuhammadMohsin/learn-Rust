fn main() {

    // Last value is exclusive in loop

    for number in 1..10 {
      println!("value of number is: {}", number)
    }

    for number in (1..5).rev() {
      println!("value of number is: {}", number)
    }

   let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

   for item in  months.iter() {
      println!("value of item is: {}", item);
    }

}
