fn main() {
    println!("Hello from rust!!!");

    let a: i32 = 1;
    let b: i32 = 10;
    let sum: i32 = a + b;

    // mutable can change
    let mut total: i32 = a + b;
    let mut multiply: i32 = a * b;
    let mut divide: i32 = a / b;
    let mut subtract: i32 = a - b; 

    total += b;
    multiply *= b;
    divide /= b;
    subtract -= b;

    println!("The sum of {} and {} = {}", a, b, sum);
    println!("The total = {}", total);
    println!("The multiply = {}", multiply);
    println!("The divide = {}", divide);
    println!("The subtract = {}", subtract);
}
