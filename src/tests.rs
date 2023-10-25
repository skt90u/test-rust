#[cfg(test)]
mod tests {

    fn sum(a:i32, b:i32) -> i32 {
        return a + b;
    }

    #[test]
    fn it_works() {
        let result = sum(3, 4);
        assert_eq!(result, 7);
    }
}