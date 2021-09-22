pub fn add_two(input: i32) -> i32 {
    internal_adder(input, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    // use super::add_two;
    use crate::internal_adder;

    #[test]
    fn internal() {
        assert_eq!(internal_adder(2, 3), 5);
    }
}
