fn main() {
    let s = "hello"; //문자열 리터럴 - > stack 저장
    {
        let s = String::from("hello"); //String 타입 - > heap 저장
    } // s 가 스코프를 벗어나면서 drop 함수 호출하고 메모리에서 해제됨
    //drop은 해당 타입 개발자가 메모리 해제하게끔 구현해놓은 함수

    let mut v = String::from("hello world");

    let word = first_word(&v); // word는 값 5를 가짐

    v.clear(); // String을 비워서 ""으로 만듦

    // 여기서 word에는 여전히 5가 들어있지만, 이 5를 의미있게 쓸 수 있는
    // 문자열은 더 이상 없습니다. word는 이제 전혀 유효하지 않음. 
    // -> v가 바뀌어도, 구해놓은 첫 공백 위치 5라는 값이 그대로 word에 남아있음.. 
    //word, v의 동기화가 깨질 수 있음.. 컴파일은 잘 되거든.. 

    let w = "Hello, world!"; //문자열 슬라이스 - > stack 저장, &str 타입.. (문자열 슬라이스임) 

    let my_string = Stirng::from("hello world"); //String 타입, heap 저장
    let word = first_word_slice(&my_string[..]); //String 전체 슬라이스
    let word = first_word_slice(&my_string[0..7]); //String 부분 슬라이스
    let word = first_word_slice(&my_string); //String 참조자도 ok

    let my_string_literal = "hello world"; //&str 타입, 문자열 슬라이스
    word = first_word_slice(&my_string_literal[..]); //문자열 슬라이스 전체
    word = first_word_slice(&my_string_literal[0..6]); //문자열 슬라이스 부분
    word = first_word_slice(my_string_literal); //애초에 &str 타입이므로 참조자 불필요
    

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); //문자열 개별 검사하려고 바이트 배열로 변환

    for (i, &item) in bytes.iter().enumerate() {
        // iter 메서드는 컬렉션의 각 요소를 반환
        // enumerate 메서드는 인덱스와 값을 튜플로 반환
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}