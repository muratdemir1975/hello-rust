

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

pub fn merhaba_de() {
    println!("selamlar");
}

pub fn closures () {
    //merhaba_de();
    let selam=merhaba_de;
    selam();

    let arti_bir = |x:i32|-> i32{x+1};
    let a = 8;
    println!("{} + 1 = {}", a, arti_bir(a));

    let mut iki = 2;
    {
        let arti_iki = |x| {
            let mut z = x;
            z+=iki;
            z
        };

        println!("{} + 2 = {}",22, arti_iki(22));
        iki = 8;
    }

    let odunc_al = &iki;
    println!("{}", odunc_al);

/*  
    let arti_uc = |x:&mut i32| *x+=3;
    let mut k = 14;
    arti_uc(&mut k);
    println!("k={}",k)
 */

    let arti_uc = |mut x: i32| x+=3;
    let mut k = 14;
    arti_uc(k);
    println!("k={}",k)

}

pub fn is_even (x:u32)->bool {
    x%2==0
}

pub fn buyuk_mu(limit: u32)->impl Fn(u32)->bool {
    move |y| y > limit
}

pub fn fonksiyonlar_iki () {
    let limit = 500;
    let mut sum = 0;

    //let limite_kadar = |y| y>limit;
    let limite_kadar = buyuk_mu(limit);

    for i in 0.. {
        let deger = i*i;
        if limite_kadar(deger){
            break;
        }else if is_even(deger) {
            sum+=deger
        }
        println!("döngünün toplamı = {}", sum);
    }

    let toplam = (0..)
        .map(|x| x*x)
        .take_while(|&x| x<limit)
        .filter(|x| is_even(*x))
        .fold(0,|sum,x| sum+x);

    println!("fonksiyonlar ileri yöntemle toplamı: {}", toplam);


}

