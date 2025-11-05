fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        y.push(42); //이 아래에서y 안쓰면, 이 가변참조는 여기까지 유효한것. 호출당하는 영역내에, 같은 것을 가변참조하는게 2개 이상있으면 안되는거임 ㅇㅇ
        let z = &mut x;
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
