#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

use core::f32;
use std::mem;
use std::io::stdin;

pub fn kontrol_akisi() {
    let x = 5;
    if x == 5 {
        println!("x equals 5");
    } else {
        println!("x does not equal 5");
    }
}

pub fn if_statements() {
    let sicaklik = 30;
    if sicaklik > 25 {
        println!("Hava sıcak, dışarı çıkabilirsin.");
    } else if sicaklik < 15 {
        println!("Hava soğuk, kalın giyinmelisin.");
    } else {
        println!("Hava ılıman, dışarı çıkabilirsin.");
    }

    println!("Hava durumu özet: {}", if sicaklik > 25 {
        "Sıcak"
    } else if sicaklik < 15 {
        "Soğuk"
    } else {
        "Ilıman"
    });
}

pub fn while_loop_example() {
    let mut sayac = 0;
    while sayac < 5 {
        println!("Sayaç: {}", sayac);
        sayac += 1;
    }
}

pub fn while_loop_with_break() {
    let mut sayac = 0;
    while sayac < 10 {
        if sayac == 5 {
            println!("Sayaç 5'e ulaştı, döngüden çıkılıyor.");
            break;
        }
        println!("Sayaç: {}", sayac);
        sayac += 1;
    }
}

pub fn while_loop_with_continue() {
    let mut sayac = 0;
    while sayac < 10 {
        sayac += 1;
        if sayac % 2 == 0 {
            continue; // Çift sayıları atla
        }
        println!("Sayaç (tek sayılar): {}", sayac);
    }
}

pub fn for_loop_example() {
    for i in 0..10 {
        if i==2 {continue;}// 2'yi atla
        if i==5 {break;}// 5'te döngüyü kır
        println!("For döngüsü sayacı: {}", i);
    }

    println!("--enumerate ile for döngüsü--");

    for (poz,y) in (20..25).enumerate() {
        println!("poz: {}, y: {}", poz, y);
    }

    println!("--enumerate ile for döngüsü--");

    let meyveler = vec!["Elma", "Muz", "Kiraz"];
    for (index, meyve) in meyveler.iter().enumerate() {
        println!("Meyve {}: {}", index, meyve);
    }

}

pub fn match_case_example() {
    let ukle_kodu = 90;
    let ulke_adi = match ukle_kodu {
        1 => "ABD",
        44 => "İngiltere",
        90 => "Türkiye",
        _ => "Bilinmeyen ülke",
    };
    println!("Ülke adı: {}", ulke_adi);
}

enum State {
    Locked,
    Unlocked,
    Failed
}

pub fn example1 (){
    let code = String::from("1234");
    let mut state = State::Locked;

    let mut entry = String::new();

    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        entry.push_str(&input.trim_end());
                    }
                    Err(_) => {
                        println!("Giriş okunamadı.");
                        continue;
                    }
                }
                if entry == code {
                    state = State::Unlocked;
                    continue;
                }
                if !code.starts_with(&entry) {
                    state = State::Failed;
                }
            }
            State::Failed => {
                println!("Hatalı giriş! Tekrar deneyin.");
                entry.clear();
                state = State::Locked;
                continue;
            }
            State::Unlocked => {
                println!("Giriş başarılı! Sistem açıldı.");
                break;
            }
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

struct line {
    start: Point,
    end: Point,
}

pub fn structs (){
    let p = Point { x: 10, y: 20 };
    println!("Point coordinates: ({}, {})", p.x, p.y);

    let l = line {
        start: Point { x: 0, y: 0 },
        end: Point { x: 10, y: 10 },
    };
    println!("Line coordinates: ({}, {}) to ({}, {})", l.start.x, l.start.y, l.end.x, l.end.y);
}


enum Color {
    Red,
    Green,
    Blue,
    RGBColor(u8, u8, u8),
    CMYKColor { cyan: u8, magenta: u8, yellow: u8, black: u8 }
}

pub fn enums (){
    let c = Color::RGBColor(255, 0, 0);
    match c {
        Color::Red => println!("Color is Red"),
        Color::Green => println!("Color is Green"),
        Color::Blue => println!("Color is Blue"),
        Color::RGBColor(255, 0, 0) |
        Color::CMYKColor { cyan: 255, magenta: 0, yellow: 0, black: 0 } => println!("Color is Red (RGB)"),
        _ => println!("Color is something else")
    }
}


pub union IntOrFloat {
    pub i: i32,
    pub f: f32
}

pub fn unions() {
    let mut u = IntOrFloat { i: 42 };
    unsafe {
        println!("Union value as integer: {}", u.i);
        u.f = 3.14;
        println!("Union value as float: {}", u.f);
    }
}

pub fn process_value(iof:IntOrFloat) {
    unsafe {
        match iof{
            IntOrFloat {i:44} => {println!("Değer: 44 dür");}
            IntOrFloat {f} => {println!("2 durum Değer: {f}");}
            _ => {println!("Değer: bilinmiyor");}

        }
    }
}


pub fn example2() {
    let x = 2.0;
    let y = 1.0;

    let result = if y != 0.0 { Some(x/y) } else {None};

    match result {
        Some(value) => println!("{} / {} Sonuç: {}",x,y, value),
        None => println!("Hata: Sıfıra bölme hatası!"),
    }

    if let Some(value) = result {
        println!("{} / {} Sonuç: {}",x,y, value);
    } else {
        println!("Hata: Sıfıra bölme hatası!");
    }
}


