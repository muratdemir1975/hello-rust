

pub fn temel_fonksiyon () {
    println!("temel fonksiyon");
}

// fonksiyona parametre verme
pub fn deger_yaz (x:i32) {
    println!("girdiğiniz değer: {}", x);
}

pub fn deger_artir(x:&mut i32) {
    *x+=1;
}

pub fn carpma_islemi(x:i32,y:i32)->i32{
    x*y
}

pub fn fonksiyonlar() {
    temel_fonksiyon();
    deger_yaz(10);
    let mut z = 22 ;
    deger_artir(&mut z);
    println!("z degerinin artırılmış hali: {}", z);
    let a=2;
    let b=8;
    println!("{} * {} = {}", a, b, carpma_islemi(a, b));
}

