// test3.rs
// This is a test for the following sections:
// - Tests

// This test isn't testing our function -- make it do that in such a way that
// the test passes. Then write a second test that tests that we get the result
// we expect to get when we call `times_two` with a negative number.
// No hints, you can do this :)

// I AM NOT DONE

pub fn times_two(num: i32) -> i32 {
    num * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_twice_of_positive_numbers() {
        assert_eq!(times_two(4), ???);
    }

    #[test]
    fn returns_twice_of_negative_numbers() {
        // TODO write an assert for `times_two(-4)`
    }
fn main() {
    ("blue");
    ("red".to_string());
    (String::from("hi"));
    ("rust is fun!".to_owned());
    ("nice weather".to_string());
    (format!("Interpolation {}", "Station"));
    (&String::from("abc")[0..1]);
    ("  hello there ".trim());
    ("Happy Monday!".to_string().replace("Mon", "Tues"));
    ("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
