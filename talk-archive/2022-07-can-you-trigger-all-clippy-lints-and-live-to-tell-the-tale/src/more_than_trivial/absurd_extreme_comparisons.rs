#![allow(unused_comparisons)]

pub fn trigger_from_clippy_docs() {
    let vec: Vec<isize> = Vec::new();
    if vec.len() <= 0 {}
    if 100 > i32::MAX {}
}

pub fn trigger_1() {
    let x: u16 = 123;

    // Only clippy
    if x > u16::MAX {
        println!("Great success!");
    }

    // Clippy but also rustc
    if x > 65535 {
        println!("Great success!");
    }
}

pub fn trigger_2() {
    let x: u8 = 123;
    if x > 255 {
        println!("Great success!");
    }
}
