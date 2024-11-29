// - The ability to run some code depending on whether a condition is true and
//   to run some code repeatedly while a condition is true are basic building blocks
//   in most programming languages.
// - The most common constructs that let you control the flow of execution of Rust
//   code are if expressions and loops.
fn main() {
    // - An if expression allows you to branch your code depending on conditions.
    // - You provide a condition and then state, "If this condition is met, run
    //   this block of code. If the condition is not met, do not run this block
    //   of code."

    let number = 3;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    // - All if expressions start with keyword if, followed by a condition.
    // - In this case, the condition check whether or not the variable number has
    //   a value less than 5.
    // - We place the block of code to execute if the condition is true immediately
    //   after the condition inside curly brackets.
    // - Blocks of code associated with the conditions in if expressions are some-
    //   times called arms.
    // - Optionally, we can also include an else expression to give the program an
    //   alternative blcok of code to execute should the condition evaluates to fal-
    //   se. If you don't provide an else expression and the condition is false, t-
    //   he program will just skip the if block and move on to the next bit of code.

    // - It's also worth noting that the condition in this code must be a bool.
    // - If the condition isn't a bool, we'll get an error.

    // if number {
    //     println!("Number was tree");
    // }

    // - Unlike languages such as Ruby and JavaScript, Rust will not automatically
    //   try to convert non-Boolean types to a Boolean. You must be explicit and
    //   always provide if with a Boolean as its condition.

    if number != 0 {
        println!("number was something other than zero");
    }

    // You can use multiple conditions by combining if and else in an else if 
    // expression.
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // - When this program executes, it checks each if expression in turn and
    //   executes the first body for which the condition evaluates to true.
    // - Note that even though 6 is divisible by 2, we don't see the output
    //   number is divisible by 2, nor do we see the number is not divisible by
    //   4, 3, or 2 text from the else block. That's because Rust only executes
    //   the block for the first true condition, and once it finds one, it doesn't
    //   even checks the rest.

    // Because if is an expression, we can use it on the right size of a let stat-
    // ement to assign the outcome to a variable.
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // The number variable will be bound to value based on the outcome of the if
    // expression.

    // Remember that blocks of code evaluate to the last expresssion in them, and
    // numbers by themselves are also expressions. In this case, the value of the 
    // whole if expression depends on which block of code executes.
    // This means the values that have the potential to be results from each arm
    // of the if must be the same type.

    let _condition = false;
    // let number = if !condition { 5 } else { "six" };

    // The expression in the if block evaluates to an integer, and the expression
    // in the else block evaluates to string.
    // This won't work because because variables must have a single type, and Rust
    // needs to know at compile time what type the number variable is, definitively.
    // Knowing the type of number lets the compiler verify the type is valid every-
    // where we use number. Rust wouldn't be able to do that if the type of number
    // was only determined at runtime; the compiler would be more complex and would
    // make fewer guarantees about the code if it had to keep track of multiple hy-
    // pothetical types for any variable.

    // - It's often useful to execute a block of code more than once. For this task,
    //   Rust provides several loops, which will run through the code inside the 
    //   loop body to the end and then start immediately back at the beginning.
    // - Rust has three kinds of loops: loop, while, and for.

    // - The loop keyword tells Rust to execute a block of code over and over again
    //   forever or until you explicitly tell it to stop.
    // loop {
    //     println!("again");
    // }

    // When we run this program, we'll se again printed over and over continuously
    // until we stop the program manually.
    // Fortunately, Rust also provides a way to break out of a loop using code.
    // You can place the break keywor within the loop to tell the program when to
    // stop executing the loop.
    // - continue keyword can be used to skip over any remaining code in the current iteration of the loop and go to the next iteration.

    // - One of the uses of a loop is to retry an operation you know might fail,
    //   such as checking whether a thread has completed its job.
    // - You might also need to pass the result of that operation out of the loop
    //   to the rest of your code. To do this, you can add the value you want ret-
    //   urned after the break expression you use to stop the loop; that value wi-
    //   ll be returned out of the loop so you can use it.
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
    // - Before the loop, we declare a variable named counter and initialize it to
    //   0. Then we declare a variable named result to hold the value returned
    //   from the loop. On every iteration of the loop, we add 1 to the counter
    //   variable, and then check whether the counter is equal to 10. When it is,
    //   we use the break keyword with the value counter * 2.
    // - After the loop, we use a semicolon to end the statement that assigns the
    //   value to result.
    // - You can also return from inside a loop. While break only exits the curr-
    //   ent loop, return always exits the current function.

    // - If you have loops within loops, break and continue apply to the innermost
    //   loop at that point. You can optionally specify a loop label on a loop 
    //   that you can then use with break or continue to specify that those keyw-
    //   ords apply to the labeled loop instead of the innermost loop.
    // - Loop labels must begin with a single quote.
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    // - The outer loop has the label 'counting_up, and it will count up from 0
    //   to 2. The inner loop without a label counts down from 10 to 9. The first
    //   break that doesn't specify a label will exit the inner loop only. The b-
    //   reak 'counting_up; statement will exit the outer loop.

    // - A program will often need to evaluate a condition within a loop. While
    //   the condition is true, the loop runs. When the condition ceases to be 
    //   true, the program calls break, stopping the loop.
    // - It's possible to implement behaviour like this using a combination of 
    //   loop, if, else, and break; However, this pattern is so common Rust has
    //   a built-in language construct for it, called a while loop.

    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // - This construct eliminates a lot of nesting that would be necessary if
    //   you used loop, if, else, and break, and it's clearer.

    // - You can also use the while construct to loop over the elements of a col-
    //   lection, such as an array.
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // - However, this approach is error prone; we could cause the program to 
    //   panic if the index value or test condition is incorrect.
    // - It's also slow, because the compiler adds runtime code to perform the
    //   conditional check of whether the index is within the bounds of the array
    //   on every iteration through the loop.
    // - As a more concise alternative, you can use a for loop and execute some 
    //   code for each item in a collection.
    for element in a {
        println!("{element}");
    }

    // - Using the for loop, you wouldn't need to remember to change any other
    //   code if you changed the number of values in the array.
    // - The safety and conciseness of for loops make them the most commonly used
    //   loop construct in Rust.
    // - Even in situations in which you want to run some code a certain number 
    //   of times, you'd use for loop.
    // - The way to do that would be to use a Range which generates all numbers in
    //   sequence starting from one number and ending before another number.

    for number in (1..4).rev() {
        println!("{number}");
    }

    println!("LIFTOFF!!!");
}
