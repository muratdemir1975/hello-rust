#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

use core::f32;
use std::mem;
use std::io::stdin;


pub fn Strings () {
    // string literal, binary'nin static/read-only verisinde saklanır; stack'te sadece &str referansı (pointer+uzunluk) var
    let str = "Hello, world!"; // &str, string literal, immutable
    println!("str: {} ve boyutu: {}", str, mem::size_of_val(&str));

    if let Some(ilk_karakter) = str.chars().nth(0) {
        println!("ilk karakter: {}", ilk_karakter);
    }

    // heap üzerinde saklanan string
    let mut s = String::new();
    println!("Bir string giriniz: ");
    stdin().read_line(&mut s).expect("Hata oluştu");
    println!("Girdiğiniz string: {}", s);


    let mut karakterler = String::new();
    let mut a = 'a' as u8;
    while a <= 'z' as u8 {
        karakterler.push(a as char);
        karakterler.push_str(", ");
        a += 1;
    }
    println!("karakterler: {}", karakterler);

    // str to String conversion
    let str2 = "Hello, Rust!";
    let string2 = str2.to_string();
    println!("str2: {} ve boyutu: {}", str2, mem::size_of_val(&str2));
    println!("string2: {} ve boyutu: {}", string2, mem::size_of_val(&string2));

    // String + str
    let str3 = "Hello, ";
    let string3 = "Rust!".to_string();
    let combined = str3.to_string() + &string3;
    println!("combined: {} ve boyutu: {}", combined, mem::size_of_val(&combined));

    let mut metin = String::from("Hello, Rust!");
    let mut metin2 = metin.clone(); // clone() ile heap'teki veriyi kopyalarız, metin2 artık metin'in bir kopyasıdır
    println!("metin: {} ve boyutu: {}", metin, mem::size_of_val(&metin));
    println!("metin2: {} ve boyutu: {}", metin2, mem::size_of_val(&metin2));
    println!("metin ve metin2 aynı mı? {}", metin == metin2);

    // String slicing
    let mut metin3 = String::from("Hello, Rust!");
    let slice = &metin3[0..5]; // slicing, metin3'ün ilk 5 karakterini alır
    println!("slice: {} ve boyutu: {}", slice, mem::size_of_val(&slice));

    // String concatenation
    let mut metin4 = String::from("Hello, ");   
    metin4.push_str("Rust!");
    println!("metin4: {} ve boyutu: {}", metin4, mem::size_of_val(&metin4));

    // String remove
    println!("'{0}' çıkan karakter. Son durum {1}",metin.remove(4), metin);

}

pub fn Strings2 () {
    let isim = "Murat";
    let selamlama = format!("merhaba ben {},\n iyi günler",isim);
    println!("{}", selamlama);

    let selam = "selam";
    let rust = "Rust";
    let selam_rust = format!("{}, {}",selam, rust);
    println!("{}", selam_rust);


}

