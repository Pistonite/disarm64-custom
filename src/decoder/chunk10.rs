#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
impl InsnOpcode for FCCMPE_Fn_S_S_Fm_S_S_NZCV_COND {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMEQ_Vd_Vn_Vm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmeq",
        aliases: &[],
        opcode: 0xe402400,
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
            mnemonic: Mnemonic::r#fcmeq,
            operation: Operation::ASIMDSAME(ASIMDSAME::FCMEQ_Vd_Vn_Vm(FCMEQ_Vd_Vn_Vm::from(bits))),
        }
    }
}
impl InsnOpcode for FCMEQ_Vd_Vn_Vm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMEQ_Sd_Sn_Sm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmeq",
        aliases: &[],
        opcode: 0x5e402400,
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
            mnemonic: Mnemonic::r#fcmeq,
            operation: Operation::ASISDSAME(ASISDSAME::FCMEQ_Sd_Sn_Sm(FCMEQ_Sd_Sn_Sm::from(bits))),
        }
    }
}
impl InsnOpcode for FCMEQ_Sd_Sn_Sm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMEQ_Vd_Vn_FPIMM0 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmeq",
        aliases: &[],
        opcode: 0xea0d800,
        mask: 0xbfbffc00,
        class: InsnClass::ASIMDMISC,
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
                kind: InsnOperandKind::FPIMM0,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcmeq,
            operation: Operation::ASIMDMISC(ASIMDMISC::FCMEQ_Vd_Vn_FPIMM0(
                FCMEQ_Vd_Vn_FPIMM0::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCMEQ_Vd_Vn_FPIMM0 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMEQ_Sd_Sn_FPIMM0 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmeq",
        aliases: &[],
        opcode: 0x5ea0d800,
        mask: 0xffbffc00,
        class: InsnClass::ASISDMISC,
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
                kind: InsnOperandKind::FPIMM0,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMD_SCALAR_SIZE.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcmeq,
            operation: Operation::ASISDMISC(ASISDMISC::FCMEQ_Sd_Sn_FPIMM0(
                FCMEQ_Sd_Sn_FPIMM0::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCMEQ_Sd_Sn_FPIMM0 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMEQ_Vd_V_4H_Vn_V_4H_FPIMM0 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmeq",
        aliases: &[],
        opcode: 0xef8d800,
        mask: 0xbffffc00,
        class: InsnClass::ASIMDMISC,
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
                kind: InsnOperandKind::FPIMM0,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcmeq,
            operation: Operation::ASIMDMISC(ASIMDMISC::FCMEQ_Vd_V_4H_Vn_V_4H_FPIMM0(
                FCMEQ_Vd_V_4H_Vn_V_4H_FPIMM0::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCMEQ_Vd_V_4H_Vn_V_4H_FPIMM0 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMEQ_Sd_S_H_Sn_S_H_FPIMM0 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmeq",
        aliases: &[],
        opcode: 0x5ef8d800,
        mask: 0xfffffc00,
        class: InsnClass::ASISDMISC,
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
                kind: InsnOperandKind::FPIMM0,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMD_SCALAR_SIZE.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcmeq,
            operation: Operation::ASISDMISC(ASISDMISC::FCMEQ_Sd_S_H_Sn_S_H_FPIMM0(
                FCMEQ_Sd_S_H_Sn_S_H_FPIMM0::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCMEQ_Sd_S_H_Sn_S_H_FPIMM0 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMEQ_Vd_V_2S_Vn_V_2S_Vm_V_2S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmeq",
        aliases: &[],
        opcode: 0xe20e400,
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
            mnemonic: Mnemonic::r#fcmeq,
            operation: Operation::ASIMDSAME(ASIMDSAME::FCMEQ_Vd_V_2S_Vn_V_2S_Vm_V_2S(
                FCMEQ_Vd_V_2S_Vn_V_2S_Vm_V_2S::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCMEQ_Vd_V_2S_Vn_V_2S_Vm_V_2S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMEQ_Sd_S_S_Sn_S_S_Sm_S_S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmeq",
        aliases: &[],
        opcode: 0x5e20e400,
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
            mnemonic: Mnemonic::r#fcmeq,
            operation: Operation::ASISDSAME(ASISDSAME::FCMEQ_Sd_S_S_Sn_S_S_Sm_S_S(
                FCMEQ_Sd_S_S_Sn_S_S_Sm_S_S::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCMEQ_Sd_S_S_Sn_S_S_Sm_S_S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMGE_Vd_Vn_Vm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmge",
        aliases: &[],
        opcode: 0x2e402400,
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
            mnemonic: Mnemonic::r#fcmge,
            operation: Operation::ASIMDSAME(ASIMDSAME::FCMGE_Vd_Vn_Vm(FCMGE_Vd_Vn_Vm::from(bits))),
        }
    }
}
impl InsnOpcode for FCMGE_Vd_Vn_Vm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMGE_Sd_Sn_Sm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmge",
        aliases: &[],
        opcode: 0x7e402400,
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
            mnemonic: Mnemonic::r#fcmge,
            operation: Operation::ASISDSAME(ASISDSAME::FCMGE_Sd_Sn_Sm(FCMGE_Sd_Sn_Sm::from(bits))),
        }
    }
}
impl InsnOpcode for FCMGE_Sd_Sn_Sm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMGE_Vd_Vn_FPIMM0 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmge",
        aliases: &[],
        opcode: 0x2ea0c800,
        mask: 0xbfbffc00,
        class: InsnClass::ASIMDMISC,
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
                kind: InsnOperandKind::FPIMM0,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcmge,
            operation: Operation::ASIMDMISC(ASIMDMISC::FCMGE_Vd_Vn_FPIMM0(
                FCMGE_Vd_Vn_FPIMM0::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCMGE_Vd_Vn_FPIMM0 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMGE_Sd_Sn_FPIMM0 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmge",
        aliases: &[],
        opcode: 0x7ea0c800,
        mask: 0xffbffc00,
        class: InsnClass::ASISDMISC,
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
                kind: InsnOperandKind::FPIMM0,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMD_SCALAR_SIZE.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcmge,
            operation: Operation::ASISDMISC(ASISDMISC::FCMGE_Sd_Sn_FPIMM0(
                FCMGE_Sd_Sn_FPIMM0::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCMGE_Sd_Sn_FPIMM0 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMGE_Vd_V_4H_Vn_V_4H_FPIMM0 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmge",
        aliases: &[],
        opcode: 0x2ef8c800,
        mask: 0xbffffc00,
        class: InsnClass::ASIMDMISC,
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
                kind: InsnOperandKind::FPIMM0,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcmge,
            operation: Operation::ASIMDMISC(ASIMDMISC::FCMGE_Vd_V_4H_Vn_V_4H_FPIMM0(
                FCMGE_Vd_V_4H_Vn_V_4H_FPIMM0::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCMGE_Vd_V_4H_Vn_V_4H_FPIMM0 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMGE_Sd_S_H_Sn_S_H_FPIMM0 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmge",
        aliases: &[],
        opcode: 0x7ef8c800,
        mask: 0xfffffc00,
        class: InsnClass::ASISDMISC,
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
                kind: InsnOperandKind::FPIMM0,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMD_SCALAR_SIZE.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcmge,
            operation: Operation::ASISDMISC(ASISDMISC::FCMGE_Sd_S_H_Sn_S_H_FPIMM0(
                FCMGE_Sd_S_H_Sn_S_H_FPIMM0::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCMGE_Sd_S_H_Sn_S_H_FPIMM0 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMGE_Vd_V_2S_Vn_V_2S_Vm_V_2S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmge",
        aliases: &[],
        opcode: 0x2e20e400,
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
            mnemonic: Mnemonic::r#fcmge,
            operation: Operation::ASIMDSAME(ASIMDSAME::FCMGE_Vd_V_2S_Vn_V_2S_Vm_V_2S(
                FCMGE_Vd_V_2S_Vn_V_2S_Vm_V_2S::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCMGE_Vd_V_2S_Vn_V_2S_Vm_V_2S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMGE_Sd_S_S_Sn_S_S_Sm_S_S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmge",
        aliases: &[],
        opcode: 0x7e20e400,
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
            mnemonic: Mnemonic::r#fcmge,
            operation: Operation::ASISDSAME(ASISDSAME::FCMGE_Sd_S_S_Sn_S_S_Sm_S_S(
                FCMGE_Sd_S_S_Sn_S_S_Sm_S_S::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCMGE_Sd_S_S_Sn_S_S_Sm_S_S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMGT_Vd_Vn_Vm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmgt",
        aliases: &[],
        opcode: 0x2ec02400,
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
            mnemonic: Mnemonic::r#fcmgt,
            operation: Operation::ASIMDSAME(ASIMDSAME::FCMGT_Vd_Vn_Vm(FCMGT_Vd_Vn_Vm::from(bits))),
        }
    }
}
impl InsnOpcode for FCMGT_Vd_Vn_Vm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMGT_Sd_Sn_Sm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmgt",
        aliases: &[],
        opcode: 0x7ec02400,
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
            mnemonic: Mnemonic::r#fcmgt,
            operation: Operation::ASISDSAME(ASISDSAME::FCMGT_Sd_Sn_Sm(FCMGT_Sd_Sn_Sm::from(bits))),
        }
    }
}
impl InsnOpcode for FCMGT_Sd_Sn_Sm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMGT_Vd_Vn_FPIMM0 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmgt",
        aliases: &[],
        opcode: 0xea0c800,
        mask: 0xbfbffc00,
        class: InsnClass::ASIMDMISC,
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
                kind: InsnOperandKind::FPIMM0,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcmgt,
            operation: Operation::ASIMDMISC(ASIMDMISC::FCMGT_Vd_Vn_FPIMM0(
                FCMGT_Vd_Vn_FPIMM0::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCMGT_Vd_Vn_FPIMM0 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMGT_Sd_Sn_FPIMM0 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmgt",
        aliases: &[],
        opcode: 0x5ea0c800,
        mask: 0xffbffc00,
        class: InsnClass::ASISDMISC,
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
                kind: InsnOperandKind::FPIMM0,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMD_SCALAR_SIZE.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcmgt,
            operation: Operation::ASISDMISC(ASISDMISC::FCMGT_Sd_Sn_FPIMM0(
                FCMGT_Sd_Sn_FPIMM0::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCMGT_Sd_Sn_FPIMM0 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMGT_Vd_V_4H_Vn_V_4H_FPIMM0 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmgt",
        aliases: &[],
        opcode: 0xef8c800,
        mask: 0xbffffc00,
        class: InsnClass::ASIMDMISC,
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
                kind: InsnOperandKind::FPIMM0,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcmgt,
            operation: Operation::ASIMDMISC(ASIMDMISC::FCMGT_Vd_V_4H_Vn_V_4H_FPIMM0(
                FCMGT_Vd_V_4H_Vn_V_4H_FPIMM0::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCMGT_Vd_V_4H_Vn_V_4H_FPIMM0 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMGT_Sd_S_H_Sn_S_H_FPIMM0 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmgt",
        aliases: &[],
        opcode: 0x5ef8c800,
        mask: 0xfffffc00,
        class: InsnClass::ASISDMISC,
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
                kind: InsnOperandKind::FPIMM0,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMD_SCALAR_SIZE.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcmgt,
            operation: Operation::ASISDMISC(ASISDMISC::FCMGT_Sd_S_H_Sn_S_H_FPIMM0(
                FCMGT_Sd_S_H_Sn_S_H_FPIMM0::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCMGT_Sd_S_H_Sn_S_H_FPIMM0 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMGT_Vd_V_2S_Vn_V_2S_Vm_V_2S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmgt",
        aliases: &[],
        opcode: 0x2ea0e400,
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
            mnemonic: Mnemonic::r#fcmgt,
            operation: Operation::ASIMDSAME(ASIMDSAME::FCMGT_Vd_V_2S_Vn_V_2S_Vm_V_2S(
                FCMGT_Vd_V_2S_Vn_V_2S_Vm_V_2S::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCMGT_Vd_V_2S_Vn_V_2S_Vm_V_2S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMGT_Sd_S_S_Sn_S_S_Sm_S_S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmgt",
        aliases: &[],
        opcode: 0x7ea0e400,
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
            mnemonic: Mnemonic::r#fcmgt,
            operation: Operation::ASISDSAME(ASISDSAME::FCMGT_Sd_S_S_Sn_S_S_Sm_S_S(
                FCMGT_Sd_S_S_Sn_S_S_Sm_S_S::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCMGT_Sd_S_S_Sn_S_S_Sm_S_S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMLA_Vd_Vn_Vm_IMM_ROT1 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmla",
        aliases: &[],
        opcode: 0x2e00c400,
        mask: 0xbf20e400,
        class: InsnClass::ASIMDSAME,
        feature_set: InsnFeatureSet::COMPNUM,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
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
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
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
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
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
            InsnOperand {
                kind: InsnOperandKind::IMM_ROT1,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::rotate1,
                    lsb: 11,
                    width: 2,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcmla,
            operation: Operation::ASIMDSAME(ASIMDSAME::FCMLA_Vd_Vn_Vm_IMM_ROT1(
                FCMLA_Vd_Vn_Vm_IMM_ROT1::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCMLA_Vd_Vn_Vm_IMM_ROT1 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMLA_Vd_Vn_Em_IMM_ROT2 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmla",
        aliases: &[],
        opcode: 0x2f001000,
        mask: 0xbf009400,
        class: InsnClass::ASIMDELEM,
        feature_set: InsnFeatureSet::COMPNUM,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_4S,
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
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_8H,
                    InsnOperandQualifier::V_4S,
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
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rm,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::IMM_ROT2,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::rotate2,
                    lsb: 13,
                    width: 2,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcmla,
            operation: Operation::ASIMDELEM(ASIMDELEM::FCMLA_Vd_Vn_Em_IMM_ROT2(
                FCMLA_Vd_Vn_Em_IMM_ROT2::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCMLA_Vd_Vn_Em_IMM_ROT2 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMLE_Vd_Vn_FPIMM0 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmle",
        aliases: &[],
        opcode: 0x2ea0d800,
        mask: 0xbfbffc00,
        class: InsnClass::ASIMDMISC,
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
                kind: InsnOperandKind::FPIMM0,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcmle,
            operation: Operation::ASIMDMISC(ASIMDMISC::FCMLE_Vd_Vn_FPIMM0(
                FCMLE_Vd_Vn_FPIMM0::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCMLE_Vd_Vn_FPIMM0 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMLE_Sd_Sn_FPIMM0 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmle",
        aliases: &[],
        opcode: 0x7ea0d800,
        mask: 0xffbffc00,
        class: InsnClass::ASISDMISC,
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
                kind: InsnOperandKind::FPIMM0,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMD_SCALAR_SIZE.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcmle,
            operation: Operation::ASISDMISC(ASISDMISC::FCMLE_Sd_Sn_FPIMM0(
                FCMLE_Sd_Sn_FPIMM0::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCMLE_Sd_Sn_FPIMM0 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMLE_Vd_V_4H_Vn_V_4H_FPIMM0 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmle",
        aliases: &[],
        opcode: 0x2ef8d800,
        mask: 0xbffffc00,
        class: InsnClass::ASIMDMISC,
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
                kind: InsnOperandKind::FPIMM0,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcmle,
            operation: Operation::ASIMDMISC(ASIMDMISC::FCMLE_Vd_V_4H_Vn_V_4H_FPIMM0(
                FCMLE_Vd_V_4H_Vn_V_4H_FPIMM0::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCMLE_Vd_V_4H_Vn_V_4H_FPIMM0 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMLE_Sd_S_H_Sn_S_H_FPIMM0 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmle",
        aliases: &[],
        opcode: 0x7ef8d800,
        mask: 0xfffffc00,
        class: InsnClass::ASISDMISC,
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
                kind: InsnOperandKind::FPIMM0,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMD_SCALAR_SIZE.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcmle,
            operation: Operation::ASISDMISC(ASISDMISC::FCMLE_Sd_S_H_Sn_S_H_FPIMM0(
                FCMLE_Sd_S_H_Sn_S_H_FPIMM0::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCMLE_Sd_S_H_Sn_S_H_FPIMM0 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMLT_Vd_Vn_FPIMM0 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmlt",
        aliases: &[],
        opcode: 0xea0e800,
        mask: 0xbfbffc00,
        class: InsnClass::ASIMDMISC,
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
                kind: InsnOperandKind::FPIMM0,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcmlt,
            operation: Operation::ASIMDMISC(ASIMDMISC::FCMLT_Vd_Vn_FPIMM0(
                FCMLT_Vd_Vn_FPIMM0::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCMLT_Vd_Vn_FPIMM0 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMLT_Sd_Sn_FPIMM0 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmlt",
        aliases: &[],
        opcode: 0x5ea0e800,
        mask: 0xffbffc00,
        class: InsnClass::ASISDMISC,
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
                kind: InsnOperandKind::FPIMM0,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMD_SCALAR_SIZE.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcmlt,
            operation: Operation::ASISDMISC(ASISDMISC::FCMLT_Sd_Sn_FPIMM0(
                FCMLT_Sd_Sn_FPIMM0::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCMLT_Sd_Sn_FPIMM0 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMLT_Vd_V_4H_Vn_V_4H_FPIMM0 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmlt",
        aliases: &[],
        opcode: 0xef8e800,
        mask: 0xbffffc00,
        class: InsnClass::ASIMDMISC,
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
                kind: InsnOperandKind::FPIMM0,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcmlt,
            operation: Operation::ASIMDMISC(ASIMDMISC::FCMLT_Vd_V_4H_Vn_V_4H_FPIMM0(
                FCMLT_Vd_V_4H_Vn_V_4H_FPIMM0::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCMLT_Vd_V_4H_Vn_V_4H_FPIMM0 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMLT_Sd_S_H_Sn_S_H_FPIMM0 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmlt",
        aliases: &[],
        opcode: 0x5ef8e800,
        mask: 0xfffffc00,
        class: InsnClass::ASISDMISC,
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
                kind: InsnOperandKind::FPIMM0,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMD_SCALAR_SIZE.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcmlt,
            operation: Operation::ASISDMISC(ASISDMISC::FCMLT_Sd_S_H_Sn_S_H_FPIMM0(
                FCMLT_Sd_S_H_Sn_S_H_FPIMM0::from(bits),
            )),
        }
    }
}
