fn main() {

    let add = |n1: i32, n2: i32 | -> i32 { n1 + n2 };
    let numbers = 0..10;

    let sum_range = || {
        let mut sum = 0;
        for i in numbers.into_iter() { // can remove into_iter() and it will be called automatically
            sum = add(sum, i);
        }

        sum
    };

    println!("The sum of all numbers from 0 to 9 is {}", sum_range());
}
