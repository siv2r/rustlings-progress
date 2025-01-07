// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.


fn trim_me(input: &str) -> String {
    let mut res = input.to_string();
    while !res.is_empty() && res.as_bytes()[0] == b' ' {
        res.remove(0);
    }
    while !res.is_empty() && res.as_bytes()[res.len() - 1] == b' ' {
        res.pop();
    }
    res
}

fn compose_me(input: &str) -> String {
    let mut res = input.to_string();
    res.push_str(" world!");
    res
}

fn replace_me(input: &str) -> String {
    let mut res = String::new();
    for (i, word) in input.split_whitespace().enumerate() {
        if i > 0 {
            res.push(' ');
        }
        if word == "cars" {
            res.push_str("balloons")
        } else {
            res.push_str(word)
        }
    }
    res
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
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
