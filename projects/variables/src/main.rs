fn main() {
    // Mutable variable (기본적으로 let으로 선언된 변수는 불변.)
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constant (상수는 기본적으로 불변이 아니라 항상 불변이며(mut 붙이지 않는다.), 타입을 명시해야 함.)
    const MAX_POINTS: u32 = 100_000;

    // Shadowing (같은 이름의 변수를 다시 선언하여 값을 변경할 수 있음.===변수명 재사용)
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");

    //그럼 let mut y 였다면 y = y + 1 , let y 였다면 let y = y + 1 로 작성해야함!? 
    // -> immutable 변수 값을 바꾸고 싶으면 let으로 계속 변수를 재선언하기

    //가장 큰 차이는 타입!! let mut 은 같은 타입만 가능하지만 let 변수 재선언은 타입이 달라도 가능)

    //Type
    let decimal = 9_8222_u32;       // 98222라는 수를 u32타입으로 저장
    let hex = 0xff_u8;            // 255라는 수를 u8 타입으로 저장  
    let octal = 0o77_u8;          // 63이라는 수를 u8 타입으로 저장
    let binary = 0b1111_0000_u8;  // 240이라는 수를 u8타입으로 저장

    println!("decimal: {}", decimal);
    println!("hex: {}", hex);
    println!("octal: {}", octal);  
    println!("binary: {}", binary);

    println!("{}", -5/3); //-1 인데, 0 방향으로 소수점 제거!!! 

    let c = 'z';
    let z: char = 'ℤ'; // 명시적인 타입 어노테이션
    let heart_eyed_cat = '😻';

    println!("c: {}, z: {}, heart_eyed_cat: {}", c, z, heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {y}");
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("five_hundred: {}, six_point_four: {}, one: {}", five_hundred, six_point_four, one);

    // 배열은 크기 고정. (vector는 크기 변동)
    let a = [1, 2, 3, 4, 5];  //mut 아니니까 원소 수정 불가
    let a: [i32; 5] = [1, 2, 3, 4, 5]; //배열은 튜플과 다르게 모든 원소 타입 일치해야함
    let first = a[0];
    let second = a[1];

}
