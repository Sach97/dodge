use dodge::wrapper;

//expected
// fn add(a: i32, b: i32) -> i32 { a + b }
// #[no_mangle]
// pub extern "C" fn rust_add(a: i32, b: i32) -> i32 { add(a, b) }

#[wrapper]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    add(1, 2);
}
