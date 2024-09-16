pub fn main() {
    std::fs::write(
        std::env::var("OUT_DIR").unwrap() + "/words.rs",
        [String::from("[")]
            .into_iter()
            .chain(
                std::fs::read_to_string("words.txt")
                    .unwrap()
                    .split('\n')
                    .map(|word| format!("\"{word}\",")),
            )
            .chain([String::from("]")])
            .collect::<String>(),
    )
    .unwrap();
}
