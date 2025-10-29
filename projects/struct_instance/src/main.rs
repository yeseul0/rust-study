//구조체
struct User {
    active: bool,
    username: String, //&str 참조자로 할거면, 구조체가 존재하는 동안에 구조체 내 참조자가 가리키는 데이터의 유효함을 보장하는 lifetime을 명시해야함..
    email: String,
    sign_in_count: u64,
}
//튜플 구조체
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
//이름을 지어주거나 특정 튜플을 다른 튜플과 구분하고는 싶지만, 필드 이름이 필요하지는 않을 때 유용

//유닛 구조체
struct AlwaysEqual;
//필드가 없는 () 구조체. 타입의 특성만 필요할 때 유용


fn main() {
    let mut user1 = User { //가변성은 인스턴스 전체가 지닌다. 필드만 가변으로 만들 수는 없다. 
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");


    let user2 = User {
        //active: user1.active,
        //username: user1.username,
        email: String::from("another@example.com"),
        //sign_in_count: user1.sign_in_count,
        ..user1 //나머지 필드는 user1 인스턴스에서 복사.
        //username 필드의 String(heap 데이터)는 user1->user2 로 move됨.) 이제 접근 불가. 
        //active, sign_in_count 필드는 (stack에만 저장되는 필드) copy됨. 이 필드들은 접근 여전히 가능.
    };
    println!("user2's username is {}", user2.username);
    println!("user1's active is {}", user1.active);
    println!("user2's active is {}", user2.active);
    println!("user1's email is {}", user1.email); //유효
    //println!("user1's username is {}", user1.username);//무효

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}


fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,//:username,
        email, //: email,
        sign_in_count: 1,
    }
    //필드와 매개변수 이름이 같으면 축약형(field init shorthand)으로 작성 가능
}

