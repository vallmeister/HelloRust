#![allow(dead_code)]
#![allow(unused_imports)]

use std::mem;

fn main() {
    hello_world();

    // basic data types
    let a: u8 = 123; // u = unsigned, 8 bits, 0...255
    println!("a = {}", a); // ! means makro
    // all variables are immutable by default

    // u = unsigned
    // i = signed
    let mut b: i8 = 0; // mutable 8-bit integer
    println!("b = {} before", b);
    b = 42;
    println!("b = {} after", b);

    // Rust can do type inference

    let mut c = 123456789; // i32
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {}", c);

    // usize isize: bitness of computer
    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z * 8);
}

// #1 Hello world
fn hello_world() {
    println!("Hello world");
}
