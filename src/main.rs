//fn main() {
//    println!("Hello, world!");
//}


use std::mem;
fn veri() {
    let a = 55;
    print!("a: {}", a);
}

fn main() {
    //veri();
    let a = 55;
    {
        let a = 66;
        print!("döngü içi a: {}\n", a);
    }
    print!("döngü dışı a: {}\n", a);
}