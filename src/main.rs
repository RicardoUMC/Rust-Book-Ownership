#![allow(unused)]

fn main() {
    // Using :: operatior thar allows us to namespace `from` function
    // under the String type
    let mut s = String::from("hello");
    s.push_str(", world!");

    println!("{s}");


    //======> Variables an Data Interacting with Move
    
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
    println!("{s1} world!");

}
