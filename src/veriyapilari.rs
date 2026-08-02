
use std::mem;

pub fn arrays() {
    let mut dizi = [1, 2, 3, 4, 5];
    println!("Dizinin ilk elemanı: {}", dizi[0]);
    println!("Dizinin uzunluğu: {}", dizi.len());

    dizi[0] = 10;
    println!("Dizinin ilk elemanı güncellendi: {}", dizi[0]);

    for i in 0..dizi.len() {
        println!("Dizinin {}. elemanı: {}", i, dizi[i]);
    }

    let sorgu: bool = dizi[0] == 10;
    println!("Dizinin ilk elemanı 10 mu? {}", sorgu);

    // assert_eq! değer döndürmez, sadece doğrular: eşitse devam eder, değilse panic ile durur
    assert_eq!(dizi[0], 10, "Dizinin ilk elemanı 10 olmalıydı, ama {} bulundu", dizi[0]);
    println!("Doğrulama başarılı: dizinin ilk elemanı 10.");

    let b = [1;10];
    for i in 0..b.len() {
        println!("b dizisinin {}. elemanı: {}", i+1, b[i]);
    }

    println!("b dizisi: {:?}", b);
    println!("b dizisi {} byte boyutundadır", mem::size_of_val(&b));

}