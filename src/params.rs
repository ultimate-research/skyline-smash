pub mod common;
pub use common::CommonParams;

#[repr(C)]
#[derive(Debug, Default)]
pub struct Params {
    pub vtable: u64,
}

pub trait Filepath {
    fn filepath() -> &'static str;
}

pub trait FromRawPtr<T> {
    unsafe fn from_u64_mut(param_obj: u64) -> Result<&'static mut T, &'static str>;
}

macro_rules! impl_from_raw_ptr {
    (
        $(
            $ty:ty
        )*
    ) => {
        $(
            impl FromRawPtr<$ty> for $ty {
                unsafe fn from_u64_mut(param_obj: u64) -> Result<&'static mut $ty, &'static str> {
                    if param_obj == 0 {
                        panic!("Supplied from_u64_mut with a nullptr")
                    } else {
                        Ok(&mut *(param_obj as *mut $ty))
                    }
                }
            }
        )*
    };
}

impl_from_raw_ptr!(Params CommonParams);
