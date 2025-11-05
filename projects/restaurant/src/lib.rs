//front_of_house 모듈 내에 hosting, serving 모듈 nested
fn deliver_order() {}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }

    mod back_of_house {
        fn fix_incorrect_order() {
            cook_order();
            super::deliver_order(); //여기서 super모듈은 crate임.
        }

        fn cook_order() {}

        pub struct Breakfast {
            pub toast: String,
            seosonal_fruit: String,
        }
        impl Breakfast {
            //Breakfast 구조체는 seosonal_fruit 비공필드 있어서, 꼭 비공필드 값 넣어주는 연관함수가 필요함!!
            // (없으면 객체 생성 불가!)
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast{
                    toast: String::from(toast),
                    seasonal_fruit: String::from("peaches"),
                }
            }
        }

        pub enum Appetizer {
            Soup,
            Salad,
        }
    }
}

// module tree
// crate (아 crate가 module tree의 루트가 되는구나.)
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment

use crate::front_of_house::hosting; //호출할 함수의 부모 묘듈까지만 단축하는게 관례
//이제 hosting만으로도 접근 가능



//front_of_house 공개 API
pub fn eat_at_restaurant() {
    //use (like file system - symbolic link)
    hosting::add_to_waitlist(); //use crate::front_of_house::hosting; 이 같은 스코프에 있음!!! 
    //eat_at_restaurant()가 다른 mod로 감싸져 있어서 use문이랑 다른 스코프에 들어가면 compile error

    //absolute path : crate 루트부터 시작. (file system이랑 비슷))
    crate::front_of_house::hosting::add_to_waitlist();
    // -> eat_at_waitlist()만 이동시에는 절대경로가 유리

    //relative path : 작성되고 있는 현재 모듈 기준. self, super 식별 사용 가능
    front_of_house::hosting::add_to_waitlist();
    // -> front_of_house , eat_at_restaurant() 같이 이동시에 상대경로가 유리

    //hosting이 private이라 compile error -> pub mod
    //근데 hosting 내부 함수도 private임. -> pub fn
    //pub는 상위 모듈이 해당 모듈을 가리킬 수 있도록만 함. 내부 구현은 따로 열어둬야함

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("i'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    
}

