use regex::Regex;

fn is_valid_ip(ip: &str) -> bool {
    let re = Regex::new(r"^(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})$").expect("Invalid regex.");
    for cap in re.captures_iter(ip) {

        if cap.len() != 5 {
            return false;
        }

        for i in 0..4 {
            let digit = &cap[i+1];
            if digit.len() > 1 && digit.starts_with('0') {
                return false;
            }
            let c: i32 = digit.parse().unwrap();
            if c < 0 || c > 255 {
                return false;
            }
        }
    }

    true
}

fn main() {
    let ip = String::from("255.255.255.255.255");
    let result = is_valid_ip(&ip);
    println!("{} is {}", ip, result);
}
