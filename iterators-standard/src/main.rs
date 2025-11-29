fn main() {
    // calculate the sum of all even numbers from 1 to 99
    let m1_sum = (1..100).
    filter(|num| num % 2 == 0).
    sum::<i32>();

    let m2_sum : i32 = (1..100).
    filter(|num| num % 2 == 0).
    sum();

    println!("method 1: The sum of all even numbers from 1 to 99 is {}", m1_sum);
    println!("method 2: The sum of all even numbers from 1 to 99 is {}", m2_sum);
}
