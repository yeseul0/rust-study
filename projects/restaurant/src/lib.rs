//front_of_house 모듈 내에 hosting, serving 모듈 nested
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
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

//front_of_house 공개 API
pub fn eat_at_restaurant() {
    //absolute path : crate 루트부터 시작. (file system이랑 비슷))
    crate::front_of_house::hosting::add_to_waitlist();
    // -> eat_at_waitlist()만 이동시에는 절대경로가 유리

    //relative path : 작성되고 있는 현재 모듈 기준. self, super 식별 사용 가능
    front_of_house::hosting::add_to_waitlist();
    // -> front_of_house , eat_at_restaurant() 같이 이동시에 상대경로가 유리

    //hosting이 private이라 compile error

}
