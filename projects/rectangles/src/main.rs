// 단일 변수 버전
// fn main () {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1,height1)
//     );
// }

// fn area(width: u32, height: u32) ->  u32 { //들어오는 저 인자 둘이 연관이 있는지 알수가 없음. 
//     width * height
// }

//튜플 버전
// fn main () {
//     let rect1 = (30,50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1 //인덱스로 튜플 접근.. 인덱스 많아지면 의미 기억하기가 어려움
// }

//구조체 버전
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { //Rectangle 구조체에 대한 implementation 블록.. 
    fn area(&self) -> u32 { //구조체 내부 함수 = method, 첫 인자 무조건 자기참조
        self.width * self.height
    }
    //이 블록 내에서 Self는 impl 블록 대상이 되는 타입의 참조자임
    // Self 타입의 self이름의 매개변수를 가져야하는 게 impl 관례
    //여기선 읽기 전용 참조자이므로 &self
    //바꿀수도 있으면 &mut self (대신 개체도 가변이겠지)
    //그냥 self로 받으면 소유권이 넘어옴 주의

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> String {
        if self.width > other.width && self.height > other.height {
            String::from("yes")
        } else {
            String::from("no")
        }
    }

    //첫 인자가 Self가 아니므로 메소드는 아닌 연관함수. 
    fn square(size: u32) -> Self { //Self는 impl 블록 대상이 되는 타입의 별칭!!!! 여기선 Rectangle
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1) //참조자로 넘기기
        //구조체의 소유권을 넘겨 버리면 다른 변수로 다시 받아내야함..
        //rectangle 매개변수의 타입을 불변 참조자로 해서 소유권을 빌려주기만 하자!
    );
    println!("rect1 is {:#?}", rect1); 

    let scale = 2 ;
    let rect2 = Rectangle {
        width: dbg!(30 * scale), //dbg! 매크로는 값을 출력하고 그 값을 반환함(= 여기선 stack 변수라서 move 아니고 copy)
        height: 50,
    };

    
    dbg!(&rect2); //참조자로 넘기기(얘도 copy)
    //dbg!(rect2); //이렇게 하면 소유권이 dbg! 매크로로 넘어갔다 오면서 받아주는 변수가 없으니.. 버려짐..
    //println!("rect2 is {:#?}", &rect2); rect2를 사용할 수 없음! 

    let temp_rect = dbg!(&rect2); //temp_rect은 &Rectangle 타입. rect2를 참조중
    println!("temp_rect is {:#?}", temp_rect);  //참조자로 넘겨서 괜찮.

    println!("rect2 is {:#?}", rect2); //참조자로 넘겨서 괜찮. 만약 위에서 dbg!결과를 누가 받았다면 얘는 쓸 수 없어짐.


    //method
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    //매개변수 많은 메소드
    let rect_1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect_2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect_3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect_1 hold rect_2? {}", rect_1.can_hold(&rect_2));
    println!("Can rect_1 hold rect_3? {}", rect_1.can_hold(&rect_3));

    let sq=Rectangle::square(3);
    println!("Square is {:#?}", sq);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height //의미가 훨씬 명료해짐!
}
