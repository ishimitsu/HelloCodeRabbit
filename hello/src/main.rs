fn main () {
    add(1, 2);
}

fn add(a :i32, b :i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use crate::add;

    #[test]
    fn hello_test() {
        let a:i32 = 1;
        let b:i32 = 2;
        assert_eq!(add(a, b), a + b);
    }
}