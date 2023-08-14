mod Foo {
    pub fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[no_mangle]
pub extern "C" fn my_demo_function_alias(a: u32) -> u32 {
    Foo::my_demo_function(a)
}
