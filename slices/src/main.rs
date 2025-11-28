fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let values = [10, 32, 11, 43, -9, 22];
    let mut sum = 0;

    for num in &values[0..2] {
        sum = add(sum, *num);
    }

    for num in &values[2..6] {
        sum = add(sum, *num);
    }

    println!("Sum: {}", sum);
}
