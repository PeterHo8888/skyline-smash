pub mod common;
pub use common::CommonParams;
pub use core::convert::TryFrom;
use paste::paste;

#[repr(C)]
#[derive(Debug, Default)]
pub struct Params {
    pub vtable: u64,
}

pub trait Filepath {
    fn filepath() -> &'static str;
}

macro_rules! impl_try_from_static_mut {
    (
        $(
            $ty:ty
        )*
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
            }
        )*
    };
}

impl_try_from_static_mut!(Params CommonParams);
