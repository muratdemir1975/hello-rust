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


pub fn hash_sets() {
    use std::collections::HashSet;

    let mut veri = HashSet::new();
    veri.insert("gama");
    veri.insert("alfa");
    veri.insert("beta");
    println!("veri: {:?}", veri);
    
    veri.insert("beta");
    println!("veri: {:?}", veri);

    let eklendi_mi = veri.insert("vega");
    if eklendi_mi {
        println!("vega eklendi");
    } else {
        println!("{:?} vega zaten var", veri);
    }

    if !veri.contains("delta") {
        println!("delta yok, ekliyoruz");
        veri.insert("delta");
    }

    let eleman_sil = veri.remove("alfa");
    if eleman_sil{
        println!("alfa silindi");
    } else {
        println!("alfa yoktu");
    }

    println!("veri: {:?}", veri);

    let _1_5:HashSet<_  > = (1..5).collect();
    let _6_10:HashSet<_> = (6..10).collect();
    let _1_10:HashSet<_> = (1..10).collect();
    let _2_8:HashSet<_> = (2..8).collect();

    println!("{:?} U {:?} = {:?}", _1_5, _6_10, _1_5.union(&_6_10).collect::<HashSet<_>>());
    println!("{:?} ∩ {:?} = {:?}", _1_10, _2_8, _1_10.intersection(&_2_8).collect::<HashSet<_>>());
    println!("{:?} - {:?} = {:?}", _1_10, _2_8, _1_10.difference(&_2_8).collect::<HashSet<_>>());

}


pub fn iters() {
    let mut v = vec![1, 2, 3, 4, 5];

    for i in &v {
        println!("i: {}", i);
    }

    for i in v.iter() {
        println!("i: {}", i);
    }

    for i in v.iter().rev() {
        println!("i: {}", i);
    }

    for i in &mut v {
        *i += 10;
    }

    for i in &v {
        println!("i: {}", i);
    }

    let mut v2 = vec![1, 2, 3, 4, 5];
    v2.extend(v);
    println!("v2: {:?}", v2);



}   
