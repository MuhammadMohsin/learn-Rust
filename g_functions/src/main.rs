fn main() {

    let ans1 = sum(2, 3);
    print_result(ans1);
    
    print_result(difference(5, 1));

    let ans1 = percentage(80.0, 250.0);
    println!("{}",ans1);

}

fn print_result(ans: i32) {
    println!("Your answer is: {}", ans)
}

fn sum(a: i32, b: i32) -> i32 {
    // Expression without ";" consider as a return value in function
    a + b
}

fn difference(a: i32, b: i32) -> i32 {
    return a - b;
}

fn divide(a: f32, b: f32) -> f32 {
   a / b
}

fn percentage (a: f32, b: f32) -> f32 {
   divide(a,b) * 100.0
}
