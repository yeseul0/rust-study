//version 1
// enum IpAddrKind{
//     V4,
//     V6,
// }
// struct IpAddr {
//     kind: IpAddrKind, //enum이 구조체의 일부 필드가 됨.
//     address: String
// }

// let home = IpAddr {
//     kind: IpAddrKind::V4,
//     address: String::from("127.0.0.1"),
// };

// let loopback = IpAddr {
//     kind: IpAddrKind::V6,
//     address: String::from("::1"),
// };

//version 2
// enum IpAddr {
//     V4(String), //V4, V6 varient는 각각 String 데이터를 가짐
//     V6(String), //열거 인스턴스의 생성자 함수인 꼴이 됨. 
// }
// //직접 데이터를 붙이면서 구조체 정의 안 해도 됨

// let home = IpAddr::V4(String::from("127.0.0.1"));

// let loopback = IpAddr::V6(String::from("::1"));

// version 3
// 각 배리언트는 다른 타입과 다른 양의 연관된 데이터를 가질 수 있다.
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// let home = IpAddr::V4(127,0,0,1);
// let loopback = IpAddr::V6(String::from("::1"));


//standard library version
struct Ipv4Addr {
    // -- 생략 --
}

struct Ipv6Addr {
    // -- 생략 --
}   

enum IpAddr{ //오호 실제로는 구조체가 열거형 타입으로 들어가있네
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move{x:i32, y:i32}, //뭔가 구조체 형태.. 
    Write(String),
    ChangeColor(i32, i32, i32),
}
//위 Message 열거형을 구조체 4개로 분해 정의하면 아래와 같음
struct QuitMessage; //유닛 구조체
struct MoveMessage {
    x:i32,
    y:i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

impl Message {

    fn call(&self) {
        // method body would be defined here
    }
}


fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    let m = Message::Write(String::from("hello"));
    m.call();
}

// fn route(ip_kind: IpAddrKind) {
    
// }