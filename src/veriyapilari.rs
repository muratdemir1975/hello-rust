
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


pub fn matris() {
    let mtx:[[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
    println!("Matrisin boyutu: {} satır, {} sütun", mtx.len(), mtx[0].len());
    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            println!("Matrisin: [{}] [{}] elemanı: {}", i+1, j+1, mtx[i][j]);
        }
    }
}

pub fn use_slice(slice:&mut [i32]) {
    println!("Slice boyutu: {}", slice.len());
    for i in 0..slice.len() {
        println!("Slice elemanı {}: {}", i+1, slice[i]);
    }
}

pub fn slices() {
    let mut data = [1, 2, 3, 4, 5];
    use_slice(&mut data[1..4]); // 2, 3, 4
}

