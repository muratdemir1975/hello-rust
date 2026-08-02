#![allow(dead_code)]

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

