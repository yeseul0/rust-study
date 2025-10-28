fn main() {
    loop {
        println!("Hello, world!");
        break;
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2 ;  //break 표현식임!! 이 loop의 리턴값이 된다.
            // (세미콜론 찍어도 안찍어도 상관없음) why? : 
            // compiler가 ';'을 만나면: 
            // 1. 앞의 표현식을 구문으로 변환 
            // 2. 반환값을 () (unit type)으로 변경!!! 하는데,
            //여기서는 세미콜론을 만나기 전에 break이 제어권을 가져가버림.!!! 
        }
    };

    println!("The result is {result}");



    let mut count = 0;
    'counting_up: loop { //라벨 붙인 루프
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; //지금 속해있는 loop가 아닌 라벨이 붙은 루프를 나간다. (현재 loop, 상위 loop 라벨만 가능)
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let a = [10, 20, 30, 40, 50];

    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    } //만약 index<6 이런식이면, index가 5일때, a[5]에 접근하려고 해서 런타임 에러 발생!!위험!!

    for element in a {
        println!("the value is: {element}");
    } //배열 끝까지만 빼먹지 않고 안전하게 반복 가능. (while문보다 선호됨)

    for number in (1..4).rev() { //python range(1,4) 과 동일. rust (1..4) -> 1,2,3 / (1..=4) -> 1,2,3,4
        println!("{number}!");
    }
    println!("LIFTOFF!!!");


}
