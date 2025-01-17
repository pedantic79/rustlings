fn trim_me(input: &str) -> &str {
    // TODO: Remove whitespace from both ends of a string.

    let mut start = 0;
    let mut end = input.len();

    for (i, c) in input.char_indices() {
        if !c.is_whitespace() {
            start = i;
            break;
        }
    }

    for (i, c) in input.char_indices().rev() {
        if !c.is_whitespace() {
            end = i + c.len_utf8();
            break;
        }
    }

    &input[start..end]
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this.
    input.to_string() + " world!"
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons".
    let mut result = String::new();
    let mut i = 0;
    let input = input.as_bytes();

    while i < input.len() {
        if input[i..].starts_with(b"cars") {
            result.push_str("balloons");
            i += "cars".len();
        } else {
            result.push(input[i] as char);
            i += 1;
        }
    }
    result
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
