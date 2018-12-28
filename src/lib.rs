pub mod types;
pub mod instruction;
pub mod reader;

#[macro_export]
macro_rules! transmute {
    ($e:expr) => {
        unsafe {
            ::std::mem::transmute($e)
        }
    };
}