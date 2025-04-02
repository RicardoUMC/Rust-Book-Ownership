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

    // =====> Variables and Data Interacting with Move

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
    let s2 = s1; // They share the same pointer, it is not an actual copy of s1

    // The previous will make `s1` no longer valid for Rust. Avoiding memory corryption
    // (potential vulnerability), addressing the "double free" error when trying
    // free the memory twice.
    // ----> Example:
    // ----->   println!("{s1} world!");

    // =====> Scope and Assignment

    // When assigning a completely new value to an existing variable, Rust will `drop`
    // the previous variable and free their allocated memory.
    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!");

    // =====> Variables and Data Interacting with Clone

    // When we actually want to "deeply copy" the heap data of a `String` and not the
    // stack, we can use the method `clone`.
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
    // The next part is to see the difference when operating with integers (fixed size
    // data).
    println!("x = {x}, y = {y}");

    // =====> Ownership and Functions (check down below)

    // Passing a value to a function will move or copy the variable, depending on whether
    // the data type implements the Copy trait.
    let s = String::from("hello");
    takes_ownership(s); // <------ Here, s's value is moved into the function...

    let x = 5;
    makes_copy(x); // <----------- Here, x's value is copied.
    println!("{}", x); // <------- So it is safe to use x afterward.

    // =====> Return Values and Scope (check down below)

    // We can also transfer ownership by returning values.
    let s1 = gives_ownership(); // <----------- gives_ownership moves its return.
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // <---- s2 is moved into
                                       //       takes_and_gives_back, which also
                                       //       moves its return value into s3.

    // Before seeing what are the `references` we can use tuples for returning a
    // necessary value which we passed in to a function.
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");

    /******************************/
    /*  References and Borrowing  */
    /******************************/

    // Instead of taking ownership of a value, we could give a "reference", which is
    // similar to a pointer, but with the difference that it will point to a valid
    // value for the life of that reference.
    let s1 = String::from("hello");

    let len = calculate_length_2(&s1);

    println!("The length of '{s1}' is {len}.");

    // Error example when trying to modify something borrowed (see down below):
    let s = String::from("hello");
    change(&s);

    // In order to modify a reference we need to use a mutable reference:
    let mut s = String::from("hello");
    change_mutable(&mut s);

    /***
     * When using mutable references, there's must be only one reference to a value.
     * You can have no other references to that value as the code will fail.
     *
     * This occurs because to "data race":
     * - Two or more pointers access the same data at the same time.
     * - At least one of the pointers is being used to synchronize access to the data.
     * - There's no mechanism being used to synchronize access to the data.
     *
     * Uncomment to see the code fail: */
    /*
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}, {}", r1, r2);
    */

    // We can use bracket to create a scope, which allows for multiple mutable
    // references. Just keep in mind that they are not simultaneous.
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;

    /***
     * Something particular of this, is that we also cannot have a mutable reference
     * while having an immutable reference to the same value. Although creating multiple
     * immutable references is allow due to the impossibility of affecting another reading.
     * Uncomment to see the error:
     */
    /*
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM
    println!("{}, {}, and {}", r1, r2, r3);
    */

    // Note that a reference’s scope starts from where it is introduced and continues
    // through the last time that reference is used (the compiler can tell when the
    // references are no longer being used):
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}"); // variables r1 and r2 will not be used after this point
    let r3 = &mut s; // no problem
    println!("{r3}");

    // =====> Dangling References

    // Unlike languages with pointers, Rust compiler guarantees the referenced data will
    // not go out of scope before the reference to the data does. (Just like freeing 
    // some memory while preserving a pointer to that memory).
    let reference_to_nothing = dangle();

    /******************************/
    /*         Slice Type         */
    /******************************/

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

//-------------------------------------------------------------------------------------//

// This is an option: To pass back a value we want to use it again. Sadly, not all
// times we might want to transfer ownerships.
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

/***************************/
/*        Borrowing        */
/***************************/

// Here we are giving the function the reference for use it without taking
// ownership.
fn calculate_length_2(s: &String) -> usize {
    // s is a reference to a String
    // When using references as parameters in functions, there is no need in
    // returning the values in order to give back ownership.
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what
  // it referss to, the value is not dropped.

// Error example
fn change(some_string: &String) {
    // Uncomment to see error.
    // some_string.push_str(", world");
}

// Using mutable reference
fn change_mutable(some_string: &mut String) {
    some_string.push_str(", world")
}

/***************************/
/*        Dangling         */
/***************************/

// Problem
/*
fn dangle() -> &String { // dangle return a reference to a String
    let s = String::from("hello"); // s is a new String
    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
*/

// Solution
fn dangle() -> String { // dangle return a reference to a String
    let s = String::from("hello"); // s is a new String
    s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

//-------------------------------------------------------------------------------------//
