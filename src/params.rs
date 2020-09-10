pub mod common;
pub mod fighter_param;

pub use common::CommonParams;
pub use fighter_param::FighterParams;

pub use core::convert::TryFrom;
use paste::paste;

#[repr(C)]
#[derive(Debug, Default)]
pub struct Params {
    pub vtable: u64,
}

#[derive(Debug)]
pub struct ParamsInfo<'a> {
    pub object_ptr: &'a u64,
    pub filepath_hash: &'a u64,
}

impl<'a> ParamsInfo<'a> {
    // A function only to be used by param_hook in order to construct ParamsInfos. Since fields may be
    /// added, this API is subject to change.
    #[cfg(feature = "params_internal")]
    pub fn new(object_ptr: &'a u64, filepath_hash: &'a u64) -> Self {
        Self {
            object_ptr,
            filepath_hash,
        }
    }

    pub fn is_type<T: Filepath>(&self) -> bool {
        crate::hash40(T::filepath()) == *self.filepath_hash
    }

    pub fn get<T: TryFrom<u64, Error = &'static str> + Filepath>(&self) -> Result<T, T::Error> {
        if self.is_type::<T>() {
            T::try_from(*self.object_ptr)
        } else {
            Err("Attempting to pull params object as invalid type!")
        }
    }
}

pub type Callback = fn(&ParamsInfo);

/// An error representing the NRO hook plugin not successfully being linked against
#[derive(Debug, Clone, Copy)]
pub struct ParamsHookPluginMissing;

#[allow(improper_ctypes)]
extern "C" {
    fn add_param_load_hook(callback: Callback);
}

/// A function to allow adding a hook for immediately after an NRO has been loaded.
///
/// **Note:** Requires the Param hook plugin. Will return an error otherwise.
pub fn add_hook(callback: Callback) -> Result<(), ParamsHookPluginMissing> {
    if (add_param_load_hook as *const ()).is_null() {
        Err(ParamsHookPluginMissing)
    } else {
        unsafe {
            add_param_load_hook(callback);
        }
        Ok(())
    }
}

pub trait Filepath {
    fn filepath() -> &'static str;
}

macro_rules! impl_static_mut_traits {
    (
        $(
            $ty:ty
        ),*
    ) => {
        $(
            paste! {
                pub type [<Static $ty>] = &'static mut $ty;

                impl core::convert::TryFrom<u64> for &'static mut $ty {
                    type Error = &'static str;

                    fn try_from(param_obj: u64) -> Result<Self, Self::Error> {
                        if param_obj == 0 {
                            Err("Supplied from_u64_mut with a nullptr")
                        } else {
                            unsafe { Ok(&mut *(param_obj as *mut $ty)) }
                        }
                    }
                }

                impl Filepath for &'static mut $ty {
                    fn filepath() -> &'static str {
                        $ty::filepath()
                    }
                }
            }
        )*
    };
}

impl_static_mut_traits!(CommonParams, FighterParams);
