pub mod types;
pub mod instruction;
pub mod reader;
pub mod interpreter;
pub mod utils;
pub mod frame;

#[macro_export]
macro_rules! transmute {
    ($e:expr) => {
        unsafe {
            ::std::mem::transmute($e)
        }
    };
}

