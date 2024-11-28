fn main() {
    // variables are immutable by default.
    let x = 5;

    println!("The value of x is: {x}");

    // x = 6;  // won't compile as x is immutable

    // won't be printed as compile will throw an error before
    // println!("The value of x is: {x}");

    // to make a variable mutable, add mut keyword before the variable name
    let mut name = String::new();

    println!("The value of name is: {name}");

    // name was declared as an empty string first
    // now, I will change it
    name = String::from("Can");
    println!("The value of name is: {name}");

    // constants are that are bound to a name and are not allowed
    // to change just like immutable variables.
    
    /* There are a few differences between immutable variables and constants
    * - mut can't be used with constant just as immutable variables as they are always immutable,
    * - Constants are declared using const keyword instead of let and the type of the value must be annotated.
    * - Constants can be declared in any scope, including global scope, which makes them useful for values that many parts of code need to know about.
    * - Constants may only be set to a constant expression, not the result of a value that could be only computed at runtime.
    */

    const MY_NAME: &str = "Can";
    
    println!("My name is {MY_NAME}");

    // Declaring a new variable with the same is called shadowing
    // Second variable would be the variable the compiler will see
    // when the name of the variable is used.
    // In effect, the second variable overshadows the first, taking any uses
    // of the variable name to itself until it itself is shadowed or the scope ends.
    // A variable is shadowed by using the same variable's name and repeating the use of the let
    // keyword
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // Shadowing is different from marking a variable as mut because we'll
    // get a compile-time error if we accidentally try to reassign to this variable
    // using the let keyword.
    
    // When shadowing is performed, essentially a new variable is created as let keyword is used
    // again, the type of the value can be changed but reuse the same name.

    // this would throw an error as types do not match, and is different from shadowing.
    let mut spaces = "    ";
    spaces = spaces.len();
}
