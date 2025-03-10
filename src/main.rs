#![allow(unused)]

fn main() {

    /******************************/
    /* Ownership and how it works */
    /******************************/

    // Using :: operator thar allows us to namespace `from` function under the String
    // type.
    let mut s = String::from("hello");
    s.push_str(", world!");

    println!("{s}");

    //======> Variables and Data Interacting with Move
    
    // Using fixed size data types would make a copy of the value
    let x = 5;
    let y = x;
    /***
     * Using string is different.
     *  - ptr           Memory that holds the contents of the string
     *  - len           How much memory (bytes) are currently used
     *  - capacity      Total amount of memoty (bytes) it has received from allocation
     *  
     *  Meaning, they will reference the same the same address
    */ 
    let s1 = String::from("hello");
    let s2 = s1;        // They share the same pointer, it is not an actual copy of s1
    
    // The previous will make `s1` no longer valid for Rust. Avoiding memory corryption 
    // (potential vulnerability), addressing the "double free" error when trying 
    // free the memory twice.
    //-----> Example:
    //------>   println!("{s1} world!"); 

    //======> Scope and Assignment

    // When assigning a completely new value to an existing variable, Rust will `drop`
    // the previous variable and free their allocated memory.
    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!");

    //======> Variables and Data Interacting with Clone

    // When we actually want to "deeply copy" the heap data of a `String` and not the
    // stack, we can use the method `clone`.
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
    // The next part is to see the difference when operating with integers (fixed size
    // data).
    println!("x = {x}, y = {y}");

    //======> Ownership and Functions

    // Passing a value to a funtion will move or copy the variable, depending on whether
    // the data type implements the Copy trait.
    let s = String::from("hello");
    takes_ownership(s); //<------ Here, s's value is moved into the function...

    let x = 5;
    makes_copy(x); //<----------- Here, x's value is copied.
    println!("{}", x); //<------- So it is safe to use x afterward.

    //======> Return Values and Scope 

    // We can also transfer ownership by returning values.
    let s1 = gives_ownership(); // <----------- gives_ownership moves its return.
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // <---- s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3.

    // Before seeing what are the `references` we can use tuples for returning a
    // necessary value which we passed in to a function.
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

/***************************/
/* Ownership and Functions */
/***************************/

fn takes_ownership(some_string: String) {
    println!("{some_string}")
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

/***************************/
/* Return Values and Scope */
/***************************/

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// This is an option: To pass back a value we want to use it again. Sadly, not all 
// times we might want to transfer ownerships.
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
