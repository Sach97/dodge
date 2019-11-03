use dodge::wrapper;

// #[no_mangle]
// pub extern fn rust_add(a: i32, b: i32) -> i32 {
//     a + b
// }

#[wrapper]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    add(1, 2);
}
