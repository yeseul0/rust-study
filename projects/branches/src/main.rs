fn main() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if number {
    // }  -> 오류 발생!! 조건식은 반드시 bool 타입이어야 함. (rust는 bool타입으로 자동변환 안 해줌)

    let number = 6; 
    if number % 4 ==0 {
        println!("number is divisible by 4");
    } else if number % 3 ==0 {
        println!("number is divisible by 3");
    } else if number % 2 ==0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // 조건식이 if 표현식으로 사용될 때 (숫자는 그 자체로 표현식!!)
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
    //let number = if condition { 5 } else { "six" }; 
    // 에러!! -> if 표현식은 모든 반환 타입이 호환되어야한다.!! 
}