// Auto-generated.
// The changes will be LOST.

#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use bitfield_struct::bitfield;
use disarm64_defn::defn::Insn;
use disarm64_defn::defn::InsnOpcode;
use disarm64_defn::defn::InsnOperand;
use disarm64_defn::BitfieldSpec;
use disarm64_defn::InsnBitField;
use disarm64_defn::InsnClass;
use disarm64_defn::InsnFeatureSet;
use disarm64_defn::InsnFlags;
use disarm64_defn::InsnOperandClass;
use disarm64_defn::InsnOperandKind;
use disarm64_defn::InsnOperandQualifier;
#[doc = r" Leaf nodes in the decision tree"]
pub(crate) struct Leaf {
    insn: &'static Insn,
    factory: fn(u32) -> Opcode,
}
#[doc = r" The decision tree node"]
pub(crate) enum Decode {
    #[doc = r" Branch in the decision tree"]
    Branch {
        mask: u32,
        next: [Option<u16>; 2],
    },
    Leaf(&'static [Leaf]),
}
#[doc = r" The decode table"]
pub(crate) type DecodeTable = &'static [Decode];
mod chunk0;
pub use chunk0::*;
mod chunk1;
pub use chunk1::*;
mod chunk2;
pub use chunk2::*;
mod chunk3;
pub use chunk3::*;
mod chunk4;
pub use chunk4::*;
mod chunk5;
pub use chunk5::*;
mod chunk6;
pub use chunk6::*;
mod chunk7;
pub use chunk7::*;
mod chunk8;
pub use chunk8::*;
mod chunk9;
pub use chunk9::*;
mod chunk10;
pub use chunk10::*;
mod chunk11;
pub use chunk11::*;
mod chunk12;
pub use chunk12::*;
mod chunk13;
pub use chunk13::*;
mod chunk14;
pub use chunk14::*;
mod chunk15;
pub use chunk15::*;
mod chunk16;
pub use chunk16::*;
mod chunk17;
pub use chunk17::*;
mod chunk18;
pub use chunk18::*;
mod chunk19;
pub use chunk19::*;
mod chunk20;
pub use chunk20::*;
mod chunk21;
pub use chunk21::*;
mod chunk22;
pub use chunk22::*;
mod chunk23;
pub use chunk23::*;
mod chunk24;
pub use chunk24::*;
mod chunk25;
pub use chunk25::*;
mod chunk26;
pub use chunk26::*;
mod chunk27;
pub use chunk27::*;
mod chunk28;
pub use chunk28::*;
mod chunk29;
pub use chunk29::*;
mod chunk30;
pub use chunk30::*;
mod chunk31;
pub use chunk31::*;
mod chunk32;
pub use chunk32::*;
mod chunk33;
pub use chunk33::*;
