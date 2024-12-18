fn main() {
    // println!("Please enter which Fibonnacci number you want to find");
    // let mut input = String::new();
    //
    // std::io::stdin()
    //     .read_line(&mut input)
    //     .expect("Failed to read line");
    //
    // let input: u64 = input.trim().parse().expect("Please enter a number");
    //
    // println!("The Fibonnacci number you are looking for is: {0}", fibonnacci(input));
    println!("The Fibonnacci number you are looking for is: {}", fibonnacci(90));
}

fn fibonnacci(n: u64) -> u64 {
    if n == 0 || n == 1 {
        1
    } else {
        let mut counter = 2;
        let mut prior = 1;
        let mut current = 1;

        // 1, 1, 2, 3, 5, 8, 13
        while counter < n {
            current += prior;
            prior = current - prior;
            counter += 1;
        }
        current
    }
}
