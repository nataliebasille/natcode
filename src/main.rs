fn greet(name: &str) -> String {
    format!("Hello, {name}! Welcome to natcode 🦀")
}

fn add(a: i64, b: i64) -> i64 {
    a + b
}

fn main() {
    println!("{}", greet("world"));
    println!("2 + 3 = {}", add(2, 3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_includes_name() {
        assert_eq!(greet("Nat"), "Hello, Nat! Welcome to natcode 🦀");
    }

    #[test]
    fn add_works() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-4, 4), 0);
    }
}
