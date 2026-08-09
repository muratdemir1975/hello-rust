

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

struct Point {
    x:f64,
    y:f64,
}

struct Line {
    start:Point,
    end:Point,
}

impl Line{
    fn len(&self)->f64 {
        let dx=self.start.x-self.end.x;
        let dy=self.start.y-self.end.y;
        (dx*dx+dy*dy).sqrt()
    }
}

pub fn metodlar() {
    let p=Point{x:3.0, y:4.0};
    let p2=Point{x:5.0, y:10.0};
    let myLine=Line{start:p, end:p2};
    println!("Mesafe: {}", myLine.len());
}
