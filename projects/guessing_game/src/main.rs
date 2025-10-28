use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    //rand::thread_rng()  OS가 시드 (seed) 를 정하고 현재 스레드에서만 사용되는 난수 생성기 반환

    println!("The secret number is: {secret_number}"); 
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        // String::new()  타입 자체에서 호출. (new라는 함수가 새로운 String 인스턴스를 생성하는 함수로써 String 타입에 속해있는것!!!)
        // impl에서 정의하는 함수는 method랑 연관함수가 있음.
        //연관함수는 타입 레벨에서 호출이 됨. 
        // 메소드는 특정 인스턴스에서 호출되는것. 

        io::stdin()
            .read_line(&mut guess) //사용자 입력이 guess 변수에 저장됨. read_line 함수는 애초에 가변 참조자를 인자로 받음
            //read_line 함수는 Result<usize, io::Error> 타입 반환. (Result <T, E> 제네릭 타입) 
            //Result 타입은 열거형으로 Ok(T) 와 Err(E) 두 가지 배리언트를 가짐
            .expect("Failed to read line");//expect는 Result 의 메소드

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // let guess: u32 = guess.trim().parse().expect("Please type a number!"); 
        //guess 변수를 정적 변수로 다시 선언 (섀도잉).

        println!("You guessed: {guess}");

        

        match guess.cmp(&secret_number) {
            // cmp 메소드는 두 값을 비교하여 Ordering 열거형을 반환함
            // Less, Greater, Equal이라는 배리언트가 있음!!
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break
            }
        }
        //match 표현식은 Ordering::Greater or Ordering::Less or Ordering::Equal 중 하나를 받아서 각 arm의 패턴 확인
        //매칭 성공하면 해당 arm의 코드를 실행 후 match 표현식 종료

        }

}