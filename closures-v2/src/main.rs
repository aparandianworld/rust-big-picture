fn main() {
    let add = | n1, n2 | { n1 + n2};
    let (from, to) = (0, 10);

    let sum_range = || {
        let mut sum = 0;
        for i in from..to {
            sum = add(sum, i); // contrived example of closure within closure but it works
        }

        sum
    };

    println!("The sum of all number from {} to {} is {}", from, to, sum_range());
}
