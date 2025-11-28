fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

fn main() {
    let values = [1, 2, 3, 4, 5];
    let mut sum = 0;

    for n in values {
        sum = add(sum, n);
    }

    println!("Sum: {}", sum);

}
