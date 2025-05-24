#![no_std]

#[rustfmt::skip]
pub mod decoder;
mod disarm64;
pub use disarm64::*;

pub mod arm64 {
    #[doc(inline)]
    pub use disarm64_defn::defn::*;
    #[doc(inline)]
    pub use disarm64_defn::*;
}
