use rand::Rng;
use std::io::stdin;

pub fn sayi_tahmin_oyunu () {
    let number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Tahmininizi girin: ");
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer){
            Ok(_)=>{
                let parsed = buffer.trim_end().parse::<i64>();
                match parsed {
                    Ok(tahmin)=>{
                        if tahmin<1 || tahmin>100 {
                            println!("1-100 arasınnda değer girin")
                        }else if tahmin>number {
                            println!("Tahmininiz yüksek daha düşük bir değer giriniz.");
                        }else if tahmin<number {
                            println!("Tahmininiz düşük daha yüksek bir değer giriniz.");
                        }else {
                            println!("Tabrikler. Tahmininiz doğru.");
                            break;
                        }
                    },
                    Err(e)=>{
                        println!("Input veri okunaması. {} Tekrar deneyim",e)
                    }
                }
            },
            Err(_)=>continue,
        }
    }
}