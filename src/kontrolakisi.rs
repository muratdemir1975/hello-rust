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
