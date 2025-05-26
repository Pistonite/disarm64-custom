#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
impl InsnOpcode for LD1_LEt_SIMD_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LD1_LVt_SIMD_ADDR_POST {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ld1",
        aliases: &[],
        opcode: 0xcc00000,
        mask: 0xbfe00000,
        class: InsnClass::ASISDLSEP,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LVt,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_1D,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_POST,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_1.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_3.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_5.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits()
                | InsnFlags::HAS_SIZEQ_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ld1,
            operation: Operation::ASISDLSEP(ASISDLSEP::LD1_LVt_SIMD_ADDR_POST(
                LD1_LVt_SIMD_ADDR_POST::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LD1_LVt_SIMD_ADDR_POST {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LD1_LEt_SIMD_ADDR_POST {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ld1",
        aliases: &[],
        opcode: 0xdc00000,
        mask: 0xbfe02000,
        class: InsnClass::ASISDLSOP,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LEt,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_POST,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_1.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_3.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_5.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ld1,
            operation: Operation::ASISDLSOP(ASISDLSOP::LD1_LEt_SIMD_ADDR_POST(
                LD1_LEt_SIMD_ADDR_POST::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LD1_LEt_SIMD_ADDR_POST {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LD1R_LVt_AL_SIMD_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ld1r",
        aliases: &[],
        opcode: 0xd40c000,
        mask: 0xbffff000,
        class: InsnClass::ASISDLSO,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LVt_AL,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_1D,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_1.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_3.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_5.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits()
                | InsnFlags::HAS_SIZEQ_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ld1r,
            operation: Operation::ASISDLSO(ASISDLSO::LD1R_LVt_AL_SIMD_ADDR_SIMPLE(
                LD1R_LVt_AL_SIMD_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LD1R_LVt_AL_SIMD_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LD1R_LVt_AL_SIMD_ADDR_POST {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ld1r",
        aliases: &[],
        opcode: 0xdc0c000,
        mask: 0xbfe0f000,
        class: InsnClass::ASISDLSOP,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LVt_AL,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_1D,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_POST,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_1.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_3.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_5.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits()
                | InsnFlags::HAS_SIZEQ_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ld1r,
            operation: Operation::ASISDLSOP(ASISDLSOP::LD1R_LVt_AL_SIMD_ADDR_POST(
                LD1R_LVt_AL_SIMD_ADDR_POST::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LD1R_LVt_AL_SIMD_ADDR_POST {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LD2_LVt_SIMD_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ld2",
        aliases: &[],
        opcode: 0xc400000,
        mask: 0xbfff0000,
        class: InsnClass::ASISDLSE,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LVt,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_2.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_3.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_6.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits()
                | InsnFlags::HAS_SIZEQ_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ld2,
            operation: Operation::ASISDLSE(ASISDLSE::LD2_LVt_SIMD_ADDR_SIMPLE(
                LD2_LVt_SIMD_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LD2_LVt_SIMD_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LD2_LEt_SIMD_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ld2",
        aliases: &[],
        opcode: 0xd600000,
        mask: 0xbfff2000,
        class: InsnClass::ASISDLSO,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LEt,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_2.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_3.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_6.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ld2,
            operation: Operation::ASISDLSO(ASISDLSO::LD2_LEt_SIMD_ADDR_SIMPLE(
                LD2_LEt_SIMD_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LD2_LEt_SIMD_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LD2_LVt_SIMD_ADDR_POST {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ld2",
        aliases: &[],
        opcode: 0xcc00000,
        mask: 0xbfe00000,
        class: InsnClass::ASISDLSEP,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LVt,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_POST,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_2.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_3.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_6.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits()
                | InsnFlags::HAS_SIZEQ_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ld2,
            operation: Operation::ASISDLSEP(ASISDLSEP::LD2_LVt_SIMD_ADDR_POST(
                LD2_LVt_SIMD_ADDR_POST::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LD2_LVt_SIMD_ADDR_POST {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LD2_LEt_SIMD_ADDR_POST {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ld2",
        aliases: &[],
        opcode: 0xde00000,
        mask: 0xbfe02000,
        class: InsnClass::ASISDLSOP,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LEt,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_POST,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_2.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_3.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_6.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ld2,
            operation: Operation::ASISDLSOP(ASISDLSOP::LD2_LEt_SIMD_ADDR_POST(
                LD2_LEt_SIMD_ADDR_POST::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LD2_LEt_SIMD_ADDR_POST {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LD2R_LVt_AL_SIMD_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ld2r",
        aliases: &[],
        opcode: 0xd60c000,
        mask: 0xbffff000,
        class: InsnClass::ASISDLSO,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LVt_AL,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_1D,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_2.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_3.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_6.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits()
                | InsnFlags::HAS_SIZEQ_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ld2r,
            operation: Operation::ASISDLSO(ASISDLSO::LD2R_LVt_AL_SIMD_ADDR_SIMPLE(
                LD2R_LVt_AL_SIMD_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LD2R_LVt_AL_SIMD_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LD2R_LVt_AL_SIMD_ADDR_POST {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ld2r",
        aliases: &[],
        opcode: 0xde0c000,
        mask: 0xbfe0f000,
        class: InsnClass::ASISDLSOP,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LVt_AL,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_1D,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_POST,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_2.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_3.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_6.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits()
                | InsnFlags::HAS_SIZEQ_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ld2r,
            operation: Operation::ASISDLSOP(ASISDLSOP::LD2R_LVt_AL_SIMD_ADDR_POST(
                LD2R_LVt_AL_SIMD_ADDR_POST::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LD2R_LVt_AL_SIMD_ADDR_POST {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LD3_LVt_SIMD_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ld3",
        aliases: &[],
        opcode: 0xc400000,
        mask: 0xbfff0000,
        class: InsnClass::ASISDLSE,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LVt,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_1.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_2.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_3.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_5.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_6.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits()
                | InsnFlags::HAS_SIZEQ_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ld3,
            operation: Operation::ASISDLSE(ASISDLSE::LD3_LVt_SIMD_ADDR_SIMPLE(
                LD3_LVt_SIMD_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LD3_LVt_SIMD_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LD3_LEt_SIMD_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ld3",
        aliases: &[],
        opcode: 0xd402000,
        mask: 0xbfff2000,
        class: InsnClass::ASISDLSO,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LEt,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_1.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_2.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_3.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_5.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_6.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ld3,
            operation: Operation::ASISDLSO(ASISDLSO::LD3_LEt_SIMD_ADDR_SIMPLE(
                LD3_LEt_SIMD_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LD3_LEt_SIMD_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LD3_LVt_SIMD_ADDR_POST {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ld3",
        aliases: &[],
        opcode: 0xcc00000,
        mask: 0xbfe00000,
        class: InsnClass::ASISDLSEP,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LVt,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_POST,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_1.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_2.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_3.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_5.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_6.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits()
                | InsnFlags::HAS_SIZEQ_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ld3,
            operation: Operation::ASISDLSEP(ASISDLSEP::LD3_LVt_SIMD_ADDR_POST(
                LD3_LVt_SIMD_ADDR_POST::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LD3_LVt_SIMD_ADDR_POST {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LD3_LEt_SIMD_ADDR_POST {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ld3",
        aliases: &[],
        opcode: 0xdc02000,
        mask: 0xbfe02000,
        class: InsnClass::ASISDLSOP,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LEt,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_POST,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_1.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_2.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_3.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_5.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_6.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ld3,
            operation: Operation::ASISDLSOP(ASISDLSOP::LD3_LEt_SIMD_ADDR_POST(
                LD3_LEt_SIMD_ADDR_POST::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LD3_LEt_SIMD_ADDR_POST {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LD3R_LVt_AL_SIMD_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ld3r",
        aliases: &[],
        opcode: 0xd40e000,
        mask: 0xbffff000,
        class: InsnClass::ASISDLSO,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LVt_AL,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_1D,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_1.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_2.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_3.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_5.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_6.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits()
                | InsnFlags::HAS_SIZEQ_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ld3r,
            operation: Operation::ASISDLSO(ASISDLSO::LD3R_LVt_AL_SIMD_ADDR_SIMPLE(
                LD3R_LVt_AL_SIMD_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LD3R_LVt_AL_SIMD_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LD3R_LVt_AL_SIMD_ADDR_POST {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ld3r",
        aliases: &[],
        opcode: 0xdc0e000,
        mask: 0xbfe0f000,
        class: InsnClass::ASISDLSOP,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LVt_AL,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_1D,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_POST,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_1.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_2.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_3.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_5.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_6.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits()
                | InsnFlags::HAS_SIZEQ_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ld3r,
            operation: Operation::ASISDLSOP(ASISDLSOP::LD3R_LVt_AL_SIMD_ADDR_POST(
                LD3R_LVt_AL_SIMD_ADDR_POST::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LD3R_LVt_AL_SIMD_ADDR_POST {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LD4_LVt_SIMD_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ld4",
        aliases: &[],
        opcode: 0xc400000,
        mask: 0xbfff0000,
        class: InsnClass::ASISDLSE,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LVt,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_4.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_5.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_6.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits()
                | InsnFlags::HAS_SIZEQ_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ld4,
            operation: Operation::ASISDLSE(ASISDLSE::LD4_LVt_SIMD_ADDR_SIMPLE(
                LD4_LVt_SIMD_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LD4_LVt_SIMD_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LD4_LEt_SIMD_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ld4",
        aliases: &[],
        opcode: 0xd602000,
        mask: 0xbfff2000,
        class: InsnClass::ASISDLSO,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LEt,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_4.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_5.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_6.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ld4,
            operation: Operation::ASISDLSO(ASISDLSO::LD4_LEt_SIMD_ADDR_SIMPLE(
                LD4_LEt_SIMD_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LD4_LEt_SIMD_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LD4_LVt_SIMD_ADDR_POST {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ld4",
        aliases: &[],
        opcode: 0xcc00000,
        mask: 0xbfe00000,
        class: InsnClass::ASISDLSEP,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LVt,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_POST,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_4.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_5.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_6.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits()
                | InsnFlags::HAS_SIZEQ_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ld4,
            operation: Operation::ASISDLSEP(ASISDLSEP::LD4_LVt_SIMD_ADDR_POST(
                LD4_LVt_SIMD_ADDR_POST::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LD4_LVt_SIMD_ADDR_POST {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LD4_LEt_SIMD_ADDR_POST {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ld4",
        aliases: &[],
        opcode: 0xde02000,
        mask: 0xbfe02000,
        class: InsnClass::ASISDLSOP,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LEt,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_POST,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_4.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_5.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_6.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ld4,
            operation: Operation::ASISDLSOP(ASISDLSOP::LD4_LEt_SIMD_ADDR_POST(
                LD4_LEt_SIMD_ADDR_POST::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LD4_LEt_SIMD_ADDR_POST {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LD4R_LVt_AL_SIMD_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ld4r",
        aliases: &[],
        opcode: 0xd60e000,
        mask: 0xbffff000,
        class: InsnClass::ASISDLSO,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LVt_AL,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_1D,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_4.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_5.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_6.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits()
                | InsnFlags::HAS_SIZEQ_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ld4r,
            operation: Operation::ASISDLSO(ASISDLSO::LD4R_LVt_AL_SIMD_ADDR_SIMPLE(
                LD4R_LVt_AL_SIMD_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LD4R_LVt_AL_SIMD_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LD4R_LVt_AL_SIMD_ADDR_POST {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ld4r",
        aliases: &[],
        opcode: 0xde0e000,
        mask: 0xbfe0f000,
        class: InsnClass::ASISDLSOP,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LVt_AL,
                class: InsnOperandClass::SIMD_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_1D,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_POST,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_4.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_5.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_6.bits()
                | InsnFlags::HAS_OPCODE_DEPENDENT_FIELD_7.bits()
                | InsnFlags::HAS_SIZEQ_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ld4r,
            operation: Operation::ASISDLSOP(ASISDLSOP::LD4R_LVt_AL_SIMD_ADDR_POST(
                LD4R_LVt_AL_SIMD_ADDR_POST::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LD4R_LVt_AL_SIMD_ADDR_POST {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDAPUR_Rt_ADDR_OFFSET {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldapur",
        aliases: &[],
        opcode: 0x99400000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_OFFSET,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldapur,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDAPUR_Rt_ADDR_OFFSET(
                LDAPUR_Rt_ADDR_OFFSET::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDAPUR_Rt_ADDR_OFFSET {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDAPUR_Rt_X_ADDR_OFFSET {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldapur",
        aliases: &[],
        opcode: 0xd9400000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_OFFSET,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldapur,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDAPUR_Rt_X_ADDR_OFFSET(
                LDAPUR_Rt_X_ADDR_OFFSET::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDAPUR_Rt_X_ADDR_OFFSET {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDAPURB_Rt_ADDR_OFFSET {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldapurb",
        aliases: &[],
        opcode: 0x19400000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_OFFSET,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldapurb,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDAPURB_Rt_ADDR_OFFSET(
                LDAPURB_Rt_ADDR_OFFSET::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDAPURB_Rt_ADDR_OFFSET {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDAPURH_Rt_ADDR_OFFSET {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldapurh",
        aliases: &[],
        opcode: 0x59400000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_OFFSET,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldapurh,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDAPURH_Rt_ADDR_OFFSET(
                LDAPURH_Rt_ADDR_OFFSET::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDAPURH_Rt_ADDR_OFFSET {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDAPURSB_Rt_ADDR_OFFSET {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldapursb",
        aliases: &[],
        opcode: 0x19800000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_OFFSET,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldapursb,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDAPURSB_Rt_ADDR_OFFSET(
                LDAPURSB_Rt_ADDR_OFFSET::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDAPURSB_Rt_ADDR_OFFSET {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDAPURSB_Rt_W_ADDR_OFFSET {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldapursb",
        aliases: &[],
        opcode: 0x19c00000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_OFFSET,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldapursb,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDAPURSB_Rt_W_ADDR_OFFSET(
                LDAPURSB_Rt_W_ADDR_OFFSET::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDAPURSB_Rt_W_ADDR_OFFSET {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDAPURSH_Rt_ADDR_OFFSET {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldapursh",
        aliases: &[],
        opcode: 0x59800000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_OFFSET,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldapursh,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDAPURSH_Rt_ADDR_OFFSET(
                LDAPURSH_Rt_ADDR_OFFSET::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDAPURSH_Rt_ADDR_OFFSET {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDAPURSH_Rt_W_ADDR_OFFSET {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldapursh",
        aliases: &[],
        opcode: 0x59c00000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_OFFSET,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldapursh,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDAPURSH_Rt_W_ADDR_OFFSET(
                LDAPURSH_Rt_W_ADDR_OFFSET::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDAPURSH_Rt_W_ADDR_OFFSET {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDAPURSW_Rt_ADDR_OFFSET {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldapursw",
        aliases: &[],
        opcode: 0x99800000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_OFFSET,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldapursw,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDAPURSW_Rt_ADDR_OFFSET(
                LDAPURSW_Rt_ADDR_OFFSET::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDAPURSW_Rt_ADDR_OFFSET {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDG_Rt_ADDR_SIMM13 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldg",
        aliases: &[],
        opcode: 0xd9600000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM13,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::imm_tag],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldg,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDG_Rt_ADDR_SIMM13(
                LDG_Rt_ADDR_SIMM13::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDG_Rt_ADDR_SIMM13 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDP_Rt_Rt2_ADDR_SIMM7 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldp",
        aliases: &[],
        opcode: 0x29400000,
        mask: 0x7fc00000,
        class: InsnClass::LDSTPAIR_OFF,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm7,
                        lsb: 15,
                        width: 7,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index2,
                        lsb: 24,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SF_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldp,
            operation: Operation::LDSTPAIR_OFF(LDSTPAIR_OFF::LDP_Rt_Rt2_ADDR_SIMM7(
                LDP_Rt_Rt2_ADDR_SIMM7::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDP_Rt_Rt2_ADDR_SIMM7 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldp",
        aliases: &[],
        opcode: 0x28c00000,
        mask: 0x7ec00000,
        class: InsnClass::LDSTPAIR_INDEXED,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm7,
                        lsb: 15,
                        width: 7,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index2,
                        lsb: 24,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SF_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldp,
            operation: Operation::LDSTPAIR_INDEXED(
                LDSTPAIR_INDEXED::LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S(
                    LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDP_Ft_Ft2_ADDR_SIMM7 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldp",
        aliases: &[],
        opcode: 0x2d400000,
        mask: 0x3fc00000,
        class: InsnClass::LDSTPAIR_OFF,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Ft2,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm7,
                        lsb: 15,
                        width: 7,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index2,
                        lsb: 24,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldp,
            operation: Operation::LDSTPAIR_OFF(LDSTPAIR_OFF::LDP_Ft_Ft2_ADDR_SIMM7(
                LDP_Ft_Ft2_ADDR_SIMM7::from(bits),
            )),
        }
    }
}
