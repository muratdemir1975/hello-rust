#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

use core::f32;
use std::mem;
use std::io::stdin;

pub fn vektors() {
/*
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    println!("Vec a: {:?}", a);
*/

    let mut b = vec![1, 2, 3, 4, 5,1,1,1]; // [1; 5] = 1,2,3,4,5
    b.push(6);
    println!("Vec b: {:?}", b);

    let index=4;
    println!("b[{}]: {}", index, b[index]);

    match b.get(12) {
        Some(value) => println!("b[{}]: {:?}", index, value),
        None => println!("b[{}] is out of bounds", index),
    }

    for i in &b {
        println!("i: {}", i);
    }

    let son_eleman = b.pop();
    println!("son eleman: {:?}, b dizisi: {:?}", son_eleman, b);

    b.push(99);
    b.push(100);
    b.push(101);

    while let Some(x)=b.pop() {
        println!("x: {}", x);
    }

}


pub fn hashmaps() {
    use std::collections::HashMap;

    let mut sekiller = HashMap::new();
    let ucgen = String::from("ucgen");
    let kare = String::from("kare");

    sekiller.insert(ucgen, 3);
    sekiller.insert(kare, 4);

    println!("sekiller: {:?}", sekiller);

    for (key, value) in &sekiller {
        println!("{}: {}", key, value);
    }

    sekiller.entry(String::from("daire")).or_insert(1);
    println!("sekiller: {:?}", sekiller);
}
