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

}
