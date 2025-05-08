struct Foo {
    numbers: Vec<i32>,
}

impl Foo {
    #[no_mangle]
    pub extern "C" fn new_foo() -> Box<Self> {
        Box::new(Foo {
            numbers: vec![1, 2, 3],
        })
    }

    #[no_mangle]
    pub extern "C" fn free_foo(_foo: Box<Foo>) {}

    #[no_mangle]
    pub extern "C" fn sum_foo(&self) -> i32 {
        self.numbers.iter().sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_foo() {
        let foo = Foo::new_foo();
        assert_eq!(foo.sum_foo(), 6);
        Foo::free_foo(foo);
    }
}
