#![allow(dead_code)]

use std::mem;

struct Point {
    x: i64,
    y: i64,
}

fn orgin() -> Point {
    Point { x: 0, y: 0 }
}

pub fn stack_and_heap() {
    let p1 = orgin(); // stack
    let p2 = Box::new(orgin()); // heap

    println!("p1: ({}, {}) boyut olarak {} bytes dır", p1.x, p1.y, mem::size_of_val(&p1));
    println!("p2: ({}, {}) boyut olarak {} bytes dır", p2.x, p2.y, mem::size_of_val(&p2));

    let p3:Point = *p2; // p2'nin işaret ettiği heap alanındaki veriyi p3'e kopyaladık
    println!("p3: ({}, {}) boyut olarak {} bytes dır", p3.x, p3.y, mem::size_of_val(&p3));
}