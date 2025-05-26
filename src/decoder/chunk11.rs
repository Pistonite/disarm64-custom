#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
impl InsnOpcode for FCMLT_Sd_S_H_Sn_S_H_FPIMM0 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMP_Fn_Fm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmp",
        aliases: &[],
        opcode: 0x1ee02000,
        mask: 0xffe0fc1f,
        class: InsnClass::FLOATCMP,
        feature_set: InsnFeatureSet::FP_F16,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Fn,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Fm,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rm,
                    lsb: 16,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_FPTYPE_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcmp,
            operation: Operation::FLOATCMP(FLOATCMP::FCMP_Fn_Fm(FCMP_Fn_Fm::from(bits))),
        }
    }
}
impl InsnOpcode for FCMP_Fn_Fm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMP_Fn_S_S_Fm_S_S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmp",
        aliases: &[],
        opcode: 0x1e202000,
        mask: 0xff20fc1f,
        class: InsnClass::FLOATCMP,
        feature_set: InsnFeatureSet::FP,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Fn,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Fm,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rm,
                    lsb: 16,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_FPTYPE_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcmp,
            operation: Operation::FLOATCMP(FLOATCMP::FCMP_Fn_S_S_Fm_S_S(FCMP_Fn_S_S_Fm_S_S::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for FCMP_Fn_S_S_Fm_S_S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMP_Fn_FPIMM0 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmp",
        aliases: &[],
        opcode: 0x1ee02008,
        mask: 0xffe0fc1f,
        class: InsnClass::FLOATCMP,
        feature_set: InsnFeatureSet::FP_F16,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Fn,
                class: InsnOperandClass::FP_REG,
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
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_FPTYPE_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcmp,
            operation: Operation::FLOATCMP(FLOATCMP::FCMP_Fn_FPIMM0(FCMP_Fn_FPIMM0::from(bits))),
        }
    }
}
impl InsnOpcode for FCMP_Fn_FPIMM0 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMP_Fn_S_S_FPIMM0 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmp",
        aliases: &[],
        opcode: 0x1e202008,
        mask: 0xff20fc1f,
        class: InsnClass::FLOATCMP,
        feature_set: InsnFeatureSet::FP,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Fn,
                class: InsnOperandClass::FP_REG,
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
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_FPTYPE_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcmp,
            operation: Operation::FLOATCMP(FLOATCMP::FCMP_Fn_S_S_FPIMM0(FCMP_Fn_S_S_FPIMM0::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for FCMP_Fn_S_S_FPIMM0 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMPE_Fn_Fm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmpe",
        aliases: &[],
        opcode: 0x1ee02010,
        mask: 0xffe0fc1f,
        class: InsnClass::FLOATCMP,
        feature_set: InsnFeatureSet::FP_F16,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Fn,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Fm,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rm,
                    lsb: 16,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_FPTYPE_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcmpe,
            operation: Operation::FLOATCMP(FLOATCMP::FCMPE_Fn_Fm(FCMPE_Fn_Fm::from(bits))),
        }
    }
}
impl InsnOpcode for FCMPE_Fn_Fm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMPE_Fn_S_S_Fm_S_S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmpe",
        aliases: &[],
        opcode: 0x1e202010,
        mask: 0xff20fc1f,
        class: InsnClass::FLOATCMP,
        feature_set: InsnFeatureSet::FP,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Fn,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Fm,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rm,
                    lsb: 16,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_FPTYPE_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcmpe,
            operation: Operation::FLOATCMP(FLOATCMP::FCMPE_Fn_S_S_Fm_S_S(
                FCMPE_Fn_S_S_Fm_S_S::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCMPE_Fn_S_S_Fm_S_S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMPE_Fn_FPIMM0 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmpe",
        aliases: &[],
        opcode: 0x1ee02018,
        mask: 0xffe0fc1f,
        class: InsnClass::FLOATCMP,
        feature_set: InsnFeatureSet::FP_F16,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Fn,
                class: InsnOperandClass::FP_REG,
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
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_FPTYPE_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcmpe,
            operation: Operation::FLOATCMP(FLOATCMP::FCMPE_Fn_FPIMM0(FCMPE_Fn_FPIMM0::from(bits))),
        }
    }
}
impl InsnOpcode for FCMPE_Fn_FPIMM0 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCMPE_Fn_S_S_FPIMM0 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcmpe",
        aliases: &[],
        opcode: 0x1e202018,
        mask: 0xff20fc1f,
        class: InsnClass::FLOATCMP,
        feature_set: InsnFeatureSet::FP,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Fn,
                class: InsnOperandClass::FP_REG,
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
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_FPTYPE_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcmpe,
            operation: Operation::FLOATCMP(FLOATCMP::FCMPE_Fn_S_S_FPIMM0(
                FCMPE_Fn_S_S_FPIMM0::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCMPE_Fn_S_S_FPIMM0 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCSEL_Fd_Fn_Fm_COND {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcsel",
        aliases: &[],
        opcode: 0x1ee00c00,
        mask: 0xffe00c00,
        class: InsnClass::FLOATSEL,
        feature_set: InsnFeatureSet::FP_F16,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Fd,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Fn,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Fm,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rm,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::COND,
                class: InsnOperandClass::COND,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_FPTYPE_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcsel,
            operation: Operation::FLOATSEL(FLOATSEL::FCSEL_Fd_Fn_Fm_COND(
                FCSEL_Fd_Fn_Fm_COND::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCSEL_Fd_Fn_Fm_COND {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCSEL_Fd_S_S_Fn_S_S_Fm_S_S_COND {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcsel",
        aliases: &[],
        opcode: 0x1e200c00,
        mask: 0xff200c00,
        class: InsnClass::FLOATSEL,
        feature_set: InsnFeatureSet::FP,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Fd,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Fn,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Fm,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rm,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::COND,
                class: InsnOperandClass::COND,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_FPTYPE_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcsel,
            operation: Operation::FLOATSEL(FLOATSEL::FCSEL_Fd_S_S_Fn_S_S_Fm_S_S_COND(
                FCSEL_Fd_S_S_Fn_S_S_Fm_S_S_COND::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCSEL_Fd_S_S_Fn_S_S_Fm_S_S_COND {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCVTAS_Rd_Fn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcvtas",
        aliases: &[],
        opcode: 0x1ee40000,
        mask: 0x7ffffc00,
        class: InsnClass::FLOAT2INT,
        feature_set: InsnFeatureSet::FP_F16,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Fn,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_H, InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_FPTYPE_FIELD.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcvtas,
            operation: Operation::FLOAT2INT(FLOAT2INT::FCVTAS_Rd_Fn(FCVTAS_Rd_Fn::from(bits))),
        }
    }
}
impl InsnOpcode for FCVTAS_Rd_Fn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCVTAS_Rd_W_Fn_S_D {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcvtas",
        aliases: &[],
        opcode: 0x1e240000,
        mask: 0x7f3ffc00,
        class: InsnClass::FLOAT2INT,
        feature_set: InsnFeatureSet::FP,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[
                    InsnOperandQualifier::W,
                    InsnOperandQualifier::W,
                    InsnOperandQualifier::X,
                    InsnOperandQualifier::X,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Fn,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_S,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_FPTYPE_FIELD.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcvtas,
            operation: Operation::FLOAT2INT(FLOAT2INT::FCVTAS_Rd_W_Fn_S_D(
                FCVTAS_Rd_W_Fn_S_D::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCVTAS_Rd_W_Fn_S_D {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCVTAS_Vd_Vn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcvtas",
        aliases: &[],
        opcode: 0xe21c800,
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
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcvtas,
            operation: Operation::ASIMDMISC(ASIMDMISC::FCVTAS_Vd_Vn(FCVTAS_Vd_Vn::from(bits))),
        }
    }
}
impl InsnOpcode for FCVTAS_Vd_Vn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCVTAS_Sd_Sn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcvtas",
        aliases: &[],
        opcode: 0x5e21c800,
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
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMD_SCALAR_SIZE.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcvtas,
            operation: Operation::ASISDMISC(ASISDMISC::FCVTAS_Sd_Sn(FCVTAS_Sd_Sn::from(bits))),
        }
    }
}
impl InsnOpcode for FCVTAS_Sd_Sn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCVTAS_Vd_V_4H_Vn_V_4H {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcvtas",
        aliases: &[],
        opcode: 0xe79c800,
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
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcvtas,
            operation: Operation::ASIMDMISC(ASIMDMISC::FCVTAS_Vd_V_4H_Vn_V_4H(
                FCVTAS_Vd_V_4H_Vn_V_4H::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCVTAS_Vd_V_4H_Vn_V_4H {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCVTAS_Sd_S_H_Sn_S_H {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcvtas",
        aliases: &[],
        opcode: 0x5e79c800,
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
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMD_SCALAR_SIZE.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcvtas,
            operation: Operation::ASISDMISC(ASISDMISC::FCVTAS_Sd_S_H_Sn_S_H(
                FCVTAS_Sd_S_H_Sn_S_H::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCVTAS_Sd_S_H_Sn_S_H {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCVTAU_Rd_Fn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcvtau",
        aliases: &[],
        opcode: 0x1ee50000,
        mask: 0x7ffffc00,
        class: InsnClass::FLOAT2INT,
        feature_set: InsnFeatureSet::FP_F16,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Fn,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_H, InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_FPTYPE_FIELD.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcvtau,
            operation: Operation::FLOAT2INT(FLOAT2INT::FCVTAU_Rd_Fn(FCVTAU_Rd_Fn::from(bits))),
        }
    }
}
impl InsnOpcode for FCVTAU_Rd_Fn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCVTAU_Rd_W_Fn_S_D {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcvtau",
        aliases: &[],
        opcode: 0x1e250000,
        mask: 0x7f3ffc00,
        class: InsnClass::FLOAT2INT,
        feature_set: InsnFeatureSet::FP,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[
                    InsnOperandQualifier::W,
                    InsnOperandQualifier::W,
                    InsnOperandQualifier::X,
                    InsnOperandQualifier::X,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Fn,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_S,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_FPTYPE_FIELD.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcvtau,
            operation: Operation::FLOAT2INT(FLOAT2INT::FCVTAU_Rd_W_Fn_S_D(
                FCVTAU_Rd_W_Fn_S_D::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCVTAU_Rd_W_Fn_S_D {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCVTAU_Vd_Vn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcvtau",
        aliases: &[],
        opcode: 0x2e21c800,
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
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcvtau,
            operation: Operation::ASIMDMISC(ASIMDMISC::FCVTAU_Vd_Vn(FCVTAU_Vd_Vn::from(bits))),
        }
    }
}
impl InsnOpcode for FCVTAU_Vd_Vn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCVTAU_Sd_Sn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcvtau",
        aliases: &[],
        opcode: 0x7e21c800,
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
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMD_SCALAR_SIZE.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcvtau,
            operation: Operation::ASISDMISC(ASISDMISC::FCVTAU_Sd_Sn(FCVTAU_Sd_Sn::from(bits))),
        }
    }
}
impl InsnOpcode for FCVTAU_Sd_Sn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCVTAU_Vd_V_4H_Vn_V_4H {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcvtau",
        aliases: &[],
        opcode: 0x2e79c800,
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
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcvtau,
            operation: Operation::ASIMDMISC(ASIMDMISC::FCVTAU_Vd_V_4H_Vn_V_4H(
                FCVTAU_Vd_V_4H_Vn_V_4H::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCVTAU_Vd_V_4H_Vn_V_4H {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCVTAU_Sd_S_H_Sn_S_H {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcvtau",
        aliases: &[],
        opcode: 0x7e79c800,
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
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMD_SCALAR_SIZE.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcvtau,
            operation: Operation::ASISDMISC(ASISDMISC::FCVTAU_Sd_S_H_Sn_S_H(
                FCVTAU_Sd_S_H_Sn_S_H::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCVTAU_Sd_S_H_Sn_S_H {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCVTL_Vd_Vn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcvtl",
        aliases: &[],
        opcode: 0xe217800,
        mask: 0xffbffc00,
        class: InsnClass::ASIMDMISC,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_4S, InsnOperandQualifier::V_2D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Vn,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_4H, InsnOperandQualifier::V_2S],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SPEC_DECODE_RULES.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcvtl,
            operation: Operation::ASIMDMISC(ASIMDMISC::FCVTL_Vd_Vn(FCVTL_Vd_Vn::from(bits))),
        }
    }
}
impl InsnOpcode for FCVTL_Vd_Vn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCVTL2_Vd_Vn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcvtl2",
        aliases: &[],
        opcode: 0x4e217800,
        mask: 0xffbffc00,
        class: InsnClass::ASIMDMISC,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_4S, InsnOperandQualifier::V_2D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Vn,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_8H, InsnOperandQualifier::V_4S],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SPEC_DECODE_RULES.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcvtl2,
            operation: Operation::ASIMDMISC(ASIMDMISC::FCVTL2_Vd_Vn(FCVTL2_Vd_Vn::from(bits))),
        }
    }
}
impl InsnOpcode for FCVTL2_Vd_Vn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCVTMS_Rd_Fn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcvtms",
        aliases: &[],
        opcode: 0x1ef00000,
        mask: 0x7ffffc00,
        class: InsnClass::FLOAT2INT,
        feature_set: InsnFeatureSet::FP_F16,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Fn,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_H, InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_FPTYPE_FIELD.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcvtms,
            operation: Operation::FLOAT2INT(FLOAT2INT::FCVTMS_Rd_Fn(FCVTMS_Rd_Fn::from(bits))),
        }
    }
}
impl InsnOpcode for FCVTMS_Rd_Fn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCVTMS_Rd_W_Fn_S_D {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcvtms",
        aliases: &[],
        opcode: 0x1e300000,
        mask: 0x7f3ffc00,
        class: InsnClass::FLOAT2INT,
        feature_set: InsnFeatureSet::FP,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[
                    InsnOperandQualifier::W,
                    InsnOperandQualifier::W,
                    InsnOperandQualifier::X,
                    InsnOperandQualifier::X,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Fn,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_S,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_FPTYPE_FIELD.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcvtms,
            operation: Operation::FLOAT2INT(FLOAT2INT::FCVTMS_Rd_W_Fn_S_D(
                FCVTMS_Rd_W_Fn_S_D::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCVTMS_Rd_W_Fn_S_D {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCVTMS_Vd_Vn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcvtms",
        aliases: &[],
        opcode: 0xe21b800,
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
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcvtms,
            operation: Operation::ASIMDMISC(ASIMDMISC::FCVTMS_Vd_Vn(FCVTMS_Vd_Vn::from(bits))),
        }
    }
}
impl InsnOpcode for FCVTMS_Vd_Vn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCVTMS_Sd_Sn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcvtms",
        aliases: &[],
        opcode: 0x5e21b800,
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
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMD_SCALAR_SIZE.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcvtms,
            operation: Operation::ASISDMISC(ASISDMISC::FCVTMS_Sd_Sn(FCVTMS_Sd_Sn::from(bits))),
        }
    }
}
impl InsnOpcode for FCVTMS_Sd_Sn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCVTMS_Vd_V_4H_Vn_V_4H {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcvtms",
        aliases: &[],
        opcode: 0xe79b800,
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
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcvtms,
            operation: Operation::ASIMDMISC(ASIMDMISC::FCVTMS_Vd_V_4H_Vn_V_4H(
                FCVTMS_Vd_V_4H_Vn_V_4H::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCVTMS_Vd_V_4H_Vn_V_4H {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCVTMS_Sd_S_H_Sn_S_H {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcvtms",
        aliases: &[],
        opcode: 0x5e79b800,
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
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMD_SCALAR_SIZE.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcvtms,
            operation: Operation::ASISDMISC(ASISDMISC::FCVTMS_Sd_S_H_Sn_S_H(
                FCVTMS_Sd_S_H_Sn_S_H::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCVTMS_Sd_S_H_Sn_S_H {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCVTMU_Rd_Fn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcvtmu",
        aliases: &[],
        opcode: 0x1ef10000,
        mask: 0x7ffffc00,
        class: InsnClass::FLOAT2INT,
        feature_set: InsnFeatureSet::FP_F16,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Fn,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_H, InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_FPTYPE_FIELD.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcvtmu,
            operation: Operation::FLOAT2INT(FLOAT2INT::FCVTMU_Rd_Fn(FCVTMU_Rd_Fn::from(bits))),
        }
    }
}
impl InsnOpcode for FCVTMU_Rd_Fn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCVTMU_Rd_W_Fn_S_D {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcvtmu",
        aliases: &[],
        opcode: 0x1e310000,
        mask: 0x7f3ffc00,
        class: InsnClass::FLOAT2INT,
        feature_set: InsnFeatureSet::FP,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[
                    InsnOperandQualifier::W,
                    InsnOperandQualifier::W,
                    InsnOperandQualifier::X,
                    InsnOperandQualifier::X,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Fn,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_S,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_FPTYPE_FIELD.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcvtmu,
            operation: Operation::FLOAT2INT(FLOAT2INT::FCVTMU_Rd_W_Fn_S_D(
                FCVTMU_Rd_W_Fn_S_D::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCVTMU_Rd_W_Fn_S_D {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCVTMU_Vd_Vn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcvtmu",
        aliases: &[],
        opcode: 0x2e21b800,
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
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcvtmu,
            operation: Operation::ASIMDMISC(ASIMDMISC::FCVTMU_Vd_Vn(FCVTMU_Vd_Vn::from(bits))),
        }
    }
}
impl InsnOpcode for FCVTMU_Vd_Vn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCVTMU_Sd_Sn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcvtmu",
        aliases: &[],
        opcode: 0x7e21b800,
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
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMD_SCALAR_SIZE.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcvtmu,
            operation: Operation::ASISDMISC(ASISDMISC::FCVTMU_Sd_Sn(FCVTMU_Sd_Sn::from(bits))),
        }
    }
}
impl InsnOpcode for FCVTMU_Sd_Sn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCVTMU_Vd_V_4H_Vn_V_4H {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcvtmu",
        aliases: &[],
        opcode: 0x2e79b800,
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
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcvtmu,
            operation: Operation::ASIMDMISC(ASIMDMISC::FCVTMU_Vd_V_4H_Vn_V_4H(
                FCVTMU_Vd_V_4H_Vn_V_4H::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCVTMU_Vd_V_4H_Vn_V_4H {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCVTMU_Sd_S_H_Sn_S_H {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcvtmu",
        aliases: &[],
        opcode: 0x7e79b800,
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
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMD_SCALAR_SIZE.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcvtmu,
            operation: Operation::ASISDMISC(ASISDMISC::FCVTMU_Sd_S_H_Sn_S_H(
                FCVTMU_Sd_S_H_Sn_S_H::from(bits),
            )),
        }
    }
}
impl InsnOpcode for FCVTMU_Sd_S_H_Sn_S_H {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCVTN_Vd_Vn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcvtn",
        aliases: &[],
        opcode: 0xe216800,
        mask: 0xffbffc00,
        class: InsnClass::ASIMDMISC,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_4H, InsnOperandQualifier::V_2S],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Vn,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_4S, InsnOperandQualifier::V_2D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SPEC_DECODE_RULES.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcvtn,
            operation: Operation::ASIMDMISC(ASIMDMISC::FCVTN_Vd_Vn(FCVTN_Vd_Vn::from(bits))),
        }
    }
}
impl InsnOpcode for FCVTN_Vd_Vn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCVTN2_Vd_Vn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcvtn2",
        aliases: &[],
        opcode: 0x4e216800,
        mask: 0xffbffc00,
        class: InsnClass::ASIMDMISC,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_8H, InsnOperandQualifier::V_4S],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Vn,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_4S, InsnOperandQualifier::V_2D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SPEC_DECODE_RULES.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcvtn2,
            operation: Operation::ASIMDMISC(ASIMDMISC::FCVTN2_Vd_Vn(FCVTN2_Vd_Vn::from(bits))),
        }
    }
}
impl InsnOpcode for FCVTN2_Vd_Vn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCVTNS_Rd_Fn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcvtns",
        aliases: &[],
        opcode: 0x1ee00000,
        mask: 0x7ffffc00,
        class: InsnClass::FLOAT2INT,
        feature_set: InsnFeatureSet::FP_F16,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Fn,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_H, InsnOperandQualifier::S_H],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_FPTYPE_FIELD.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcvtns,
            operation: Operation::FLOAT2INT(FLOAT2INT::FCVTNS_Rd_Fn(FCVTNS_Rd_Fn::from(bits))),
        }
    }
}
impl InsnOpcode for FCVTNS_Rd_Fn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl FCVTNS_Rd_W_Fn_S_D {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "fcvtns",
        aliases: &[],
        opcode: 0x1e200000,
        mask: 0x7f3ffc00,
        class: InsnClass::FLOAT2INT,
        feature_set: InsnFeatureSet::FP,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[
                    InsnOperandQualifier::W,
                    InsnOperandQualifier::W,
                    InsnOperandQualifier::X,
                    InsnOperandQualifier::X,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Fn,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_S,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_FPTYPE_FIELD.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#fcvtns,
            operation: Operation::FLOAT2INT(FLOAT2INT::FCVTNS_Rd_W_Fn_S_D(
                FCVTNS_Rd_W_Fn_S_D::from(bits),
            )),
        }
    }
}
