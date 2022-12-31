
/*
    A customer ID is given and then a car number plate needs to be generated.

        - It first cycles from 1 to 999, and after that the left-most a is 
          increased to b, and the counter starts at 1.
        - customer ID 0 maps to aaa0001.

    Possibilities per position:    26-26-26-999 

    aaa001...aaa999

*/
fn main() {
    println!("Hello, world!");
}


fn find_the_number_plate(n: u32) -> String {
    let mut quotient = n / 999;
    let remainder = n % 999;

    let mut left_part = String::from("");


    for _ in 0..3 {
        if quotient >= 26 {
            left_part.push('z');
            quotient /= 26;
        }
        else if quotient <= 0 {
            left_part.push('a');
        }
        else {
            left_part.push(char::from_u32(quotient + 'a' as u32).expect("Err"));
            quotient = 0;
        }
    }

    println!("Customer ID: {}", n);
    println!("Remainder: {}", remainder);
    println!("Quotient: {}", quotient);

    format!("{}{:0>3}", left_part, remainder + 1)
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

fn dotest(n: u32, expected: &str) {
    let actual = find_the_number_plate(n);
    assert!(actual == expected, 
        "With n = {n}\nExpected \"{expected}\" but got \"{actual}\"")
}

#[test]
fn fixed_tests() {
    dotest(3,"aaa004");
    dotest(1487,"baa489");
    dotest(40000,"oba041");
    dotest(17558423,"zzz999");
    dotest(43056,"rba100");
    dotest(234567,"aja802");
}