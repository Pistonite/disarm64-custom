#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LDST_POS {
    LDRB_Rt_ADDR_UIMM12(LDRB_Rt_ADDR_UIMM12),
    LDRH_Rt_ADDR_UIMM12(LDRH_Rt_ADDR_UIMM12),
    LDRSB_Rt_ADDR_UIMM12(LDRSB_Rt_ADDR_UIMM12),
    LDRSH_Rt_ADDR_UIMM12(LDRSH_Rt_ADDR_UIMM12),
    LDRSW_Rt_ADDR_UIMM12(LDRSW_Rt_ADDR_UIMM12),
    LDR_Ft_ADDR_UIMM12(LDR_Ft_ADDR_UIMM12),
    LDR_Rt_ADDR_UIMM12(LDR_Rt_ADDR_UIMM12),
    PRFM_PRFOP_ADDR_UIMM12(PRFM_PRFOP_ADDR_UIMM12),
    STRB_Rt_ADDR_UIMM12(STRB_Rt_ADDR_UIMM12),
    STRH_Rt_ADDR_UIMM12(STRH_Rt_ADDR_UIMM12),
    STR_Ft_ADDR_UIMM12(STR_Ft_ADDR_UIMM12),
    STR_Rt_ADDR_UIMM12(STR_Rt_ADDR_UIMM12),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LDST_REGOFF {
    LDRB_Rt_ADDR_REGOFF(LDRB_Rt_ADDR_REGOFF),
    LDRH_Rt_ADDR_REGOFF(LDRH_Rt_ADDR_REGOFF),
    LDRSB_Rt_ADDR_REGOFF(LDRSB_Rt_ADDR_REGOFF),
    LDRSH_Rt_ADDR_REGOFF(LDRSH_Rt_ADDR_REGOFF),
    LDRSW_Rt_ADDR_REGOFF(LDRSW_Rt_ADDR_REGOFF),
    LDR_Ft_ADDR_REGOFF(LDR_Ft_ADDR_REGOFF),
    LDR_Rt_ADDR_REGOFF(LDR_Rt_ADDR_REGOFF),
    PRFM_PRFOP_ADDR_REGOFF(PRFM_PRFOP_ADDR_REGOFF),
    STRB_Rt_ADDR_REGOFF(STRB_Rt_ADDR_REGOFF),
    STRH_Rt_ADDR_REGOFF(STRH_Rt_ADDR_REGOFF),
    STR_Ft_ADDR_REGOFF(STR_Ft_ADDR_REGOFF),
    STR_Rt_ADDR_REGOFF(STR_Rt_ADDR_REGOFF),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LDST_UNPRIV {
    LDTRB_Rt_ADDR_SIMM9(LDTRB_Rt_ADDR_SIMM9),
    LDTRH_Rt_ADDR_SIMM9(LDTRH_Rt_ADDR_SIMM9),
    LDTRSB_Rt_ADDR_SIMM9(LDTRSB_Rt_ADDR_SIMM9),
    LDTRSH_Rt_ADDR_SIMM9(LDTRSH_Rt_ADDR_SIMM9),
    LDTRSW_Rt_ADDR_SIMM9(LDTRSW_Rt_ADDR_SIMM9),
    LDTR_Rt_ADDR_SIMM9(LDTR_Rt_ADDR_SIMM9),
    STTRB_Rt_ADDR_SIMM9(STTRB_Rt_ADDR_SIMM9),
    STTRH_Rt_ADDR_SIMM9(STTRH_Rt_ADDR_SIMM9),
    STTR_Rt_ADDR_SIMM9(STTR_Rt_ADDR_SIMM9),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LDST_UNSCALED {
    LDAPURB_Rt_ADDR_OFFSET(LDAPURB_Rt_ADDR_OFFSET),
    LDAPURH_Rt_ADDR_OFFSET(LDAPURH_Rt_ADDR_OFFSET),
    LDAPURSB_Rt_ADDR_OFFSET(LDAPURSB_Rt_ADDR_OFFSET),
    LDAPURSB_Rt_W_ADDR_OFFSET(LDAPURSB_Rt_W_ADDR_OFFSET),
    LDAPURSH_Rt_ADDR_OFFSET(LDAPURSH_Rt_ADDR_OFFSET),
    LDAPURSH_Rt_W_ADDR_OFFSET(LDAPURSH_Rt_W_ADDR_OFFSET),
    LDAPURSW_Rt_ADDR_OFFSET(LDAPURSW_Rt_ADDR_OFFSET),
    LDAPUR_Rt_ADDR_OFFSET(LDAPUR_Rt_ADDR_OFFSET),
    LDAPUR_Rt_X_ADDR_OFFSET(LDAPUR_Rt_X_ADDR_OFFSET),
    LDG_Rt_ADDR_SIMM13(LDG_Rt_ADDR_SIMM13),
    LDURB_Rt_ADDR_SIMM9(LDURB_Rt_ADDR_SIMM9),
    LDURH_Rt_ADDR_SIMM9(LDURH_Rt_ADDR_SIMM9),
    LDURSB_Rt_ADDR_SIMM9(LDURSB_Rt_ADDR_SIMM9),
    LDURSH_Rt_ADDR_SIMM9(LDURSH_Rt_ADDR_SIMM9),
    LDURSW_Rt_ADDR_SIMM9(LDURSW_Rt_ADDR_SIMM9),
    LDUR_Ft_ADDR_SIMM9(LDUR_Ft_ADDR_SIMM9),
    LDUR_Rt_ADDR_SIMM9(LDUR_Rt_ADDR_SIMM9),
    PRFUM_PRFOP_ADDR_SIMM9(PRFUM_PRFOP_ADDR_SIMM9),
    ST2G_Rt_SP_ADDR_SIMM13(ST2G_Rt_SP_ADDR_SIMM13),
    STG_Rt_SP_ADDR_SIMM13(STG_Rt_SP_ADDR_SIMM13),
    STLURB_Rt_ADDR_OFFSET(STLURB_Rt_ADDR_OFFSET),
    STLURH_Rt_ADDR_OFFSET(STLURH_Rt_ADDR_OFFSET),
    STLUR_Rt_ADDR_OFFSET(STLUR_Rt_ADDR_OFFSET),
    STLUR_Rt_X_ADDR_OFFSET(STLUR_Rt_X_ADDR_OFFSET),
    STURB_Rt_ADDR_SIMM9(STURB_Rt_ADDR_SIMM9),
    STURH_Rt_ADDR_SIMM9(STURH_Rt_ADDR_SIMM9),
    STUR_Ft_ADDR_SIMM9(STUR_Ft_ADDR_SIMM9),
    STUR_Rt_ADDR_SIMM9(STUR_Rt_ADDR_SIMM9),
    STZ2G_Rt_SP_ADDR_SIMM13(STZ2G_Rt_SP_ADDR_SIMM13),
    STZG_Rt_SP_ADDR_SIMM13(STZG_Rt_SP_ADDR_SIMM13),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LOG_IMM {
    ANDS_Rd_Rn_LIMM(ANDS_Rd_Rn_LIMM),
    AND_Rd_SP_Rn_LIMM(AND_Rd_SP_Rn_LIMM),
    EOR_Rd_SP_Rn_LIMM(EOR_Rd_SP_Rn_LIMM),
    ORR_Rd_SP_Rn_LIMM(ORR_Rd_SP_Rn_LIMM),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LOG_SHIFT {
    ANDS_Rd_Rn_Rm_SFT(ANDS_Rd_Rn_Rm_SFT),
    AND_Rd_Rn_Rm_SFT(AND_Rd_Rn_Rm_SFT),
    BICS_Rd_Rn_Rm_SFT(BICS_Rd_Rn_Rm_SFT),
    BIC_Rd_Rn_Rm_SFT(BIC_Rd_Rn_Rm_SFT),
    EON_Rd_Rn_Rm_SFT(EON_Rd_Rn_Rm_SFT),
    EOR_Rd_Rn_Rm_SFT(EOR_Rd_Rn_Rm_SFT),
    ORN_Rd_Rn_Rm_SFT(ORN_Rd_Rn_Rm_SFT),
    ORR_Rd_Rn_Rm_SFT(ORR_Rd_Rn_Rm_SFT),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum MOVEWIDE {
    MOVK_Rd_HALF(MOVK_Rd_HALF),
    MOVN_Rd_HALF(MOVN_Rd_HALF),
    MOVZ_Rd_HALF(MOVZ_Rd_HALF),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum PCRELADDR {
    ADRP_Rd_ADDR_ADRP(ADRP_Rd_ADDR_ADRP),
    ADR_Rd_ADDR_PCREL21(ADR_Rd_ADDR_PCREL21),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TESTBRANCH {
    TBNZ_Rt_BIT_NUM_ADDR_PCREL14(TBNZ_Rt_BIT_NUM_ADDR_PCREL14),
    TBZ_Rt_BIT_NUM_ADDR_PCREL14(TBZ_Rt_BIT_NUM_ADDR_PCREL14),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Operation {
    ADDSUB_CARRY(ADDSUB_CARRY),
    ADDSUB_EXT(ADDSUB_EXT),
    ADDSUB_IMM(ADDSUB_IMM),
    ADDSUB_SHIFT(ADDSUB_SHIFT),
    ASIMDALL(ASIMDALL),
    ASIMDDIFF(ASIMDDIFF),
    ASIMDELEM(ASIMDELEM),
    ASIMDEXT(ASIMDEXT),
    ASIMDIMM(ASIMDIMM),
    ASIMDINS(ASIMDINS),
    ASIMDMISC(ASIMDMISC),
    ASIMDPERM(ASIMDPERM),
    ASIMDSAME(ASIMDSAME),
    ASIMDSHF(ASIMDSHF),
    ASIMDTBL(ASIMDTBL),
    ASISDDIFF(ASISDDIFF),
    ASISDELEM(ASISDELEM),
    ASISDLSE(ASISDLSE),
    ASISDLSEP(ASISDLSEP),
    ASISDLSO(ASISDLSO),
    ASISDLSOP(ASISDLSOP),
    ASISDMISC(ASISDMISC),
    ASISDONE(ASISDONE),
    ASISDPAIR(ASISDPAIR),
    ASISDSAME(ASISDSAME),
    ASISDSHF(ASISDSHF),
    BITFIELD(BITFIELD),
    BRANCH_IMM(BRANCH_IMM),
    BRANCH_REG(BRANCH_REG),
    COMPBRANCH(COMPBRANCH),
    CONDBRANCH(CONDBRANCH),
    CONDCMP_IMM(CONDCMP_IMM),
    CONDCMP_REG(CONDCMP_REG),
    CONDSEL(CONDSEL),
    DP_1SRC(DP_1SRC),
    DP_2SRC(DP_2SRC),
    DP_3SRC(DP_3SRC),
    FLOAT2FIX(FLOAT2FIX),
    FLOAT2INT(FLOAT2INT),
    FLOATCCMP(FLOATCCMP),
    FLOATCMP(FLOATCMP),
    FLOATIMM(FLOATIMM),
    FLOATSEL(FLOATSEL),
    LDSTEXCL(LDSTEXCL),
    LDSTNAPAIR_OFFS(LDSTNAPAIR_OFFS),
    LDSTPAIR_INDEXED(LDSTPAIR_INDEXED),
    LDSTPAIR_OFF(LDSTPAIR_OFF),
    LDST_IMM10(LDST_IMM10),
    LDST_IMM9(LDST_IMM9),
    LDST_POS(LDST_POS),
    LDST_REGOFF(LDST_REGOFF),
    LDST_UNPRIV(LDST_UNPRIV),
    LDST_UNSCALED(LDST_UNSCALED),
    LOG_IMM(LOG_IMM),
    LOG_SHIFT(LOG_SHIFT),
    MOVEWIDE(MOVEWIDE),
    PCRELADDR(PCRELADDR),
    TESTBRANCH(TESTBRANCH),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Opcode {
    pub mnemonic: Mnemonic,
    pub operation: Operation,
}
impl ABS_Vd_Vn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "abs",
        aliases: &[],
        opcode: 0xe20b800,
        mask: 0xbf3ffc00,
        class: InsnClass::ASIMDMISC,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
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
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
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
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#abs,
            operation: Operation::ASIMDMISC(ASIMDMISC::ABS_Vd_Vn(ABS_Vd_Vn::from(bits))),
        }
    }
}
impl InsnOpcode for ABS_Vd_Vn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ABS_Sd_Sn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "abs",
        aliases: &[],
        opcode: 0x5e20b800,
        mask: 0xff3ffc00,
        class: InsnClass::ASISDMISC,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Sd,
                class: InsnOperandClass::SISD_REG,
                qualifiers: &[InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Sn,
                class: InsnOperandClass::SISD_REG,
                qualifiers: &[InsnOperandQualifier::S_D],
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
            mnemonic: Mnemonic::r#abs,
            operation: Operation::ASISDMISC(ASISDMISC::ABS_Sd_Sn(ABS_Sd_Sn::from(bits))),
        }
    }
}
impl InsnOpcode for ABS_Sd_Sn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ADC_Rd_Rn_Rm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "adc",
        aliases: &[],
        opcode: 0x1a000000,
        mask: 0x7fe0fc00,
        class: InsnClass::ADDSUB_CARRY,
        feature_set: InsnFeatureSet::V8,
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
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rm,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rm,
                    lsb: 16,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SF_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#adc,
            operation: Operation::ADDSUB_CARRY(ADDSUB_CARRY::ADC_Rd_Rn_Rm(ADC_Rd_Rn_Rm::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for ADC_Rd_Rn_Rm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ADCS_Rd_Rn_Rm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "adcs",
        aliases: &[],
        opcode: 0x3a000000,
        mask: 0x7fe0fc00,
        class: InsnClass::ADDSUB_CARRY,
        feature_set: InsnFeatureSet::V8,
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
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rm,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rm,
                    lsb: 16,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SF_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#adcs,
            operation: Operation::ADDSUB_CARRY(ADDSUB_CARRY::ADCS_Rd_Rn_Rm(ADCS_Rd_Rn_Rm::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for ADCS_Rd_Rn_Rm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ADD_Rd_SP_Rn_SP_AIMM {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "add",
        aliases: &[],
        opcode: 0x11000000,
        mask: 0x7f800000,
        class: InsnClass::ADDSUB_IMM,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::AIMM,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::shift,
                        lsb: 22,
                        width: 2,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm12,
                        lsb: 10,
                        width: 12,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#add,
            operation: Operation::ADDSUB_IMM(ADDSUB_IMM::ADD_Rd_SP_Rn_SP_AIMM(
                ADD_Rd_SP_Rn_SP_AIMM::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ADD_Rd_SP_Rn_SP_AIMM {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ADD_Rd_Rn_Rm_SFT {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "add",
        aliases: &[],
        opcode: 0xb000000,
        mask: 0x7f200000,
        class: InsnClass::ADDSUB_SHIFT,
        feature_set: InsnFeatureSet::V8,
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
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rm_SFT,
                class: InsnOperandClass::MODIFIED_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SF_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#add,
            operation: Operation::ADDSUB_SHIFT(ADDSUB_SHIFT::ADD_Rd_Rn_Rm_SFT(
                ADD_Rd_Rn_Rm_SFT::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ADD_Rd_Rn_Rm_SFT {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ADD_Rd_SP_Rn_SP_Rm_EXT {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "add",
        aliases: &[],
        opcode: 0xb200000,
        mask: 0x7fe00000,
        class: InsnClass::ADDSUB_EXT,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[
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
                kind: InsnOperandKind::Rn_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[
                    InsnOperandQualifier::W,
                    InsnOperandQualifier::X,
                    InsnOperandQualifier::X,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rm_EXT,
                class: InsnOperandClass::MODIFIED_REG,
                qualifiers: &[
                    InsnOperandQualifier::W,
                    InsnOperandQualifier::W,
                    InsnOperandQualifier::X,
                ],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SF_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#add,
            operation: Operation::ADDSUB_EXT(ADDSUB_EXT::ADD_Rd_SP_Rn_SP_Rm_EXT(
                ADD_Rd_SP_Rn_SP_Rm_EXT::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ADD_Rd_SP_Rn_SP_Rm_EXT {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ADD_Vd_Vn_Vm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "add",
        aliases: &[],
        opcode: 0xe208400,
        mask: 0xbf20fc00,
        class: InsnClass::ASIMDSAME,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
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
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
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
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
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
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#add,
            operation: Operation::ASIMDSAME(ASIMDSAME::ADD_Vd_Vn_Vm(ADD_Vd_Vn_Vm::from(bits))),
        }
    }
}
impl InsnOpcode for ADD_Vd_Vn_Vm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ADD_Sd_Sn_Sm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "add",
        aliases: &[],
        opcode: 0x5ee08400,
        mask: 0xffe0fc00,
        class: InsnClass::ASISDSAME,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Sd,
                class: InsnOperandClass::SISD_REG,
                qualifiers: &[InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Sn,
                class: InsnOperandClass::SISD_REG,
                qualifiers: &[InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Sm,
                class: InsnOperandClass::SISD_REG,
                qualifiers: &[InsnOperandQualifier::S_D],
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
            mnemonic: Mnemonic::r#add,
            operation: Operation::ASISDSAME(ASISDSAME::ADD_Sd_Sn_Sm(ADD_Sd_Sn_Sm::from(bits))),
        }
    }
}
impl InsnOpcode for ADD_Sd_Sn_Sm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ADDG_Rd_SP_Rn_SP_UIMM10_UIMM4_ADDG {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "addg",
        aliases: &[],
        opcode: 0x91800000,
        mask: 0xffc0c000,
        class: InsnClass::ADDSUB_IMM,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::UIMM10,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::immr,
                    lsb: 16,
                    width: 6,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::UIMM4_ADDG,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::imm4_10,
                    lsb: 10,
                    width: 4,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#addg,
            operation: Operation::ADDSUB_IMM(ADDSUB_IMM::ADDG_Rd_SP_Rn_SP_UIMM10_UIMM4_ADDG(
                ADDG_Rd_SP_Rn_SP_UIMM10_UIMM4_ADDG::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ADDG_Rd_SP_Rn_SP_UIMM10_UIMM4_ADDG {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ADDHN_Vd_Vn_Vm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "addhn",
        aliases: &[],
        opcode: 0xe204000,
        mask: 0xff20fc00,
        class: InsnClass::ASIMDDIFF,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_4H,
                    InsnOperandQualifier::V_2S,
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
                    InsnOperandQualifier::V_8H,
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
                    InsnOperandQualifier::V_8H,
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
            mnemonic: Mnemonic::r#addhn,
            operation: Operation::ASIMDDIFF(ASIMDDIFF::ADDHN_Vd_Vn_Vm(ADDHN_Vd_Vn_Vm::from(bits))),
        }
    }
}
impl InsnOpcode for ADDHN_Vd_Vn_Vm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ADDHN2_Vd_Vn_Vm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "addhn2",
        aliases: &[],
        opcode: 0x4e204000,
        mask: 0xff20fc00,
        class: InsnClass::ASIMDDIFF,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[
                    InsnOperandQualifier::V_16B,
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
                    InsnOperandQualifier::V_8H,
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
                    InsnOperandQualifier::V_8H,
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
            mnemonic: Mnemonic::r#addhn2,
            operation: Operation::ASIMDDIFF(ASIMDDIFF::ADDHN2_Vd_Vn_Vm(ADDHN2_Vd_Vn_Vm::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for ADDHN2_Vd_Vn_Vm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ADDP_Sd_Vn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "addp",
        aliases: &[],
        opcode: 0x5e31b800,
        mask: 0xff3ffc00,
        class: InsnClass::ASISDPAIR,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Sd,
                class: InsnOperandClass::SISD_REG,
                qualifiers: &[InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Vn,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_2D],
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
            mnemonic: Mnemonic::r#addp,
            operation: Operation::ASISDPAIR(ASISDPAIR::ADDP_Sd_Vn(ADDP_Sd_Vn::from(bits))),
        }
    }
}
impl InsnOpcode for ADDP_Sd_Vn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ADDP_Vd_Vn_Vm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "addp",
        aliases: &[],
        opcode: 0xe20bc00,
        mask: 0xbf20fc00,
        class: InsnClass::ASIMDSAME,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
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
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
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
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
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
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#addp,
            operation: Operation::ASIMDSAME(ASIMDSAME::ADDP_Vd_Vn_Vm(ADDP_Vd_Vn_Vm::from(bits))),
        }
    }
}
impl InsnOpcode for ADDP_Vd_Vn_Vm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ADDS_Rd_Rn_SP_AIMM {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "adds",
        aliases: &[],
        opcode: 0x31000000,
        mask: 0x7f800000,
        class: InsnClass::ADDSUB_IMM,
        feature_set: InsnFeatureSet::V8,
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
                kind: InsnOperandKind::Rn_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::AIMM,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::shift,
                        lsb: 22,
                        width: 2,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm12,
                        lsb: 10,
                        width: 12,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#adds,
            operation: Operation::ADDSUB_IMM(ADDSUB_IMM::ADDS_Rd_Rn_SP_AIMM(
                ADDS_Rd_Rn_SP_AIMM::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ADDS_Rd_Rn_SP_AIMM {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ADDS_Rd_Rn_Rm_SFT {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "adds",
        aliases: &[],
        opcode: 0x2b000000,
        mask: 0x7f200000,
        class: InsnClass::ADDSUB_SHIFT,
        feature_set: InsnFeatureSet::V8,
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
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rm_SFT,
                class: InsnOperandClass::MODIFIED_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#adds,
            operation: Operation::ADDSUB_SHIFT(ADDSUB_SHIFT::ADDS_Rd_Rn_Rm_SFT(
                ADDS_Rd_Rn_Rm_SFT::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ADDS_Rd_Rn_Rm_SFT {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ADDS_Rd_Rn_SP_Rm_EXT {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "adds",
        aliases: &[],
        opcode: 0x2b200000,
        mask: 0x7fe00000,
        class: InsnClass::ADDSUB_EXT,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[
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
                kind: InsnOperandKind::Rn_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[
                    InsnOperandQualifier::W,
                    InsnOperandQualifier::X,
                    InsnOperandQualifier::X,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rm_EXT,
                class: InsnOperandClass::MODIFIED_REG,
                qualifiers: &[
                    InsnOperandQualifier::W,
                    InsnOperandQualifier::W,
                    InsnOperandQualifier::X,
                ],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#adds,
            operation: Operation::ADDSUB_EXT(ADDSUB_EXT::ADDS_Rd_Rn_SP_Rm_EXT(
                ADDS_Rd_Rn_SP_Rm_EXT::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ADDS_Rd_Rn_SP_Rm_EXT {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ADDV_Fd_Vn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "addv",
        aliases: &[],
        opcode: 0xe31b800,
        mask: 0xbf3ffc00,
        class: InsnClass::ASIMDALL,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Fd,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
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
                    InsnOperandQualifier::V_8B,
                    InsnOperandQualifier::V_16B,
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
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SIZEQ_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#addv,
            operation: Operation::ASIMDALL(ASIMDALL::ADDV_Fd_Vn(ADDV_Fd_Vn::from(bits))),
        }
    }
}
impl InsnOpcode for ADDV_Fd_Vn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ADR_Rd_ADDR_PCREL21 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "adr",
        aliases: &[],
        opcode: 0x10000000,
        mask: 0x9f000000,
        class: InsnClass::PCRELADDR,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_PCREL21,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::immhi,
                        lsb: 5,
                        width: 19,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::immlo,
                        lsb: 29,
                        width: 2,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#adr,
            operation: Operation::PCRELADDR(PCRELADDR::ADR_Rd_ADDR_PCREL21(
                ADR_Rd_ADDR_PCREL21::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ADR_Rd_ADDR_PCREL21 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ADRP_Rd_ADDR_ADRP {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "adrp",
        aliases: &[],
        opcode: 0x90000000,
        mask: 0x9f000000,
        class: InsnClass::PCRELADDR,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_ADRP,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::immhi,
                        lsb: 5,
                        width: 19,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::immlo,
                        lsb: 29,
                        width: 2,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#adrp,
            operation: Operation::PCRELADDR(PCRELADDR::ADRP_Rd_ADDR_ADRP(ADRP_Rd_ADDR_ADRP::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for ADRP_Rd_ADDR_ADRP {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl AND_Rd_SP_Rn_LIMM {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "and",
        aliases: &[],
        opcode: 0x12000000,
        mask: 0x7f800000,
        class: InsnClass::LOG_IMM,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::LIMM,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::N,
                        lsb: 22,
                        width: 1,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::immr,
                        lsb: 16,
                        width: 6,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imms,
                        lsb: 10,
                        width: 6,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#and,
            operation: Operation::LOG_IMM(LOG_IMM::AND_Rd_SP_Rn_LIMM(AND_Rd_SP_Rn_LIMM::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for AND_Rd_SP_Rn_LIMM {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl AND_Rd_Rn_Rm_SFT {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "and",
        aliases: &[],
        opcode: 0xa000000,
        mask: 0x7f200000,
        class: InsnClass::LOG_SHIFT,
        feature_set: InsnFeatureSet::V8,
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
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rm_SFT,
                class: InsnOperandClass::MODIFIED_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SF_FIELD.bits()),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#and,
            operation: Operation::LOG_SHIFT(LOG_SHIFT::AND_Rd_Rn_Rm_SFT(AND_Rd_Rn_Rm_SFT::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for AND_Rd_Rn_Rm_SFT {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl AND_Vd_Vn_Vm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "and",
        aliases: &[],
        opcode: 0xe201c00,
        mask: 0xbfe0fc00,
        class: InsnClass::ASIMDSAME,
        feature_set: InsnFeatureSet::SIMD,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Vd,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_8B, InsnOperandQualifier::V_16B],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Vn,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_8B, InsnOperandQualifier::V_16B],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Vm,
                class: InsnOperandClass::SIMD_REG,
                qualifiers: &[InsnOperandQualifier::V_8B, InsnOperandQualifier::V_16B],
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
            mnemonic: Mnemonic::r#and,
            operation: Operation::ASIMDSAME(ASIMDSAME::AND_Vd_Vn_Vm(AND_Vd_Vn_Vm::from(bits))),
        }
    }
}
impl InsnOpcode for AND_Vd_Vn_Vm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ANDS_Rd_Rn_LIMM {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ands",
        aliases: &[],
        opcode: 0x72000000,
        mask: 0x7f800000,
        class: InsnClass::LOG_IMM,
        feature_set: InsnFeatureSet::V8,
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
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::LIMM,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::N,
                        lsb: 22,
                        width: 1,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::immr,
                        lsb: 16,
                        width: 6,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imms,
                        lsb: 10,
                        width: 6,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ands,
            operation: Operation::LOG_IMM(LOG_IMM::ANDS_Rd_Rn_LIMM(ANDS_Rd_Rn_LIMM::from(bits))),
        }
    }
}
impl InsnOpcode for ANDS_Rd_Rn_LIMM {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ANDS_Rd_Rn_Rm_SFT {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ands",
        aliases: &[],
        opcode: 0x6a000000,
        mask: 0x7f200000,
        class: InsnClass::LOG_SHIFT,
        feature_set: InsnFeatureSet::V8,
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
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rm_SFT,
                class: InsnOperandClass::MODIFIED_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#ands,
            operation: Operation::LOG_SHIFT(LOG_SHIFT::ANDS_Rd_Rn_Rm_SFT(ANDS_Rd_Rn_Rm_SFT::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for ANDS_Rd_Rn_Rm_SFT {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ASRV_Rd_Rn_Rm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "asrv",
        aliases: &[],
        opcode: 0x1ac02800,
        mask: 0x7fe0fc00,
        class: InsnClass::DP_2SRC,
        feature_set: InsnFeatureSet::V8,
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
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rm,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rm,
                    lsb: 16,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#asrv,
            operation: Operation::DP_2SRC(DP_2SRC::ASRV_Rd_Rn_Rm(ASRV_Rd_Rn_Rm::from(bits))),
        }
    }
}
impl InsnOpcode for ASRV_Rd_Rn_Rm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl AUTDA_Rd_Rn_SP {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "autda",
        aliases: &[],
        opcode: 0xdac11800,
        mask: 0xfffffc00,
        class: InsnClass::DP_1SRC,
        feature_set: InsnFeatureSet::PAC,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
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
            mnemonic: Mnemonic::r#autda,
            operation: Operation::DP_1SRC(DP_1SRC::AUTDA_Rd_Rn_SP(AUTDA_Rd_Rn_SP::from(bits))),
        }
    }
}
impl InsnOpcode for AUTDA_Rd_Rn_SP {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl AUTDB_Rd_Rn_SP {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "autdb",
        aliases: &[],
        opcode: 0xdac11c00,
        mask: 0xfffffc00,
        class: InsnClass::DP_1SRC,
        feature_set: InsnFeatureSet::PAC,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
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
            mnemonic: Mnemonic::r#autdb,
            operation: Operation::DP_1SRC(DP_1SRC::AUTDB_Rd_Rn_SP(AUTDB_Rd_Rn_SP::from(bits))),
        }
    }
}
impl InsnOpcode for AUTDB_Rd_Rn_SP {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl AUTDZA_Rd {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "autdza",
        aliases: &[],
        opcode: 0xdac13be0,
        mask: 0xffffffe0,
        class: InsnClass::DP_1SRC,
        feature_set: InsnFeatureSet::PAC,
        operands: &[InsnOperand {
            kind: InsnOperandKind::Rd,
            class: InsnOperandClass::INT_REG,
            qualifiers: &[InsnOperandQualifier::X],
            bit_fields: &[BitfieldSpec {
                bitfield: InsnBitField::Rd,
                lsb: 0,
                width: 5,
            }],
        }],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#autdza,
            operation: Operation::DP_1SRC(DP_1SRC::AUTDZA_Rd(AUTDZA_Rd::from(bits))),
        }
    }
}
impl InsnOpcode for AUTDZA_Rd {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl AUTDZB_Rd {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "autdzb",
        aliases: &[],
        opcode: 0xdac13fe0,
        mask: 0xffffffe0,
        class: InsnClass::DP_1SRC,
        feature_set: InsnFeatureSet::PAC,
        operands: &[InsnOperand {
            kind: InsnOperandKind::Rd,
            class: InsnOperandClass::INT_REG,
            qualifiers: &[InsnOperandQualifier::X],
            bit_fields: &[BitfieldSpec {
                bitfield: InsnBitField::Rd,
                lsb: 0,
                width: 5,
            }],
        }],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#autdzb,
            operation: Operation::DP_1SRC(DP_1SRC::AUTDZB_Rd(AUTDZB_Rd::from(bits))),
        }
    }
}
impl InsnOpcode for AUTDZB_Rd {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl AUTIA_Rd_Rn_SP {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "autia",
        aliases: &[],
        opcode: 0xdac11000,
        mask: 0xfffffc00,
        class: InsnClass::DP_1SRC,
        feature_set: InsnFeatureSet::PAC,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
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
            mnemonic: Mnemonic::r#autia,
            operation: Operation::DP_1SRC(DP_1SRC::AUTIA_Rd_Rn_SP(AUTIA_Rd_Rn_SP::from(bits))),
        }
    }
}
