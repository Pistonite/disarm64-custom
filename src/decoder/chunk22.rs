#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
impl InsnOpcode for FMUL_Sd_Sn_Em {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMULX_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmulx",
        aliases: &[],
        opcode: 0x650a8000,
        mask: 0xff3fe000,
        class: InsnClass::SVE_SIZE_HSD,
        feature_set: InsnFeatureSet::SVE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SVE_Zd,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Pg3,
                class: InsnOperandClass::PRED_REG,
                qualifiers: &[
                    InsnOperandQualifier::P_M,
                    InsnOperandQualifier::P_M,
                    InsnOperandQualifier::P_M,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Pg3,
                    lsb: 10,
                    width: 3,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Zd,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Zm_5,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zm_5,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmulx,
            operation: Operation::SVE_SIZE_HSD(SVE_SIZE_HSD::FMULX_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(
                FMULX_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FMULX_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMULX_Vd_Vn_Vm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmulx",
        aliases: &[],
        opcode: 0xe401c00,
        mask: 0xbfe0fc00,
        class: InsnClass::ASIMDSAME,
        feature_set: InsnFeatureSet::SIMD_F16,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_4H, InsnOperandQualifier::V_8H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Vn,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_4H, InsnOperandQualifier::V_8H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Vm,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_4H, InsnOperandQualifier::V_8H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rm,
                    lsb: 16,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmulx,
            operation: Operation::ASIMDSAME(ASIMDSAME::FMULX_Vd_Vn_Vm(FMULX_Vd_Vn_Vm::from(bits))),
        }
    }
}
impl InsnOpcode for FMULX_Vd_Vn_Vm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMULX_Sd_Sn_Sm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmulx",
        aliases: &[],
        opcode: 0x5e401c00,
        mask: 0xffe0fc00,
        class: InsnClass::ASISDSAME,
        feature_set: InsnFeatureSet::SIMD_F16,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Sd,
                class: InsnOperandClass::SISD_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Sn,
                class: InsnOperandClass::SISD_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Sm,
                class: InsnOperandClass::SISD_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rm,
                    lsb: 16,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMD_SCALAR_SIZE.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmulx,
            operation: Operation::ASISDSAME(ASISDSAME::FMULX_Sd_Sn_Sm(FMULX_Sd_Sn_Sm::from(bits))),
        }
    }
}
impl InsnOpcode for FMULX_Sd_Sn_Sm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMULX_Vd_V_2S_Vn_V_2S_Vm_V_2S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmulx",
        aliases: &[],
        opcode: 0xe20dc00,
        mask: 0xbfa0fc00,
        class: InsnClass::ASIMDSAME,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Vn,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Vm,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rm,
                    lsb: 16,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmulx,
            operation: Operation::ASIMDSAME(ASIMDSAME::FMULX_Vd_V_2S_Vn_V_2S_Vm_V_2S(
                FMULX_Vd_V_2S_Vn_V_2S_Vm_V_2S::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FMULX_Vd_V_2S_Vn_V_2S_Vm_V_2S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMULX_Sd_S_S_Sn_S_S_Sm_S_S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmulx",
        aliases: &[],
        opcode: 0x5e20dc00,
        mask: 0xffa0fc00,
        class: InsnClass::ASISDSAME,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Sd,
                class: InsnOperandClass::SISD_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Sn,
                class: InsnOperandClass::SISD_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Sm,
                class: InsnOperandClass::SISD_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rm,
                    lsb: 16,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMD_SCALAR_SIZE.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmulx,
            operation: Operation::ASISDSAME(ASISDSAME::FMULX_Sd_S_S_Sn_S_S_Sm_S_S(
                FMULX_Sd_S_S_Sn_S_S_Sm_S_S::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FMULX_Sd_S_S_Sn_S_S_Sm_S_S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMULX_Vd_Vn_Em16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmulx",
        aliases: &[],
        opcode: 0x2f009000,
        mask: 0xbfc0f400,
        class: InsnClass::ASIMDELEM,
        feature_set: InsnFeatureSet::SIMD_F16,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_4H, InsnOperandQualifier::V_8H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Vn,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_4H, InsnOperandQualifier::V_8H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Em16,
                class: InsnOperandClass::SIMD_ELEMENT,
                qualifiers: &[InsnOperandQualifier::S_H, InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rm,
                    lsb: 16,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmulx,
            operation: Operation::ASIMDELEM(ASIMDELEM::FMULX_Vd_Vn_Em16(FMULX_Vd_Vn_Em16::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for FMULX_Vd_Vn_Em16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMULX_Vd_Vn_Em {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmulx",
        aliases: &[],
        opcode: 0x2f809000,
        mask: 0xbf80f400,
        class: InsnClass::ASIMDELEM,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Vn,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[
                    InsnOperandQualifier::V_2S,
                    InsnOperandQualifier::V_4S,
                    InsnOperandQualifier::V_2D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Em,
                class: InsnOperandClass::SIMD_ELEMENT,
                qualifiers: &[
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rm,
                    lsb: 16,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmulx,
            operation: Operation::ASIMDELEM(ASIMDELEM::FMULX_Vd_Vn_Em(FMULX_Vd_Vn_Em::from(bits))),
        }
    }
}
impl InsnOpcode for FMULX_Vd_Vn_Em {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMULX_Sd_Sn_Em16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmulx",
        aliases: &[],
        opcode: 0x7f009000,
        mask: 0xffc0f400,
        class: InsnClass::ASISDELEM,
        feature_set: InsnFeatureSet::SIMD_F16,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Sd,
                class: InsnOperandClass::SISD_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Sn,
                class: InsnOperandClass::SISD_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Em16,
                class: InsnOperandClass::SIMD_ELEMENT,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rm,
                    lsb: 16,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMD_SCALAR_SIZE.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmulx,
            operation: Operation::ASISDELEM(ASISDELEM::FMULX_Sd_Sn_Em16(FMULX_Sd_Sn_Em16::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for FMULX_Sd_Sn_Em16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FMULX_Sd_Sn_Em {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fmulx",
        aliases: &[],
        opcode: 0x7f809000,
        mask: 0xff80f400,
        class: InsnClass::ASISDELEM,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Sd,
                class: InsnOperandClass::SISD_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Sn,
                class: InsnOperandClass::SISD_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Em,
                class: InsnOperandClass::SIMD_ELEMENT,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rm,
                    lsb: 16,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMD_SCALAR_SIZE.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fmulx,
            operation: Operation::ASISDELEM(ASISDELEM::FMULX_Sd_Sn_Em(FMULX_Sd_Sn_Em::from(bits))),
        }
    }
}
impl InsnOpcode for FMULX_Sd_Sn_Em {
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
impl InsnOpcode for LDP_Ft_Ft2_ADDR_SIMM7 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldp",
        aliases: &[],
        opcode: 0x2cc00000,
        mask: 0x3ec00000,
        class: InsnClass::LDSTPAIR_INDEXED,
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
            operation: Operation::LDSTPAIR_INDEXED(
                LDSTPAIR_INDEXED::LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S(
                    LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDPSW_Rt_Rt2_ADDR_SIMM7 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldpsw",
        aliases: &[],
        opcode: 0x69400000,
        mask: 0xffc00000,
        class: InsnClass::LDSTPAIR_OFF,
        feature_set: InsnFeatureSet::V8,
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
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S],
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
            mnemonic: Mnemonic::r#ldpsw,
            operation: Operation::LDSTPAIR_OFF(LDSTPAIR_OFF::LDPSW_Rt_Rt2_ADDR_SIMM7(
                LDPSW_Rt_Rt2_ADDR_SIMM7::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDPSW_Rt_Rt2_ADDR_SIMM7 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDPSW_Rt_X_Rt2_X_ADDR_SIMM7_S_S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldpsw",
        aliases: &[],
        opcode: 0x68c00000,
        mask: 0xfec00000,
        class: InsnClass::LDSTPAIR_INDEXED,
        feature_set: InsnFeatureSet::V8,
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
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S],
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
            mnemonic: Mnemonic::r#ldpsw,
            operation: Operation::LDSTPAIR_INDEXED(
                LDSTPAIR_INDEXED::LDPSW_Rt_X_Rt2_X_ADDR_SIMM7_S_S(
                    LDPSW_Rt_X_Rt2_X_ADDR_SIMM7_S_S::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for LDPSW_Rt_X_Rt2_X_ADDR_SIMM7_S_S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDR_SME_ZA_array_off4_SME_ADDR_RI_U4xVL {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0xe1000000,
        mask: 0xffff9c10,
        class: InsnClass::SME_LDR,
        feature_set: InsnFeatureSet::SME,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_off4,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_Rv,
                        lsb: 13,
                        width: 2,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm4_0,
                        lsb: 0,
                        width: 4,
                    },
                ],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_ADDR_RI_U4xVL,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm4_0,
                        lsb: 0,
                        width: 4,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldr,
            operation: Operation::SME_LDR(SME_LDR::LDR_SME_ZA_array_off4_SME_ADDR_RI_U4xVL(
                LDR_SME_ZA_array_off4_SME_ADDR_RI_U4xVL::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDR_SME_ZA_array_off4_SME_ADDR_RI_U4xVL {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDR_SME_ZT0_SIMD_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0xe11f8000,
        mask: 0xfffffc1f,
        class: InsnClass::SME_MISC,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZT0,
                class: InsnOperandClass::SYSTEM,
                qualifiers: &[],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SIMD_ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldr,
            operation: Operation::SME_MISC(SME_MISC::LDR_SME_ZT0_SIMD_ADDR_SIMPLE(
                LDR_SME_ZT0_SIMD_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDR_SME_ZT0_SIMD_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDR_Rt_ADDR_PCREL19 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0x18000000,
        mask: 0xbf000000,
        class: InsnClass::LOADLIT,
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
                kind: InsnOperandKind::ADDR_PCREL19,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::imm19,
                    lsb: 5,
                    width: 19,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldr,
            operation: Operation::LOADLIT(LOADLIT::LDR_Rt_ADDR_PCREL19(LDR_Rt_ADDR_PCREL19::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for LDR_Rt_ADDR_PCREL19 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDR_Rt_ADDR_REGOFF {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0xb8600800,
        mask: 0xbfe00c00,
        class: InsnClass::LDST_REGOFF,
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
                kind: InsnOperandKind::ADDR_REGOFF,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldr,
            operation: Operation::LDST_REGOFF(LDST_REGOFF::LDR_Rt_ADDR_REGOFF(
                LDR_Rt_ADDR_REGOFF::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDR_Rt_ADDR_REGOFF {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDR_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0xb8400400,
        mask: 0xbfe00400,
        class: InsnClass::LDST_IMM9,
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
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
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
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldr,
            operation: Operation::LDST_IMM9(LDST_IMM9::LDR_Rt_ADDR_SIMM9(LDR_Rt_ADDR_SIMM9::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for LDR_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDR_Rt_ADDR_UIMM12 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0xb9400000,
        mask: 0xbfc00000,
        class: InsnClass::LDST_POS,
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
                kind: InsnOperandKind::ADDR_UIMM12,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm12,
                        lsb: 10,
                        width: 12,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldr,
            operation: Operation::LDST_POS(LDST_POS::LDR_Rt_ADDR_UIMM12(LDR_Rt_ADDR_UIMM12::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for LDR_Rt_ADDR_UIMM12 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDR_SVE_Pt_SVE_ADDR_RI_S9xVL {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0x85800000,
        mask: 0xffc0e010,
        class: InsnClass::SVE_MISC,
        feature_set: InsnFeatureSet::SVE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SVE_Pt,
                class: InsnOperandClass::PRED_REG,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Pt,
                    lsb: 0,
                    width: 4,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_ADDR_RI_S9xVL,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldr,
            operation: Operation::SVE_MISC(SVE_MISC::LDR_SVE_Pt_SVE_ADDR_RI_S9xVL(
                LDR_SVE_Pt_SVE_ADDR_RI_S9xVL::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDR_SVE_Pt_SVE_ADDR_RI_S9xVL {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDR_SVE_PNt_SVE_ADDR_RI_S9xVL {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0x85800000,
        mask: 0xffc0e010,
        class: InsnClass::SVE_MISC,
        feature_set: InsnFeatureSet::SVE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SVE_PNt,
                class: InsnOperandClass::PRED_REG,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Pt,
                    lsb: 0,
                    width: 4,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_ADDR_RI_S9xVL,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldr,
            operation: Operation::SVE_MISC(SVE_MISC::LDR_SVE_PNt_SVE_ADDR_RI_S9xVL(
                LDR_SVE_PNt_SVE_ADDR_RI_S9xVL::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDR_SVE_PNt_SVE_ADDR_RI_S9xVL {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDR_SVE_Zt_SVE_ADDR_RI_S9xVL {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0x85804000,
        mask: 0xffc0e000,
        class: InsnClass::SVE_MISC,
        feature_set: InsnFeatureSet::SVE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SVE_Zt,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_ADDR_RI_S9xVL,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldr,
            operation: Operation::SVE_MISC(SVE_MISC::LDR_SVE_Zt_SVE_ADDR_RI_S9xVL(
                LDR_SVE_Zt_SVE_ADDR_RI_S9xVL::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDR_SVE_Zt_SVE_ADDR_RI_S9xVL {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDR_Ft_ADDR_PCREL19 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0x1c000000,
        mask: 0x3f000000,
        class: InsnClass::LOADLIT,
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
                kind: InsnOperandKind::ADDR_PCREL19,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::imm19,
                    lsb: 5,
                    width: 19,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldr,
            operation: Operation::LOADLIT(LOADLIT::LDR_Ft_ADDR_PCREL19(LDR_Ft_ADDR_PCREL19::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for LDR_Ft_ADDR_PCREL19 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDR_Ft_ADDR_REGOFF {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0x3c600800,
        mask: 0x3f600c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
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
                kind: InsnOperandKind::ADDR_REGOFF,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldr,
            operation: Operation::LDST_REGOFF(LDST_REGOFF::LDR_Ft_ADDR_REGOFF(
                LDR_Ft_ADDR_REGOFF::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDR_Ft_ADDR_REGOFF {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDR_Ft_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0x3c400400,
        mask: 0x3f600400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
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
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
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
            mnemonic: Mnemonic::r#ldr,
            operation: Operation::LDST_IMM9(LDST_IMM9::LDR_Ft_ADDR_SIMM9(LDR_Ft_ADDR_SIMM9::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for LDR_Ft_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDR_Ft_ADDR_UIMM12 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0x3d400000,
        mask: 0x3f400000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
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
                kind: InsnOperandKind::ADDR_UIMM12,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm12,
                        lsb: 10,
                        width: 12,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldr,
            operation: Operation::LDST_POS(LDST_POS::LDR_Ft_ADDR_UIMM12(LDR_Ft_ADDR_UIMM12::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for LDR_Ft_ADDR_UIMM12 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRAA_Rt_ADDR_SIMM10 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldraa",
        aliases: &[],
        opcode: 0xf8200400,
        mask: 0xffa00400,
        class: InsnClass::LDST_IMM10,
        feature_set: InsnFeatureSet::PAC,
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
                kind: InsnOperandKind::ADDR_SIMM10,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::S_imm10,
                        lsb: 22,
                        width: 1,
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
            mnemonic: Mnemonic::r#ldraa,
            operation: Operation::LDST_IMM10(LDST_IMM10::LDRAA_Rt_ADDR_SIMM10(
                LDRAA_Rt_ADDR_SIMM10::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRAA_Rt_ADDR_SIMM10 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRAB_Rt_ADDR_SIMM10 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrab",
        aliases: &[],
        opcode: 0xf8a00400,
        mask: 0xffa00400,
        class: InsnClass::LDST_IMM10,
        feature_set: InsnFeatureSet::PAC,
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
                kind: InsnOperandKind::ADDR_SIMM10,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::S_imm10,
                        lsb: 22,
                        width: 1,
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
            mnemonic: Mnemonic::r#ldrab,
            operation: Operation::LDST_IMM10(LDST_IMM10::LDRAB_Rt_ADDR_SIMM10(
                LDRAB_Rt_ADDR_SIMM10::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRAB_Rt_ADDR_SIMM10 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRB_Rt_ADDR_REGOFF {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrb",
        aliases: &[],
        opcode: 0x38600800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
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
                kind: InsnOperandKind::ADDR_REGOFF,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldrb,
            operation: Operation::LDST_REGOFF(LDST_REGOFF::LDRB_Rt_ADDR_REGOFF(
                LDRB_Rt_ADDR_REGOFF::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRB_Rt_ADDR_REGOFF {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRB_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrb",
        aliases: &[],
        opcode: 0x38400400,
        mask: 0xffe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
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
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B],
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
            mnemonic: Mnemonic::r#ldrb,
            operation: Operation::LDST_IMM9(LDST_IMM9::LDRB_Rt_ADDR_SIMM9(
                LDRB_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRB_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRB_Rt_ADDR_UIMM12 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrb",
        aliases: &[],
        opcode: 0x39400000,
        mask: 0xffc00000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
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
                kind: InsnOperandKind::ADDR_UIMM12,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm12,
                        lsb: 10,
                        width: 12,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldrb,
            operation: Operation::LDST_POS(LDST_POS::LDRB_Rt_ADDR_UIMM12(
                LDRB_Rt_ADDR_UIMM12::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRB_Rt_ADDR_UIMM12 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRH_Rt_ADDR_REGOFF {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrh",
        aliases: &[],
        opcode: 0x78600800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
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
                kind: InsnOperandKind::ADDR_REGOFF,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ldrh,
            operation: Operation::LDST_REGOFF(LDST_REGOFF::LDRH_Rt_ADDR_REGOFF(
                LDRH_Rt_ADDR_REGOFF::from(bits),
            )),
        }
    }
}
