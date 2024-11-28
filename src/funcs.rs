fn main() {
    println!("Hello, world");

    another_function();
    /* - We can call any function we've defined by entering its name followed by
     *   a set of parantheses.
     * - It doesn't matter where you've defined your function as long as they are
     *   in a scope that can be seen by the caller. */

    /* - Functions can be defined to have parameters, which are special variables
    *    that are part of a function's signature.
    *  - When a function has parameters, you can provide it with concrete values
    *    for those parameters. Technically, the concrete values are called argu-
    *    ments.
    */

    const PARAGON_LVL: u8 = 235;
    some_function(PARAGON_LVL);
    print_labeled_measurement(5, 'm');

    /* - Statements do not return values. Therefore, you can't assign a let
    *    statement to another variable.
    *    let x = (let y = 6);
    *  - The let y = 6 statement does not return a value, so there isn't anything
    *    for x to bind to.
    *  - This is different from what happens in C, where the assignment returns
    *    the value of the assignment.
    */

    /* - Expressions evaluate to a value and make up most of the rest of the code
     *   you'll write in Rust.
     * - Consider a math operation, such as 5 + 6, which is an expression that ev-
     *   aluates to the value 11.
     * - Expressions can be part of statements: Calling a function is an expression.
     *   Calling a macro is an expression. A new scope created with curly brackets
     *   is an expression.
     */

    let y = {
        let x = 3;
        x + 1
    };

    /* - This expression is a block that, in this case, evaluate to 4.
     * - That value gets bound to y as part of the let statement.
     * - Note that the x + 1 line doesn't have a semicolon at the end.
     * - Expressions do not include ending semicolons. If you add a semicolon
     *   to the end of an expression, you turn it into a statement, and it will
     *   not return a value.
     */
    println!("The value of y is: {y}");

    /* - Functions can return values to the code that calls them. We don't name
     *   return values, but we must declare their type after an arrow (->).
     * - In Rust, the return value of the function is synonymous with the value
     *   of the final expression in the block of the body of a function.
     * - You can return early from a function by using the return keyword and sp-
     *   ecifying a value, but most function return the last expression implicitly
     */

    let z = five();

    println!("The value of z is: {z}");

    let t = plus_one(19);

    println!("The value of t is: {t}");

    let w = plus_two(24);

    println!("The value of t is: {w}");
}

fn another_function() {
    println!("Another function");
}

fn some_function(x: u8) {
    println!("I am currently at {x} paragon level on Diablo IV.");
}

/* - The declaration of some_function has one parameter name x.
*  - The type of x is specified as u8. When we pass PARAGON_LVL into the function
*    the println! macro puts PARAGON_LVL where the pair of curly brackets contai-
*    ning x was in the format string.
*  - In function signatures, you must declare the type of each parameter.
*  - Requiring type annotations in function definitions means the compiler almost
*    never needs you to use them elsewhere in the code to figure out what type
*    you mean.
*  - The compiler is also able to give more helpful messages if it knows what ty-
*    pes the function expects.
*/

/* - When defining multiple parameters, seperate the parameter declarations with
*    commas.
*  - Function bodies are made up of a series of statements optionally ending in
*    an expression.
*  - Statements are instructions that perform some action and do not return value.
*  - Expressions evaluate to a resultant value.
*  - Function definitions are also statements; calling a function is not a state-
*    ment.
* **/

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

/* -  The 5 in five is the function's return value, which is why the return type
 *    is i32.
 *  - The line let x = five(); shows that we're using the return value of a func-
 *    tion to initialize a variable. Because the function five returns five ret-
 *    urns a 5, that line is the same as the following let x = 5;
 *  - The five function has no parameters and defines the type of the return value,
 *    but the body of the function is a lonely 5 with no semicolon because it's an
 *    expression whose value we want to return.
 */

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn plus_two(x: i32) -> i32 {
    x + 2
}
