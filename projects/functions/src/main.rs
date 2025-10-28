fn main() {
    println!("Hello, world!");

    another_function(5);
    print_labeled_measurement(5, 'h');
    let x= five();
    println!("The value of x is: {x}");

    let y = expression_statement();
    println!("The value of y is: {y}");

    let z = plus_one(5);
    println!("The value of z is: {z}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

//expression(표현식)과 statement(구문) 차이
fn expression_statement() -> i32 {
    //statement(구문) : 값을 반환하지 않음. ex) let x = 5; , fn foo() {}
    //let x = (let y = 6); 
    //let y=6은 식 자체가 어떤 값을 나타내지 않아서 구문임. 

    let y = {
        let x=3;
        x+1 // 함수는 마지막 표현식 값을 반환하거든 ㅇㅇ 얘가 리턴값인 것
    }; //얘는 표현식인 것!!
    y //; 세미 콜론 붙이면 구문이 돼서 반환값이 없어짐!! () 반환함. 
}

fn five() -> i32 { //함수가 값을 반환하려면 타입을 시그니처에 꼭 꼭 선언해야함.. 
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1 //마지막 표현식이 반환값
}