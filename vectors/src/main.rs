fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let mut values = vec![1, 2, -9, 22, 14];
    let mut sum = 0;
    
    values.push(100);
    values.push(-12);
    values.push(5);

    for num in values {
        sum = add(sum, num);
    }

    println!("Sum: {}", sum);
    
}
