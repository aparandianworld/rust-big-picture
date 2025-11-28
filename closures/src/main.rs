// is_even is a standard function
fn is_even(num: i32) -> bool { num % 2 == 0 }

fn main() {
    let num = 42;
    // is_odd is a closure
    let is_odd = | num: i32 | { num % 2 != 0 };

    println!("The number is: {}", num);
    println!("The number is_even: {}", is_even(num));
    println!("The number is_odd: {}", is_odd(num));
}
