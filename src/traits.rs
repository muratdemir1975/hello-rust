use std::result;
use std::fmt::Debug;

trait Animal {
    fn create(name:&'static str)->Self;
    fn name(&self)->&'static str;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    isim:&'static str
}

struct Cat {
    name:&'static str
}

impl Animal for Human
{
    fn create(name:&'static str)->Human{
        Human{isim:name}
    }
    fn name(&self)->&'static str {
        self.isim
    }
    fn talk(&self) {
        println!("{} says hello", self.name());
    }
}

impl Animal for Cat
{
    fn create(name:&'static str)->Self {
        Cat{name:name}
    }
    fn name(&self)->&'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says meow", self.name);
    }
}

trait Summable<T> {
    fn sum(&self)->T;
}

impl Summable<i32> for Vec<i32>
{
    fn sum(&self)->i32{
        let mut result = 0;
        for x in self {
            result += *x;
        }
        result
    }
}

pub fn traits(){
        // let h=Human{isim:"Murat"};
        // let h=Human::create("Murat");
        let h:Human=Animal::create("Murat");
        h.talk();

        let c=Cat{name:"Toprak"};
        c.talk();

        let a=vec![1,2,3];
        println!("{:?}",a);
        println!("sum = {}",a.sum());
}

#[derive(Debug)]
pub struct Circle {
    pub radius:f64,
}

#[derive(Debug)]
pub struct Square {
    pub side:f64,
}

pub trait Shape {
    fn area(&self)->f64;
}

impl Shape for Square{
    fn area(&self)->f64{
        self.side * self.side
    }
}

impl Shape for Circle{
    fn area(&self)->f64{
        self.radius * self.radius * std::f64::consts::PI
    }
}

// pub fn bilgi_ver(shape:impl Shape + Debug){
// pub fn bilgi_ver<T:Shape + Debug>(shape:T){
pub fn bilgi_ver<T>(shape:T)
where T:Shape + Debug{
    println!("{:?}", shape);
    println!("Alan: {}",shape.area())
}

pub struct Person {
    pub name: String
}

// impl Person {
//     pub fn new(name:&str)->Person{
//         Person { name:name.to_string() }
//     }

impl Person {
    pub fn new<S: Into<String>>(name:S)->Person{
        Person { name:name.into() }
}

}