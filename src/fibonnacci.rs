fn main() {
    println!("Hello, world");
    println!("The fifth element of Fibonnacci sequence is: {0}", fibonnacci(19));
}

fn fibonnacci(n: u32) -> u32 {
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
