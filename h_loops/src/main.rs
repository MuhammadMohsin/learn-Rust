fn main() {
    let mut x = 1;
    loop {
        println!("Value of x is: {}", x);
        x = x + 1;
        if x == 5 {
            break;
        }
    }
}
