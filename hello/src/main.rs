fn main () {
    hello();
}

fn hello() -> bool {
    println!("Hello World!");
}

#[cfg(test)]
mod tests {
    use crate::hello

    #[test]
    fn hello_test() {
        assert_eq!(hello(), true);
    }
}