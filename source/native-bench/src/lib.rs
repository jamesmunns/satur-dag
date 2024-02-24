

extern "C" {
    pub fn cbrrr_parse_object(
        buf: *const u8,
        len: usize,
        value: *mut *mut (),
        cid_ctor: *mut (),
        atjson_mode: isize,
    ) -> isize;

    pub fn PyBytes_FromStringAndSize(
        src: *const u8,
        len: usize,
    ) -> *mut ();
}

#[inline]
pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}
