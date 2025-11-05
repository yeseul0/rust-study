
enum SpreadsheetCell{
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v = vec![100, 32, 57];
    for i in &v { //i가 각요소 불변 참조자로 iterating
        println!("{i}");
    }

    let mut mut_v = vec![100, 32, 57];
    for i in &mut mut_v { //가변 벡터를 참조할 가변 참조자로 iterating
        *i +=50; //참조자 i 의 역참조. 값에 접근하는 연산자 * (요소 값 수정 가능)
        //mut_v.push(4); 안돼!!!(why? 이중 가변 참조: 이미 mut_v 가변 참조가 8번줄에 존재. )
        //push 가변 참조를 넘겨주는 메소드임. push(&mut v, 4)로 변환됨 -> 이중 가변참조 

    }//for 루프가 돌아가는 동안에는 참조당하는 벡터들의 아이템 추가/삭제(벡터 구조) 수정 절대 불가.
    //-> for 스코프 벗어나야 mut_v 구조 변경 가능 

    //for 스코프 내부에서 mut_v 전체를 가변으로 빌린 상태에서 
    //mut_v.push() 하면 -> vec<T> 내부 함수로 소유권이 넘어감 

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blud"),)
    ];
    //실제론 int, float, string 타입이지만, SpreadsheetCell 타입으로 묶어서 하나의 타입으로 통일했으니 벡터에 같이 넣을 수 있음

    let mut s = String::new();
    let data = "initial contents";
    let s2 = data.to_string(); //리터럴 타입에 Display 트레이트가 구현되어있어서 to_string 메소드 사용 가능. 
    let s3 = String::from("initial contents"); //바로 윗줄과 동일 코드임
    //String::from(리터럴) 과 리터럴.to_string은 똑같이 작동한다!!

    let mut s4 = String::from("foo"); //s4 에 foo
    //push_str(&mut self, string: &str) 메소드
    s4.push_str("bar"); //s4애 foobar
    // &str이 인자라서 소유권 안 가져감.

    //push(&mut self, ch: char) 메소드
    s4.push('h'); //한 글자만 매개변수로 추가. u8 크기. 

    //String에 + 연산자 -> fn add(self, s: &str) -> String {
    //아!! String에는 String을 더하는 게 아니라 &str 슬라이스를 더해야한다!!
    let s_1 = String::from("Hello, ");
    let s_2 = String::from("world!");
    let s_3 = s_1 + &s_2; // s_1은 add함수로 이동되어 더이상 쓸 수 없음, +연산자 뒤에는 &str이 와야함. (연산자 재정의 add 함수 시그니처 때문.)
    // &s_2는 &String이지만, &str로 강제됨
    //s_1 + &s_2 -> s_1.add(&s_2)랑 같음

    let s5 = String::from("hello");
    // let h = s5[0]; //String 인덱스 접근은 에러. 
    // 보이는 문자 하나가 항상 메모리에서 1바이트씩 차지하는 게 아닐 수도 있음..(UFT-8 인코딩)

    //그래서 slice로 !!
    let hello = "Здравствуйте";
    let s = &hello[0..4]; 
    let s2 = &hello[0..1]; //runtime error (String에 slice치는건 문법은 맞아서 compile 가능))

    for c in "Зд".chars() {
        println!("{c}");
    } 
    //З
    //д 출력


    for b in "Зд".bytes() {
        println!("{b}");
    }
    // 208
    // 151
    // 208
    // 180


}   
