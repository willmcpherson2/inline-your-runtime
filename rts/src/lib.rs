#[no_mangle]
pub extern "C" fn foo(n: i32) -> i32 {
    n * 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_foo() {
        assert_eq!(foo(42), 84)
    }
}
