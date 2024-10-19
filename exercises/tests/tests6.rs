// tests6.rs
//
// In this example we take a shallow dive into the Rust standard library's
// unsafe functions. Fix all the question marks and todos to make the test
// pass.
//
// Execute `rustlings hint tests6` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
///
/// The `ptr` must contain an owned box of `Foo`.
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    // SAFETY: The `ptr` contains an owned box of `Foo` by contract. We
    // simply reconstruct the box from that pointer.
    unsafe {  Box::from_raw(ptr) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let data = Box::new(Foo { a: 1, b: None });

        // SAFETY: We pass an owned box of `Foo`.
        let ptr = Box::into_raw(data);

        let ret = unsafe { raw_pointer_to_box(ptr) };

        // 由于我们已经重建了 Box，我们需要从 `ret` 读取字段
        let ptr_1 = &ret.a as *const u128;
        let ptr_2 = &ret.a as *const u128;

        assert!(ptr_1 == ptr_2);
        assert!(ret.b.is_none());
    }
}