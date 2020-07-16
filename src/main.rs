
fn say_hello(name: &str) -> String {
    format!("Hello {}!", name)
}

fn main() {
    println!("{}", say_hello("world"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_say_hello() {
        assert_eq!("Hello world!", say_hello("world"));
    }
}
