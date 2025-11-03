#[derive(Debug)] //enum variant도 println! {:?} or {:#?} 가능
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin { //어떤 표현식이든 가능 (지금은 enum type) 
        Coin::Penny => 1, //arm (패턴 => 표현식)
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { //state는 UsState :: Alabama, Alaska 중 하나
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

//Option<i32>를 매개변수로 받아서, 내부에 값이 있으면 그 값에 1을 더하는 함수
fn plus_one(x:Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1), //Some(i)는 Option<i32> 내부 Some(i32) 패턴 매칭
    }
}

//주사위 3 : 움직X 모자얻, 주사위 7 : 움직O 모자 잃, 나머지 : 그냥 이동


fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces : u8) {}

fn main() {
    let my_coin = Coin::Quarter(UsState::Alabama);
    println!("Value in cents:{}", value_in_cents(&my_coin));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), //_ => () 이렇게 패스 할 수도 있음. (+ 전달값 바인딩 안해도 될 떄)
    }

    //if let 제어 (match version)
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    //if let version
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    //if let + else
    let mut count = 0;
    if let Coin::Quarter(state) = my_coin { //참고!!! state가 my_coin의 UsState 값이랑 바인딩 되면서 소유권 가져간다. 싫으면 &my_coin으로 !!
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    
}