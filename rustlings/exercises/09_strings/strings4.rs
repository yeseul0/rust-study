// Calls of this function should be replaced with calls of `string_slice` or `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {
    // 1. 문자열 리터럴 → &str
    string_slice("blue");

    // 2. to_string() → String
    string("red".to_string());

    // 3. String::from() → String
    string(String::from("hi"));

    // 4. to_owned() → String (소유권 복사)
    string("rust is fun!".to_owned());

    // 5. into() → 목적지 타입(String)으로 추론
    string("nice weather".into());

    // 6. format!() → String (Interplation Station)
    string(format!("Interpolation {}", "Station"));

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    // 7. 슬라이싱 → &str (a)
    string_slice(&String::from("abc")[0..1]);

    // 8. trim() → &str (hello there)
    string_slice("  hello there ".trim());

    // 9. replace() → String (Happy TuesDay)
    string("Happy Monday!".replace("Mon", "Tues"));

    // 10. to_lowercase() → String (my shift key is sticky)
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
