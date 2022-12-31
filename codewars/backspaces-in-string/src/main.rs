fn main() {
    println!("Hello, world!");

    clean_string(&String::from("aa#a#b##c"));
}

fn clean_string(s: &str) -> String {
    let mut chars: Vec<char> = Vec::new();
    for c in s.chars() {
        if c == '#' {
            if chars.len() > 0 {
                chars.pop();
            }
        }
        else {
            chars.push(c);
        }
    }
    chars.iter().collect::<String>()
}
