#![allow(dead_code)]
#![allow(unused_variables)]
mod sh;
mod kontrolakisi;

use std::mem;

const VERI:i8 = 23; // bellirli bir adresi yoktur, sabit bir değer tutar, değiştirilemez
static mut VERI2:i8 = 23; // bellekte bir adresi vardır, değiştirilebilir


fn veri_tipleri(){
    let a:u8 = 125; // u = unsigned, 0-255 8 bits 0 - 2^(N-1)
    println!("a: {}", a);

    let mut b:i8 = 0; // i = signed, -128 to 127 8 bits -2^(N-1) - 2^(N-1)
    println!("önce b: {}", b);

    b = 22;
    println!("sonra b: {}", b);

    let c = 123456789; // i32 default, -2,147,483,648 to 2,147,483,647 32 bits
    println!("c: {} ve boyutu: {}", c, mem::size_of_val(&c));

    let d:isize = -200; // isize default, -2,147,483,648 to 2,147,483,647 32 bits
    println!("d: {} ve boyutu: {}", d, mem::size_of_val(&d));

    let e:char = 'A'; // char, 4 bytes, 32 bits
    println!("e: {} ve boyutu: {}", e, mem::size_of_val(&e));

    let f:f32 = 3.14; // f32, 4 bytes, 32 bits
    println!("f: {} ve boyutu: {}", f, mem::size_of_val(&f));

    let g:bool = false; // bool, 1 byte, 8 bits
    println!("g: {} ve boyutu: {}", g, mem::size_of_val(&g));
}

fn aritmetik_islemler(){
    let mut a = 10+5+9;
    println!("a: {}", a);

    a = a - 5; // -- veya ++ Rust'da yoktur
    println!("a: {}", a);

    a += 10; // a = a + 10
    println!("a: {}", a);

    a *= 2; // a = a * 2
    println!("a: {}", a);

    a /= 3; // a = a / 3
    println!("a: {}", a);

    println!("kalan {} / {} = {} işleminden kalan", a, 3, a % 3);

    let a_kupu = i32::pow(a, 3); // a^3
    println!("a^3 = {}", a_kupu);

    let b = 2.5;
    let b_kupu = f64::powi(b, 3); // b^3
    println!("b^3 = {}", b_kupu);

    let b_ustu_pi = f64::powf(b, std::f64::consts::PI); // b^pi
    println!("b^pi = {}", b_ustu_pi);

}   

fn bitwise_logic(){
   let c = 1|2; // 0001 | 0010 = 0011 = 3
   println!("1|2 = {}", c);

   // mantıksal işlemlerinde <, >, <=, >=, ==, != operatörleri kullanılır
   let pi_kucuk_mu = std::f64::consts::PI < 3.0; // false
   println!("pi < 3.0 = {}", pi_kucuk_mu);

}



fn main() {
    //veri_tipleri();
    //aritmetik_islemler();
    //bitwise_logic();
    //println!("VERI: {}", VERI);

    //unsafe {
    //    VERI2 = 25;
    //}
    //let veri2_kopya = unsafe { VERI2 };
    //println!("VERI2: {}", veri2_kopya);

    //sh::stack_and_heap();

    //kontrolakisi::kontrol_akisi();
    //kontrolakisi::if_statements();

    //kontrolakisi::while_loop_example();
    //kontrolakisi::while_loop_with_break();
    //kontrolakisi::while_loop_with_continue();
    //kontrolakisi::for_loop_example();
    //kontrolakisi::match_case_example();
    //kontrolakisi::example1();
    //kontrolakisi::structs();
    //kontrolakisi::enums();
    //kontrolakisi::unions();

    let iof2 = kontrolakisi::IntOrFloat { f: 3.14 };
    kontrolakisi::process_value(iof2);

    let iof3 = kontrolakisi::IntOrFloat { i: 44 };
    kontrolakisi::process_value(iof3);

    let iof4 = kontrolakisi::IntOrFloat { i:1 };
    kontrolakisi::process_value(iof4);

    
}
