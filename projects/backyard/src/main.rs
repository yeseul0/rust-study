use crate::garden::vegetables::Asparagus;
//이제 Asparagus 타입에 앞 경로 안붙여도 됨.

pub mod garden;
// src/main.rs 루트 크레이트에서 모듈 선언.
// src/ 안에 있는  
// ./garden.rs 또는 garden/mod.rs를 찾는다. 

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", planet);
}
