mod error;
pub use error::ItmErrCode;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("itm-sys/include/itm-wrapper.h");

        fn poc();
    }
}

pub fn poc() {
    ffi::poc();
}

#[cfg(test)]
mod tests {
    use super::poc;

    #[test]
    fn test_poc() {
        poc();
    }
}
