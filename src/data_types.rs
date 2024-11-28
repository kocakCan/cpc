/* Rust is a statically typed language, meaning it must know the types of all
*  variables at compile time.
*  The compiler can usually infer what type wanted to be used based on the value
*  and how it is used.
*  In cases when many types are possible, type annotation must be added.
*/
fn main() {
    // this won't compile as many types are possible.
    // let age = "27".parse().expect("Not a number!");
    let age: u8 = "27".parse().expect("Not a number!");

    println!("I am {age} years old.");

    // A scalar type represents a single value.
    // Rust has four primary scalar types: integers, floating-point numbers
    // Booleans, and characters.

    // An integer is a number without a fractional component.
    // If the integer type starts with 'u', then it's unsigned,
    // If it starts with 'i', then it's signed integer.
    // Signed and unsigned refer to whether it's possible for the number
    // to be negative.
    
    // Each signed variant can store numbers from -(2^n-1) to 2^n-1 - 1 inclusive,
    // where n is the number of bits that variant uses.
    // So an i8 can store numbers from -(2^7) to 2^7 - 1, which equals -128 to 127.
    // Unsigned variants can store numbers from 0 to 2^n - 1, so a u8 can store
    // numbers from 0 to 2^8 - 1, which equals 0 to 255.
    
    // Number literals that can be multiple numeric types allow a type suffix:
    let decimal_number = 98_222;
    let hex_number = 0xff;
    let octal_number = 0o77;
    let binary_number = 0b1111_0000;
    let byte = b'A';

    let numbers: [u32; 5] = [decimal_number, hex_number, octal_number, binary_number, byte.into()];

    for number in numbers {
        println!("{number}");
    }

    // Number literals can also use _ as a visual seperator to make the number
    // easier to read.

    // The primary situation in which you'd use isize or usize is when indexing
    // some sort of collection.
    
    // Rust has two primitive types for floating-point numbers, which are numbers
    // with decimal points (f32 and f64).
    // The default type is f64 and is more precise than f32.
    // All floating-point types are signed.
    let x: f32 = 2.0;
    let y = 3.0;        // f64
    let floats = [x, y];

    for float in floats {
        println!("{float}");
    }

    // Numeric Operations
    // Integer division truncates toward zero to the nearest integer.
    let sum = 5.0 + 10.9;
    let difference = 95.5 - 4.3;
    let product = 4.0 * 30.138;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;         // Results in -1
    let remainder = 43 % 5;

    let nops = [sum, difference, product, quotient];

    for nop in nops {
        println!("{nop}");
    }

    println!("Integer division: {truncated}");
    println!("Remainder: {remainder}");

    // Boolean type in Rust has two possible values: true and false.
    // Booleans are one byte in size, and are specified using bool
    let t = true;
    let f: bool = false;            // with explicit type annotation

    let bools: [bool;2] = [t, f];

    for i in bools {
        println!("{i}");
    }

    // The main way to use Boolean values is through conditionals, such as an
    // if expression.

    // char type in Rust is the most primitive alphabetic type.
    // char literals are specified using single quotes, as opposed to string
    // literals, which use double quotes.
    // char type is four bytes in size and represents Unicode Scalar Value.
    let c = 'z';
    let z: char = 'ℤ';
    let hugging_face = '🤗';

    let chars: [char;3] = [c, z, hugging_face];

    for c in chars {
        println!("{c}");
    }

    /* Compound types can group multiple values into one type.
    *  Rust has two primitive compound types: tuples and arrays.

    *  - A tuple is a general way of grouping together a number of values with
    *    a variety of types into one compound type.
    *  - Tuples have a fixed length: once declared, they cannot grow or shrink
    *    in size.
    *  - Tuples are created by writing a comma-seperated list of values inside
    *    parantheses.
    *  - Each position in the tuple has a type, and the types of the different
    *    values in the tuple don't have to be the same. */

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // The variable tup binds to the entire tuple because a tuple is considered
    // a single compound element.
    
    // To get the individual values out of a tuple, we can use pattern matching to
    // destructure a tuple value as following:
    let (x, y, z) = tup;

    println!("The value of y is: {x}");
    println!("The value of y is: {y}");
    println!("The value of y is: {z}");
    // Element access can also be achieved using a period (.)
    println!("The value of x is: {0}", tup.0);

    // A tuple without any values has a special name, unit.
    // This value and its corresponding type are both written () and represent
    // an empty value or an empty return type.
    // Expressions implicitly return the unit value if the don't return any other
    // value.
    
    /*      - Arrays are another way to have a collection of multiple values.
     *      - Unlike a tuple, every element of an array must have the same type.
     *      - In Rust, arrays are fixed in size.
     *      - Arrays are created using a comma-seperated list inside square brackets.
     *      - Arrays are useful when you want your data allocated on the stack rather
     *        than the heap or when you want to ensure you always have a fixed number
     *        of elements.
     *      - An array isn't as flexible as the vector type. 
     *      - A vector is a similar collection type that is allowed to grow or shrink in size.
     *      - Arrays are most useful when you know the number of elements will not need to change.
     *      - You write an array's type using square brackets with the type of each element, a
     *        semicolon, and then the number of elements in the array.
     *      - You can also initialize an array to contain the same value for each element by specifying
     *        the initial value, followed by a semicolon, and then the length of the array in square
     *        brackets such as let a = [3; 5];
     *      - An array is a single chunk of memory of a known, fixed size that can be
     *        allocated on the stack. You can access elements of an array using indexing such as let
     *        first = a[0];
     */

    let _a = [1, 2, 3, 4, 5];
    let _months = ["January", "February", "March", "April", "May", "June", "July",
                    "August", "September", "October", "November", "December"];

    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let _a = [3; 5];
    let a = [1, 2, 3, 4, 5];

    let _first = a[0];
    let _second = a[1];

    // Invalid Array Element Access
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
    // The program resulted in a runtime error at the point of using an invalid
    // value in the indexing operation.
    // The program exited with an error message and didn't execute the final
    // println! statement.
    // When you attempt to access an element using indexing, Rust will check that
    // the index you've specified is less than the array length. If the index is
    // greater than or equal to the lenght, Rust will panic.
    // This check has to happen at runtime, especially in this case, because the
    // compiler can't possibly know what value a user will enter when they run
    // the code later.
    // Rust uses the term panicking when a program exits with an error.
}
