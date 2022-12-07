fn main() {
    let test: &str = "hello";
    let wanted = "hey programming";
    let chars: Vec<char> = test.chars().filter(|s| wanted.contains(s)).collect();
    drop(test);
    println!("{:?}", chars);
}
