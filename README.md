# A set of proc macros for generating a C API automatically

## Usage

```rust
use dodge::wrapper;

#[wrapper]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Expand to
// fn add(a: i32, b: i32) -> i32 { a + b }
// #[no_mangle]
// pub extern "C" fn rust_add(a: i32, b: i32) -> i32 { add(a, b) }


fn main() {
    add(1, 2);
}


```


# TODOs
- [x] function
- [ ] method
- [ ] class
- [ ] list all public facing struct, impl traits of a trait etc and generate wrapper around them