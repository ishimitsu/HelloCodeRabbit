fn main () {
}

fn hello() -> bool {
    println!("Hello World!");
    true
}

#[cfg(test)]
mod tests {
    use crate::hello;

    #[test]
    fn hello_test() {
        assert_eq!(hello(), true);
    }
}