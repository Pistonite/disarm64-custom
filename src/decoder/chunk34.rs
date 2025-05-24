#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
impl InsnOpcode for LDST_IMM10 {
    fn definition(&self) -> &'static Insn {
        match self {
            LDST_IMM10::LDRAA_Rt_ADDR_SIMM10(opcode) => opcode.definition(),
            LDST_IMM10::LDRAB_Rt_ADDR_SIMM10(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LDST_IMM10::LDRAA_Rt_ADDR_SIMM10(opcode) => opcode.bits(),
            LDST_IMM10::LDRAB_Rt_ADDR_SIMM10(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LDST_IMM9 {
    fn definition(&self) -> &'static Insn {
        match self {
            LDST_IMM9::LDRB_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::LDRH_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::LDRSB_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::LDRSH_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::LDRSW_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::LDR_Ft_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::LDR_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::STRB_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::STRH_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::STR_Ft_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::STR_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::STZ2G_Rt_SP_X_ADDR_SIMM13_imm_tag(opcode) => opcode.definition(),
            LDST_IMM9::STZG_Rt_SP_X_ADDR_SIMM13_imm_tag(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LDST_IMM9::LDRB_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::LDRH_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::LDRSB_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::LDRSH_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::LDRSW_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::LDR_Ft_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::LDR_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::STRB_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::STRH_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::STR_Ft_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::STR_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::STZ2G_Rt_SP_X_ADDR_SIMM13_imm_tag(opcode) => opcode.bits(),
            LDST_IMM9::STZG_Rt_SP_X_ADDR_SIMM13_imm_tag(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LDST_POS {
    fn definition(&self) -> &'static Insn {
        match self {
            LDST_POS::LDRB_Rt_ADDR_UIMM12(opcode) => opcode.definition(),
            LDST_POS::LDRH_Rt_ADDR_UIMM12(opcode) => opcode.definition(),
            LDST_POS::LDRSB_Rt_ADDR_UIMM12(opcode) => opcode.definition(),
            LDST_POS::LDRSH_Rt_ADDR_UIMM12(opcode) => opcode.definition(),
            LDST_POS::LDRSW_Rt_ADDR_UIMM12(opcode) => opcode.definition(),
            LDST_POS::LDR_Ft_ADDR_UIMM12(opcode) => opcode.definition(),
            LDST_POS::LDR_Rt_ADDR_UIMM12(opcode) => opcode.definition(),
            LDST_POS::STRB_Rt_ADDR_UIMM12(opcode) => opcode.definition(),
            LDST_POS::STRH_Rt_ADDR_UIMM12(opcode) => opcode.definition(),
            LDST_POS::STR_Ft_ADDR_UIMM12(opcode) => opcode.definition(),
            LDST_POS::STR_Rt_ADDR_UIMM12(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LDST_POS::LDRB_Rt_ADDR_UIMM12(opcode) => opcode.bits(),
            LDST_POS::LDRH_Rt_ADDR_UIMM12(opcode) => opcode.bits(),
            LDST_POS::LDRSB_Rt_ADDR_UIMM12(opcode) => opcode.bits(),
            LDST_POS::LDRSH_Rt_ADDR_UIMM12(opcode) => opcode.bits(),
            LDST_POS::LDRSW_Rt_ADDR_UIMM12(opcode) => opcode.bits(),
            LDST_POS::LDR_Ft_ADDR_UIMM12(opcode) => opcode.bits(),
            LDST_POS::LDR_Rt_ADDR_UIMM12(opcode) => opcode.bits(),
            LDST_POS::STRB_Rt_ADDR_UIMM12(opcode) => opcode.bits(),
            LDST_POS::STRH_Rt_ADDR_UIMM12(opcode) => opcode.bits(),
            LDST_POS::STR_Ft_ADDR_UIMM12(opcode) => opcode.bits(),
            LDST_POS::STR_Rt_ADDR_UIMM12(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LDST_REGOFF {
    fn definition(&self) -> &'static Insn {
        match self {
            LDST_REGOFF::LDRB_Rt_ADDR_REGOFF(opcode) => opcode.definition(),
            LDST_REGOFF::LDRH_Rt_ADDR_REGOFF(opcode) => opcode.definition(),
            LDST_REGOFF::LDRSB_Rt_ADDR_REGOFF(opcode) => opcode.definition(),
            LDST_REGOFF::LDRSH_Rt_ADDR_REGOFF(opcode) => opcode.definition(),
            LDST_REGOFF::LDRSW_Rt_ADDR_REGOFF(opcode) => opcode.definition(),
            LDST_REGOFF::LDR_Ft_ADDR_REGOFF(opcode) => opcode.definition(),
            LDST_REGOFF::LDR_Rt_ADDR_REGOFF(opcode) => opcode.definition(),
            LDST_REGOFF::STRB_Rt_ADDR_REGOFF(opcode) => opcode.definition(),
            LDST_REGOFF::STRH_Rt_ADDR_REGOFF(opcode) => opcode.definition(),
            LDST_REGOFF::STR_Ft_ADDR_REGOFF(opcode) => opcode.definition(),
            LDST_REGOFF::STR_Rt_ADDR_REGOFF(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LDST_REGOFF::LDRB_Rt_ADDR_REGOFF(opcode) => opcode.bits(),
            LDST_REGOFF::LDRH_Rt_ADDR_REGOFF(opcode) => opcode.bits(),
            LDST_REGOFF::LDRSB_Rt_ADDR_REGOFF(opcode) => opcode.bits(),
            LDST_REGOFF::LDRSH_Rt_ADDR_REGOFF(opcode) => opcode.bits(),
            LDST_REGOFF::LDRSW_Rt_ADDR_REGOFF(opcode) => opcode.bits(),
            LDST_REGOFF::LDR_Ft_ADDR_REGOFF(opcode) => opcode.bits(),
            LDST_REGOFF::LDR_Rt_ADDR_REGOFF(opcode) => opcode.bits(),
            LDST_REGOFF::STRB_Rt_ADDR_REGOFF(opcode) => opcode.bits(),
            LDST_REGOFF::STRH_Rt_ADDR_REGOFF(opcode) => opcode.bits(),
            LDST_REGOFF::STR_Ft_ADDR_REGOFF(opcode) => opcode.bits(),
            LDST_REGOFF::STR_Rt_ADDR_REGOFF(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LDST_UNPRIV {
    fn definition(&self) -> &'static Insn {
        match self {
            LDST_UNPRIV::STTRB_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNPRIV::STTRH_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNPRIV::STTR_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LDST_UNPRIV::STTRB_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNPRIV::STTRH_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNPRIV::STTR_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LDST_UNSCALED {
    fn definition(&self) -> &'static Insn {
        match self {
            LDST_UNSCALED::LDURB_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::LDURH_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::LDURSB_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::LDURSH_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::LDURSW_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::LDUR_Ft_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::LDUR_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::STURB_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::STURH_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::STUR_Ft_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::STUR_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::STZ2G_Rt_SP_ADDR_SIMM13(opcode) => opcode.definition(),
            LDST_UNSCALED::STZG_Rt_SP_ADDR_SIMM13(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LDST_UNSCALED::LDURB_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::LDURH_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::LDURSB_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::LDURSH_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::LDURSW_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::LDUR_Ft_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::LDUR_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::STURB_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::STURH_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::STUR_Ft_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::STUR_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::STZ2G_Rt_SP_ADDR_SIMM13(opcode) => opcode.bits(),
            LDST_UNSCALED::STZG_Rt_SP_ADDR_SIMM13(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LOADLIT {
    fn definition(&self) -> &'static Insn {
        match self {
            LOADLIT::LDRSW_Rt_ADDR_PCREL19(opcode) => opcode.definition(),
            LOADLIT::LDR_Ft_ADDR_PCREL19(opcode) => opcode.definition(),
            LOADLIT::LDR_Rt_ADDR_PCREL19(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LOADLIT::LDRSW_Rt_ADDR_PCREL19(opcode) => opcode.bits(),
            LOADLIT::LDR_Ft_ADDR_PCREL19(opcode) => opcode.bits(),
            LOADLIT::LDR_Rt_ADDR_PCREL19(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LOG_IMM {
    fn definition(&self) -> &'static Insn {
        match self {
            LOG_IMM::ANDS_Rd_Rn_LIMM(opcode) => opcode.definition(),
            LOG_IMM::AND_Rd_SP_Rn_LIMM(opcode) => opcode.definition(),
            LOG_IMM::EOR_Rd_SP_Rn_LIMM(opcode) => opcode.definition(),
            LOG_IMM::ORR_Rd_SP_Rn_LIMM(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LOG_IMM::ANDS_Rd_Rn_LIMM(opcode) => opcode.bits(),
            LOG_IMM::AND_Rd_SP_Rn_LIMM(opcode) => opcode.bits(),
            LOG_IMM::EOR_Rd_SP_Rn_LIMM(opcode) => opcode.bits(),
            LOG_IMM::ORR_Rd_SP_Rn_LIMM(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LOG_SHIFT {
    fn definition(&self) -> &'static Insn {
        match self {
            LOG_SHIFT::ANDS_Rd_Rn_Rm_SFT(opcode) => opcode.definition(),
            LOG_SHIFT::AND_Rd_Rn_Rm_SFT(opcode) => opcode.definition(),
            LOG_SHIFT::BICS_Rd_Rn_Rm_SFT(opcode) => opcode.definition(),
            LOG_SHIFT::BIC_Rd_Rn_Rm_SFT(opcode) => opcode.definition(),
            LOG_SHIFT::EOR_Rd_Rn_Rm_SFT(opcode) => opcode.definition(),
            LOG_SHIFT::ORN_Rd_Rn_Rm_SFT(opcode) => opcode.definition(),
            LOG_SHIFT::ORR_Rd_Rn_Rm_SFT(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LOG_SHIFT::ANDS_Rd_Rn_Rm_SFT(opcode) => opcode.bits(),
            LOG_SHIFT::AND_Rd_Rn_Rm_SFT(opcode) => opcode.bits(),
            LOG_SHIFT::BICS_Rd_Rn_Rm_SFT(opcode) => opcode.bits(),
            LOG_SHIFT::BIC_Rd_Rn_Rm_SFT(opcode) => opcode.bits(),
            LOG_SHIFT::EOR_Rd_Rn_Rm_SFT(opcode) => opcode.bits(),
            LOG_SHIFT::ORN_Rd_Rn_Rm_SFT(opcode) => opcode.bits(),
            LOG_SHIFT::ORR_Rd_Rn_Rm_SFT(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for MOVEWIDE {
    fn definition(&self) -> &'static Insn {
        match self {
            MOVEWIDE::MOVK_Rd_HALF(opcode) => opcode.definition(),
            MOVEWIDE::MOVN_Rd_HALF(opcode) => opcode.definition(),
            MOVEWIDE::MOVZ_Rd_HALF(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            MOVEWIDE::MOVK_Rd_HALF(opcode) => opcode.bits(),
            MOVEWIDE::MOVN_Rd_HALF(opcode) => opcode.bits(),
            MOVEWIDE::MOVZ_Rd_HALF(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for PCRELADDR {
    fn definition(&self) -> &'static Insn {
        match self {
            PCRELADDR::ADRP_Rd_ADDR_ADRP(opcode) => opcode.definition(),
            PCRELADDR::ADR_Rd_ADDR_PCREL21(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            PCRELADDR::ADRP_Rd_ADDR_ADRP(opcode) => opcode.bits(),
            PCRELADDR::ADR_Rd_ADDR_PCREL21(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for SME2_MOV {
    fn definition(&self) -> &'static Insn {
        match self {
            SME2_MOV::MOVA_SME_ZA_array_off3_0_SME_Znx2(opcode) => opcode.definition(),
            SME2_MOV::MOVA_SME_ZA_array_off3_0_SME_Znx4(opcode) => opcode.definition(),
            SME2_MOV::MOVA_SME_Zdnx2_SME_ZA_array_off3_5(opcode) => opcode.definition(),
            SME2_MOV::MOVA_SME_Zdnx4_SME_ZA_array_off3_5(opcode) => opcode.definition(),
            SME2_MOV::MOV_SME_ZA_array_off3_0_SME_Znx2(opcode) => opcode.definition(),
            SME2_MOV::MOV_SME_ZA_array_off3_0_SME_Znx4(opcode) => opcode.definition(),
            SME2_MOV::MOV_SME_Zdnx2_SME_ZA_array_off3_5(opcode) => opcode.definition(),
            SME2_MOV::MOV_SME_Zdnx4_SME_ZA_array_off3_5(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            SME2_MOV::MOVA_SME_ZA_array_off3_0_SME_Znx2(opcode) => opcode.bits(),
            SME2_MOV::MOVA_SME_ZA_array_off3_0_SME_Znx4(opcode) => opcode.bits(),
            SME2_MOV::MOVA_SME_Zdnx2_SME_ZA_array_off3_5(opcode) => opcode.bits(),
            SME2_MOV::MOVA_SME_Zdnx4_SME_ZA_array_off3_5(opcode) => opcode.bits(),
            SME2_MOV::MOV_SME_ZA_array_off3_0_SME_Znx2(opcode) => opcode.bits(),
            SME2_MOV::MOV_SME_ZA_array_off3_0_SME_Znx4(opcode) => opcode.bits(),
            SME2_MOV::MOV_SME_Zdnx2_SME_ZA_array_off3_5(opcode) => opcode.bits(),
            SME2_MOV::MOV_SME_Zdnx4_SME_ZA_array_off3_5(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for SME2_MOVAZ {
    fn definition(&self) -> &'static Insn {
        match self {
            SME2_MOVAZ::MOVAZ_SME_Zdnx2_SME_ZA_array_vrsb_1(opcode) => opcode.definition(),
            SME2_MOVAZ::MOVAZ_SME_Zdnx2_SME_ZA_array_vrsd_1(opcode) => opcode.definition(),
            SME2_MOVAZ::MOVAZ_SME_Zdnx2_SME_ZA_array_vrsh_1(opcode) => opcode.definition(),
            SME2_MOVAZ::MOVAZ_SME_Zdnx2_SME_ZA_array_vrss_1(opcode) => opcode.definition(),
            SME2_MOVAZ::MOVAZ_SME_Zdnx4_SME_ZA_array_vrsb_2(opcode) => opcode.definition(),
            SME2_MOVAZ::MOVAZ_SME_Zdnx4_SME_ZA_array_vrsd_2(opcode) => opcode.definition(),
            SME2_MOVAZ::MOVAZ_SME_Zdnx4_SME_ZA_array_vrsh_2(opcode) => opcode.definition(),
            SME2_MOVAZ::MOVAZ_SME_Zdnx4_SME_ZA_array_vrss_2(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            SME2_MOVAZ::MOVAZ_SME_Zdnx2_SME_ZA_array_vrsb_1(opcode) => opcode.bits(),
            SME2_MOVAZ::MOVAZ_SME_Zdnx2_SME_ZA_array_vrsd_1(opcode) => opcode.bits(),
            SME2_MOVAZ::MOVAZ_SME_Zdnx2_SME_ZA_array_vrsh_1(opcode) => opcode.bits(),
            SME2_MOVAZ::MOVAZ_SME_Zdnx2_SME_ZA_array_vrss_1(opcode) => opcode.bits(),
            SME2_MOVAZ::MOVAZ_SME_Zdnx4_SME_ZA_array_vrsb_2(opcode) => opcode.bits(),
            SME2_MOVAZ::MOVAZ_SME_Zdnx4_SME_ZA_array_vrsd_2(opcode) => opcode.bits(),
            SME2_MOVAZ::MOVAZ_SME_Zdnx4_SME_ZA_array_vrsh_2(opcode) => opcode.bits(),
            SME2_MOVAZ::MOVAZ_SME_Zdnx4_SME_ZA_array_vrss_2(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for SME_FP_SD {
    fn definition(&self) -> &'static Insn {
        match self {
            SME_FP_SD::FADD_SME_ZA_array_off3_0_SME_Znx2(opcode) => opcode.definition(),
            SME_FP_SD::FADD_SME_ZA_array_off3_0_SME_Znx4(opcode) => opcode.definition(),
            SME_FP_SD::FSUB_SME_ZA_array_off3_0_SME_Znx2(opcode) => opcode.definition(),
            SME_FP_SD::FSUB_SME_ZA_array_off3_0_SME_Znx4(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            SME_FP_SD::FADD_SME_ZA_array_off3_0_SME_Znx2(opcode) => opcode.bits(),
            SME_FP_SD::FADD_SME_ZA_array_off3_0_SME_Znx4(opcode) => opcode.bits(),
            SME_FP_SD::FSUB_SME_ZA_array_off3_0_SME_Znx2(opcode) => opcode.bits(),
            SME_FP_SD::FSUB_SME_ZA_array_off3_0_SME_Znx4(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for SME_INT_SD {
    fn definition(&self) -> &'static Insn {
        match self {
            SME_INT_SD::ADD_SME_ZA_array_off3_0_SME_Znx2(opcode) => opcode.definition(),
            SME_INT_SD::ADD_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2(opcode) => opcode.definition(),
            SME_INT_SD::ADD_SME_ZA_array_off3_0_SME_Znx4(opcode) => opcode.definition(),
            SME_INT_SD::ADD_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4(opcode) => opcode.definition(),
            SME_INT_SD::ADD_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm(opcode) => opcode.definition(),
            SME_INT_SD::ADD_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_S_SME_Zm_S_S(opcode) => {
                opcode.definition()
            }
            SME_INT_SD::SDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2(opcode) => opcode.definition(),
            SME_INT_SD::SDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4(opcode) => opcode.definition(),
            SME_INT_SD::SDOT_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm(opcode) => opcode.definition(),
            SME_INT_SD::SDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_B_SME_Zm_S_B(opcode) => {
                opcode.definition()
            }
            SME_INT_SD::SUB_SME_ZA_array_off3_0_SME_Znx2(opcode) => opcode.definition(),
            SME_INT_SD::SUB_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2(opcode) => opcode.definition(),
            SME_INT_SD::SUB_SME_ZA_array_off3_0_SME_Znx4(opcode) => opcode.definition(),
            SME_INT_SD::SUB_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4(opcode) => opcode.definition(),
            SME_INT_SD::SUB_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm(opcode) => opcode.definition(),
            SME_INT_SD::SUB_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_S_SME_Zm_S_S(opcode) => {
                opcode.definition()
            }
        }
    }
    fn bits(&self) -> u32 {
        match self {
            SME_INT_SD::ADD_SME_ZA_array_off3_0_SME_Znx2(opcode) => opcode.bits(),
            SME_INT_SD::ADD_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2(opcode) => opcode.bits(),
            SME_INT_SD::ADD_SME_ZA_array_off3_0_SME_Znx4(opcode) => opcode.bits(),
            SME_INT_SD::ADD_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4(opcode) => opcode.bits(),
            SME_INT_SD::ADD_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm(opcode) => opcode.bits(),
            SME_INT_SD::ADD_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_S_SME_Zm_S_S(opcode) => {
                opcode.bits()
            }
            SME_INT_SD::SDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2(opcode) => opcode.bits(),
            SME_INT_SD::SDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4(opcode) => opcode.bits(),
            SME_INT_SD::SDOT_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm(opcode) => opcode.bits(),
            SME_INT_SD::SDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_B_SME_Zm_S_B(opcode) => {
                opcode.bits()
            }
            SME_INT_SD::SUB_SME_ZA_array_off3_0_SME_Znx2(opcode) => opcode.bits(),
            SME_INT_SD::SUB_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2(opcode) => opcode.bits(),
            SME_INT_SD::SUB_SME_ZA_array_off3_0_SME_Znx4(opcode) => opcode.bits(),
            SME_INT_SD::SUB_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4(opcode) => opcode.bits(),
            SME_INT_SD::SUB_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm(opcode) => opcode.bits(),
            SME_INT_SD::SUB_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_S_SME_Zm_S_S(opcode) => {
                opcode.bits()
            }
        }
    }
}
impl InsnOpcode for SME_LDR {
    fn definition(&self) -> &'static Insn {
        match self {
            SME_LDR::LDR_SME_ZA_array_off4_SME_ADDR_RI_U4xVL(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            SME_LDR::LDR_SME_ZA_array_off4_SME_ADDR_RI_U4xVL(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for SME_MISC {
    fn definition(&self) -> &'static Insn {
        match self {
            SME_MISC::ADDHA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn(opcode) => opcode.definition(),
            SME_MISC::ADDHA_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn(opcode) => opcode.definition(),
            SME_MISC::ADDSPL_Rd_SP_SVE_Rn_SP_SVE_SIMM6(opcode) => opcode.definition(),
            SME_MISC::ADDSVL_Rd_SP_SVE_Rn_SP_SVE_SIMM6(opcode) => opcode.definition(),
            SME_MISC::ADDVA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn(opcode) => opcode.definition(),
            SME_MISC::ADDVA_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn(opcode) => opcode.definition(),
            SME_MISC::BFCVTN_SVE_Zd_SME_Znx2(opcode) => opcode.definition(),
            SME_MISC::BFCVT_SVE_Zd_SME_Znx2(opcode) => opcode.definition(),
            SME_MISC::BFDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2(opcode) => {
                opcode.definition()
            }
            SME_MISC::BFDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2(opcode) => opcode.definition(),
            SME_MISC::BFDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX2(opcode) => {
                opcode.definition()
            }
            SME_MISC::BFDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4(opcode) => opcode.definition(),
            SME_MISC::BFDOT_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm(opcode) => opcode.definition(),
            SME_MISC::BFDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H(opcode) => {
                opcode.definition()
            }
            SME_MISC::BFMLAL_SME_ZA_array_off2x2_SME_Znx2_SME_Zm_INDEX3_2(opcode) => {
                opcode.definition()
            }
            SME_MISC::BFMLAL_SME_ZA_array_off2x2_SME_Znx2_SME_Zmx2(opcode) => opcode.definition(),
            SME_MISC::BFMLAL_SME_ZA_array_off2x2_SME_Znx4_SME_Zm_INDEX3_2(opcode) => {
                opcode.definition()
            }
            SME_MISC::BFMLAL_SME_ZA_array_off2x2_SME_Znx4_SME_Zmx4(opcode) => opcode.definition(),
            SME_MISC::BFMLAL_SME_ZA_array_off2x2_SVE_ZnxN_SME_Zm(opcode) => opcode.definition(),
            SME_MISC::BFMLAL_SME_ZA_array_off2x2_S_S_SVE_ZnxN_S_H_SME_Zm_S_H(opcode) => {
                opcode.definition()
            }
            SME_MISC::BFMLAL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm(opcode) => opcode.definition(),
            SME_MISC::BFMLAL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm_INDEX3_10(opcode) => {
                opcode.definition()
            }
            SME_MISC::BFMLSL_SME_ZA_array_off2x2_SME_Znx2_SME_Zm_INDEX3_2(opcode) => {
                opcode.definition()
            }
            SME_MISC::BFMLSL_SME_ZA_array_off2x2_SME_Znx2_SME_Zmx2(opcode) => opcode.definition(),
            SME_MISC::BFMLSL_SME_ZA_array_off2x2_SME_Znx4_SME_Zm_INDEX3_2(opcode) => {
                opcode.definition()
            }
            SME_MISC::BFMLSL_SME_ZA_array_off2x2_SME_Znx4_SME_Zmx4(opcode) => opcode.definition(),
            SME_MISC::BFMLSL_SME_ZA_array_off2x2_SVE_ZnxN_SME_Zm(opcode) => opcode.definition(),
            SME_MISC::BFMLSL_SME_ZA_array_off2x2_S_S_SVE_ZnxN_S_H_SME_Zm_S_H(opcode) => {
                opcode.definition()
            }
            SME_MISC::BFMLSL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm(opcode) => opcode.definition(),
            SME_MISC::BFMLSL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm_INDEX3_10(opcode) => {
                opcode.definition()
            }
            SME_MISC::BFMOPA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(opcode) => {
                opcode.definition()
            }
            SME_MISC::BFMOPS_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(opcode) => {
                opcode.definition()
            }
            SME_MISC::BFVDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2(opcode) => {
                opcode.definition()
            }
            SME_MISC::FDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2(opcode) => {
                opcode.definition()
            }
            SME_MISC::FDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2(opcode) => opcode.definition(),
            SME_MISC::FDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX2(opcode) => {
                opcode.definition()
            }
            SME_MISC::FDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4(opcode) => opcode.definition(),
            SME_MISC::FDOT_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm(opcode) => opcode.definition(),
            SME_MISC::FDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H(opcode) => {
                opcode.definition()
            }
            SME_MISC::FMOPA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(opcode) => {
                opcode.definition()
            }
            SME_MISC::FMOPA_SME_ZAda_2b_S_S_SVE_Pg3_P_M_SME_Pm_P_M_SVE_Zn_S_H_SVE_Zm_16_S_H(
                opcode,
            ) => opcode.definition(),
            SME_MISC::FMOPA_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(opcode) => {
                opcode.definition()
            }
            SME_MISC::FMOPS_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(opcode) => {
                opcode.definition()
            }
            SME_MISC::FMOPS_SME_ZAda_2b_S_S_SVE_Pg3_P_M_SME_Pm_P_M_SVE_Zn_S_H_SVE_Zm_16_S_H(
                opcode,
            ) => opcode.definition(),
            SME_MISC::FMOPS_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(opcode) => {
                opcode.definition()
            }
            SME_MISC::LDR_SME_ZT0_SIMD_ADDR_SIMPLE(opcode) => opcode.definition(),
            SME_MISC::MOVT_Rt_SME_ZT0_INDEX(opcode) => opcode.definition(),
            SME_MISC::MOVT_SME_ZT0_INDEX_Rt(opcode) => opcode.definition(),
            SME_MISC::SDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX1(opcode) => {
                opcode.definition()
            }
            SME_MISC::SDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2(opcode) => {
                opcode.definition()
            }
            SME_MISC::SDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX1(opcode) => {
                opcode.definition()
            }
            SME_MISC::SDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX2(opcode) => {
                opcode.definition()
            }
            SME_MISC::SDOT_SME_ZA_array_off3_0_S_S_SME_Znx2_S_B_SME_Zm_INDEX2_S_B(opcode) => {
                opcode.definition()
            }
            SME_MISC::SDOT_SME_ZA_array_off3_0_S_S_SME_Znx2_S_H_SME_Zmx2_S_H(opcode) => {
                opcode.definition()
            }
            SME_MISC::SDOT_SME_ZA_array_off3_0_S_S_SME_Znx4_S_B_SME_Zm_INDEX2_S_B(opcode) => {
                opcode.definition()
            }
            SME_MISC::SDOT_SME_ZA_array_off3_0_S_S_SME_Znx4_S_H_SME_Zmx4_S_H(opcode) => {
                opcode.definition()
            }
            SME_MISC::SDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H(opcode) => {
                opcode.definition()
            }
            SME_MISC::SDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H_c1701408(opcode) => {
                opcode.definition()
            }
            SME_MISC::STR_SME_ZT0_SIMD_ADDR_SIMPLE(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            SME_MISC::ADDHA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn(opcode) => opcode.bits(),
            SME_MISC::ADDHA_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn(opcode) => opcode.bits(),
            SME_MISC::ADDSPL_Rd_SP_SVE_Rn_SP_SVE_SIMM6(opcode) => opcode.bits(),
            SME_MISC::ADDSVL_Rd_SP_SVE_Rn_SP_SVE_SIMM6(opcode) => opcode.bits(),
            SME_MISC::ADDVA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn(opcode) => opcode.bits(),
            SME_MISC::ADDVA_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn(opcode) => opcode.bits(),
            SME_MISC::BFCVTN_SVE_Zd_SME_Znx2(opcode) => opcode.bits(),
            SME_MISC::BFCVT_SVE_Zd_SME_Znx2(opcode) => opcode.bits(),
            SME_MISC::BFDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2(opcode) => opcode.bits(),
            SME_MISC::BFDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2(opcode) => opcode.bits(),
            SME_MISC::BFDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX2(opcode) => opcode.bits(),
            SME_MISC::BFDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4(opcode) => opcode.bits(),
            SME_MISC::BFDOT_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm(opcode) => opcode.bits(),
            SME_MISC::BFDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H(opcode) => {
                opcode.bits()
            }
            SME_MISC::BFMLAL_SME_ZA_array_off2x2_SME_Znx2_SME_Zm_INDEX3_2(opcode) => opcode.bits(),
            SME_MISC::BFMLAL_SME_ZA_array_off2x2_SME_Znx2_SME_Zmx2(opcode) => opcode.bits(),
            SME_MISC::BFMLAL_SME_ZA_array_off2x2_SME_Znx4_SME_Zm_INDEX3_2(opcode) => opcode.bits(),
            SME_MISC::BFMLAL_SME_ZA_array_off2x2_SME_Znx4_SME_Zmx4(opcode) => opcode.bits(),
            SME_MISC::BFMLAL_SME_ZA_array_off2x2_SVE_ZnxN_SME_Zm(opcode) => opcode.bits(),
            SME_MISC::BFMLAL_SME_ZA_array_off2x2_S_S_SVE_ZnxN_S_H_SME_Zm_S_H(opcode) => {
                opcode.bits()
            }
            SME_MISC::BFMLAL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm(opcode) => opcode.bits(),
            SME_MISC::BFMLAL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm_INDEX3_10(opcode) => opcode.bits(),
            SME_MISC::BFMLSL_SME_ZA_array_off2x2_SME_Znx2_SME_Zm_INDEX3_2(opcode) => opcode.bits(),
            SME_MISC::BFMLSL_SME_ZA_array_off2x2_SME_Znx2_SME_Zmx2(opcode) => opcode.bits(),
            SME_MISC::BFMLSL_SME_ZA_array_off2x2_SME_Znx4_SME_Zm_INDEX3_2(opcode) => opcode.bits(),
            SME_MISC::BFMLSL_SME_ZA_array_off2x2_SME_Znx4_SME_Zmx4(opcode) => opcode.bits(),
            SME_MISC::BFMLSL_SME_ZA_array_off2x2_SVE_ZnxN_SME_Zm(opcode) => opcode.bits(),
            SME_MISC::BFMLSL_SME_ZA_array_off2x2_S_S_SVE_ZnxN_S_H_SME_Zm_S_H(opcode) => {
                opcode.bits()
            }
            SME_MISC::BFMLSL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm(opcode) => opcode.bits(),
            SME_MISC::BFMLSL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm_INDEX3_10(opcode) => opcode.bits(),
            SME_MISC::BFMOPA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SME_MISC::BFMOPS_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SME_MISC::BFVDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2(opcode) => opcode.bits(),
            SME_MISC::FDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2(opcode) => opcode.bits(),
            SME_MISC::FDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2(opcode) => opcode.bits(),
            SME_MISC::FDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX2(opcode) => opcode.bits(),
            SME_MISC::FDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4(opcode) => opcode.bits(),
            SME_MISC::FDOT_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm(opcode) => opcode.bits(),
            SME_MISC::FDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H(opcode) => opcode.bits(),
            SME_MISC::FMOPA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SME_MISC::FMOPA_SME_ZAda_2b_S_S_SVE_Pg3_P_M_SME_Pm_P_M_SVE_Zn_S_H_SVE_Zm_16_S_H(
                opcode,
            ) => opcode.bits(),
            SME_MISC::FMOPA_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SME_MISC::FMOPS_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SME_MISC::FMOPS_SME_ZAda_2b_S_S_SVE_Pg3_P_M_SME_Pm_P_M_SVE_Zn_S_H_SVE_Zm_16_S_H(
                opcode,
            ) => opcode.bits(),
            SME_MISC::FMOPS_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SME_MISC::LDR_SME_ZT0_SIMD_ADDR_SIMPLE(opcode) => opcode.bits(),
            SME_MISC::MOVT_Rt_SME_ZT0_INDEX(opcode) => opcode.bits(),
            SME_MISC::MOVT_SME_ZT0_INDEX_Rt(opcode) => opcode.bits(),
            SME_MISC::SDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX1(opcode) => opcode.bits(),
            SME_MISC::SDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2(opcode) => opcode.bits(),
            SME_MISC::SDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX1(opcode) => opcode.bits(),
            SME_MISC::SDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX2(opcode) => opcode.bits(),
            SME_MISC::SDOT_SME_ZA_array_off3_0_S_S_SME_Znx2_S_B_SME_Zm_INDEX2_S_B(opcode) => {
                opcode.bits()
            }
            SME_MISC::SDOT_SME_ZA_array_off3_0_S_S_SME_Znx2_S_H_SME_Zmx2_S_H(opcode) => {
                opcode.bits()
            }
            SME_MISC::SDOT_SME_ZA_array_off3_0_S_S_SME_Znx4_S_B_SME_Zm_INDEX2_S_B(opcode) => {
                opcode.bits()
            }
            SME_MISC::SDOT_SME_ZA_array_off3_0_S_S_SME_Znx4_S_H_SME_Zmx4_S_H(opcode) => {
                opcode.bits()
            }
            SME_MISC::SDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H(opcode) => opcode.bits(),
            SME_MISC::SDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H_c1701408(opcode) => {
                opcode.bits()
            }
            SME_MISC::STR_SME_ZT0_SIMD_ADDR_SIMPLE(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for SME_MOV {
    fn definition(&self) -> &'static Insn {
        match self {
            SME_MOV::MOVA_SME_ZA_HV_idx_dest_SVE_Pg3_SVE_Zn(opcode) => opcode.definition(),
            SME_MOV::MOVA_SVE_Zd_SVE_Pg3_SME_ZA_HV_idx_src(opcode) => opcode.definition(),
            SME_MOV::MOV_SME_ZA_HV_idx_dest_SVE_Pg3_SVE_Zn(opcode) => opcode.definition(),
            SME_MOV::MOV_SVE_Zd_SVE_Pg3_SME_ZA_HV_idx_src(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            SME_MOV::MOVA_SME_ZA_HV_idx_dest_SVE_Pg3_SVE_Zn(opcode) => opcode.bits(),
            SME_MOV::MOVA_SVE_Zd_SVE_Pg3_SME_ZA_HV_idx_src(opcode) => opcode.bits(),
            SME_MOV::MOV_SME_ZA_HV_idx_dest_SVE_Pg3_SVE_Zn(opcode) => opcode.bits(),
            SME_MOV::MOV_SVE_Zd_SVE_Pg3_SME_ZA_HV_idx_src(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for SME_SIZE_22 {
    fn definition(&self) -> &'static Insn {
        match self {
            SME_SIZE_22::ADD_SME_Zdnx2_SME_Zdnx2_SME_Zm(opcode) => opcode.definition(),
            SME_SIZE_22::ADD_SME_Zdnx4_SME_Zdnx4_SME_Zm(opcode) => opcode.definition(),
            SME_SIZE_22::CNTP_Rd_SME_PNn_SME_VLxN_10(opcode) => opcode.definition(),
            SME_SIZE_22::MOVA_SME_ZA_HV_idx_destxN_SME_Znx2(opcode) => opcode.definition(),
            SME_SIZE_22::MOVA_SME_ZA_HV_idx_destxN_SME_Znx4(opcode) => opcode.definition(),
            SME_SIZE_22::MOVA_SME_Zdnx2_SME_ZA_HV_idx_srcxN(opcode) => opcode.definition(),
            SME_SIZE_22::MOVA_SME_Zdnx4_SME_ZA_HV_idx_srcxN(opcode) => opcode.definition(),
            SME_SIZE_22::MOV_SME_ZA_HV_idx_destxN_SME_Znx2(opcode) => opcode.definition(),
            SME_SIZE_22::MOV_SME_ZA_HV_idx_destxN_SME_Znx4(opcode) => opcode.definition(),
            SME_SIZE_22::MOV_SME_Zdnx2_SME_ZA_HV_idx_srcxN(opcode) => opcode.definition(),
            SME_SIZE_22::MOV_SME_Zdnx4_SME_ZA_HV_idx_srcxN(opcode) => opcode.definition(),
            SME_SIZE_22::UCLAMP_SME_Zdnx2_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SME_SIZE_22::UCLAMP_SME_Zdnx4_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            SME_SIZE_22::ADD_SME_Zdnx2_SME_Zdnx2_SME_Zm(opcode) => opcode.bits(),
            SME_SIZE_22::ADD_SME_Zdnx4_SME_Zdnx4_SME_Zm(opcode) => opcode.bits(),
            SME_SIZE_22::CNTP_Rd_SME_PNn_SME_VLxN_10(opcode) => opcode.bits(),
            SME_SIZE_22::MOVA_SME_ZA_HV_idx_destxN_SME_Znx2(opcode) => opcode.bits(),
            SME_SIZE_22::MOVA_SME_ZA_HV_idx_destxN_SME_Znx4(opcode) => opcode.bits(),
            SME_SIZE_22::MOVA_SME_Zdnx2_SME_ZA_HV_idx_srcxN(opcode) => opcode.bits(),
            SME_SIZE_22::MOVA_SME_Zdnx4_SME_ZA_HV_idx_srcxN(opcode) => opcode.bits(),
            SME_SIZE_22::MOV_SME_ZA_HV_idx_destxN_SME_Znx2(opcode) => opcode.bits(),
            SME_SIZE_22::MOV_SME_ZA_HV_idx_destxN_SME_Znx4(opcode) => opcode.bits(),
            SME_SIZE_22::MOV_SME_Zdnx2_SME_ZA_HV_idx_srcxN(opcode) => opcode.bits(),
            SME_SIZE_22::MOV_SME_Zdnx4_SME_ZA_HV_idx_srcxN(opcode) => opcode.bits(),
            SME_SIZE_22::UCLAMP_SME_Zdnx2_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SME_SIZE_22::UCLAMP_SME_Zdnx4_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for SME_SIZE_22_HSD {
    fn definition(&self) -> &'static Insn {
        match self {
            SME_SIZE_22_HSD::FCLAMP_SME_Zdnx2_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SME_SIZE_22_HSD::FCLAMP_SME_Zdnx4_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SME_SIZE_22_HSD::FCLAMP_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            SME_SIZE_22_HSD::FCLAMP_SME_Zdnx2_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SME_SIZE_22_HSD::FCLAMP_SME_Zdnx4_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SME_SIZE_22_HSD::FCLAMP_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for SME_STR {
    fn definition(&self) -> &'static Insn {
        match self {
            SME_STR::STR_SME_ZA_array_off4_SME_ADDR_RI_U4xVL(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            SME_STR::STR_SME_ZA_array_off4_SME_ADDR_RI_U4xVL(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for SVE2_URQVS {
    fn definition(&self) -> &'static Insn {
        match self {
            SVE2_URQVS::ADDQV_Vd_SVE_Pg3_SVE_Zn(opcode) => opcode.definition(),
            SVE2_URQVS::ANDQV_Vd_SVE_Pg3_SVE_Zn(opcode) => opcode.definition(),
            SVE2_URQVS::EORQV_Vd_SVE_Pg3_SVE_Zn(opcode) => opcode.definition(),
            SVE2_URQVS::FADDQV_Vd_SVE_Pg3_SVE_Zn(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            SVE2_URQVS::ADDQV_Vd_SVE_Pg3_SVE_Zn(opcode) => opcode.bits(),
            SVE2_URQVS::ANDQV_Vd_SVE_Pg3_SVE_Zn(opcode) => opcode.bits(),
            SVE2_URQVS::EORQV_Vd_SVE_Pg3_SVE_Zn(opcode) => opcode.bits(),
            SVE2_URQVS::FADDQV_Vd_SVE_Pg3_SVE_Zn(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for SVE_LIMM {
    fn definition(&self) -> &'static Insn {
        match self {
            SVE_LIMM::AND_SVE_Zd_SVE_Zd_SVE_LIMM(opcode) => opcode.definition(),
            SVE_LIMM::EOR_SVE_Zd_SVE_Zd_SVE_LIMM(opcode) => opcode.definition(),
            SVE_LIMM::ORR_SVE_Zd_SVE_Zd_SVE_LIMM(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            SVE_LIMM::AND_SVE_Zd_SVE_Zd_SVE_LIMM(opcode) => opcode.bits(),
            SVE_LIMM::EOR_SVE_Zd_SVE_Zd_SVE_LIMM(opcode) => opcode.bits(),
            SVE_LIMM::ORR_SVE_Zd_SVE_Zd_SVE_LIMM(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for SVE_MISC {
    fn definition(&self) -> &'static Insn {
        match self {
            SVE_MISC::ADDPL_Rd_SP_SVE_Rn_SP_SVE_SIMM6(opcode) => opcode.definition(),
            SVE_MISC::ADDVL_Rd_SP_SVE_Rn_SP_SVE_SIMM6(opcode) => opcode.definition(),
            SVE_MISC::ADR_SVE_Zd_SVE_ADDR_ZZ_SXTW(opcode) => opcode.definition(),
            SVE_MISC::ADR_SVE_Zd_SVE_ADDR_ZZ_UXTW(opcode) => opcode.definition(),
            SVE_MISC::ANDS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(opcode) => opcode.definition(),
            SVE_MISC::AND_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(opcode) => opcode.definition(),
            SVE_MISC::AND_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_MISC::BFADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_MISC::BFADD_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_MISC::BFCLAMP_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_MISC::BFCVTNT_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.definition(),
            SVE_MISC::BFCVT_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.definition(),
            SVE_MISC::BFDOT_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX(opcode) => opcode.definition(),
            SVE_MISC::BFDOT_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_MISC::BFMAXNM_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_MISC::BFMAX_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_MISC::BFMINNM_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_MISC::BFMIN_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_MISC::BFMLALB_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX(opcode) => opcode.definition(),
            SVE_MISC::BFMLALB_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_MISC::BFMLALT_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX(opcode) => opcode.definition(),
            SVE_MISC::BFMLALT_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_MISC::BFMLA_SVE_Zd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_MISC::BFMLA_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX(opcode) => opcode.definition(),
            SVE_MISC::BFMLSLB_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX(opcode) => opcode.definition(),
            SVE_MISC::BFMLSLB_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_MISC::BFMLSLT_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX(opcode) => opcode.definition(),
            SVE_MISC::BFMLSLT_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_MISC::BFMLS_SVE_Zd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_MISC::BFMLS_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX(opcode) => opcode.definition(),
            SVE_MISC::BFMMLA_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_MISC::BFMUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_MISC::BFMUL_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX(opcode) => opcode.definition(),
            SVE_MISC::BFMUL_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_MISC::BFSUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_MISC::BFSUB_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_MISC::BICS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(opcode) => opcode.definition(),
            SVE_MISC::BIC_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(opcode) => opcode.definition(),
            SVE_MISC::BIC_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_MISC::BRKAS_SVE_Pd_SVE_Pg4_10_SVE_Pn(opcode) => opcode.definition(),
            SVE_MISC::BRKBS_SVE_Pd_SVE_Pg4_10_SVE_Pn(opcode) => opcode.definition(),
            SVE_MISC::BRKNS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pd(opcode) => opcode.definition(),
            SVE_MISC::BRKN_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pd(opcode) => opcode.definition(),
            SVE_MISC::BRKPAS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(opcode) => opcode.definition(),
            SVE_MISC::BRKPA_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(opcode) => opcode.definition(),
            SVE_MISC::BRKPBS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(opcode) => opcode.definition(),
            SVE_MISC::BRKPB_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(opcode) => opcode.definition(),
            SVE_MISC::CDOT_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX_SVE_IMM_ROT2(opcode) => opcode.definition(),
            SVE_MISC::CDOT_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX_SVE_IMM_ROT2(opcode) => opcode.definition(),
            SVE_MISC::CMLA_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX_SVE_IMM_ROT2(opcode) => opcode.definition(),
            SVE_MISC::CMLA_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX_SVE_IMM_ROT2(opcode) => opcode.definition(),
            SVE_MISC::CNTB_Rd_SVE_PATTERN_SCALED(opcode) => opcode.definition(),
            SVE_MISC::CNTD_Rd_SVE_PATTERN_SCALED(opcode) => opcode.definition(),
            SVE_MISC::CNTH_Rd_SVE_PATTERN_SCALED(opcode) => opcode.definition(),
            SVE_MISC::CNTW_Rd_SVE_PATTERN_SCALED(opcode) => opcode.definition(),
            SVE_MISC::EOR3_SVE_Zd_SVE_Zd_SVE_Zm_16_SVE_Zn(opcode) => opcode.definition(),
            SVE_MISC::EORS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(opcode) => opcode.definition(),
            SVE_MISC::EOR_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(opcode) => opcode.definition(),
            SVE_MISC::EOR_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_MISC::FCMLA_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX_SVE_IMM_ROT2(opcode) => opcode.definition(),
            SVE_MISC::FCMLA_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX_SVE_IMM_ROT2(opcode) => opcode.definition(),
            SVE_MISC::FCVTLT_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.definition(),
            SVE_MISC::FCVTLT_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_S(opcode) => opcode.definition(),
            SVE_MISC::FCVTNT_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.definition(),
            SVE_MISC::FCVTNT_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_D(opcode) => opcode.definition(),
            SVE_MISC::FCVTN_SVE_Zd_SME_Znx2(opcode) => opcode.definition(),
            SVE_MISC::FCVTXNT_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.definition(),
            SVE_MISC::FCVTX_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.definition(),
            SVE_MISC::FCVTZS_SME_Zdnx2_SME_Znx2(opcode) => opcode.definition(),
            SVE_MISC::FCVTZS_SME_Zdnx4_SME_Znx4(opcode) => opcode.definition(),
            SVE_MISC::FCVTZS_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.definition(),
            SVE_MISC::FCVTZS_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_D(opcode) => opcode.definition(),
            SVE_MISC::FCVTZS_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_H(opcode) => opcode.definition(),
            SVE_MISC::FCVTZS_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_S(opcode) => opcode.definition(),
            SVE_MISC::FCVTZS_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_H(opcode) => opcode.definition(),
            SVE_MISC::FCVTZS_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_H(opcode) => opcode.definition(),
            SVE_MISC::FCVTZS_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_S(opcode) => opcode.definition(),
            SVE_MISC::FCVTZU_SME_Zdnx2_SME_Znx2(opcode) => opcode.definition(),
            SVE_MISC::FCVTZU_SME_Zdnx4_SME_Znx4(opcode) => opcode.definition(),
            SVE_MISC::FCVTZU_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.definition(),
            SVE_MISC::FCVTZU_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_D(opcode) => opcode.definition(),
            SVE_MISC::FCVTZU_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_H(opcode) => opcode.definition(),
            SVE_MISC::FCVTZU_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_S(opcode) => opcode.definition(),
            SVE_MISC::FCVTZU_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_H(opcode) => opcode.definition(),
            SVE_MISC::FCVTZU_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_H(opcode) => opcode.definition(),
            SVE_MISC::FCVTZU_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_S(opcode) => opcode.definition(),
            SVE_MISC::FCVT_SVE_Zd_SME_Znx2(opcode) => opcode.definition(),
            SVE_MISC::FCVT_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.definition(),
            SVE_MISC::FCVT_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_H(opcode) => opcode.definition(),
            SVE_MISC::FCVT_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_S(opcode) => opcode.definition(),
            SVE_MISC::FCVT_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_D(opcode) => opcode.definition(),
            SVE_MISC::FCVT_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_D(opcode) => opcode.definition(),
            SVE_MISC::FCVT_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_H(opcode) => opcode.definition(),
            SVE_MISC::FDOT_SVE_Zd_SVE_Zn_SVE_Zm3_19_INDEX(opcode) => opcode.definition(),
            SVE_MISC::FDOT_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_MISC::FMUL_SVE_Zd_SVE_Zn_SVE_Zm3_22_INDEX(opcode) => opcode.definition(),
            SVE_MISC::FMUL_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX(opcode) => opcode.definition(),
            SVE_MISC::FMUL_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX(opcode) => opcode.definition(),
            SVE_MISC::LDR_SVE_PNt_SVE_ADDR_RI_S9xVL(opcode) => opcode.definition(),
            SVE_MISC::LDR_SVE_Pt_SVE_ADDR_RI_S9xVL(opcode) => opcode.definition(),
            SVE_MISC::LDR_SVE_Zt_SVE_ADDR_RI_S9xVL(opcode) => opcode.definition(),
            SVE_MISC::MOVPRFX_SVE_Zd_SVE_Zn(opcode) => opcode.definition(),
            SVE_MISC::MUL_SVE_Zd_SVE_Zn_SVE_Zm3_22_INDEX(opcode) => opcode.definition(),
            SVE_MISC::MUL_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX(opcode) => opcode.definition(),
            SVE_MISC::MUL_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX(opcode) => opcode.definition(),
            SVE_MISC::ORNS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(opcode) => opcode.definition(),
            SVE_MISC::ORN_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(opcode) => opcode.definition(),
            SVE_MISC::ORRS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(opcode) => opcode.definition(),
            SVE_MISC::ORR_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(opcode) => opcode.definition(),
            SVE_MISC::ORR_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_MISC::SCVTF_SME_Zdnx2_SME_Znx2(opcode) => opcode.definition(),
            SVE_MISC::SCVTF_SME_Zdnx4_SME_Znx4(opcode) => opcode.definition(),
            SVE_MISC::SCVTF_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.definition(),
            SVE_MISC::SCVTF_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_D(opcode) => opcode.definition(),
            SVE_MISC::SCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_D(opcode) => opcode.definition(),
            SVE_MISC::SCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_H(opcode) => opcode.definition(),
            SVE_MISC::SCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_S(opcode) => opcode.definition(),
            SVE_MISC::SCVTF_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_D(opcode) => opcode.definition(),
            SVE_MISC::SCVTF_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_S(opcode) => opcode.definition(),
            SVE_MISC::SDOT_SVE_Zd_SVE_Zn_SVE_Zm3_19_INDEX(opcode) => opcode.definition(),
            SVE_MISC::SDOT_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX(opcode) => opcode.definition(),
            SVE_MISC::SDOT_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX(opcode) => opcode.definition(),
            SVE_MISC::SDOT_SVE_Zd_S_S_SVE_Zn_S_H_SVE_Zm_16_S_H(opcode) => opcode.definition(),
            SVE_MISC::SMULLB_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX(opcode) => opcode.definition(),
            SVE_MISC::SMULLB_SVE_Zd_SVE_Zn_SVE_Zm4_11_INDEX(opcode) => opcode.definition(),
            SVE_MISC::SMULLT_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX(opcode) => opcode.definition(),
            SVE_MISC::SMULLT_SVE_Zd_SVE_Zn_SVE_Zm4_11_INDEX(opcode) => opcode.definition(),
            SVE_MISC::STR_SVE_PNt_SVE_ADDR_RI_S9xVL(opcode) => opcode.definition(),
            SVE_MISC::STR_SVE_Pt_SVE_ADDR_RI_S9xVL(opcode) => opcode.definition(),
            SVE_MISC::STR_SVE_Zt_SVE_ADDR_RI_S9xVL(opcode) => opcode.definition(),
            SVE_MISC::SXTW_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.definition(),
            SVE_MISC::TRN1_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_MISC::TRN2_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_MISC::UCVTF_SME_Zdnx2_SME_Znx2(opcode) => opcode.definition(),
            SVE_MISC::UCVTF_SME_Zdnx4_SME_Znx4(opcode) => opcode.definition(),
            SVE_MISC::UCVTF_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.definition(),
            SVE_MISC::UCVTF_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_D(opcode) => opcode.definition(),
            SVE_MISC::UCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_D(opcode) => opcode.definition(),
            SVE_MISC::UCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_H(opcode) => opcode.definition(),
            SVE_MISC::UCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_S(opcode) => opcode.definition(),
            SVE_MISC::UCVTF_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_D(opcode) => opcode.definition(),
            SVE_MISC::UCVTF_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_S(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            SVE_MISC::ADDPL_Rd_SP_SVE_Rn_SP_SVE_SIMM6(opcode) => opcode.bits(),
            SVE_MISC::ADDVL_Rd_SP_SVE_Rn_SP_SVE_SIMM6(opcode) => opcode.bits(),
            SVE_MISC::ADR_SVE_Zd_SVE_ADDR_ZZ_SXTW(opcode) => opcode.bits(),
            SVE_MISC::ADR_SVE_Zd_SVE_ADDR_ZZ_UXTW(opcode) => opcode.bits(),
            SVE_MISC::ANDS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(opcode) => opcode.bits(),
            SVE_MISC::AND_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(opcode) => opcode.bits(),
            SVE_MISC::AND_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_MISC::BFADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_MISC::BFADD_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_MISC::BFCLAMP_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_MISC::BFCVTNT_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.bits(),
            SVE_MISC::BFCVT_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.bits(),
            SVE_MISC::BFDOT_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX(opcode) => opcode.bits(),
            SVE_MISC::BFDOT_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_MISC::BFMAXNM_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_MISC::BFMAX_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_MISC::BFMINNM_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_MISC::BFMIN_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_MISC::BFMLALB_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX(opcode) => opcode.bits(),
            SVE_MISC::BFMLALB_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_MISC::BFMLALT_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX(opcode) => opcode.bits(),
            SVE_MISC::BFMLALT_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_MISC::BFMLA_SVE_Zd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_MISC::BFMLA_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX(opcode) => opcode.bits(),
            SVE_MISC::BFMLSLB_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX(opcode) => opcode.bits(),
            SVE_MISC::BFMLSLB_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_MISC::BFMLSLT_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX(opcode) => opcode.bits(),
            SVE_MISC::BFMLSLT_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_MISC::BFMLS_SVE_Zd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_MISC::BFMLS_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX(opcode) => opcode.bits(),
            SVE_MISC::BFMMLA_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_MISC::BFMUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_MISC::BFMUL_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX(opcode) => opcode.bits(),
            SVE_MISC::BFMUL_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_MISC::BFSUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_MISC::BFSUB_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_MISC::BICS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(opcode) => opcode.bits(),
            SVE_MISC::BIC_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(opcode) => opcode.bits(),
            SVE_MISC::BIC_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_MISC::BRKAS_SVE_Pd_SVE_Pg4_10_SVE_Pn(opcode) => opcode.bits(),
            SVE_MISC::BRKBS_SVE_Pd_SVE_Pg4_10_SVE_Pn(opcode) => opcode.bits(),
            SVE_MISC::BRKNS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pd(opcode) => opcode.bits(),
            SVE_MISC::BRKN_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pd(opcode) => opcode.bits(),
            SVE_MISC::BRKPAS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(opcode) => opcode.bits(),
            SVE_MISC::BRKPA_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(opcode) => opcode.bits(),
            SVE_MISC::BRKPBS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(opcode) => opcode.bits(),
            SVE_MISC::BRKPB_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(opcode) => opcode.bits(),
            SVE_MISC::CDOT_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX_SVE_IMM_ROT2(opcode) => opcode.bits(),
            SVE_MISC::CDOT_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX_SVE_IMM_ROT2(opcode) => opcode.bits(),
            SVE_MISC::CMLA_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX_SVE_IMM_ROT2(opcode) => opcode.bits(),
            SVE_MISC::CMLA_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX_SVE_IMM_ROT2(opcode) => opcode.bits(),
            SVE_MISC::CNTB_Rd_SVE_PATTERN_SCALED(opcode) => opcode.bits(),
            SVE_MISC::CNTD_Rd_SVE_PATTERN_SCALED(opcode) => opcode.bits(),
            SVE_MISC::CNTH_Rd_SVE_PATTERN_SCALED(opcode) => opcode.bits(),
            SVE_MISC::CNTW_Rd_SVE_PATTERN_SCALED(opcode) => opcode.bits(),
            SVE_MISC::EOR3_SVE_Zd_SVE_Zd_SVE_Zm_16_SVE_Zn(opcode) => opcode.bits(),
            SVE_MISC::EORS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(opcode) => opcode.bits(),
            SVE_MISC::EOR_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(opcode) => opcode.bits(),
            SVE_MISC::EOR_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_MISC::FCMLA_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX_SVE_IMM_ROT2(opcode) => opcode.bits(),
            SVE_MISC::FCMLA_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX_SVE_IMM_ROT2(opcode) => opcode.bits(),
            SVE_MISC::FCVTLT_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.bits(),
            SVE_MISC::FCVTLT_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_S(opcode) => opcode.bits(),
            SVE_MISC::FCVTNT_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.bits(),
            SVE_MISC::FCVTNT_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_D(opcode) => opcode.bits(),
            SVE_MISC::FCVTN_SVE_Zd_SME_Znx2(opcode) => opcode.bits(),
            SVE_MISC::FCVTXNT_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.bits(),
            SVE_MISC::FCVTX_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.bits(),
            SVE_MISC::FCVTZS_SME_Zdnx2_SME_Znx2(opcode) => opcode.bits(),
            SVE_MISC::FCVTZS_SME_Zdnx4_SME_Znx4(opcode) => opcode.bits(),
            SVE_MISC::FCVTZS_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.bits(),
            SVE_MISC::FCVTZS_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_D(opcode) => opcode.bits(),
            SVE_MISC::FCVTZS_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_H(opcode) => opcode.bits(),
            SVE_MISC::FCVTZS_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_S(opcode) => opcode.bits(),
            SVE_MISC::FCVTZS_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_H(opcode) => opcode.bits(),
            SVE_MISC::FCVTZS_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_H(opcode) => opcode.bits(),
            SVE_MISC::FCVTZS_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_S(opcode) => opcode.bits(),
            SVE_MISC::FCVTZU_SME_Zdnx2_SME_Znx2(opcode) => opcode.bits(),
            SVE_MISC::FCVTZU_SME_Zdnx4_SME_Znx4(opcode) => opcode.bits(),
            SVE_MISC::FCVTZU_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.bits(),
            SVE_MISC::FCVTZU_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_D(opcode) => opcode.bits(),
            SVE_MISC::FCVTZU_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_H(opcode) => opcode.bits(),
            SVE_MISC::FCVTZU_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_S(opcode) => opcode.bits(),
            SVE_MISC::FCVTZU_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_H(opcode) => opcode.bits(),
            SVE_MISC::FCVTZU_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_H(opcode) => opcode.bits(),
            SVE_MISC::FCVTZU_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_S(opcode) => opcode.bits(),
            SVE_MISC::FCVT_SVE_Zd_SME_Znx2(opcode) => opcode.bits(),
            SVE_MISC::FCVT_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.bits(),
            SVE_MISC::FCVT_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_H(opcode) => opcode.bits(),
            SVE_MISC::FCVT_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_S(opcode) => opcode.bits(),
            SVE_MISC::FCVT_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_D(opcode) => opcode.bits(),
            SVE_MISC::FCVT_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_D(opcode) => opcode.bits(),
            SVE_MISC::FCVT_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_H(opcode) => opcode.bits(),
            SVE_MISC::FDOT_SVE_Zd_SVE_Zn_SVE_Zm3_19_INDEX(opcode) => opcode.bits(),
            SVE_MISC::FDOT_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_MISC::FMUL_SVE_Zd_SVE_Zn_SVE_Zm3_22_INDEX(opcode) => opcode.bits(),
            SVE_MISC::FMUL_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX(opcode) => opcode.bits(),
            SVE_MISC::FMUL_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX(opcode) => opcode.bits(),
            SVE_MISC::LDR_SVE_PNt_SVE_ADDR_RI_S9xVL(opcode) => opcode.bits(),
            SVE_MISC::LDR_SVE_Pt_SVE_ADDR_RI_S9xVL(opcode) => opcode.bits(),
            SVE_MISC::LDR_SVE_Zt_SVE_ADDR_RI_S9xVL(opcode) => opcode.bits(),
            SVE_MISC::MOVPRFX_SVE_Zd_SVE_Zn(opcode) => opcode.bits(),
            SVE_MISC::MUL_SVE_Zd_SVE_Zn_SVE_Zm3_22_INDEX(opcode) => opcode.bits(),
            SVE_MISC::MUL_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX(opcode) => opcode.bits(),
            SVE_MISC::MUL_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX(opcode) => opcode.bits(),
            SVE_MISC::ORNS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(opcode) => opcode.bits(),
            SVE_MISC::ORN_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(opcode) => opcode.bits(),
            SVE_MISC::ORRS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(opcode) => opcode.bits(),
            SVE_MISC::ORR_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(opcode) => opcode.bits(),
            SVE_MISC::ORR_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_MISC::SCVTF_SME_Zdnx2_SME_Znx2(opcode) => opcode.bits(),
            SVE_MISC::SCVTF_SME_Zdnx4_SME_Znx4(opcode) => opcode.bits(),
            SVE_MISC::SCVTF_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.bits(),
            SVE_MISC::SCVTF_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_D(opcode) => opcode.bits(),
            SVE_MISC::SCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_D(opcode) => opcode.bits(),
            SVE_MISC::SCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_H(opcode) => opcode.bits(),
            SVE_MISC::SCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_S(opcode) => opcode.bits(),
            SVE_MISC::SCVTF_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_D(opcode) => opcode.bits(),
            SVE_MISC::SCVTF_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_S(opcode) => opcode.bits(),
            SVE_MISC::SDOT_SVE_Zd_SVE_Zn_SVE_Zm3_19_INDEX(opcode) => opcode.bits(),
            SVE_MISC::SDOT_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX(opcode) => opcode.bits(),
            SVE_MISC::SDOT_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX(opcode) => opcode.bits(),
            SVE_MISC::SDOT_SVE_Zd_S_S_SVE_Zn_S_H_SVE_Zm_16_S_H(opcode) => opcode.bits(),
            SVE_MISC::SMULLB_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX(opcode) => opcode.bits(),
            SVE_MISC::SMULLB_SVE_Zd_SVE_Zn_SVE_Zm4_11_INDEX(opcode) => opcode.bits(),
            SVE_MISC::SMULLT_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX(opcode) => opcode.bits(),
            SVE_MISC::SMULLT_SVE_Zd_SVE_Zn_SVE_Zm4_11_INDEX(opcode) => opcode.bits(),
            SVE_MISC::STR_SVE_PNt_SVE_ADDR_RI_S9xVL(opcode) => opcode.bits(),
            SVE_MISC::STR_SVE_Pt_SVE_ADDR_RI_S9xVL(opcode) => opcode.bits(),
            SVE_MISC::STR_SVE_Zt_SVE_ADDR_RI_S9xVL(opcode) => opcode.bits(),
            SVE_MISC::SXTW_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.bits(),
            SVE_MISC::TRN1_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_MISC::TRN2_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_MISC::UCVTF_SME_Zdnx2_SME_Znx2(opcode) => opcode.bits(),
            SVE_MISC::UCVTF_SME_Zdnx4_SME_Znx4(opcode) => opcode.bits(),
            SVE_MISC::UCVTF_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.bits(),
            SVE_MISC::UCVTF_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_D(opcode) => opcode.bits(),
            SVE_MISC::UCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_D(opcode) => opcode.bits(),
            SVE_MISC::UCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_H(opcode) => opcode.bits(),
            SVE_MISC::UCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_S(opcode) => opcode.bits(),
            SVE_MISC::UCVTF_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_D(opcode) => opcode.bits(),
            SVE_MISC::UCVTF_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_S(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for SVE_MOVPRFX {
    fn definition(&self) -> &'static Insn {
        match self {
            SVE_MOVPRFX::MOVPRFX_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            SVE_MOVPRFX::MOVPRFX_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for SVE_PRED_ZM {
    fn definition(&self) -> &'static Insn {
        match self {
            SVE_PRED_ZM::BRKA_SVE_Pd_SVE_Pg4_10_SVE_Pn(opcode) => opcode.definition(),
            SVE_PRED_ZM::BRKB_SVE_Pd_SVE_Pg4_10_SVE_Pn(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            SVE_PRED_ZM::BRKA_SVE_Pd_SVE_Pg4_10_SVE_Pn(opcode) => opcode.bits(),
            SVE_PRED_ZM::BRKB_SVE_Pd_SVE_Pg4_10_SVE_Pn(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for SVE_SHIFT_PRED {
    fn definition(&self) -> &'static Insn {
        match self {
            SVE_SHIFT_PRED::ASRD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHRIMM_PRED(opcode) => {
                opcode.definition()
            }
            SVE_SHIFT_PRED::ASR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHRIMM_PRED(opcode) => {
                opcode.definition()
            }
            SVE_SHIFT_PRED::LSL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHLIMM_PRED(opcode) => {
                opcode.definition()
            }
            SVE_SHIFT_PRED::LSR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHRIMM_PRED(opcode) => {
                opcode.definition()
            }
        }
    }
    fn bits(&self) -> u32 {
        match self {
            SVE_SHIFT_PRED::ASRD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHRIMM_PRED(opcode) => opcode.bits(),
            SVE_SHIFT_PRED::ASR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHRIMM_PRED(opcode) => opcode.bits(),
            SVE_SHIFT_PRED::LSL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHLIMM_PRED(opcode) => opcode.bits(),
            SVE_SHIFT_PRED::LSR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHRIMM_PRED(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for SVE_SHIFT_UNPRED {
    fn definition(&self) -> &'static Insn {
        match self {
            SVE_SHIFT_UNPRED::ASR_SVE_Zd_SVE_Zn_SVE_SHRIMM_UNPRED(opcode) => opcode.definition(),
            SVE_SHIFT_UNPRED::LSL_SVE_Zd_SVE_Zn_SVE_SHLIMM_UNPRED(opcode) => opcode.definition(),
            SVE_SHIFT_UNPRED::LSR_SVE_Zd_SVE_Zn_SVE_SHRIMM_UNPRED(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            SVE_SHIFT_UNPRED::ASR_SVE_Zd_SVE_Zn_SVE_SHRIMM_UNPRED(opcode) => opcode.bits(),
            SVE_SHIFT_UNPRED::LSL_SVE_Zd_SVE_Zn_SVE_SHLIMM_UNPRED(opcode) => opcode.bits(),
            SVE_SHIFT_UNPRED::LSR_SVE_Zd_SVE_Zn_SVE_SHRIMM_UNPRED(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for SVE_SIZE_BHS {
    fn definition(&self) -> &'static Insn {
        match self {
            SVE_SIZE_BHS::ASR_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_BHS::ASR_SVE_Zd_S_B_SVE_Pg3_P_M_SVE_Zd_S_B_SVE_Zm_5_S_D(opcode) => {
                opcode.definition()
            }
            SVE_SIZE_BHS::CMPEQ_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_BHS::CMPGE_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_BHS::CMPGT_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_BHS::CMPHI_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_D(opcode) => {
                opcode.definition()
            }
            SVE_SIZE_BHS::CMPHS_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_D(opcode) => {
                opcode.definition()
            }
            SVE_SIZE_BHS::CMPLE_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_BHS::CMPLO_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_BHS::CMPLS_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_BHS::CMPLT_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_BHS::CMPNE_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_BHS::LSL_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_BHS::LSL_SVE_Zd_S_B_SVE_Pg3_P_M_SVE_Zd_S_B_SVE_Zm_5_S_D(opcode) => {
                opcode.definition()
            }
            SVE_SIZE_BHS::LSR_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_BHS::LSR_SVE_Zd_S_B_SVE_Pg3_P_M_SVE_Zd_S_B_SVE_Zm_5_S_D(opcode) => {
                opcode.definition()
            }
        }
    }
    fn bits(&self) -> u32 {
        match self {
            SVE_SIZE_BHS::ASR_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_BHS::ASR_SVE_Zd_S_B_SVE_Pg3_P_M_SVE_Zd_S_B_SVE_Zm_5_S_D(opcode) => {
                opcode.bits()
            }
            SVE_SIZE_BHS::CMPEQ_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_BHS::CMPGE_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_BHS::CMPGT_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_BHS::CMPHI_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_D(opcode) => {
                opcode.bits()
            }
            SVE_SIZE_BHS::CMPHS_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_D(opcode) => {
                opcode.bits()
            }
            SVE_SIZE_BHS::CMPLE_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_BHS::CMPLO_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_BHS::CMPLS_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_BHS::CMPLT_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_BHS::CMPNE_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_BHS::LSL_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_BHS::LSL_SVE_Zd_S_B_SVE_Pg3_P_M_SVE_Zd_S_B_SVE_Zm_5_S_D(opcode) => {
                opcode.bits()
            }
            SVE_SIZE_BHS::LSR_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_BHS::LSR_SVE_Zd_S_B_SVE_Pg3_P_M_SVE_Zd_S_B_SVE_Zm_5_S_D(opcode) => {
                opcode.bits()
            }
        }
    }
}
impl InsnOpcode for SVE_SIZE_BHSD {
    fn definition(&self) -> &'static Insn {
        match self {
            SVE_SIZE_BHSD::ADDP_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zn(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::ADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::ADD_SVE_Zd_SVE_Zd_SVE_AIMM(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::ADD_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::ANDV_SVE_Vd_SVE_Pg3_SVE_Zn(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::AND_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::ASRR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::ASR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::BIC_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::CLASTA_Rd_SVE_Pg3_Rd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::CLASTA_SVE_Vd_SVE_Pg3_SVE_Vd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::CLASTA_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::CLASTB_Rd_SVE_Pg3_Rd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::CLASTB_SVE_Vd_SVE_Pg3_SVE_Vd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::CLASTB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::CLS_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::CLZ_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::CMLA_SVE_Zd_SVE_Zn_SVE_Zm_16_SVE_IMM_ROT2(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::CMPEQ_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::CMPEQ_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_B(opcode) => {
                opcode.definition()
            }
            SVE_SIZE_BHSD::CMPGE_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::CMPGE_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_B(opcode) => {
                opcode.definition()
            }
            SVE_SIZE_BHSD::CMPGT_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::CMPGT_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_B(opcode) => {
                opcode.definition()
            }
            SVE_SIZE_BHSD::CMPHI_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_UIMM7(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::CMPHI_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::CMPHS_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_UIMM7(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::CMPHS_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::CMPLE_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::CMPLO_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_UIMM7(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::CMPLS_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_UIMM7(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::CMPLT_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::CMPNE_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::CMPNE_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_B(opcode) => {
                opcode.definition()
            }
            SVE_SIZE_BHSD::CNOT_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::CNTP_Rd_SVE_Pg4_10_SVE_Pn(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::CNT_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::EORBT_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::EORTB_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::EORV_SVE_Vd_SVE_Pg3_SVE_Zn(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::EOR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::LSLR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::LSL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::LSRR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::LSR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::MUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::MUL_SVE_Zd_SVE_Zd_SVE_SIMM8(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::MUL_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::NEG_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::ORR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::ORV_SVE_Vd_SVE_Pg3_SVE_Zn(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::SMULH_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::SMULH_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::SUBR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::SUBR_SVE_Zd_SVE_Zd_SVE_AIMM(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::SUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::SUB_SVE_Zd_SVE_Zd_SVE_AIMM(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::SUB_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::TBL_SVE_Zd_SVE_ZnxN_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::TBL_SVE_Zd_S_B_SVE_ZnxN_S_B_SVE_Zm_16_S_B(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::TBX_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::TRN1_SVE_Pd_SVE_Pn_SVE_Pm(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::TRN1_SVE_Zd_S_B_SVE_Zn_S_B_SVE_Zm_16_S_B(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::TRN2_SVE_Pd_SVE_Pn_SVE_Pm(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::TRN2_SVE_Zd_S_B_SVE_Zn_S_B_SVE_Zm_16_S_B(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::UADDV_SVE_Vd_SVE_Pg3_SVE_Zn(opcode) => opcode.definition(),
            SVE_SIZE_BHSD::UCLAMP_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            SVE_SIZE_BHSD::ADDP_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zn(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::ADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::ADD_SVE_Zd_SVE_Zd_SVE_AIMM(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::ADD_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::ANDV_SVE_Vd_SVE_Pg3_SVE_Zn(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::AND_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::ASRR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::ASR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::BIC_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::CLASTA_Rd_SVE_Pg3_Rd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::CLASTA_SVE_Vd_SVE_Pg3_SVE_Vd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::CLASTA_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::CLASTB_Rd_SVE_Pg3_Rd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::CLASTB_SVE_Vd_SVE_Pg3_SVE_Vd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::CLASTB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::CLS_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::CLZ_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::CMLA_SVE_Zd_SVE_Zn_SVE_Zm_16_SVE_IMM_ROT2(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::CMPEQ_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::CMPEQ_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_B(opcode) => {
                opcode.bits()
            }
            SVE_SIZE_BHSD::CMPGE_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::CMPGE_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_B(opcode) => {
                opcode.bits()
            }
            SVE_SIZE_BHSD::CMPGT_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::CMPGT_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_B(opcode) => {
                opcode.bits()
            }
            SVE_SIZE_BHSD::CMPHI_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_UIMM7(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::CMPHI_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::CMPHS_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_UIMM7(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::CMPHS_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::CMPLE_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::CMPLO_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_UIMM7(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::CMPLS_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_UIMM7(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::CMPLT_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::CMPNE_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::CMPNE_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_B(opcode) => {
                opcode.bits()
            }
            SVE_SIZE_BHSD::CNOT_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::CNTP_Rd_SVE_Pg4_10_SVE_Pn(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::CNT_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::EORBT_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::EORTB_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::EORV_SVE_Vd_SVE_Pg3_SVE_Zn(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::EOR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::LSLR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::LSL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::LSRR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::LSR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::MUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::MUL_SVE_Zd_SVE_Zd_SVE_SIMM8(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::MUL_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::NEG_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::ORR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::ORV_SVE_Vd_SVE_Pg3_SVE_Zn(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::SMULH_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::SMULH_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::SUBR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::SUBR_SVE_Zd_SVE_Zd_SVE_AIMM(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::SUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::SUB_SVE_Zd_SVE_Zd_SVE_AIMM(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::SUB_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::TBL_SVE_Zd_SVE_ZnxN_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::TBL_SVE_Zd_S_B_SVE_ZnxN_S_B_SVE_Zm_16_S_B(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::TBX_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::TRN1_SVE_Pd_SVE_Pn_SVE_Pm(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::TRN1_SVE_Zd_S_B_SVE_Zn_S_B_SVE_Zm_16_S_B(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::TRN2_SVE_Pd_SVE_Pn_SVE_Pm(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::TRN2_SVE_Zd_S_B_SVE_Zn_S_B_SVE_Zm_16_S_B(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::UADDV_SVE_Vd_SVE_Pg3_SVE_Zn(opcode) => opcode.bits(),
            SVE_SIZE_BHSD::UCLAMP_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for SVE_SIZE_HSD {
    fn definition(&self) -> &'static Insn {
        match self {
            SVE_SIZE_HSD::ADDHNB_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_HSD::ADDHNT_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_HSD::FADDA_SVE_Vd_SVE_Pg3_SVE_Vd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_SIZE_HSD::FADDP_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zn(opcode) => opcode.definition(),
            SVE_SIZE_HSD::FADDV_SVE_Vd_SVE_Pg3_SVE_Zn(opcode) => opcode.definition(),
            SVE_SIZE_HSD::FADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_ONE(opcode) => opcode.definition(),
            SVE_SIZE_HSD::FADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_SIZE_HSD::FADD_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_HSD::FCADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5_SVE_IMM_ROT1(opcode) => {
                opcode.definition()
            }
            SVE_SIZE_HSD::FCMEQ_SVE_Pd_SVE_Pg3_SVE_Zn_FPIMM0(opcode) => opcode.definition(),
            SVE_SIZE_HSD::FCMEQ_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_HSD::FCMGE_SVE_Pd_SVE_Pg3_SVE_Zn_FPIMM0(opcode) => opcode.definition(),
            SVE_SIZE_HSD::FCMGE_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_HSD::FCMGT_SVE_Pd_SVE_Pg3_SVE_Zn_FPIMM0(opcode) => opcode.definition(),
            SVE_SIZE_HSD::FCMGT_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_HSD::FCMLA_SVE_Zd_SVE_Pg3_SVE_Zn_SVE_Zm_16_IMM_ROT2(opcode) => {
                opcode.definition()
            }
            SVE_SIZE_HSD::FCMLE_SVE_Pd_SVE_Pg3_SVE_Zn_FPIMM0(opcode) => opcode.definition(),
            SVE_SIZE_HSD::FCMLT_SVE_Pd_SVE_Pg3_SVE_Zn_FPIMM0(opcode) => opcode.definition(),
            SVE_SIZE_HSD::FCMNE_SVE_Pd_SVE_Pg3_SVE_Zn_FPIMM0(opcode) => opcode.definition(),
            SVE_SIZE_HSD::FCMNE_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_HSD::FCMUO_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_HSD::FCPY_SVE_Zd_SVE_Pg4_16_SVE_FPIMM8(opcode) => opcode.definition(),
            SVE_SIZE_HSD::FDIVR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_SIZE_HSD::FDIV_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_SIZE_HSD::FDUP_SVE_Zd_SVE_FPIMM8(opcode) => opcode.definition(),
            SVE_SIZE_HSD::FEXPA_SVE_Zd_SVE_Zn(opcode) => opcode.definition(),
            SVE_SIZE_HSD::FMSB_SVE_Zd_SVE_Pg3_SVE_Zm_5_SVE_Za_16(opcode) => opcode.definition(),
            SVE_SIZE_HSD::FMULX_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_SIZE_HSD::FMUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_TWO(opcode) => opcode.definition(),
            SVE_SIZE_HSD::FMUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_SIZE_HSD::FMUL_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_HSD::FSUBR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_ONE(opcode) => {
                opcode.definition()
            }
            SVE_SIZE_HSD::FSUBR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_SIZE_HSD::FSUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_ONE(opcode) => opcode.definition(),
            SVE_SIZE_HSD::FSUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_SIZE_HSD::FSUB_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_HSD::SMULLB_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_HSD::SMULLT_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_HSD::SUBHNB_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_HSD::SUBHNT_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_HSD::SXTB_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.definition(),
            SVE_SIZE_HSD::UADDLB_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_HSD::UADDLT_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_HSD::UADDWB_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_HSD::UADDWT_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            SVE_SIZE_HSD::ADDHNB_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_HSD::ADDHNT_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FADDA_SVE_Vd_SVE_Pg3_SVE_Vd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FADDP_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zn(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FADDV_SVE_Vd_SVE_Pg3_SVE_Zn(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_ONE(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FADD_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FCADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5_SVE_IMM_ROT1(opcode) => {
                opcode.bits()
            }
            SVE_SIZE_HSD::FCMEQ_SVE_Pd_SVE_Pg3_SVE_Zn_FPIMM0(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FCMEQ_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FCMGE_SVE_Pd_SVE_Pg3_SVE_Zn_FPIMM0(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FCMGE_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FCMGT_SVE_Pd_SVE_Pg3_SVE_Zn_FPIMM0(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FCMGT_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FCMLA_SVE_Zd_SVE_Pg3_SVE_Zn_SVE_Zm_16_IMM_ROT2(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FCMLE_SVE_Pd_SVE_Pg3_SVE_Zn_FPIMM0(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FCMLT_SVE_Pd_SVE_Pg3_SVE_Zn_FPIMM0(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FCMNE_SVE_Pd_SVE_Pg3_SVE_Zn_FPIMM0(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FCMNE_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FCMUO_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FCPY_SVE_Zd_SVE_Pg4_16_SVE_FPIMM8(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FDIVR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FDIV_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FDUP_SVE_Zd_SVE_FPIMM8(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FEXPA_SVE_Zd_SVE_Zn(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FMSB_SVE_Zd_SVE_Pg3_SVE_Zm_5_SVE_Za_16(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FMULX_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FMUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_TWO(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FMUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FMUL_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FSUBR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_ONE(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FSUBR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FSUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_ONE(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FSUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_SIZE_HSD::FSUB_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_HSD::SMULLB_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_HSD::SMULLT_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_HSD::SUBHNB_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_HSD::SUBHNT_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_HSD::SXTB_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.bits(),
            SVE_SIZE_HSD::UADDLB_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_HSD::UADDLT_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_HSD::UADDWB_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_HSD::UADDWT_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for SVE_SIZE_SD {
    fn definition(&self) -> &'static Insn {
        match self {
            SVE_SIZE_SD::ADR_SVE_Zd_SVE_ADDR_ZZ_LSL(opcode) => opcode.definition(),
            SVE_SIZE_SD::CDOT_SVE_Zd_SVE_Zn_SVE_Zm_16_SVE_IMM_ROT2(opcode) => opcode.definition(),
            SVE_SIZE_SD::CTERMEQ_Rn_Rm(opcode) => opcode.definition(),
            SVE_SIZE_SD::CTERMNE_Rn_Rm(opcode) => opcode.definition(),
            SVE_SIZE_SD::SBCLB_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_SD::SBCLT_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_SD::SDIVR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_SIZE_SD::SDIV_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.definition(),
            SVE_SIZE_SD::SDOT_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.definition(),
            SVE_SIZE_SD::SXTH_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            SVE_SIZE_SD::ADR_SVE_Zd_SVE_ADDR_ZZ_LSL(opcode) => opcode.bits(),
            SVE_SIZE_SD::CDOT_SVE_Zd_SVE_Zn_SVE_Zm_16_SVE_IMM_ROT2(opcode) => opcode.bits(),
            SVE_SIZE_SD::CTERMEQ_Rn_Rm(opcode) => opcode.bits(),
            SVE_SIZE_SD::CTERMNE_Rn_Rm(opcode) => opcode.bits(),
            SVE_SIZE_SD::SBCLB_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_SD::SBCLT_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_SD::SDIVR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_SIZE_SD::SDIV_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(opcode) => opcode.bits(),
            SVE_SIZE_SD::SDOT_SVE_Zd_SVE_Zn_SVE_Zm_16(opcode) => opcode.bits(),
            SVE_SIZE_SD::SXTH_SVE_Zd_SVE_Pg3_SVE_Zn(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for TESTBRANCH {
    fn definition(&self) -> &'static Insn {
        match self {
            TESTBRANCH::TBNZ_Rt_BIT_NUM_ADDR_PCREL14(opcode) => opcode.definition(),
            TESTBRANCH::TBZ_Rt_BIT_NUM_ADDR_PCREL14(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            TESTBRANCH::TBNZ_Rt_BIT_NUM_ADDR_PCREL14(opcode) => opcode.bits(),
            TESTBRANCH::TBZ_Rt_BIT_NUM_ADDR_PCREL14(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for Operation {
    fn definition(&self) -> &'static Insn {
        match self {
            Operation::AARCH64_MISC(class) => class.definition(),
            Operation::ADDSUB_CARRY(class) => class.definition(),
            Operation::ADDSUB_EXT(class) => class.definition(),
            Operation::ADDSUB_IMM(class) => class.definition(),
            Operation::ADDSUB_SHIFT(class) => class.definition(),
            Operation::ASIMDALL(class) => class.definition(),
            Operation::ASIMDDIFF(class) => class.definition(),
            Operation::ASIMDELEM(class) => class.definition(),
            Operation::ASIMDIMM(class) => class.definition(),
            Operation::ASIMDMISC(class) => class.definition(),
            Operation::ASIMDPERM(class) => class.definition(),
            Operation::ASIMDSAME(class) => class.definition(),
            Operation::ASIMDSHF(class) => class.definition(),
            Operation::ASIMDTBL(class) => class.definition(),
            Operation::ASISDELEM(class) => class.definition(),
            Operation::ASISDMISC(class) => class.definition(),
            Operation::ASISDPAIR(class) => class.definition(),
            Operation::ASISDSAME(class) => class.definition(),
            Operation::ASISDSHF(class) => class.definition(),
            Operation::BFLOAT16(class) => class.definition(),
            Operation::BITFIELD(class) => class.definition(),
            Operation::BRANCH_IMM(class) => class.definition(),
            Operation::BRANCH_REG(class) => class.definition(),
            Operation::COMPBRANCH(class) => class.definition(),
            Operation::CONDCMP_IMM(class) => class.definition(),
            Operation::CONDCMP_REG(class) => class.definition(),
            Operation::CONDSEL(class) => class.definition(),
            Operation::CRYPTOSHA3(class) => class.definition(),
            Operation::CSSC(class) => class.definition(),
            Operation::DOTPRODUCT(class) => class.definition(),
            Operation::DP_1SRC(class) => class.definition(),
            Operation::DP_2SRC(class) => class.definition(),
            Operation::DP_3SRC(class) => class.definition(),
            Operation::EXCEPTION(class) => class.definition(),
            Operation::FLOAT2FIX(class) => class.definition(),
            Operation::FLOAT2INT(class) => class.definition(),
            Operation::FLOATCCMP(class) => class.definition(),
            Operation::FLOATCMP(class) => class.definition(),
            Operation::FLOATDP1(class) => class.definition(),
            Operation::FLOATDP2(class) => class.definition(),
            Operation::FLOATDP3(class) => class.definition(),
            Operation::FLOATIMM(class) => class.definition(),
            Operation::FLOATSEL(class) => class.definition(),
            Operation::IC_SYSTEM(class) => class.definition(),
            Operation::LDSTEXCL(class) => class.definition(),
            Operation::LDSTPAIR_INDEXED(class) => class.definition(),
            Operation::LDSTPAIR_OFF(class) => class.definition(),
            Operation::LDST_IMM10(class) => class.definition(),
            Operation::LDST_IMM9(class) => class.definition(),
            Operation::LDST_POS(class) => class.definition(),
            Operation::LDST_REGOFF(class) => class.definition(),
            Operation::LDST_UNPRIV(class) => class.definition(),
            Operation::LDST_UNSCALED(class) => class.definition(),
            Operation::LOADLIT(class) => class.definition(),
            Operation::LOG_IMM(class) => class.definition(),
            Operation::LOG_SHIFT(class) => class.definition(),
            Operation::MOVEWIDE(class) => class.definition(),
            Operation::PCRELADDR(class) => class.definition(),
            Operation::SME2_MOV(class) => class.definition(),
            Operation::SME2_MOVAZ(class) => class.definition(),
            Operation::SME_FP_SD(class) => class.definition(),
            Operation::SME_INT_SD(class) => class.definition(),
            Operation::SME_LDR(class) => class.definition(),
            Operation::SME_MISC(class) => class.definition(),
            Operation::SME_MOV(class) => class.definition(),
            Operation::SME_SIZE_22(class) => class.definition(),
            Operation::SME_SIZE_22_HSD(class) => class.definition(),
            Operation::SME_STR(class) => class.definition(),
            Operation::SVE2_URQVS(class) => class.definition(),
            Operation::SVE_LIMM(class) => class.definition(),
            Operation::SVE_MISC(class) => class.definition(),
            Operation::SVE_MOVPRFX(class) => class.definition(),
            Operation::SVE_PRED_ZM(class) => class.definition(),
            Operation::SVE_SHIFT_PRED(class) => class.definition(),
            Operation::SVE_SHIFT_UNPRED(class) => class.definition(),
            Operation::SVE_SIZE_BHS(class) => class.definition(),
            Operation::SVE_SIZE_BHSD(class) => class.definition(),
            Operation::SVE_SIZE_HSD(class) => class.definition(),
            Operation::SVE_SIZE_SD(class) => class.definition(),
            Operation::TESTBRANCH(class) => class.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            Operation::AARCH64_MISC(class) => class.bits(),
            Operation::ADDSUB_CARRY(class) => class.bits(),
            Operation::ADDSUB_EXT(class) => class.bits(),
            Operation::ADDSUB_IMM(class) => class.bits(),
            Operation::ADDSUB_SHIFT(class) => class.bits(),
            Operation::ASIMDALL(class) => class.bits(),
            Operation::ASIMDDIFF(class) => class.bits(),
            Operation::ASIMDELEM(class) => class.bits(),
            Operation::ASIMDIMM(class) => class.bits(),
            Operation::ASIMDMISC(class) => class.bits(),
            Operation::ASIMDPERM(class) => class.bits(),
            Operation::ASIMDSAME(class) => class.bits(),
            Operation::ASIMDSHF(class) => class.bits(),
            Operation::ASIMDTBL(class) => class.bits(),
            Operation::ASISDELEM(class) => class.bits(),
            Operation::ASISDMISC(class) => class.bits(),
            Operation::ASISDPAIR(class) => class.bits(),
            Operation::ASISDSAME(class) => class.bits(),
            Operation::ASISDSHF(class) => class.bits(),
            Operation::BFLOAT16(class) => class.bits(),
            Operation::BITFIELD(class) => class.bits(),
            Operation::BRANCH_IMM(class) => class.bits(),
            Operation::BRANCH_REG(class) => class.bits(),
            Operation::COMPBRANCH(class) => class.bits(),
            Operation::CONDCMP_IMM(class) => class.bits(),
            Operation::CONDCMP_REG(class) => class.bits(),
            Operation::CONDSEL(class) => class.bits(),
            Operation::CRYPTOSHA3(class) => class.bits(),
            Operation::CSSC(class) => class.bits(),
            Operation::DOTPRODUCT(class) => class.bits(),
            Operation::DP_1SRC(class) => class.bits(),
            Operation::DP_2SRC(class) => class.bits(),
            Operation::DP_3SRC(class) => class.bits(),
            Operation::EXCEPTION(class) => class.bits(),
            Operation::FLOAT2FIX(class) => class.bits(),
            Operation::FLOAT2INT(class) => class.bits(),
            Operation::FLOATCCMP(class) => class.bits(),
            Operation::FLOATCMP(class) => class.bits(),
            Operation::FLOATDP1(class) => class.bits(),
            Operation::FLOATDP2(class) => class.bits(),
            Operation::FLOATDP3(class) => class.bits(),
            Operation::FLOATIMM(class) => class.bits(),
            Operation::FLOATSEL(class) => class.bits(),
            Operation::IC_SYSTEM(class) => class.bits(),
            Operation::LDSTEXCL(class) => class.bits(),
            Operation::LDSTPAIR_INDEXED(class) => class.bits(),
            Operation::LDSTPAIR_OFF(class) => class.bits(),
            Operation::LDST_IMM10(class) => class.bits(),
            Operation::LDST_IMM9(class) => class.bits(),
            Operation::LDST_POS(class) => class.bits(),
            Operation::LDST_REGOFF(class) => class.bits(),
            Operation::LDST_UNPRIV(class) => class.bits(),
            Operation::LDST_UNSCALED(class) => class.bits(),
            Operation::LOADLIT(class) => class.bits(),
            Operation::LOG_IMM(class) => class.bits(),
            Operation::LOG_SHIFT(class) => class.bits(),
            Operation::MOVEWIDE(class) => class.bits(),
            Operation::PCRELADDR(class) => class.bits(),
            Operation::SME2_MOV(class) => class.bits(),
            Operation::SME2_MOVAZ(class) => class.bits(),
            Operation::SME_FP_SD(class) => class.bits(),
            Operation::SME_INT_SD(class) => class.bits(),
            Operation::SME_LDR(class) => class.bits(),
            Operation::SME_MISC(class) => class.bits(),
            Operation::SME_MOV(class) => class.bits(),
            Operation::SME_SIZE_22(class) => class.bits(),
            Operation::SME_SIZE_22_HSD(class) => class.bits(),
            Operation::SME_STR(class) => class.bits(),
            Operation::SVE2_URQVS(class) => class.bits(),
            Operation::SVE_LIMM(class) => class.bits(),
            Operation::SVE_MISC(class) => class.bits(),
            Operation::SVE_MOVPRFX(class) => class.bits(),
            Operation::SVE_PRED_ZM(class) => class.bits(),
            Operation::SVE_SHIFT_PRED(class) => class.bits(),
            Operation::SVE_SHIFT_UNPRED(class) => class.bits(),
            Operation::SVE_SIZE_BHS(class) => class.bits(),
            Operation::SVE_SIZE_BHSD(class) => class.bits(),
            Operation::SVE_SIZE_HSD(class) => class.bits(),
            Operation::SVE_SIZE_SD(class) => class.bits(),
            Operation::TESTBRANCH(class) => class.bits(),
        }
    }
}
impl InsnOpcode for Opcode {
    fn definition(&self) -> &'static Insn {
        self.operation.definition()
    }
    fn bits(&self) -> u32 {
        self.operation.bits()
    }
}
pub fn decode(insn: u32) -> Option<Opcode> {
    if insn & 0x4000000 == 0 {
        if insn & 0x2000000 == 0 {
            if insn & 0x8000000 == 0 {
                if insn & 0x1000000 == 0 {
                    if insn & 0x10000000 == 0 {
                        if insn & 0x40000000 == 0 {
                            if insn & 0x000010 == 0 {
                                if insn & 0x400000 == 0 {
                                    if insn & 0xffe0001c == 0x80800000 {
                                        return Some (FMOPA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                    }
                                } else {
                                    if insn & 0xffe00018 == 0x80c00000 {
                                        return Some (FMOPA_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                    }
                                }
                            } else {
                                if insn & 0x400000 == 0 {
                                    if insn & 0xffe0001c == 0x80800010 {
                                        return Some (FMOPS_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                    }
                                } else {
                                    if insn & 0xffe00018 == 0x80c00010 {
                                        return Some (FMOPS_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                    }
                                }
                            }
                        } else {
                            if insn & 0x020000 == 0 {
                                if insn & 0x040000 == 0 {
                                    if insn & 0x100000 == 0 {
                                        if insn & 0xff3e0010 == 0xc0000000 {
                                            return Some(
                                                MOVA_SME_ZA_HV_idx_dest_SVE_Pg3_SVE_Zn::make_opcode(
                                                    insn,
                                                ),
                                            );
                                        }
                                        if insn & 0xff3e0010 == 0xc0000000 {
                                            return Some(
                                                MOVA_SME_ZA_HV_idx_dest_SVE_Pg3_SVE_Zn::make_opcode(
                                                    insn,
                                                ),
                                            );
                                        }
                                    } else {
                                        if insn & 0x010000 == 0 {
                                            if insn & 0x400000 == 0 {
                                                if insn & 0xffff001c == 0xc0900000 {
                                                    return Some (ADDHA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn :: make_opcode (insn)) ;
                                                }
                                            } else {
                                                if insn & 0xffff0018 == 0xc0d00000 {
                                                    return Some (ADDHA_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn :: make_opcode (insn)) ;
                                                }
                                            }
                                        } else {
                                            if insn & 0x400000 == 0 {
                                                if insn & 0xffff001c == 0xc0910000 {
                                                    return Some (ADDVA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn :: make_opcode (insn)) ;
                                                }
                                            } else {
                                                if insn & 0xffff0018 == 0xc0d10000 {
                                                    return Some (ADDVA_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn :: make_opcode (insn)) ;
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x000020 == 0 {
                                        if insn & 0x000400 == 0 {
                                            if insn & 0x000800 == 0 {
                                                if insn & 0xff3f1c38 == 0xc0040000 {
                                                    return Some (MOVA_SME_ZA_HV_idx_destxN_SME_Znx2 :: make_opcode (insn)) ;
                                                }
                                                if insn & 0xff3f1c38 == 0xc0040000 {
                                                    return Some (MOVA_SME_ZA_HV_idx_destxN_SME_Znx2 :: make_opcode (insn)) ;
                                                }
                                            } else {
                                                if insn & 0xffff9c38 == 0xc0040800 {
                                                    return Some (MOVA_SME_ZA_array_off3_0_SME_Znx2 :: make_opcode (insn)) ;
                                                }
                                                if insn & 0xffff9c38 == 0xc0040800 {
                                                    return Some (MOVA_SME_ZA_array_off3_0_SME_Znx2 :: make_opcode (insn)) ;
                                                }
                                            }
                                        } else {
                                            if insn & 0x000800 == 0 {
                                                if insn & 0xff3f1c78 == 0xc0040400 {
                                                    return Some (MOVA_SME_ZA_HV_idx_destxN_SME_Znx4 :: make_opcode (insn)) ;
                                                }
                                                if insn & 0xff3f1c78 == 0xc0040400 {
                                                    return Some (MOVA_SME_ZA_HV_idx_destxN_SME_Znx4 :: make_opcode (insn)) ;
                                                }
                                            } else {
                                                if insn & 0xffff9c78 == 0xc0040c00 {
                                                    return Some (MOVA_SME_ZA_array_off3_0_SME_Znx4 :: make_opcode (insn)) ;
                                                }
                                                if insn & 0xffff9c78 == 0xc0040c00 {
                                                    return Some (MOVA_SME_ZA_array_off3_0_SME_Znx4 :: make_opcode (insn)) ;
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0xffff8fe0 == 0xc04c03e0 {
                                            return Some(MOVT_Rt_SME_ZT0_INDEX::make_opcode(insn));
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x000200 == 0 {
                                    if insn & 0x040000 == 0 {
                                        if insn & 0xff3e0200 == 0xc0020000 {
                                            return Some(
                                                MOVA_SVE_Zd_SVE_Pg3_SME_ZA_HV_idx_src::make_opcode(
                                                    insn,
                                                ),
                                            );
                                        }
                                        if insn & 0xff3e0200 == 0xc0020000 {
                                            return Some(
                                                MOVA_SVE_Zd_SVE_Pg3_SME_ZA_HV_idx_src::make_opcode(
                                                    insn,
                                                ),
                                            );
                                        }
                                    } else {
                                        if insn & 0x000400 == 0 {
                                            if insn & 0x000800 == 0 {
                                                if insn & 0xff3f1f01 == 0xc0060000 {
                                                    return Some (MOVA_SME_Zdnx2_SME_ZA_HV_idx_srcxN :: make_opcode (insn)) ;
                                                }
                                                if insn & 0xff3f1f01 == 0xc0060000 {
                                                    return Some (MOVA_SME_Zdnx2_SME_ZA_HV_idx_srcxN :: make_opcode (insn)) ;
                                                }
                                            } else {
                                                if insn & 0xffff9f01 == 0xc0060800 {
                                                    return Some (MOVA_SME_Zdnx2_SME_ZA_array_off3_5 :: make_opcode (insn)) ;
                                                }
                                                if insn & 0xffff9f01 == 0xc0060800 {
                                                    return Some (MOVA_SME_Zdnx2_SME_ZA_array_off3_5 :: make_opcode (insn)) ;
                                                }
                                            }
                                        } else {
                                            if insn & 0x000800 == 0 {
                                                if insn & 0xff3f1f03 == 0xc0060400 {
                                                    return Some (MOVA_SME_Zdnx4_SME_ZA_HV_idx_srcxN :: make_opcode (insn)) ;
                                                }
                                                if insn & 0xff3f1f03 == 0xc0060400 {
                                                    return Some (MOVA_SME_Zdnx4_SME_ZA_HV_idx_srcxN :: make_opcode (insn)) ;
                                                }
                                            } else {
                                                if insn & 0xffff9f03 == 0xc0060c00 {
                                                    return Some (MOVA_SME_Zdnx4_SME_ZA_array_off3_5 :: make_opcode (insn)) ;
                                                }
                                                if insn & 0xffff9f03 == 0xc0060c00 {
                                                    return Some (MOVA_SME_Zdnx4_SME_ZA_array_off3_5 :: make_opcode (insn)) ;
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x000100 == 0 {
                                        if insn & 0x000400 == 0 {
                                            if insn & 0x400000 == 0 {
                                                if insn & 0x800000 == 0 {
                                                    if insn & 0xffff1f01 == 0xc0060200 {
                                                        return Some (MOVAZ_SME_Zdnx2_SME_ZA_array_vrsb_1 :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0xffff1f01 == 0xc0860200 {
                                                        return Some (MOVAZ_SME_Zdnx2_SME_ZA_array_vrss_1 :: make_opcode (insn)) ;
                                                    }
                                                }
                                            } else {
                                                if insn & 0x800000 == 0 {
                                                    if insn & 0xffff1f01 == 0xc0460200 {
                                                        return Some (MOVAZ_SME_Zdnx2_SME_ZA_array_vrsh_1 :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0xffff1f01 == 0xc0c60200 {
                                                        return Some (MOVAZ_SME_Zdnx2_SME_ZA_array_vrsd_1 :: make_opcode (insn)) ;
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x400000 == 0 {
                                                if insn & 0x800000 == 0 {
                                                    if insn & 0xffff1f83 == 0xc0060600 {
                                                        return Some (MOVAZ_SME_Zdnx4_SME_ZA_array_vrsb_2 :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0xffff1f83 == 0xc0860600 {
                                                        return Some (MOVAZ_SME_Zdnx4_SME_ZA_array_vrss_2 :: make_opcode (insn)) ;
                                                    }
                                                }
                                            } else {
                                                if insn & 0x800000 == 0 {
                                                    if insn & 0xffff1f83 == 0xc0460600 {
                                                        return Some (MOVAZ_SME_Zdnx4_SME_ZA_array_vrsh_2 :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0xffff1f03 == 0xc0c60600 {
                                                        return Some (MOVAZ_SME_Zdnx4_SME_ZA_array_vrsd_2 :: make_opcode (insn)) ;
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0xffff8fe0 == 0xc04e03e0 {
                                            return Some(MOVT_SME_ZT0_INDEX_Rt::make_opcode(insn));
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        if insn & 0x80000000 == 0 {
                            if insn & 0x9f000000 == 0x10000000 {
                                return Some(ADR_Rd_ADDR_PCREL21::make_opcode(insn));
                            }
                        } else {
                            if insn & 0x9f000000 == 0x90000000 {
                                return Some(ADRP_Rd_ADDR_ADRP::make_opcode(insn));
                            }
                        }
                    }
                } else {
                    if insn & 0x10000000 == 0 {
                        if insn & 0x200000 == 0 {
                            if insn & 0x000010 == 0 {
                                if insn & 0x400000 == 0 {
                                    if insn & 0x800000 == 0 {
                                        if insn & 0x008000 == 0 {
                                            if insn & 0xffff9c10 == 0xe1000000 {
                                                return Some (LDR_SME_ZA_array_off4_SME_ADDR_RI_U4xVL :: make_opcode (insn)) ;
                                            }
                                        } else {
                                            if insn & 0xfffffc1f == 0xe11f8000 {
                                                return Some(
                                                    LDR_SME_ZT0_SIMD_ADDR_SIMPLE::make_opcode(insn),
                                                );
                                            }
                                        }
                                    } else {
                                        if insn & 0xffe0001c == 0x81800000 {
                                            return Some (BFMOPA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                        }
                                    }
                                } else {
                                    if insn & 0x000008 == 0 {
                                        if insn & 0x000020 == 0 {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0xfff09038 == 0xc1501000 {
                                                    return Some (SDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2 :: make_opcode (insn)) ;
                                                }
                                            } else {
                                                if insn & 0xfff09078 == 0xc1509000 {
                                                    return Some (SDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX2 :: make_opcode (insn)) ;
                                                }
                                            }
                                        } else {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0xfff09038 == 0xc1501020 {
                                                    return Some (SDOT_SME_ZA_array_off3_0_S_S_SME_Znx2_S_B_SME_Zm_INDEX2_S_B :: make_opcode (insn)) ;
                                                }
                                            } else {
                                                if insn & 0xfff09078 == 0xc1509020 {
                                                    return Some (SDOT_SME_ZA_array_off3_0_S_S_SME_Znx4_S_B_SME_Zm_INDEX2_S_B :: make_opcode (insn)) ;
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x001000 == 0 {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0xfff09838 == 0xc1d00008 {
                                                    return Some (SDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX1 :: make_opcode (insn)) ;
                                                }
                                            } else {
                                                if insn & 0xfff09878 == 0xc1d08008 {
                                                    return Some (SDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX1 :: make_opcode (insn)) ;
                                                }
                                            }
                                        } else {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0xfff09038 == 0xc1501008 {
                                                    return Some (FDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2 :: make_opcode (insn)) ;
                                                }
                                            } else {
                                                if insn & 0xfff09078 == 0xc1509008 {
                                                    return Some (FDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX2 :: make_opcode (insn)) ;
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x000008 == 0 {
                                    if insn & 0x40000000 == 0 {
                                        if insn & 0xffe0001c == 0x81800010 {
                                            return Some (BFMOPS_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                        }
                                    } else {
                                        if insn & 0x100000 == 0 {
                                            if insn & 0xfff01018 == 0xc1801010 {
                                                return Some (BFMLAL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm_INDEX3_10 :: make_opcode (insn)) ;
                                            }
                                        } else {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0xfff09038 == 0xc1901010 {
                                                    return Some (BFMLAL_SME_ZA_array_off2x2_SME_Znx2_SME_Zm_INDEX3_2 :: make_opcode (insn)) ;
                                                }
                                            } else {
                                                if insn & 0xfff09078 == 0xc1909010 {
                                                    return Some (BFMLAL_SME_ZA_array_off2x2_SME_Znx4_SME_Zm_INDEX3_2 :: make_opcode (insn)) ;
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x001000 == 0 {
                                        if insn & 0xfff09038 == 0xc1500018 {
                                            return Some (BFVDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2 :: make_opcode (insn)) ;
                                        }
                                    } else {
                                        if insn & 0x100000 == 0 {
                                            if insn & 0xfff01018 == 0xc1801018 {
                                                return Some (BFMLSL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm_INDEX3_10 :: make_opcode (insn)) ;
                                            }
                                        } else {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0xfff09038 == 0xc1901018 {
                                                        return Some (BFMLSL_SME_ZA_array_off2x2_SME_Znx2_SME_Zm_INDEX3_2 :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0xfff09038 == 0xc1501018 {
                                                        return Some (BFDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2 :: make_opcode (insn)) ;
                                                    }
                                                }
                                            } else {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0xfff09078 == 0xc1909018 {
                                                        return Some (BFMLSL_SME_ZA_array_off2x2_SME_Znx4_SME_Zm_INDEX3_2 :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0xfff09078 == 0xc1509018 {
                                                        return Some (BFDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX2 :: make_opcode (insn)) ;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            if insn & 0x20000000 == 0 {
                                if insn & 0x40000000 == 0 {
                                    if insn & 0x000010 == 0 {
                                        if insn & 0xffe0001c == 0x81a00000 {
                                            return Some (FMOPA_SME_ZAda_2b_S_S_SVE_Pg3_P_M_SME_Pm_P_M_SVE_Zn_S_H_SVE_Zm_16_S_H :: make_opcode (insn)) ;
                                        }
                                    } else {
                                        if insn & 0xffe0001c == 0x81a00010 {
                                            return Some (FMOPS_SME_ZAda_2b_S_S_SVE_Pg3_P_M_SME_Pm_P_M_SVE_Zn_S_H_SVE_Zm_16_S_H :: make_opcode (insn)) ;
                                        }
                                    }
                                } else {
                                    if insn & 0x000400 == 0 {
                                        if insn & 0x000800 == 0 {
                                            if insn & 0x001000 == 0 {
                                                if insn & 0x002000 == 0 {
                                                    if insn & 0xff20fc01 == 0xc120c000 {
                                                        return Some (FCLAMP_SME_Zdnx2_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0x000020 == 0 {
                                                        if insn & 0x004000 == 0 {
                                                            if insn & 0xff30ffe1 == 0xc120a300 {
                                                                return Some (ADD_SME_Zdnx2_SME_Zdnx2_SME_Zm :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0x010000 == 0 {
                                                                if insn & 0x020000 == 0 {
                                                                    if insn & 0x400000 == 0 {
                                                                        if insn & 0xfffffc20
                                                                            == 0xc120e000
                                                                        {
                                                                            return Some (FCVT_SVE_Zd_SME_Znx2 :: make_opcode (insn)) ;
                                                                        }
                                                                    } else {
                                                                        if insn & 0xfffffc20
                                                                            == 0xc160e000
                                                                        {
                                                                            return Some (BFCVT_SVE_Zd_SME_Znx2 :: make_opcode (insn)) ;
                                                                        }
                                                                    }
                                                                } else {
                                                                    if insn & 0x100000 == 0 {
                                                                        if insn & 0xfffffc21
                                                                            == 0xc122e000
                                                                        {
                                                                            return Some (SCVTF_SME_Zdnx2_SME_Znx2 :: make_opcode (insn)) ;
                                                                        }
                                                                    } else {
                                                                        if insn & 0xfffffc63
                                                                            == 0xc132e000
                                                                        {
                                                                            return Some (SCVTF_SME_Zdnx4_SME_Znx4 :: make_opcode (insn)) ;
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0x100000 == 0 {
                                                                    if insn & 0xfffffc21
                                                                        == 0xc121e000
                                                                    {
                                                                        return Some (FCVTZS_SME_Zdnx2_SME_Znx2 :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xfffffc63
                                                                        == 0xc131e000
                                                                    {
                                                                        return Some (FCVTZS_SME_Zdnx4_SME_Znx4 :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x010000 == 0 {
                                                            if insn & 0x020000 == 0 {
                                                                if insn & 0x400000 == 0 {
                                                                    if insn & 0xfffffc20
                                                                        == 0xc120e020
                                                                    {
                                                                        return Some (FCVTN_SVE_Zd_SME_Znx2 :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xfffffc20
                                                                        == 0xc160e020
                                                                    {
                                                                        return Some (BFCVTN_SVE_Zd_SME_Znx2 :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0x100000 == 0 {
                                                                    if insn & 0xfffffc21
                                                                        == 0xc122e020
                                                                    {
                                                                        return Some (UCVTF_SME_Zdnx2_SME_Znx2 :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xfffffc63
                                                                        == 0xc132e020
                                                                    {
                                                                        return Some (UCVTF_SME_Zdnx4_SME_Znx4 :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x100000 == 0 {
                                                                if insn & 0xfffffc21 == 0xc121e020 {
                                                                    return Some (FCVTZU_SME_Zdnx2_SME_Znx2 :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xfffffc63 == 0xc131e020 {
                                                                    return Some (FCVTZU_SME_Zdnx4_SME_Znx4 :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x000010 == 0 {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0x100000 == 0 {
                                                            if insn & 0xfff09c18 == 0xc1201000 {
                                                                return Some (FDOT_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xfff09c18 == 0xc1301000 {
                                                                return Some (FDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x010000 == 0 {
                                                            if insn & 0xffe19c38 == 0xc1a01000 {
                                                                return Some (FDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2 :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xffe39c78 == 0xc1a11000 {
                                                                return Some (FDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4 :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0x100000 == 0 {
                                                            if insn & 0xfff09c18 == 0xc1201010 {
                                                                return Some (BFDOT_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xfff09c18 == 0xc1301010 {
                                                                return Some (BFDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x010000 == 0 {
                                                            if insn & 0xffe19c38 == 0xc1a01010 {
                                                                return Some (BFDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2 :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xffe39c78 == 0xc1a11010 {
                                                                return Some (BFDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4 :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x001000 == 0 {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x000008 == 0 {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x100000 == 0 {
                                                                if insn & 0xfff09c1c == 0xc1200810 {
                                                                    return Some (BFMLAL_SME_ZA_array_off2x2_SVE_ZnxN_SME_Zm :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xfff09c1c == 0xc1300810 {
                                                                    return Some (BFMLAL_SME_ZA_array_off2x2_S_S_SVE_ZnxN_S_H_SME_Zm_S_H :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x010000 == 0 {
                                                                if insn & 0xffe19c3c == 0xc1a00810 {
                                                                    return Some (BFMLAL_SME_ZA_array_off2x2_SME_Znx2_SME_Zmx2 :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xffe39c7c == 0xc1a10810 {
                                                                    return Some (BFMLAL_SME_ZA_array_off2x2_SME_Znx4_SME_Zmx4 :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x100000 == 0 {
                                                                if insn & 0xfff09c1c == 0xc1200818 {
                                                                    return Some (BFMLSL_SME_ZA_array_off2x2_SVE_ZnxN_SME_Zm :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xfff09c1c == 0xc1300818 {
                                                                    return Some (BFMLSL_SME_ZA_array_off2x2_S_S_SVE_ZnxN_S_H_SME_Zm_S_H :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x010000 == 0 {
                                                                if insn & 0xffe19c3c == 0xc1a00818 {
                                                                    return Some (BFMLSL_SME_ZA_array_off2x2_SME_Znx2_SME_Zmx2 :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xffe39c7c == 0xc1a10818 {
                                                                    return Some (BFMLSL_SME_ZA_array_off2x2_SME_Znx4_SME_Zmx4 :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x002000 == 0 {
                                                        if insn & 0xff20fc03 == 0xc120c800 {
                                                            return Some (FCLAMP_SME_Zdnx4_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0xff30ffe3 == 0xc120ab00 {
                                                            return Some (ADD_SME_Zdnx4_SME_Zdnx4_SME_Zm :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x000008 == 0 {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0x100000 == 0 {
                                                            if insn & 0xffb09c18 == 0xc1201810 {
                                                                return Some (ADD_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xffb09c18 == 0xc1301810 {
                                                                return Some (ADD_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_S_SME_Zm_S_S :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x010000 == 0 {
                                                            if insn & 0xffa19c38 == 0xc1a01810 {
                                                                return Some (ADD_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2 :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xffa39c78 == 0xc1a11810 {
                                                                return Some (ADD_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4 :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0x100000 == 0 {
                                                            if insn & 0xffb09c18 == 0xc1201818 {
                                                                return Some (SUB_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xffb09c18 == 0xc1301818 {
                                                                return Some (SUB_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_S_SME_Zm_S_S :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x010000 == 0 {
                                                            if insn & 0xffa19c38 == 0xc1a01818 {
                                                                return Some (SUB_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2 :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xffa39c78 == 0xc1a11818 {
                                                                return Some (SUB_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4 :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x000800 == 0 {
                                            if insn & 0x001000 == 0 {
                                                if insn & 0xff20fc01 == 0xc120c401 {
                                                    return Some (UCLAMP_SME_Zdnx2_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                }
                                            } else {
                                                if insn & 0x000008 == 0 {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0x100000 == 0 {
                                                            if insn & 0xffb09c18 == 0xc1201400 {
                                                                return Some (SDOT_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xffb09c18 == 0xc1301400 {
                                                                return Some (SDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_B_SME_Zm_S_B :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x010000 == 0 {
                                                            if insn & 0xffa19c38 == 0xc1a01400 {
                                                                return Some (SDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2 :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xffa39c78 == 0xc1a11400 {
                                                                return Some (SDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4 :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0x100000 == 0 {
                                                            if insn & 0xfff09c18 == 0xc1601408 {
                                                                return Some (SDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xfff09c18 == 0xc1701408 {
                                                                return Some (SDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H_c1701408 :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x010000 == 0 {
                                                            if insn & 0xffe19c38 == 0xc1e01408 {
                                                                return Some (SDOT_SME_ZA_array_off3_0_S_S_SME_Znx2_S_H_SME_Zmx2_S_H :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xffe39c78 == 0xc1e11408 {
                                                                return Some (SDOT_SME_ZA_array_off3_0_S_S_SME_Znx4_S_H_SME_Zmx4_S_H :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x001000 == 0 {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x000008 == 0 {
                                                        if insn & 0xfff09c18 == 0xc1200c10 {
                                                            return Some (BFMLAL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0xfff09c18 == 0xc1200c18 {
                                                            return Some (BFMLSL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xff20fc03 == 0xc120cc01 {
                                                        return Some (UCLAMP_SME_Zdnx4_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                    }
                                                }
                                            } else {
                                                if insn & 0x000008 == 0 {
                                                    if insn & 0x000010 == 0 {
                                                        if insn & 0x010000 == 0 {
                                                            if insn & 0xffbf9c38 == 0xc1a01c00 {
                                                                return Some (FADD_SME_ZA_array_off3_0_SME_Znx2 :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xffbf9c78 == 0xc1a11c00 {
                                                                return Some (FADD_SME_ZA_array_off3_0_SME_Znx4 :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x010000 == 0 {
                                                            if insn & 0xffbf9c38 == 0xc1a01c10 {
                                                                return Some (ADD_SME_ZA_array_off3_0_SME_Znx2 :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xffbf9c78 == 0xc1a11c10 {
                                                                return Some (ADD_SME_ZA_array_off3_0_SME_Znx4 :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x000010 == 0 {
                                                        if insn & 0x010000 == 0 {
                                                            if insn & 0xffbf9c38 == 0xc1a01c08 {
                                                                return Some (FSUB_SME_ZA_array_off3_0_SME_Znx2 :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xffbf9c78 == 0xc1a11c08 {
                                                                return Some (FSUB_SME_ZA_array_off3_0_SME_Znx4 :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x010000 == 0 {
                                                            if insn & 0xffbf9c38 == 0xc1a01c18 {
                                                                return Some (SUB_SME_ZA_array_off3_0_SME_Znx2 :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xffbf9c78 == 0xc1a11c18 {
                                                                return Some (SUB_SME_ZA_array_off3_0_SME_Znx4 :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x008000 == 0 {
                                    if insn & 0xffff9c10 == 0xe1200000 {
                                        return Some(
                                            STR_SME_ZA_array_off4_SME_ADDR_RI_U4xVL::make_opcode(
                                                insn,
                                            ),
                                        );
                                    }
                                } else {
                                    if insn & 0xfffffc1f == 0xe13f8000 {
                                        return Some(STR_SME_ZT0_SIMD_ADDR_SIMPLE::make_opcode(
                                            insn,
                                        ));
                                    }
                                }
                            }
                        }
                    } else {
                        if insn & 0x800000 == 0 {
                            if insn & 0x20000000 == 0 {
                                if insn & 0x40000000 == 0 {
                                    if insn & 0x7f800000 == 0x11000000 {
                                        return Some(ADD_Rd_SP_Rn_SP_AIMM::make_opcode(insn));
                                    }
                                } else {
                                    if insn & 0x7f800000 == 0x51000000 {
                                        return Some(SUB_Rd_SP_Rn_SP_AIMM::make_opcode(insn));
                                    }
                                }
                            } else {
                                if insn & 0x40000000 == 0 {
                                    if insn & 0x7f800000 == 0x31000000 {
                                        return Some(ADDS_Rd_Rn_SP_AIMM::make_opcode(insn));
                                    }
                                } else {
                                    if insn & 0x7f800000 == 0x71000000 {
                                        return Some(SUBS_Rd_Rn_SP_AIMM::make_opcode(insn));
                                    }
                                }
                            }
                        } else {
                            if insn & 0x40000000 == 0 {
                                if insn & 0xffc0c000 == 0x91800000 {
                                    return Some(ADDG_Rd_SP_Rn_SP_UIMM10_UIMM4_ADDG::make_opcode(
                                        insn,
                                    ));
                                }
                            } else {
                                if insn & 0xffc0c000 == 0xd1800000 {
                                    return Some(SUBG_Rd_SP_Rn_SP_UIMM10_UIMM4_ADDG::make_opcode(
                                        insn,
                                    ));
                                }
                            }
                        }
                    }
                }
            } else {
                if insn & 0x10000000 == 0 {
                    if insn & 0x400000 == 0 {
                        if insn & 0x800000 == 0 {
                            if insn & 0x1000000 == 0 {
                                if insn & 0x200000 == 0 {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xffe0fc00 == 0x8007c00 {
                                                return Some(STXRB_Rs_Rt_ADDR_SIMPLE::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        } else {
                                            if insn & 0xffe0fc00 == 0x48007c00 {
                                                return Some(STXRH_Rs_Rt_ADDR_SIMPLE::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    } else {
                                        if insn & 0xbfe0fc00 == 0x88007c00 {
                                            return Some(STXR_Rs_Rt_ADDR_SIMPLE::make_opcode(insn));
                                        }
                                    }
                                } else {
                                    if insn & 0xbfe08000 == 0x88200000 {
                                        return Some(STXP_Rs_Rt_Rt2_ADDR_SIMPLE::make_opcode(insn));
                                    }
                                }
                            } else {
                                if insn & 0x7fc00000 == 0x29000000 {
                                    return Some(STP_Rt_Rt2_ADDR_SIMM7::make_opcode(insn));
                                }
                            }
                        } else {
                            if insn & 0x7ec00000 == 0x28800000 {
                                return Some(STP_Rt_W_Rt2_W_ADDR_SIMM7_S_S::make_opcode(insn));
                            }
                        }
                    } else {
                        if insn & 0x800000 == 0 {
                            if insn & 0x40000000 == 0 {
                                if insn & 0x7fc00000 == 0x29400000 {
                                    return Some(LDP_Rt_Rt2_ADDR_SIMM7::make_opcode(insn));
                                }
                            } else {
                                if insn & 0xffc00000 == 0x69400000 {
                                    return Some(LDPSW_Rt_Rt2_ADDR_SIMM7::make_opcode(insn));
                                }
                            }
                        } else {
                            if insn & 0x40000000 == 0 {
                                if insn & 0x7ec00000 == 0x28c00000 {
                                    return Some(LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S::make_opcode(insn));
                                }
                            } else {
                                if insn & 0xfec00000 == 0x68c00000 {
                                    return Some(LDPSW_Rt_X_Rt2_X_ADDR_SIMM7_S_S::make_opcode(
                                        insn,
                                    ));
                                }
                            }
                        }
                    }
                } else {
                    if insn & 0x1000000 == 0 {
                        if insn & 0x20000000 == 0 {
                            if insn & 0x80000000 == 0 {
                                if insn & 0xbf000000 == 0x18000000 {
                                    return Some(LDR_Rt_ADDR_PCREL19::make_opcode(insn));
                                }
                            } else {
                                if insn & 0xff000000 == 0x98000000 {
                                    return Some(LDRSW_Rt_ADDR_PCREL19::make_opcode(insn));
                                }
                            }
                        } else {
                            if insn & 0x000400 == 0 {
                                if insn & 0x000800 == 0 {
                                    if insn & 0x800000 == 0 {
                                        if insn & 0x400000 == 0 {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x38000000 {
                                                        return Some(
                                                            STURB_Rt_ADDR_SIMM9::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x78000000 {
                                                        return Some(
                                                            STURH_Rt_ADDR_SIMM9::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbfe00c00 == 0xb8000000 {
                                                    return Some(STUR_Rt_ADDR_SIMM9::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        } else {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x38400000 {
                                                        return Some(
                                                            LDURB_Rt_ADDR_SIMM9::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x78400000 {
                                                        return Some(
                                                            LDURH_Rt_ADDR_SIMM9::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbfe00c00 == 0xb8400000 {
                                                    return Some(LDUR_Rt_ADDR_SIMM9::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0xffa00c00 == 0x38800000 {
                                                    return Some(
                                                        LDURSB_Rt_ADDR_SIMM9::make_opcode(insn),
                                                    );
                                                }
                                            } else {
                                                if insn & 0xffe00c00 == 0xb8800000 {
                                                    return Some(
                                                        LDURSW_Rt_ADDR_SIMM9::make_opcode(insn),
                                                    );
                                                }
                                            }
                                        } else {
                                            if insn & 0xffa00c00 == 0x78800000 {
                                                return Some(LDURSH_Rt_ADDR_SIMM9::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x200000 == 0 {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xffe00c00 == 0x38000800 {
                                                    return Some(STTRB_Rt_ADDR_SIMM9::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            } else {
                                                if insn & 0xffe00c00 == 0x78000800 {
                                                    return Some(STTRH_Rt_ADDR_SIMM9::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        } else {
                                            if insn & 0xbfe00c00 == 0xb8000800 {
                                                return Some(STTR_Rt_ADDR_SIMM9::make_opcode(insn));
                                            }
                                        }
                                    } else {
                                        if insn & 0x800000 == 0 {
                                            if insn & 0x400000 == 0 {
                                                if insn & 0x80000000 == 0 {
                                                    if insn & 0x40000000 == 0 {
                                                        if insn & 0xffe00c00 == 0x38200800 {
                                                            return Some(
                                                                STRB_Rt_ADDR_REGOFF::make_opcode(
                                                                    insn,
                                                                ),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xffe00c00 == 0x78200800 {
                                                            return Some(
                                                                STRH_Rt_ADDR_REGOFF::make_opcode(
                                                                    insn,
                                                                ),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xbfe00c00 == 0xb8200800 {
                                                        return Some(
                                                            STR_Rt_ADDR_REGOFF::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0x80000000 == 0 {
                                                    if insn & 0x40000000 == 0 {
                                                        if insn & 0xffe00c00 == 0x38600800 {
                                                            return Some(
                                                                LDRB_Rt_ADDR_REGOFF::make_opcode(
                                                                    insn,
                                                                ),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xffe00c00 == 0x78600800 {
                                                            return Some(
                                                                LDRH_Rt_ADDR_REGOFF::make_opcode(
                                                                    insn,
                                                                ),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xbfe00c00 == 0xb8600800 {
                                                        return Some(
                                                            LDR_Rt_ADDR_REGOFF::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0x80000000 == 0 {
                                                    if insn & 0xffa00c00 == 0x38a00800 {
                                                        return Some(
                                                            LDRSB_Rt_ADDR_REGOFF::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0xb8a00800 {
                                                        return Some(
                                                            LDRSW_Rt_ADDR_REGOFF::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0xffa00c00 == 0x78a00800 {
                                                    return Some(
                                                        LDRSH_Rt_ADDR_REGOFF::make_opcode(insn),
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x200000 == 0 {
                                    if insn & 0x800000 == 0 {
                                        if insn & 0x400000 == 0 {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xffe00400 == 0x38000400 {
                                                        return Some(
                                                            STRB_Rt_ADDR_SIMM9::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xffe00400 == 0x78000400 {
                                                        return Some(
                                                            STRH_Rt_ADDR_SIMM9::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbfe00400 == 0xb8000400 {
                                                    return Some(STR_Rt_ADDR_SIMM9::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        } else {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xffe00400 == 0x38400400 {
                                                        return Some(
                                                            LDRB_Rt_ADDR_SIMM9::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xffe00400 == 0x78400400 {
                                                        return Some(
                                                            LDRH_Rt_ADDR_SIMM9::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbfe00400 == 0xb8400400 {
                                                    return Some(LDR_Rt_ADDR_SIMM9::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0xffa00400 == 0x38800400 {
                                                    return Some(LDRSB_Rt_ADDR_SIMM9::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            } else {
                                                if insn & 0xffe00400 == 0xb8800400 {
                                                    return Some(LDRSW_Rt_ADDR_SIMM9::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        } else {
                                            if insn & 0xffa00400 == 0x78800400 {
                                                return Some(LDRSH_Rt_ADDR_SIMM9::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x800000 == 0 {
                                        if insn & 0xffa00400 == 0xf8200400 {
                                            return Some(LDRAA_Rt_ADDR_SIMM10::make_opcode(insn));
                                        }
                                    } else {
                                        if insn & 0xffa00400 == 0xf8a00400 {
                                            return Some(LDRAB_Rt_ADDR_SIMM10::make_opcode(insn));
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        if insn & 0x800000 == 0 {
                            if insn & 0x400000 == 0 {
                                if insn & 0x20000000 == 0 {
                                    if insn & 0xfffffc00 == 0xd9200000 {
                                        return Some(STZGM_Rt_ADDR_SIMPLE::make_opcode(insn));
                                    }
                                } else {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xffc00000 == 0x39000000 {
                                                return Some(STRB_Rt_ADDR_UIMM12::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        } else {
                                            if insn & 0xffc00000 == 0x79000000 {
                                                return Some(STRH_Rt_ADDR_UIMM12::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    } else {
                                        if insn & 0xbfc00000 == 0xb9000000 {
                                            return Some(STR_Rt_ADDR_UIMM12::make_opcode(insn));
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x20000000 == 0 {
                                    if insn & 0x000400 == 0 {
                                        if insn & 0xffe00c00 == 0xd9600800 {
                                            return Some(STZG_Rt_SP_ADDR_SIMM13::make_opcode(insn));
                                        }
                                    } else {
                                        if insn & 0xffe00400 == 0xd9600400 {
                                            return Some(
                                                STZG_Rt_SP_X_ADDR_SIMM13_imm_tag::make_opcode(insn),
                                            );
                                        }
                                    }
                                } else {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xffc00000 == 0x39400000 {
                                                return Some(LDRB_Rt_ADDR_UIMM12::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        } else {
                                            if insn & 0xffc00000 == 0x79400000 {
                                                return Some(LDRH_Rt_ADDR_UIMM12::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    } else {
                                        if insn & 0xbfc00000 == 0xb9400000 {
                                            return Some(LDR_Rt_ADDR_UIMM12::make_opcode(insn));
                                        }
                                    }
                                }
                            }
                        } else {
                            if insn & 0x20000000 == 0 {
                                if insn & 0x000400 == 0 {
                                    if insn & 0xffe00c00 == 0xd9e00800 {
                                        return Some(STZ2G_Rt_SP_ADDR_SIMM13::make_opcode(insn));
                                    }
                                } else {
                                    if insn & 0xffe00400 == 0xd9e00400 {
                                        return Some(
                                            STZ2G_Rt_SP_X_ADDR_SIMM13_imm_tag::make_opcode(insn),
                                        );
                                    }
                                }
                            } else {
                                if insn & 0x40000000 == 0 {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0xff800000 == 0x39800000 {
                                            return Some(LDRSB_Rt_ADDR_UIMM12::make_opcode(insn));
                                        }
                                    } else {
                                        if insn & 0xffc00000 == 0xb9800000 {
                                            return Some(LDRSW_Rt_ADDR_UIMM12::make_opcode(insn));
                                        }
                                    }
                                } else {
                                    if insn & 0xff800000 == 0x79800000 {
                                        return Some(LDRSH_Rt_ADDR_UIMM12::make_opcode(insn));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            if insn & 0x1000000 == 0 {
                if insn & 0x8000000 == 0 {
                    if insn & 0x800000 == 0 {
                        if insn & 0x20000000 == 0 {
                            if insn & 0x40000000 == 0 {
                                if insn & 0x7f800000 == 0x12000000 {
                                    return Some(AND_Rd_SP_Rn_LIMM::make_opcode(insn));
                                }
                            } else {
                                if insn & 0x7f800000 == 0x52000000 {
                                    return Some(EOR_Rd_SP_Rn_LIMM::make_opcode(insn));
                                }
                            }
                        } else {
                            if insn & 0x40000000 == 0 {
                                if insn & 0x7f800000 == 0x32000000 {
                                    return Some(ORR_Rd_SP_Rn_LIMM::make_opcode(insn));
                                }
                            } else {
                                if insn & 0x7f800000 == 0x72000000 {
                                    return Some(ANDS_Rd_Rn_LIMM::make_opcode(insn));
                                }
                            }
                        }
                    } else {
                        if insn & 0x20000000 == 0 {
                            if insn & 0x40000000 == 0 {
                                if insn & 0x7f800000 == 0x12800000 {
                                    return Some(MOVN_Rd_HALF::make_opcode(insn));
                                }
                            } else {
                                if insn & 0x7f800000 == 0x52800000 {
                                    return Some(MOVZ_Rd_HALF::make_opcode(insn));
                                }
                            }
                        } else {
                            if insn & 0x7f800000 == 0x72800000 {
                                return Some(MOVK_Rd_HALF::make_opcode(insn));
                            }
                        }
                    }
                } else {
                    if insn & 0x200000 == 0 {
                        if insn & 0x10000000 == 0 {
                            if insn & 0x20000000 == 0 {
                                if insn & 0x40000000 == 0 {
                                    if insn & 0x7f200000 == 0xa000000 {
                                        return Some(AND_Rd_Rn_Rm_SFT::make_opcode(insn));
                                    }
                                } else {
                                    if insn & 0x7f200000 == 0x4a000000 {
                                        return Some(EOR_Rd_Rn_Rm_SFT::make_opcode(insn));
                                    }
                                }
                            } else {
                                if insn & 0x40000000 == 0 {
                                    if insn & 0x7f200000 == 0x2a000000 {
                                        return Some(ORR_Rd_Rn_Rm_SFT::make_opcode(insn));
                                    }
                                } else {
                                    if insn & 0x7f200000 == 0x6a000000 {
                                        return Some(ANDS_Rd_Rn_Rm_SFT::make_opcode(insn));
                                    }
                                }
                            }
                        } else {
                            if insn & 0x000400 == 0 {
                                if insn & 0x000800 == 0 {
                                    if insn & 0x400000 == 0 {
                                        if insn & 0x800000 == 0 {
                                            if insn & 0x20000000 == 0 {
                                                if insn & 0x7fe0fc00 == 0x5a000000 {
                                                    return Some(SBC_Rd_Rn_Rm::make_opcode(insn));
                                                }
                                            } else {
                                                if insn & 0x7fe0fc00 == 0x7a000000 {
                                                    return Some(SBCS_Rd_Rn_Rm::make_opcode(insn));
                                                }
                                            }
                                        } else {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0x7fe00c00 == 0x1a800000 {
                                                    return Some(CSEL_Rd_Rn_Rm_COND::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            } else {
                                                if insn & 0x7fe00c00 == 0x5a800000 {
                                                    return Some(CSINV_Rd_Rn_Rm_COND::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x800000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0x7fe00c10 == 0x3a400000 {
                                                    return Some(
                                                        CCMN_Rn_Rm_NZCV_COND::make_opcode(insn),
                                                    );
                                                }
                                            } else {
                                                if insn & 0x7fe00c10 == 0x7a400000 {
                                                    return Some(
                                                        CCMP_Rn_Rm_NZCV_COND::make_opcode(insn),
                                                    );
                                                }
                                            }
                                        } else {
                                            if insn & 0x001000 == 0 {
                                                if insn & 0x002000 == 0 {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0xffe0fc00 == 0x9ac00000 {
                                                            return Some(
                                                                SUBP_Rd_Rn_SP_Rm_SP::make_opcode(
                                                                    insn,
                                                                ),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xffe0fc00 == 0xbac00000 {
                                                            return Some(
                                                                SUBPS_Rd_Rn_SP_Rm_SP::make_opcode(
                                                                    insn,
                                                                ),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x7fe0fc00 == 0x1ac02000 {
                                                        return Some(LSLV_Rd_Rn_Rm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                }
                                            } else {
                                                if insn & 0x7ffffc00 == 0x5ac01000 {
                                                    return Some(CLZ_Rd_Rn::make_opcode(insn));
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x800000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0x7fe00c10 == 0x3a400800 {
                                                return Some(
                                                    CCMN_Rn_CCMP_IMM_NZCV_COND::make_opcode(insn),
                                                );
                                            }
                                        } else {
                                            if insn & 0x7fe00c10 == 0x7a400800 {
                                                return Some(
                                                    CCMP_Rn_CCMP_IMM_NZCV_COND::make_opcode(insn),
                                                );
                                            }
                                        }
                                    } else {
                                        if insn & 0x001000 == 0 {
                                            if insn & 0x7fe0fc00 == 0x1ac02800 {
                                                return Some(ASRV_Rd_Rn_Rm::make_opcode(insn));
                                            }
                                        } else {
                                            if insn & 0x7ffffc00 == 0x5ac01800 {
                                                return Some(CTZ_Rd_Rn::make_opcode(insn));
                                            }
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x000800 == 0 {
                                    if insn & 0x400000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0x7fe00c00 == 0x1a800400 {
                                                return Some(CSINC_Rd_Rn_Rm_COND::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        } else {
                                            if insn & 0x7fe00c00 == 0x5a800400 {
                                                return Some(CSNEG_Rd_Rn_Rm_COND::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    } else {
                                        if insn & 0x001000 == 0 {
                                            if insn & 0x7fe0fc00 == 0x1ac02400 {
                                                return Some(LSRV_Rd_Rn_Rm::make_opcode(insn));
                                            }
                                        } else {
                                            if insn & 0x7ffffc00 == 0x5ac01400 {
                                                return Some(CLS_Rd_Rn::make_opcode(insn));
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x001000 == 0 {
                                        if insn & 0x7fe0fc00 == 0x1ac00c00 {
                                            return Some(SDIV_Rd_Rn_Rm::make_opcode(insn));
                                        }
                                    } else {
                                        if insn & 0x7ffffc00 == 0x5ac01c00 {
                                            return Some(CNT_Rd_Rn::make_opcode(insn));
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        if insn & 0x20000000 == 0 {
                            if insn & 0x7f200000 == 0xa200000 {
                                return Some(BIC_Rd_Rn_Rm_SFT::make_opcode(insn));
                            }
                        } else {
                            if insn & 0x40000000 == 0 {
                                if insn & 0x7f200000 == 0x2a200000 {
                                    return Some(ORN_Rd_Rn_Rm_SFT::make_opcode(insn));
                                }
                            } else {
                                if insn & 0x7f200000 == 0x6a200000 {
                                    return Some(BICS_Rd_Rn_Rm_SFT::make_opcode(insn));
                                }
                            }
                        }
                    }
                }
            } else {
                if insn & 0x8000000 == 0 {
                    if insn & 0x20000000 == 0 {
                        if insn & 0x40000000 == 0 {
                            if insn & 0x7f800000 == 0x13000000 {
                                return Some(SBFM_Rd_Rn_IMMR_IMMS::make_opcode(insn));
                            }
                        } else {
                            if insn & 0x7f800000 == 0x53000000 {
                                return Some(UBFM_Rd_Rn_IMMR_IMMS::make_opcode(insn));
                            }
                        }
                    } else {
                        if insn & 0x7f800000 == 0x33000000 {
                            return Some(BFM_Rd_Rn_IMMR_IMMS::make_opcode(insn));
                        }
                    }
                } else {
                    if insn & 0x200000 == 0 {
                        if insn & 0x10000000 == 0 {
                            if insn & 0x20000000 == 0 {
                                if insn & 0x40000000 == 0 {
                                    if insn & 0x7f200000 == 0xb000000 {
                                        return Some(ADD_Rd_Rn_Rm_SFT::make_opcode(insn));
                                    }
                                } else {
                                    if insn & 0x7f200000 == 0x4b000000 {
                                        return Some(SUB_Rd_Rn_Rm_SFT::make_opcode(insn));
                                    }
                                }
                            } else {
                                if insn & 0x40000000 == 0 {
                                    if insn & 0x7f200000 == 0x2b000000 {
                                        return Some(ADDS_Rd_Rn_Rm_SFT::make_opcode(insn));
                                    }
                                } else {
                                    if insn & 0x7f200000 == 0x6b000000 {
                                        return Some(SUBS_Rd_Rn_Rm_SFT::make_opcode(insn));
                                    }
                                }
                            }
                        } else {
                            if insn & 0x008000 == 0 {
                                if insn & 0x400000 == 0 {
                                    if insn & 0x7fe08000 == 0x1b000000 {
                                        return Some(MADD_Rd_Rn_Rm_Ra::make_opcode(insn));
                                    }
                                } else {
                                    if insn & 0xffe0fc00 == 0x9b407c00 {
                                        return Some(SMULH_Rd_Rn_Rm::make_opcode(insn));
                                    }
                                }
                            } else {
                                if insn & 0x7fe08000 == 0x1b008000 {
                                    return Some(MSUB_Rd_Rn_Rm_Ra::make_opcode(insn));
                                }
                            }
                        }
                    } else {
                        if insn & 0x10000000 == 0 {
                            if insn & 0x20000000 == 0 {
                                if insn & 0x40000000 == 0 {
                                    if insn & 0x7fe00000 == 0xb200000 {
                                        return Some(ADD_Rd_SP_Rn_SP_Rm_EXT::make_opcode(insn));
                                    }
                                } else {
                                    if insn & 0x7fe00000 == 0x4b200000 {
                                        return Some(SUB_Rd_SP_Rn_SP_Rm_EXT::make_opcode(insn));
                                    }
                                }
                            } else {
                                if insn & 0x40000000 == 0 {
                                    if insn & 0x7fe00000 == 0x2b200000 {
                                        return Some(ADDS_Rd_Rn_SP_Rm_EXT::make_opcode(insn));
                                    }
                                } else {
                                    if insn & 0x7fe00000 == 0x6b200000 {
                                        return Some(SUBS_Rd_Rn_SP_Rm_EXT::make_opcode(insn));
                                    }
                                }
                            }
                        } else {
                            if insn & 0xffe08000 == 0x9b200000 {
                                return Some(SMADDL_Rd_Rn_Rm_Ra::make_opcode(insn));
                            }
                        }
                    }
                }
            }
        }
    } else {
        if insn & 0x8000000 == 0 {
            if insn & 0x10000000 == 0 {
                if insn & 0x1000000 == 0 {
                    if insn & 0x200000 == 0 {
                        if insn & 0x008000 == 0 {
                            if insn & 0x20000000 == 0 {
                                if insn & 0x002000 == 0 {
                                    if insn & 0x40000000 == 0 {
                                        if insn & 0x010000 == 0 {
                                            if insn & 0x020000 == 0 {
                                                if insn & 0x040000 == 0 {
                                                    if insn & 0x080000 == 0 {
                                                        if insn & 0x100000 == 0 {
                                                            if insn & 0xff3fe000 == 0x4000000 {
                                                                return Some (ADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xff3fe000 == 0x4100000 {
                                                                return Some (MUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0xff3fe000 == 0x4180000 {
                                                            return Some (ORR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xffbfe000 == 0x4940000 {
                                                        return Some (SDIV_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                    }
                                                }
                                            } else {
                                                if insn & 0x040000 == 0 {
                                                    if insn & 0x080000 == 0 {
                                                        if insn & 0xff3fe000 == 0x4120000 {
                                                            return Some (SMULH_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0xff3fe000 == 0x41a0000 {
                                                            return Some (AND_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xffbfe000 == 0x4960000 {
                                                        return Some (SDIVR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x020000 == 0 {
                                                if insn & 0x080000 == 0 {
                                                    if insn & 0xff3fe000 == 0x4010000 {
                                                        return Some (SUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0xff3fe000 == 0x4190000 {
                                                        return Some (EOR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                    }
                                                }
                                            } else {
                                                if insn & 0x080000 == 0 {
                                                    if insn & 0xff3fe000 == 0x4030000 {
                                                        return Some (SUBR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0xff3fe000 == 0x41b0000 {
                                                        return Some (BIC_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x001000 == 0 {
                                            if insn & 0xffa0fc00 == 0x44800000 {
                                                return Some(
                                                    SDOT_SVE_Zd_SVE_Zn_SVE_Zm_16::make_opcode(insn),
                                                );
                                            }
                                        } else {
                                            if insn & 0xffa0f000 == 0x44801000 {
                                                return Some (CDOT_SVE_Zd_SVE_Zn_SVE_Zm_16_SVE_IMM_ROT2 :: make_opcode (insn)) ;
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x40000000 == 0 {
                                        if insn & 0x020000 == 0 {
                                            if insn & 0x040000 == 0 {
                                                if insn & 0x080000 == 0 {
                                                    if insn & 0x100000 == 0 {
                                                        if insn & 0xff3fe000 == 0x4012000 {
                                                            return Some (UADDV_SVE_Vd_SVE_Pg3_SVE_Zn :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0xff3ee000 == 0x4102000 {
                                                            return Some (MOVPRFX_SVE_Zd_SVE_Pg3_SVE_Zn :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x010000 == 0 {
                                                        if insn & 0xff3fe000 == 0x4182000 {
                                                            return Some (ORV_SVE_Vd_SVE_Pg3_SVE_Zn :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0xff3fe000 == 0x4192000 {
                                                            return Some (EORV_SVE_Vd_SVE_Pg3_SVE_Zn :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x080000 == 0 {
                                                    if insn & 0xff3fe000 == 0x4052000 {
                                                        return Some(
                                                            ADDQV_Vd_SVE_Pg3_SVE_Zn::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xff3fe000 == 0x41d2000 {
                                                        return Some(
                                                            EORQV_Vd_SVE_Pg3_SVE_Zn::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x040000 == 0 {
                                                if insn & 0xff3fe000 == 0x41a2000 {
                                                    return Some(
                                                        ANDV_SVE_Vd_SVE_Pg3_SVE_Zn::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            } else {
                                                if insn & 0xff3fe000 == 0x41e2000 {
                                                    return Some(
                                                        ANDQV_Vd_SVE_Pg3_SVE_Zn::make_opcode(insn),
                                                    );
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0xff20f000 == 0x44002000 {
                                            return Some (CMLA_SVE_Zd_SVE_Zn_SVE_Zm_16_SVE_IMM_ROT2 :: make_opcode (insn)) ;
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x40000000 == 0 {
                                    if insn & 0x000010 == 0 {
                                        if insn & 0x002000 == 0 {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0xff20e010 == 0x24000000 {
                                                    return Some (CMPHS_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                }
                                            } else {
                                                if insn & 0xff20e010 == 0x24004000 {
                                                    return Some (CMPGE_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                }
                                            }
                                        } else {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0xff20e010 == 0x24002000 {
                                                    return Some (CMPEQ_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                }
                                            } else {
                                                if insn & 0xff20e010 == 0x24006000 {
                                                    return Some (CMPLT_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x002000 == 0 {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0xff20e010 == 0x24000010 {
                                                    return Some (CMPHI_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                }
                                            } else {
                                                if insn & 0xff20e010 == 0x24004010 {
                                                    return Some (CMPGT_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                }
                                            }
                                        } else {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0xff20e010 == 0x24002010 {
                                                    return Some (CMPNE_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                }
                                            } else {
                                                if insn & 0xff20e010 == 0x24006010 {
                                                    return Some (CMPLE_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0xff208000 == 0x64000000 {
                                        return Some (FCMLA_SVE_Zd_SVE_Pg3_SVE_Zn_SVE_Zm_16_IMM_ROT2 :: make_opcode (insn)) ;
                                    }
                                }
                            }
                        } else {
                            if insn & 0x002000 == 0 {
                                if insn & 0x004000 == 0 {
                                    if insn & 0x20000000 == 0 {
                                        if insn & 0x010000 == 0 {
                                            if insn & 0x040000 == 0 {
                                                if insn & 0x080000 == 0 {
                                                    if insn & 0x100000 == 0 {
                                                        if insn & 0xff3fe000 == 0x4008000 {
                                                            return Some (ASR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHRIMM_PRED :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0xff3fe000 == 0x4108000 {
                                                            return Some (ASR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xff3fe000 == 0x4188000 {
                                                        return Some (ASR_SVE_Zd_S_B_SVE_Pg3_P_M_SVE_Zd_S_B_SVE_Zm_5_S_D :: make_opcode (insn)) ;
                                                    }
                                                }
                                            } else {
                                                if insn & 0x100000 == 0 {
                                                    if insn & 0xff3fe000 == 0x4048000 {
                                                        return Some (ASRD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHRIMM_PRED :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0xff3fe000 == 0x4148000 {
                                                        return Some (ASRR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x020000 == 0 {
                                                if insn & 0x040000 == 0 {
                                                    if insn & 0x080000 == 0 {
                                                        if insn & 0x100000 == 0 {
                                                            if insn & 0xff3fe000 == 0x4018000 {
                                                                return Some (LSR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHRIMM_PRED :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xff3fe000 == 0x4118000 {
                                                                return Some (LSR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0xff3fe000 == 0x4198000 {
                                                            return Some (LSR_SVE_Zd_S_B_SVE_Pg3_P_M_SVE_Zd_S_B_SVE_Zm_5_S_D :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xff3fe000 == 0x4158000 {
                                                        return Some (LSRR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                    }
                                                }
                                            } else {
                                                if insn & 0x040000 == 0 {
                                                    if insn & 0x080000 == 0 {
                                                        if insn & 0x100000 == 0 {
                                                            if insn & 0xff3fe000 == 0x4038000 {
                                                                return Some (LSL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHLIMM_PRED :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xff3fe000 == 0x4138000 {
                                                                return Some (LSL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0xff3fe000 == 0x41b8000 {
                                                            return Some (LSL_SVE_Zd_S_B_SVE_Pg3_P_M_SVE_Zd_S_B_SVE_Zm_5_S_D :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xff3fe000 == 0x4178000 {
                                                        return Some (LSLR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0x000010 == 0 {
                                                if insn & 0xff20e010 == 0x24008000 {
                                                    return Some (CMPGE_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_B :: make_opcode (insn)) ;
                                                }
                                            } else {
                                                if insn & 0xff20e010 == 0x24008010 {
                                                    return Some (CMPGT_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_B :: make_opcode (insn)) ;
                                                }
                                            }
                                        } else {
                                            if insn & 0x100000 == 0 {
                                                if insn & 0xff3ee000 == 0x64008000 {
                                                    return Some (FCADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5_SVE_IMM_ROT1 :: make_opcode (insn)) ;
                                                }
                                            } else {
                                                if insn & 0xff3fe000 == 0x64108000 {
                                                    return Some (FADDP_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zn :: make_opcode (insn)) ;
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x20000000 == 0 {
                                        if insn & 0x000400 == 0 {
                                            if insn & 0x800000 == 0 {
                                                if insn & 0xffe0fc00 == 0x4400c800 {
                                                    return Some (SDOT_SVE_Zd_S_S_SVE_Zn_S_H_SVE_Zm_16_S_H :: make_opcode (insn)) ;
                                                }
                                            } else {
                                                if insn & 0xffe0fc00 == 0x4480c800 {
                                                    return Some (SDOT_SVE_Zd_SVE_Zn_SVE_Zm3_19_INDEX :: make_opcode (insn)) ;
                                                }
                                            }
                                        } else {
                                            if insn & 0xff20fc00 == 0x4400c400 {
                                                return Some(
                                                    UCLAMP_SVE_Zd_SVE_Zn_SVE_Zm_16::make_opcode(
                                                        insn,
                                                    ),
                                                );
                                            }
                                        }
                                    } else {
                                        if insn & 0x000010 == 0 {
                                            if insn & 0xff20e010 == 0x2400c000 {
                                                return Some (CMPHS_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_D :: make_opcode (insn)) ;
                                            }
                                        } else {
                                            if insn & 0xff20e010 == 0x2400c010 {
                                                return Some (CMPHI_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_D :: make_opcode (insn)) ;
                                            }
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x004000 == 0 {
                                    if insn & 0x20000000 == 0 {
                                        if insn & 0x010000 == 0 {
                                            if insn & 0x020000 == 0 {
                                                if insn & 0x040000 == 0 {
                                                    if insn & 0x080000 == 0 {
                                                        if insn & 0xff3fe000 == 0x410a000 {
                                                            return Some (SXTB_SVE_Zd_SVE_Pg3_SVE_Zn :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0xff3fe000 == 0x418a000 {
                                                            return Some (CLS_SVE_Zd_SVE_Pg3_SVE_Zn :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xffffe000 == 0x4d4a000 {
                                                        return Some(
                                                            SXTW_SVE_Zd_SVE_Pg3_SVE_Zn::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0x080000 == 0 {
                                                    if insn & 0xffbfe000 == 0x492a000 {
                                                        return Some(
                                                            SXTH_SVE_Zd_SVE_Pg3_SVE_Zn::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xff3fe000 == 0x41aa000 {
                                                        return Some(
                                                            CNT_SVE_Zd_SVE_Pg3_SVE_Zn::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x020000 == 0 {
                                                if insn & 0x080000 == 0 {
                                                    if insn & 0xff3fe000 == 0x4411a000 {
                                                        return Some (ADDP_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zn :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0xff3fe000 == 0x419a000 {
                                                        return Some(
                                                            CLZ_SVE_Zd_SVE_Pg3_SVE_Zn::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0x040000 == 0 {
                                                    if insn & 0xff3fe000 == 0x41ba000 {
                                                        return Some(
                                                            CNOT_SVE_Zd_SVE_Pg3_SVE_Zn::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xff3fe000 == 0x417a000 {
                                                        return Some(
                                                            NEG_SVE_Zd_SVE_Pg3_SVE_Zn::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0x000010 == 0 {
                                                if insn & 0xff20e010 == 0x2400a000 {
                                                    return Some (CMPEQ_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_B :: make_opcode (insn)) ;
                                                }
                                            } else {
                                                if insn & 0xff20e010 == 0x2400a010 {
                                                    return Some (CMPNE_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_B :: make_opcode (insn)) ;
                                                }
                                            }
                                        } else {
                                            if insn & 0x010000 == 0 {
                                                if insn & 0x020000 == 0 {
                                                    if insn & 0x080000 == 0 {
                                                        if insn & 0xff3fe000 == 0x6410a000 {
                                                            return Some (FADDQV_Vd_SVE_Pg3_SVE_Zn :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0xffffe000 == 0x6488a000 {
                                                            return Some (FCVTNT_SVE_Zd_SVE_Pg3_SVE_Zn :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x400000 == 0 {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0xffffe000 == 0x640aa000 {
                                                                return Some (FCVTXNT_SVE_Zd_SVE_Pg3_SVE_Zn :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xffffe000 == 0x648aa000 {
                                                                return Some (BFCVTNT_SVE_Zd_SVE_Pg3_SVE_Zn :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0xffffe000 == 0x64caa000 {
                                                            return Some (FCVTNT_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_D :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x020000 == 0 {
                                                    if insn & 0xffffe000 == 0x6489a000 {
                                                        return Some (FCVTLT_SVE_Zd_SVE_Pg3_SVE_Zn :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0xffffe000 == 0x64cba000 {
                                                        return Some (FCVTLT_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_S :: make_opcode (insn)) ;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x000010 == 0 {
                                        if insn & 0xff20e010 == 0x2400e000 {
                                            return Some(
                                                CMPLO_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16::make_opcode(
                                                    insn,
                                                ),
                                            );
                                        }
                                    } else {
                                        if insn & 0xff20e010 == 0x2400e010 {
                                            return Some(
                                                CMPLS_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16::make_opcode(
                                                    insn,
                                                ),
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        if insn & 0x002000 == 0 {
                            if insn & 0x20000000 == 0 {
                                if insn & 0x001000 == 0 {
                                    if insn & 0x004000 == 0 {
                                        if insn & 0x000400 == 0 {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xff20fc00 == 0x4200000 {
                                                        return Some (ADD_SVE_Zd_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0x400000 == 0 {
                                                        if insn & 0xffe0fc00 == 0x44a00000 {
                                                            return Some (SDOT_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0xffe0fc00 == 0x44e00000 {
                                                            return Some (SDOT_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0xff20fc00 == 0x4208000 {
                                                    return Some(
                                                        ASR_SVE_Zd_SVE_Zn_SVE_Zm_16::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            }
                                        } else {
                                            if insn & 0x000800 == 0 {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0xff20fc00 == 0x4200400 {
                                                        return Some (SUB_SVE_Zd_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0xff20fc00 == 0x4208400 {
                                                        return Some (LSR_SVE_Zd_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                    }
                                                }
                                            } else {
                                                if insn & 0xff20fc00 == 0x4208c00 {
                                                    return Some(
                                                        LSL_SVE_Zd_SVE_Zn_SVE_Zm_16::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x008000 == 0 {
                                            if insn & 0x400000 == 0 {
                                                if insn & 0xffe0f000 == 0x44a04000 {
                                                    return Some (CDOT_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX_SVE_IMM_ROT2 :: make_opcode (insn)) ;
                                                }
                                            } else {
                                                if insn & 0xffe0f000 == 0x44e04000 {
                                                    return Some (CDOT_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX_SVE_IMM_ROT2 :: make_opcode (insn)) ;
                                                }
                                            }
                                        } else {
                                            if insn & 0x000400 == 0 {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0xffe0f400 == 0x44a0c000 {
                                                        return Some (SMULLB_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0xffe0f400 == 0x44e0c000 {
                                                        return Some (SMULLB_SVE_Zd_SVE_Zn_SVE_Zm4_11_INDEX :: make_opcode (insn)) ;
                                                    }
                                                }
                                            } else {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0xffe0f400 == 0x44a0c400 {
                                                        return Some (SMULLT_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0xffe0f400 == 0x44e0c400 {
                                                        return Some (SMULLT_SVE_Zd_SVE_Zn_SVE_Zm4_11_INDEX :: make_opcode (insn)) ;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x000800 == 0 {
                                        if insn & 0x004000 == 0 {
                                            if insn & 0x000400 == 0 {
                                                if insn & 0xff20fc00 == 0x4209000 {
                                                    return Some (ASR_SVE_Zd_SVE_Zn_SVE_SHRIMM_UNPRED :: make_opcode (insn)) ;
                                                }
                                            } else {
                                                if insn & 0xff20fc00 == 0x4209400 {
                                                    return Some (LSR_SVE_Zd_SVE_Zn_SVE_SHRIMM_UNPRED :: make_opcode (insn)) ;
                                                }
                                            }
                                        } else {
                                            if insn & 0x400000 == 0 {
                                                if insn & 0xffe0f800 == 0x4205000 {
                                                    return Some (ADDVL_Rd_SP_SVE_Rn_SP_SVE_SIMM6 :: make_opcode (insn)) ;
                                                }
                                            } else {
                                                if insn & 0xffe0f800 == 0x4605000 {
                                                    return Some (ADDPL_Rd_SP_SVE_Rn_SP_SVE_SIMM6 :: make_opcode (insn)) ;
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x004000 == 0 {
                                            if insn & 0xff20fc00 == 0x4209c00 {
                                                return Some (LSL_SVE_Zd_SVE_Zn_SVE_SHLIMM_UNPRED :: make_opcode (insn)) ;
                                            }
                                        } else {
                                            if insn & 0x400000 == 0 {
                                                if insn & 0xffe0f800 == 0x4205800 {
                                                    return Some (ADDSVL_Rd_SP_SVE_Rn_SP_SVE_SIMM6 :: make_opcode (insn)) ;
                                                }
                                            } else {
                                                if insn & 0xffe0f800 == 0x4605800 {
                                                    return Some (ADDSPL_Rd_SP_SVE_Rn_SP_SVE_SIMM6 :: make_opcode (insn)) ;
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x40000000 == 0 {
                                    if insn & 0x000010 == 0 {
                                        if insn & 0xff202010 == 0x24200000 {
                                            return Some(
                                                CMPHS_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_UIMM7::make_opcode(
                                                    insn,
                                                ),
                                            );
                                        }
                                    } else {
                                        if insn & 0xff202010 == 0x24200010 {
                                            return Some(
                                                CMPHI_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_UIMM7::make_opcode(
                                                    insn,
                                                ),
                                            );
                                        }
                                    }
                                } else {
                                    if insn & 0x001000 == 0 {
                                        if insn & 0x000400 == 0 {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0x000800 == 0 {
                                                    if insn & 0x400000 == 0 {
                                                        if insn & 0xffe0fc00 == 0x64208000 {
                                                            return Some (FDOT_SVE_Zd_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0xffe0fc00 == 0x64608000 {
                                                                return Some (BFDOT_SVE_Zd_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xffe0fc00 == 0x64e08000 {
                                                                return Some (BFMLALB_SVE_Zd_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xffa0fc00 == 0x64200800 {
                                                        return Some (BFMLA_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX :: make_opcode (insn)) ;
                                                    }
                                                }
                                            } else {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0xffe0fc00 == 0x64204000 {
                                                        return Some (FDOT_SVE_Zd_SVE_Zn_SVE_Zm3_19_INDEX :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0xffe0fc00 == 0x64604000 {
                                                            return Some (BFDOT_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0xffe0f400 == 0x64e04000 {
                                                            return Some (BFMLALB_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0x000800 == 0 {
                                                    if insn & 0xffe0fc00 == 0x64e08400 {
                                                        return Some (BFMLALT_SVE_Zd_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0xffa0fc00 == 0x64200c00 {
                                                        return Some (BFMLS_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX :: make_opcode (insn)) ;
                                                    }
                                                }
                                            } else {
                                                if insn & 0xffe0f400 == 0x64e04400 {
                                                    return Some (BFMLALT_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX :: make_opcode (insn)) ;
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x400000 == 0 {
                                            if insn & 0xffe0f000 == 0x64a01000 {
                                                return Some (FCMLA_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX_SVE_IMM_ROT2 :: make_opcode (insn)) ;
                                            }
                                        } else {
                                            if insn & 0xffe0f000 == 0x64e01000 {
                                                return Some (FCMLA_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX_SVE_IMM_ROT2 :: make_opcode (insn)) ;
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            if insn & 0x20000000 == 0 {
                                if insn & 0x001000 == 0 {
                                    if insn & 0x004000 == 0 {
                                        if insn & 0x800000 == 0 {
                                            if insn & 0x400000 == 0 {
                                                if insn & 0xffe0f000 == 0x420a000 {
                                                    return Some(
                                                        ADR_SVE_Zd_SVE_ADDR_ZZ_SXTW::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            } else {
                                                if insn & 0xffe0f000 == 0x460a000 {
                                                    return Some(
                                                        ADR_SVE_Zd_SVE_ADDR_ZZ_UXTW::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            }
                                        } else {
                                            if insn & 0xffa0f000 == 0x4a0a000 {
                                                return Some(
                                                    ADR_SVE_Zd_SVE_ADDR_ZZ_LSL::make_opcode(insn),
                                                );
                                            }
                                        }
                                    } else {
                                        if insn & 0x008000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0x000800 == 0 {
                                                    if insn & 0xff20fc00 == 0x4206000 {
                                                        return Some (MUL_SVE_Zd_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0xff20fc00 == 0x4206800 {
                                                        return Some (SMULH_SVE_Zd_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                    }
                                                }
                                            } else {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0xffe0f000 == 0x44a06000 {
                                                        return Some (CMLA_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX_SVE_IMM_ROT2 :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0xffe0f000 == 0x44e06000 {
                                                        return Some (CMLA_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX_SVE_IMM_ROT2 :: make_opcode (insn)) ;
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x400000 == 0 {
                                                if insn & 0x800000 == 0 {
                                                    if insn & 0xfff0fc00 == 0x420e000 {
                                                        return Some(
                                                            CNTB_Rd_SVE_PATTERN_SCALED::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xfff0fc00 == 0x4a0e000 {
                                                        return Some(
                                                            CNTW_Rd_SVE_PATTERN_SCALED::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0x800000 == 0 {
                                                    if insn & 0xfff0fc00 == 0x460e000 {
                                                        return Some(
                                                            CNTH_Rd_SVE_PATTERN_SCALED::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xfff0fc00 == 0x4e0e000 {
                                                        return Some(
                                                            CNTD_Rd_SVE_PATTERN_SCALED::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x000400 == 0 {
                                        if insn & 0x000800 == 0 {
                                            if insn & 0x400000 == 0 {
                                                if insn & 0x800000 == 0 {
                                                    if insn & 0xffe0fc00 == 0x4203000 {
                                                        return Some (AND_SVE_Zd_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0x4a03000 {
                                                        return Some (EOR_SVE_Zd_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                    }
                                                }
                                            } else {
                                                if insn & 0x800000 == 0 {
                                                    if insn & 0xffe0fc00 == 0x4603000 {
                                                        return Some (ORR_SVE_Zd_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0x4e03000 {
                                                        return Some (BIC_SVE_Zd_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0xffe0fc00 == 0x4203800 {
                                                        return Some (EOR3_SVE_Zd_SVE_Zd_SVE_Zm_16_SVE_Zn :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0xff3ffc00 == 0x420b800 {
                                                        return Some(
                                                            FEXPA_SVE_Zd_SVE_Zn::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0x800000 == 0 {
                                                    if insn & 0xffa0fc00 == 0x4420f800 {
                                                        return Some (MUL_SVE_Zd_SVE_Zn_SVE_Zm3_22_INDEX :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0x400000 == 0 {
                                                        if insn & 0xffe0fc00 == 0x44a0f800 {
                                                            return Some (MUL_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0xffe0fc00 == 0x44e0f800 {
                                                            return Some (MUL_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0xfffffc00 == 0x420bc00 {
                                            return Some(MOVPRFX_SVE_Zd_SVE_Zn::make_opcode(insn));
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x40000000 == 0 {
                                    if insn & 0x000010 == 0 {
                                        if insn & 0xff202010 == 0x24202000 {
                                            return Some(
                                                CMPLO_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_UIMM7::make_opcode(
                                                    insn,
                                                ),
                                            );
                                        }
                                    } else {
                                        if insn & 0xff202010 == 0x24202010 {
                                            return Some(
                                                CMPLS_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_UIMM7::make_opcode(
                                                    insn,
                                                ),
                                            );
                                        }
                                    }
                                } else {
                                    if insn & 0x000400 == 0 {
                                        if insn & 0x004000 == 0 {
                                            if insn & 0x000800 == 0 {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0xffa0fc00 == 0x64202000 {
                                                            return Some (FMUL_SVE_Zd_SVE_Zn_SVE_Zm3_22_INDEX :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0x400000 == 0 {
                                                            if insn & 0xffe0fc00 == 0x64a02000 {
                                                                return Some (FMUL_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xffe0fc00 == 0x64e02000 {
                                                                return Some (FMUL_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0x64e0a000 {
                                                        return Some (BFMLSLB_SVE_Zd_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                    }
                                                }
                                            } else {
                                                if insn & 0xffa0fc00 == 0x64202800 {
                                                    return Some (BFMUL_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX :: make_opcode (insn)) ;
                                                }
                                            }
                                        } else {
                                            if insn & 0xffe0f400 == 0x64e06000 {
                                                return Some (BFMLSLB_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX :: make_opcode (insn)) ;
                                            }
                                        }
                                    } else {
                                        if insn & 0x004000 == 0 {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0xffe0fc00 == 0x64202400 {
                                                    return Some (BFCLAMP_SVE_Zd_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                }
                                                if insn & 0xff20fc00 == 0x64202400 {
                                                    return Some(
                                                        FCLAMP_SVE_Zd_SVE_Zn_SVE_Zm_16::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            } else {
                                                if insn & 0xffe0fc00 == 0x64e0a400 {
                                                    return Some (BFMLSLT_SVE_Zd_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                }
                                            }
                                        } else {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0xffe0f400 == 0x64e06400 {
                                                    return Some (BFMLSLT_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX :: make_opcode (insn)) ;
                                                }
                                            } else {
                                                if insn & 0xffe0fc00 == 0x6460e400 {
                                                    return Some(
                                                        BFMMLA_SVE_Zd_SVE_Zn_SVE_Zm_16::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if insn & 0x20000000 == 0 {
                        if insn & 0x40000000 == 0 {
                            if insn & 0x80000000 == 0 {
                                if insn & 0x200000 == 0 {
                                    if insn & 0x100000 == 0 {
                                        if insn & 0x400000 == 0 {
                                            if insn & 0x800000 == 0 {
                                                if insn & 0xfffc0000 == 0x5000000 {
                                                    return Some(
                                                        ORR_SVE_Zd_SVE_Zd_SVE_LIMM::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            } else {
                                                if insn & 0xfffc0000 == 0x5800000 {
                                                    return Some(
                                                        AND_SVE_Zd_SVE_Zd_SVE_LIMM::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            }
                                        } else {
                                            if insn & 0xfffc0000 == 0x5400000 {
                                                return Some(
                                                    EOR_SVE_Zd_SVE_Zd_SVE_LIMM::make_opcode(insn),
                                                );
                                            }
                                        }
                                    } else {
                                        if insn & 0xff30e000 == 0x510c000 {
                                            return Some(
                                                FCPY_SVE_Zd_SVE_Pg4_16_SVE_FPIMM8::make_opcode(
                                                    insn,
                                                ),
                                            );
                                        }
                                    }
                                } else {
                                    if insn & 0x002000 == 0 {
                                        if insn & 0x004000 == 0 {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0x000400 == 0 {
                                                    if insn & 0xffe0fc00 == 0x5a01800 {
                                                        return Some (TRN1_SVE_Zd_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0x5a01c00 {
                                                        return Some (TRN2_SVE_Zd_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                    }
                                                }
                                            } else {
                                                if insn & 0x010000 == 0 {
                                                    if insn & 0x020000 == 0 {
                                                        if insn & 0xff3fe000 == 0x5288000 {
                                                            return Some (CLASTA_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0xff3fe000 == 0x52a8000 {
                                                            return Some (CLASTA_SVE_Vd_SVE_Pg3_SVE_Vd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x020000 == 0 {
                                                        if insn & 0xff3fe000 == 0x5298000 {
                                                            return Some (CLASTB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0xff3fe000 == 0x52b8000 {
                                                            return Some (CLASTB_SVE_Vd_SVE_Pg3_SVE_Vd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x000400 == 0 {
                                                if insn & 0xff30fe10 == 0x5205000 {
                                                    return Some(
                                                        TRN1_SVE_Pd_SVE_Pn_SVE_Pm::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            } else {
                                                if insn & 0xff30fe10 == 0x5205400 {
                                                    return Some(
                                                        TRN2_SVE_Pd_SVE_Pn_SVE_Pm::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x004000 == 0 {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0x000400 == 0 {
                                                    if insn & 0x000800 == 0 {
                                                        if insn & 0xff20fc00 == 0x5203000 {
                                                            return Some (TBL_SVE_Zd_SVE_ZnxN_SVE_Zm_16 :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0xff20fc00 == 0x5202800 {
                                                            return Some (TBL_SVE_Zd_S_B_SVE_ZnxN_S_B_SVE_Zm_16_S_B :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xff20fc00 == 0x5202c00 {
                                                        return Some (TBX_SVE_Zd_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                    }
                                                }
                                            } else {
                                                if insn & 0x010000 == 0 {
                                                    if insn & 0xff3fe000 == 0x530a000 {
                                                        return Some (CLASTA_Rd_SVE_Pg3_Rd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0xff3fe000 == 0x531a000 {
                                                        return Some (CLASTB_Rd_SVE_Pg3_Rd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x000400 == 0 {
                                                if insn & 0xff20fc00 == 0x5207000 {
                                                    return Some (TRN1_SVE_Zd_S_B_SVE_Zn_S_B_SVE_Zm_16_S_B :: make_opcode (insn)) ;
                                                }
                                            } else {
                                                if insn & 0xff20fc00 == 0x5207400 {
                                                    return Some (TRN2_SVE_Zd_S_B_SVE_Zn_S_B_SVE_Zm_16_S_B :: make_opcode (insn)) ;
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x004000 == 0 {
                                    if insn & 0xffc0e010 == 0x85800000 {
                                        return Some(LDR_SVE_PNt_SVE_ADDR_RI_S9xVL::make_opcode(
                                            insn,
                                        ));
                                    }
                                    if insn & 0xffc0e010 == 0x85800000 {
                                        return Some(LDR_SVE_PNt_SVE_ADDR_RI_S9xVL::make_opcode(
                                            insn,
                                        ));
                                    }
                                } else {
                                    if insn & 0xffc0e000 == 0x85804000 {
                                        return Some(LDR_SVE_Zt_SVE_ADDR_RI_S9xVL::make_opcode(
                                            insn,
                                        ));
                                    }
                                }
                            }
                        } else {
                            if insn & 0x000400 == 0 {
                                if insn & 0x000800 == 0 {
                                    if insn & 0x001000 == 0 {
                                        if insn & 0xff20fc00 == 0x45206000 {
                                            return Some(
                                                ADDHNB_SVE_Zd_SVE_Zn_SVE_Zm_16::make_opcode(insn),
                                            );
                                        }
                                    } else {
                                        if insn & 0x002000 == 0 {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0xff20fc00 == 0x45009000 {
                                                    return Some(
                                                        EORBT_SVE_Zd_SVE_Zn_SVE_Zm_16::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            } else {
                                                if insn & 0xffa0fc00 == 0x4580d000 {
                                                    return Some(
                                                        SBCLB_SVE_Zd_SVE_Zn_SVE_Zm_16::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            }
                                        } else {
                                            if insn & 0x200000 == 0 {
                                                if insn & 0xff20fc00 == 0x45007000 {
                                                    return Some(
                                                        SMULLB_SVE_Zd_SVE_Zn_SVE_Zm_16::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            } else {
                                                if insn & 0xff20fc00 == 0x45207000 {
                                                    return Some(
                                                        SUBHNB_SVE_Zd_SVE_Zn_SVE_Zm_16::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x004000 == 0 {
                                        if insn & 0xff20fc00 == 0x45000800 {
                                            return Some(
                                                UADDLB_SVE_Zd_SVE_Zn_SVE_Zm_16::make_opcode(insn),
                                            );
                                        }
                                    } else {
                                        if insn & 0xff20fc00 == 0x45004800 {
                                            return Some(
                                                UADDWB_SVE_Zd_SVE_Zn_SVE_Zm_16::make_opcode(insn),
                                            );
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x000800 == 0 {
                                    if insn & 0x001000 == 0 {
                                        if insn & 0xff20fc00 == 0x45206400 {
                                            return Some(
                                                ADDHNT_SVE_Zd_SVE_Zn_SVE_Zm_16::make_opcode(insn),
                                            );
                                        }
                                    } else {
                                        if insn & 0x002000 == 0 {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0xff20fc00 == 0x45009400 {
                                                    return Some(
                                                        EORTB_SVE_Zd_SVE_Zn_SVE_Zm_16::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            } else {
                                                if insn & 0xffa0fc00 == 0x4580d400 {
                                                    return Some(
                                                        SBCLT_SVE_Zd_SVE_Zn_SVE_Zm_16::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            }
                                        } else {
                                            if insn & 0x200000 == 0 {
                                                if insn & 0xff20fc00 == 0x45007400 {
                                                    return Some(
                                                        SMULLT_SVE_Zd_SVE_Zn_SVE_Zm_16::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            } else {
                                                if insn & 0xff20fc00 == 0x45207400 {
                                                    return Some(
                                                        SUBHNT_SVE_Zd_SVE_Zn_SVE_Zm_16::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x004000 == 0 {
                                        if insn & 0xff20fc00 == 0x45000c00 {
                                            return Some(
                                                UADDLT_SVE_Zd_SVE_Zn_SVE_Zm_16::make_opcode(insn),
                                            );
                                        }
                                    } else {
                                        if insn & 0xff20fc00 == 0x45004c00 {
                                            return Some(
                                                UADDWT_SVE_Zd_SVE_Zn_SVE_Zm_16::make_opcode(insn),
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        if insn & 0x004000 == 0 {
                            if insn & 0x008000 == 0 {
                                if insn & 0x002000 == 0 {
                                    if insn & 0x40000000 == 0 {
                                        if insn & 0x000010 == 0 {
                                            if insn & 0xff20e010 == 0x25000000 {
                                                return Some(
                                                    CMPGE_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5::make_opcode(
                                                        insn,
                                                    ),
                                                );
                                            }
                                        } else {
                                            if insn & 0xff20e010 == 0x25000010 {
                                                return Some(
                                                    CMPGT_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5::make_opcode(
                                                        insn,
                                                    ),
                                                );
                                            }
                                        }
                                    } else {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0x200000 == 0 {
                                                if insn & 0x000400 == 0 {
                                                    if insn & 0x000800 == 0 {
                                                        if insn & 0xffe0fc00 == 0x65000000 {
                                                            return Some (BFADD_SVE_Zd_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                        }
                                                        if insn & 0xff20fc00 == 0x65000000 {
                                                            return Some (FADD_SVE_Zd_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0xffe0fc00 == 0x65000800 {
                                                            return Some (BFMUL_SVE_Zd_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                        }
                                                        if insn & 0xff20fc00 == 0x65000800 {
                                                            return Some (FMUL_SVE_Zd_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0x65000400 {
                                                        return Some (BFSUB_SVE_Zd_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                    }
                                                    if insn & 0xff20fc00 == 0x65000400 {
                                                        return Some (FSUB_SVE_Zd_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                    }
                                                }
                                            } else {
                                                if insn & 0xffe0e000 == 0x65200000 {
                                                    return Some (BFMLA_SVE_Zd_SVE_Pg3_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                }
                                            }
                                        } else {
                                            if insn & 0xffc0e010 == 0xe5800000 {
                                                return Some(
                                                    STR_SVE_PNt_SVE_ADDR_RI_S9xVL::make_opcode(
                                                        insn,
                                                    ),
                                                );
                                            }
                                            if insn & 0xffc0e010 == 0xe5800000 {
                                                return Some(
                                                    STR_SVE_PNt_SVE_ADDR_RI_S9xVL::make_opcode(
                                                        insn,
                                                    ),
                                                );
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x200000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0x000010 == 0 {
                                                if insn & 0xff20e010 == 0x25002000 {
                                                    return Some (CMPLT_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5 :: make_opcode (insn)) ;
                                                }
                                            } else {
                                                if insn & 0xff20e010 == 0x25002010 {
                                                    return Some (CMPLE_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5 :: make_opcode (insn)) ;
                                                }
                                            }
                                        } else {
                                            if insn & 0x010000 == 0 {
                                                if insn & 0x020000 == 0 {
                                                    if insn & 0x080000 == 0 {
                                                        if insn & 0x100000 == 0 {
                                                            if insn & 0xff3fe000 == 0x65002000 {
                                                                return Some (FADDV_SVE_Vd_SVE_Pg3_SVE_Zn :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0x000010 == 0 {
                                                                if insn & 0xff3fe010 == 0x65102000 {
                                                                    return Some (FCMGE_SVE_Pd_SVE_Pg3_SVE_Zn_FPIMM0 :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xff3fe010 == 0x65102010 {
                                                                    return Some (FCMGT_SVE_Pd_SVE_Pg3_SVE_Zn_FPIMM0 :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0xff3fe000 == 0x65182000 {
                                                            return Some (FADDA_SVE_Vd_SVE_Pg3_SVE_Vd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xff3fe010 == 0x65122000 {
                                                        return Some (FCMEQ_SVE_Pd_SVE_Pg3_SVE_Zn_FPIMM0 :: make_opcode (insn)) ;
                                                    }
                                                }
                                            } else {
                                                if insn & 0x000010 == 0 {
                                                    if insn & 0x020000 == 0 {
                                                        if insn & 0xff3fe010 == 0x65112000 {
                                                            return Some (FCMLT_SVE_Pd_SVE_Pg3_SVE_Zn_FPIMM0 :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0xff3fe010 == 0x65132000 {
                                                            return Some (FCMNE_SVE_Pd_SVE_Pg3_SVE_Zn_FPIMM0 :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xff3fe010 == 0x65112010 {
                                                        return Some (FCMLE_SVE_Pd_SVE_Pg3_SVE_Zn_FPIMM0 :: make_opcode (insn)) ;
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x800000 == 0 {
                                            if insn & 0xffe0e000 == 0x65202000 {
                                                return Some (BFMLS_SVE_Zd_SVE_Pg3_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                            }
                                        } else {
                                            if insn & 0x000010 == 0 {
                                                if insn & 0xffa0fc1f == 0x25a02000 {
                                                    return Some(CTERMEQ_Rn_Rm::make_opcode(insn));
                                                }
                                            } else {
                                                if insn & 0xffa0fc1f == 0x25a02010 {
                                                    return Some(CTERMNE_Rn_Rm::make_opcode(insn));
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x200000 == 0 {
                                    if insn & 0x002000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0x000010 == 0 {
                                                if insn & 0xff20e010 == 0x25008000 {
                                                    return Some (CMPEQ_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5 :: make_opcode (insn)) ;
                                                }
                                            } else {
                                                if insn & 0xff20e010 == 0x25008010 {
                                                    return Some (CMPNE_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5 :: make_opcode (insn)) ;
                                                }
                                            }
                                        } else {
                                            if insn & 0x010000 == 0 {
                                                if insn & 0x020000 == 0 {
                                                    if insn & 0x040000 == 0 {
                                                        if insn & 0x080000 == 0 {
                                                            if insn & 0xffffe000 == 0x65008000 {
                                                                return Some (BFADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                            }
                                                            if insn & 0xff3fe000 == 0x65008000 {
                                                                return Some (FADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xff3fe3c0 == 0x65188000 {
                                                                return Some (FADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_ONE :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x080000 == 0 {
                                                            if insn & 0xffffe000 == 0x65048000 {
                                                                return Some (BFMAXNM_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xff3fe000 == 0x650c8000 {
                                                                return Some (FDIVR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x040000 == 0 {
                                                        if insn & 0x080000 == 0 {
                                                            if insn & 0xffffe000 == 0x65028000 {
                                                                return Some (BFMUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                            }
                                                            if insn & 0xff3fe000 == 0x65028000 {
                                                                return Some (FMUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0x100000 == 0 {
                                                                if insn & 0xff3fe000 == 0x650a8000 {
                                                                    return Some (FMULX_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xff3fe3c0 == 0x651a8000 {
                                                                    return Some (FMUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_TWO :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0xffffe000 == 0x65068000 {
                                                            return Some (BFMAX_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x020000 == 0 {
                                                    if insn & 0x040000 == 0 {
                                                        if insn & 0x080000 == 0 {
                                                            if insn & 0xffffe000 == 0x65018000 {
                                                                return Some (BFSUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                            }
                                                            if insn & 0xff3fe000 == 0x65018000 {
                                                                return Some (FSUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xff3fe3c0 == 0x65198000 {
                                                                return Some (FSUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_ONE :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x080000 == 0 {
                                                            if insn & 0xffffe000 == 0x65058000 {
                                                                return Some (BFMINNM_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xff3fe000 == 0x650d8000 {
                                                                return Some (FDIV_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x040000 == 0 {
                                                        if insn & 0x080000 == 0 {
                                                            if insn & 0xff3fe000 == 0x65038000 {
                                                                return Some (FSUBR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xff3fe3c0 == 0x651b8000 {
                                                                return Some (FSUBR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_ONE :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0xffffe000 == 0x65078000 {
                                                            return Some (BFMIN_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x010000 == 0 {
                                            if insn & 0x020000 == 0 {
                                                if insn & 0x040000 == 0 {
                                                    if insn & 0x080000 == 0 {
                                                        if insn & 0xffffe000 == 0x65d0a000 {
                                                            return Some (SCVTF_SVE_Zd_SVE_Pg3_SVE_Zn :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0x100000 == 0 {
                                                            if insn & 0x400000 == 0 {
                                                                if insn & 0xffffe000 == 0x6588a000 {
                                                                    return Some (FCVT_SVE_Zd_SVE_Pg3_SVE_Zn :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xffffe000 == 0x65c8a000 {
                                                                    return Some (FCVT_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_D :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xffffe000 == 0x65d8a000 {
                                                                return Some (FCVTZS_SVE_Zd_SVE_Pg3_SVE_Zn :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x080000 == 0 {
                                                        if insn & 0x400000 == 0 {
                                                            if insn & 0xffffe000 == 0x6594a000 {
                                                                return Some (SCVTF_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_S :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0x800000 == 0 {
                                                                if insn & 0xffffe000 == 0x6554a000 {
                                                                    return Some (SCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_S :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xffffe000 == 0x65d4a000 {
                                                                    return Some (SCVTF_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_D :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x400000 == 0 {
                                                            if insn & 0xffffe000 == 0x659ca000 {
                                                                return Some (FCVTZS_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_S :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0x800000 == 0 {
                                                                if insn & 0xffffe000 == 0x655ca000 {
                                                                    return Some (FCVTZS_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_H :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xffffe000 == 0x65dca000 {
                                                                    return Some (FCVTZS_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_S :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x040000 == 0 {
                                                    if insn & 0x080000 == 0 {
                                                        if insn & 0xffffe000 == 0x6552a000 {
                                                            return Some (SCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_H :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0x100000 == 0 {
                                                            if insn & 0x400000 == 0 {
                                                                if insn & 0x800000 == 0 {
                                                                    if insn & 0xffffe000
                                                                        == 0x650aa000
                                                                    {
                                                                        return Some (FCVTX_SVE_Zd_SVE_Pg3_SVE_Zn :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffffe000
                                                                        == 0x658aa000
                                                                    {
                                                                        return Some (BFCVT_SVE_Zd_SVE_Pg3_SVE_Zn :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xffffe000 == 0x65caa000 {
                                                                    return Some (FCVT_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_D :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xffffe000 == 0x655aa000 {
                                                                return Some (FCVTZS_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_H :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x080000 == 0 {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0xffffe000 == 0x6556a000 {
                                                                return Some (SCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_D :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xffffe000 == 0x65d6a000 {
                                                                return Some (SCVTF_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_D :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0xffffe000 == 0x655ea000 {
                                                                return Some (FCVTZS_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_H :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xffffe000 == 0x65dea000 {
                                                                return Some (FCVTZS_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_D :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x020000 == 0 {
                                                if insn & 0x040000 == 0 {
                                                    if insn & 0x080000 == 0 {
                                                        if insn & 0xffffe000 == 0x65d1a000 {
                                                            return Some (UCVTF_SVE_Zd_SVE_Pg3_SVE_Zn :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0x100000 == 0 {
                                                            if insn & 0x400000 == 0 {
                                                                if insn & 0xffffe000 == 0x6589a000 {
                                                                    return Some (FCVT_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_H :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xffffe000 == 0x65c9a000 {
                                                                    return Some (FCVT_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_H :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xffffe000 == 0x65d9a000 {
                                                                return Some (FCVTZU_SVE_Zd_SVE_Pg3_SVE_Zn :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x080000 == 0 {
                                                        if insn & 0x400000 == 0 {
                                                            if insn & 0xffffe000 == 0x6595a000 {
                                                                return Some (UCVTF_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_S :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0x800000 == 0 {
                                                                if insn & 0xffffe000 == 0x6555a000 {
                                                                    return Some (UCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_S :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xffffe000 == 0x65d5a000 {
                                                                    return Some (UCVTF_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_D :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x400000 == 0 {
                                                            if insn & 0xffffe000 == 0x659da000 {
                                                                return Some (FCVTZU_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_S :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0x800000 == 0 {
                                                                if insn & 0xffffe000 == 0x655da000 {
                                                                    return Some (FCVTZU_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_H :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xffffe000 == 0x65dda000 {
                                                                    return Some (FCVTZU_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_S :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x040000 == 0 {
                                                    if insn & 0x080000 == 0 {
                                                        if insn & 0xffffe000 == 0x6553a000 {
                                                            return Some (UCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_H :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0x100000 == 0 {
                                                            if insn & 0xffffe000 == 0x65cba000 {
                                                                return Some (FCVT_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_S :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xffffe000 == 0x655ba000 {
                                                                return Some (FCVTZU_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_H :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x080000 == 0 {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0xffffe000 == 0x6557a000 {
                                                                return Some (UCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_D :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xffffe000 == 0x65d7a000 {
                                                                return Some (UCVTF_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_D :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0xffffe000 == 0x655fa000 {
                                                                return Some (FCVTZU_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_H :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xffffe000 == 0x65dfa000 {
                                                                return Some (FCVTZU_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_D :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x40000000 == 0 {
                                        if insn & 0x000200 == 0 {
                                            if insn & 0xff3fc200 == 0x25208000 {
                                                return Some(
                                                    CNTP_Rd_SVE_Pg4_10_SVE_Pn::make_opcode(insn),
                                                );
                                            }
                                        } else {
                                            if insn & 0xff3ffa00 == 0x25208200 {
                                                return Some(
                                                    CNTP_Rd_SME_PNn_SME_VLxN_10::make_opcode(insn),
                                                );
                                            }
                                        }
                                    } else {
                                        if insn & 0xff20e000 == 0x6520a000 {
                                            return Some(
                                                FMSB_SVE_Zd_SVE_Pg3_SVE_Zm_5_SVE_Za_16::make_opcode(
                                                    insn,
                                                ),
                                            );
                                        }
                                    }
                                }
                            }
                        } else {
                            if insn & 0x008000 == 0 {
                                if insn & 0x40000000 == 0 {
                                    if insn & 0x000200 == 0 {
                                        if insn & 0x100000 == 0 {
                                            if insn & 0x000010 == 0 {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0xfff0c210 == 0x25004000 {
                                                            return Some (AND_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0xfff0c210 == 0x25804000 {
                                                            return Some (ORR_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0xfff0c210 == 0x25404000 {
                                                            return Some (ANDS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0xfff0c210 == 0x25c04000 {
                                                            return Some (ORRS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0xfff0c210 == 0x25004010 {
                                                            return Some (BIC_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0xfff0c210 == 0x25804010 {
                                                            return Some (ORN_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0xfff0c210 == 0x25404010 {
                                                            return Some (BICS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0xfff0c210 == 0x25c04010 {
                                                            return Some (ORNS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x080000 == 0 {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0xffffc200 == 0x25104000 {
                                                            return Some (BRKA_SVE_Pd_SVE_Pg4_10_SVE_Pn :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0xffffc200 == 0x25904000 {
                                                            return Some (BRKB_SVE_Pd_SVE_Pg4_10_SVE_Pn :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0xffffc210 == 0x25504000 {
                                                            return Some (BRKAS_SVE_Pd_SVE_Pg4_10_SVE_Pn :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0xffffc210 == 0x25d04000 {
                                                            return Some (BRKBS_SVE_Pd_SVE_Pg4_10_SVE_Pn :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0xffffc210 == 0x25184000 {
                                                        return Some (BRKN_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pd :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0xffffc210 == 0x25584000 {
                                                        return Some (BRKNS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pd :: make_opcode (insn)) ;
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x400000 == 0 {
                                            if insn & 0xfff0c210 == 0x25004200 {
                                                return Some (EOR_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm :: make_opcode (insn)) ;
                                            }
                                        } else {
                                            if insn & 0xfff0c210 == 0x25404200 {
                                                return Some (EORS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm :: make_opcode (insn)) ;
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x002000 == 0 {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0x000010 == 0 {
                                                if insn & 0xff20e010 == 0x65004000 {
                                                    return Some (FCMGE_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                }
                                            } else {
                                                if insn & 0xff20e010 == 0x65004010 {
                                                    return Some (FCMGT_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                                }
                                            }
                                        } else {
                                            if insn & 0xffc0e000 == 0xe5804000 {
                                                return Some(
                                                    STR_SVE_Zt_SVE_ADDR_RI_S9xVL::make_opcode(insn),
                                                );
                                            }
                                        }
                                    } else {
                                        if insn & 0x000010 == 0 {
                                            if insn & 0xff20e010 == 0x65006000 {
                                                return Some (FCMEQ_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                            }
                                        } else {
                                            if insn & 0xff20e010 == 0x65006010 {
                                                return Some (FCMNE_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                            }
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x200000 == 0 {
                                    if insn & 0x000010 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0x400000 == 0 {
                                                if insn & 0xfff0c210 == 0x2500c000 {
                                                    return Some (BRKPA_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm :: make_opcode (insn)) ;
                                                }
                                            } else {
                                                if insn & 0xfff0c210 == 0x2540c000 {
                                                    return Some (BRKPAS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm :: make_opcode (insn)) ;
                                                }
                                            }
                                        } else {
                                            if insn & 0xff20e010 == 0x6500c000 {
                                                return Some (FCMUO_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16 :: make_opcode (insn)) ;
                                            }
                                        }
                                    } else {
                                        if insn & 0x400000 == 0 {
                                            if insn & 0xfff0c210 == 0x2500c010 {
                                                return Some (BRKPB_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm :: make_opcode (insn)) ;
                                            }
                                        } else {
                                            if insn & 0xfff0c210 == 0x2540c010 {
                                                return Some (BRKPBS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm :: make_opcode (insn)) ;
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x010000 == 0 {
                                        if insn & 0x100000 == 0 {
                                            if insn & 0xff3fc000 == 0x2520c000 {
                                                return Some(
                                                    ADD_SVE_Zd_SVE_Zd_SVE_AIMM::make_opcode(insn),
                                                );
                                            }
                                        } else {
                                            if insn & 0xff3fe000 == 0x2530c000 {
                                                return Some(
                                                    MUL_SVE_Zd_SVE_Zd_SVE_SIMM8::make_opcode(insn),
                                                );
                                            }
                                        }
                                    } else {
                                        if insn & 0x020000 == 0 {
                                            if insn & 0x080000 == 0 {
                                                if insn & 0xff3fc000 == 0x2521c000 {
                                                    return Some(
                                                        SUB_SVE_Zd_SVE_Zd_SVE_AIMM::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            } else {
                                                if insn & 0xff3fe000 == 0x2539c000 {
                                                    return Some(
                                                        FDUP_SVE_Zd_SVE_FPIMM8::make_opcode(insn),
                                                    );
                                                }
                                            }
                                        } else {
                                            if insn & 0xff3fc000 == 0x2523c000 {
                                                return Some(
                                                    SUBR_SVE_Zd_SVE_Zd_SVE_AIMM::make_opcode(insn),
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                if insn & 0x20000000 == 0 {
                    if insn & 0x40000000 == 0 {
                        if insn & 0x80000000 == 0 {
                            if insn & 0xfc000000 == 0x14000000 {
                                return Some(B_ADDR_PCREL26::make_opcode(insn));
                            }
                        } else {
                            if insn & 0xfc000000 == 0x94000000 {
                                return Some(BL_ADDR_PCREL26::make_opcode(insn));
                            }
                        }
                    } else {
                        if insn & 0x200000 == 0 {
                            if insn & 0x001000 == 0 {
                                if insn & 0x000400 == 0 {
                                    if insn & 0x000800 == 0 {
                                        if insn & 0x000001 == 0 {
                                            if insn & 0x400000 == 0 {
                                                if insn & 0xfffffc1f == 0xd61f0000 {
                                                    return Some(BR_Rn::make_opcode(insn));
                                                }
                                            } else {
                                                if insn & 0xfffffc1f == 0xd65f0000 {
                                                    return Some(RET_Rn::make_opcode(insn));
                                                }
                                            }
                                        } else {
                                            if insn == 0xd500401f {
                                                return Some(CFINV::make_opcode(insn));
                                            }
                                        }
                                    } else {
                                        if insn & 0x400000 == 0 {
                                            if insn & 0x1000000 == 0 {
                                                if insn & 0xfffffc1f == 0xd61f081f {
                                                    return Some(BRAAZ_Rn::make_opcode(insn));
                                                }
                                            } else {
                                                if insn & 0xfffffc00 == 0xd71f0800 {
                                                    return Some(BRAA_Rn_Rd_SP::make_opcode(insn));
                                                }
                                            }
                                        } else {
                                            if insn == 0xd65f0bff {
                                                return Some(RETAA::make_opcode(insn));
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x000800 == 0 {
                                        if insn == 0xd503251f {
                                            return Some(CHKFEAT_X16::make_opcode(insn));
                                        }
                                    } else {
                                        if insn & 0x400000 == 0 {
                                            if insn & 0x1000000 == 0 {
                                                if insn & 0xfffffc1f == 0xd61f0c1f {
                                                    return Some(BRABZ_Rn::make_opcode(insn));
                                                }
                                            } else {
                                                if insn & 0xfffffc00 == 0xd71f0c00 {
                                                    return Some(BRAB_Rn_Rd_SP::make_opcode(insn));
                                                }
                                            }
                                        } else {
                                            if insn == 0xd65f0fff {
                                                return Some(RETAB::make_opcode(insn));
                                            }
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x000020 == 0 {
                                    if insn & 0xfffff0ff == 0xd503305f {
                                        return Some(CLREX_UIMM4::make_opcode(insn));
                                    }
                                } else {
                                    if insn == 0xd50330ff {
                                        return Some(SB::make_opcode(insn));
                                    }
                                }
                            }
                        } else {
                            if insn & 0x1000000 == 0 {
                                if insn & 0x000001 == 0 {
                                    if insn & 0x2000000 == 0 {
                                        if insn & 0xffe0001f == 0xd4200000 {
                                            return Some(BRK_EXCEPTION::make_opcode(insn));
                                        }
                                    } else {
                                        if insn & 0xfffffc1f == 0xd63f0000 {
                                            return Some(BLR_Rn::make_opcode(insn));
                                        }
                                    }
                                } else {
                                    if insn & 0x000400 == 0 {
                                        if insn & 0xfffffc1f == 0xd63f081f {
                                            return Some(BLRAAZ_Rn::make_opcode(insn));
                                        }
                                    } else {
                                        if insn & 0xfffffc1f == 0xd63f0c1f {
                                            return Some(BLRABZ_Rn::make_opcode(insn));
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x000400 == 0 {
                                    if insn & 0x000800 == 0 {
                                        if insn & 0x000100 == 0 {
                                            if insn & 0xffffffe0 == 0xd5233060 {
                                                return Some(TSTART_Rd::make_opcode(insn));
                                            }
                                        } else {
                                            if insn & 0xffffffe0 == 0xd5233160 {
                                                return Some(TTEST_Rd::make_opcode(insn));
                                            }
                                        }
                                    } else {
                                        if insn & 0xfffffc00 == 0xd73f0800 {
                                            return Some(BLRAA_Rn_Rd_SP::make_opcode(insn));
                                        }
                                    }
                                } else {
                                    if insn & 0xfffffc00 == 0xd73f0c00 {
                                        return Some(BLRAB_Rn_Rd_SP::make_opcode(insn));
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if insn & 0x1000000 == 0 {
                        if insn & 0x2000000 == 0 {
                            if insn & 0x7f000000 == 0x34000000 {
                                return Some(CBZ_Rt_ADDR_PCREL19::make_opcode(insn));
                            }
                        } else {
                            if insn & 0x7f000000 == 0x36000000 {
                                return Some(TBZ_Rt_BIT_NUM_ADDR_PCREL14::make_opcode(insn));
                            }
                        }
                    } else {
                        if insn & 0x2000000 == 0 {
                            if insn & 0x7f000000 == 0x35000000 {
                                return Some(CBNZ_Rt_ADDR_PCREL19::make_opcode(insn));
                            }
                        } else {
                            if insn & 0x7f000000 == 0x37000000 {
                                return Some(TBNZ_Rt_BIT_NUM_ADDR_PCREL14::make_opcode(insn));
                            }
                        }
                    }
                }
            }
        } else {
            if insn & 0x2000000 == 0 {
                if insn & 0x10000000 == 0 {
                    if insn & 0x400000 == 0 {
                        if insn & 0x800000 == 0 {
                            if insn & 0x3fc00000 == 0x2d000000 {
                                return Some(STP_Ft_Ft2_ADDR_SIMM7::make_opcode(insn));
                            }
                        } else {
                            if insn & 0x3ec00000 == 0x2c800000 {
                                return Some(STP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S::make_opcode(insn));
                            }
                        }
                    } else {
                        if insn & 0x800000 == 0 {
                            if insn & 0x3fc00000 == 0x2d400000 {
                                return Some(LDP_Ft_Ft2_ADDR_SIMM7::make_opcode(insn));
                            }
                        } else {
                            if insn & 0x3ec00000 == 0x2cc00000 {
                                return Some(LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S::make_opcode(insn));
                            }
                        }
                    }
                } else {
                    if insn & 0x1000000 == 0 {
                        if insn & 0x20000000 == 0 {
                            if insn & 0x3f000000 == 0x1c000000 {
                                return Some(LDR_Ft_ADDR_PCREL19::make_opcode(insn));
                            }
                        } else {
                            if insn & 0x000400 == 0 {
                                if insn & 0x000800 == 0 {
                                    if insn & 0x400000 == 0 {
                                        if insn & 0x3f600c00 == 0x3c000000 {
                                            return Some(STUR_Ft_ADDR_SIMM9::make_opcode(insn));
                                        }
                                    } else {
                                        if insn & 0x3f600c00 == 0x3c400000 {
                                            return Some(LDUR_Ft_ADDR_SIMM9::make_opcode(insn));
                                        }
                                    }
                                } else {
                                    if insn & 0x400000 == 0 {
                                        if insn & 0x3f600c00 == 0x3c200800 {
                                            return Some(STR_Ft_ADDR_REGOFF::make_opcode(insn));
                                        }
                                    } else {
                                        if insn & 0x3f600c00 == 0x3c600800 {
                                            return Some(LDR_Ft_ADDR_REGOFF::make_opcode(insn));
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x400000 == 0 {
                                    if insn & 0x3f600400 == 0x3c000400 {
                                        return Some(STR_Ft_ADDR_SIMM9::make_opcode(insn));
                                    }
                                } else {
                                    if insn & 0x3f600400 == 0x3c400400 {
                                        return Some(LDR_Ft_ADDR_SIMM9::make_opcode(insn));
                                    }
                                }
                            }
                        }
                    } else {
                        if insn & 0x400000 == 0 {
                            if insn & 0x3f400000 == 0x3d000000 {
                                return Some(STR_Ft_ADDR_UIMM12::make_opcode(insn));
                            }
                        } else {
                            if insn & 0x3f400000 == 0x3d400000 {
                                return Some(LDR_Ft_ADDR_UIMM12::make_opcode(insn));
                            }
                        }
                    }
                }
            } else {
                if insn & 0x1000000 == 0 {
                    if insn & 0x200000 == 0 {
                        if insn & 0x10000000 == 0 {
                            if insn & 0x008000 == 0 {
                                if insn & 0x20000000 == 0 {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0x000400 == 0 {
                                            if insn & 0x000800 == 0 {
                                                if insn & 0x001000 == 0 {
                                                    if insn & 0xbfe09c00 == 0xe000000 {
                                                        return Some(TBL_Vd_LVn_Vm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                } else {
                                                    if insn & 0xbfe09c00 == 0xe001000 {
                                                        return Some(TBX_Vd_LVn_Vm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                }
                                            } else {
                                                if insn & 0x004000 == 0 {
                                                    if insn & 0xbf20fc00 == 0xe002800 {
                                                        return Some(TRN1_Vd_Vn_Vm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                } else {
                                                    if insn & 0xbf20fc00 == 0xe006800 {
                                                        return Some(TRN2_Vd_Vn_Vm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x000800 == 0 {
                                                if insn & 0x001000 == 0 {
                                                    if insn & 0xbfe0fc00 == 0xe402400 {
                                                        return Some(FCMEQ_Vd_Vn_Vm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                } else {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0xbfe0fc00 == 0xe401400 {
                                                            return Some(
                                                                FADD_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xbfe0fc00 == 0xec01400 {
                                                            return Some(
                                                                FSUB_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbfe0fc00 == 0xe401c00 {
                                                    return Some(FMULX_Vd_Vn_Vm::make_opcode(insn));
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0xffe08000 == 0xce000000 {
                                            return Some(EOR3_Vd_Vn_Vm_Va::make_opcode(insn));
                                        }
                                    }
                                } else {
                                    if insn & 0x000800 == 0 {
                                        if insn & 0x001000 == 0 {
                                            if insn & 0x800000 == 0 {
                                                if insn & 0xbfe0fc00 == 0x2e402400 {
                                                    return Some(FCMGE_Vd_Vn_Vm::make_opcode(insn));
                                                }
                                            } else {
                                                if insn & 0xbfe0fc00 == 0x2ec02400 {
                                                    return Some(FCMGT_Vd_Vn_Vm::make_opcode(insn));
                                                }
                                            }
                                        } else {
                                            if insn & 0xbfe0fc00 == 0x2e401400 {
                                                return Some(FADDP_Vd_Vn_Vm::make_opcode(insn));
                                            }
                                        }
                                    } else {
                                        if insn & 0x002000 == 0 {
                                            if insn & 0xbfe0fc00 == 0x2e401c00 {
                                                return Some(FMUL_Vd_Vn_Vm::make_opcode(insn));
                                            }
                                        } else {
                                            if insn & 0xbfe0fc00 == 0x2e403c00 {
                                                return Some(FDIV_Vd_Vn_Vm::make_opcode(insn));
                                            }
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x002000 == 0 {
                                    if insn & 0x004000 == 0 {
                                        if insn & 0xbf20fc00 == 0xe009400 {
                                            return Some(SDOT_Vd_Vn_Vm::make_opcode(insn));
                                        }
                                    } else {
                                        if insn & 0xbf20e400 == 0x2e00c400 {
                                            return Some(FCMLA_Vd_Vn_Vm_IMM_ROT1::make_opcode(
                                                insn,
                                            ));
                                        }
                                    }
                                } else {
                                    if insn & 0x000800 == 0 {
                                        if insn & 0xbf20ec00 == 0x2e00e400 {
                                            return Some(FCADD_Vd_Vn_Vm_IMM_ROT3::make_opcode(
                                                insn,
                                            ));
                                        }
                                    } else {
                                        if insn & 0x001000 == 0 {
                                            if insn & 0xffe0fc00 == 0x6e40ec00 {
                                                return Some(BFMMLA_Vd_Vn_Vm::make_opcode(insn));
                                            }
                                        } else {
                                            if insn & 0x800000 == 0 {
                                                if insn & 0xbfe0fc00 == 0x2e40fc00 {
                                                    return Some(BFDOT_Vd_Vn_Vm::make_opcode(insn));
                                                }
                                            } else {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xffe0fc00 == 0x2ec0fc00 {
                                                        return Some(
                                                            BFMLALB_Vd_Vn_Vm::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0x6ec0fc00 {
                                                        return Some(
                                                            BFMLALT_Vd_Vn_Vm::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            if insn & 0x20000000 == 0 {
                                if insn & 0x40000000 == 0 {
                                    if insn & 0x010000 == 0 {
                                        if insn & 0x020000 == 0 {
                                            if insn & 0x7fff0000 == 0x1ed80000 {
                                                return Some(FCVTZS_Rd_Fn_FBITS::make_opcode(insn));
                                            }
                                            if insn & 0x7f3f0000 == 0x1e180000 {
                                                return Some(
                                                    FCVTZS_Rd_W_Fn_S_D_FBITS_imm_1_32::make_opcode(
                                                        insn,
                                                    ),
                                                );
                                            }
                                        } else {
                                            if insn & 0x7fff0000 == 0x1ec20000 {
                                                return Some(SCVTF_Fd_Rn_FBITS::make_opcode(insn));
                                            }
                                            if insn & 0x7f3f0000 == 0x1e020000 {
                                                return Some(
                                                    SCVTF_Fd_S_D_Rn_W_FBITS_imm_1_32::make_opcode(
                                                        insn,
                                                    ),
                                                );
                                            }
                                        }
                                    } else {
                                        if insn & 0x020000 == 0 {
                                            if insn & 0x7fff0000 == 0x1ed90000 {
                                                return Some(FCVTZU_Rd_Fn_FBITS::make_opcode(insn));
                                            }
                                            if insn & 0x7f3f0000 == 0x1e190000 {
                                                return Some(
                                                    FCVTZU_Rd_W_Fn_S_D_FBITS_imm_1_32::make_opcode(
                                                        insn,
                                                    ),
                                                );
                                            }
                                        } else {
                                            if insn & 0x7fff0000 == 0x1ec30000 {
                                                return Some(UCVTF_Fd_Rn_FBITS::make_opcode(insn));
                                            }
                                            if insn & 0x7f3f0000 == 0x1e030000 {
                                                return Some(
                                                    UCVTF_Fd_S_D_Rn_W_FBITS_imm_1_32::make_opcode(
                                                        insn,
                                                    ),
                                                );
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x000800 == 0 {
                                        if insn & 0xffe0fc00 == 0x5e402400 {
                                            return Some(FCMEQ_Sd_Sn_Sm::make_opcode(insn));
                                        }
                                    } else {
                                        if insn & 0xffe0fc00 == 0x5e401c00 {
                                            return Some(FMULX_Sd_Sn_Sm::make_opcode(insn));
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x800000 == 0 {
                                    if insn & 0xffe0fc00 == 0x7e402400 {
                                        return Some(FCMGE_Sd_Sn_Sm::make_opcode(insn));
                                    }
                                } else {
                                    if insn & 0xffe0fc00 == 0x7ec02400 {
                                        return Some(FCMGT_Sd_Sn_Sm::make_opcode(insn));
                                    }
                                }
                            }
                        }
                    } else {
                        if insn & 0x000400 == 0 {
                            if insn & 0x000800 == 0 {
                                if insn & 0x001000 == 0 {
                                    if insn & 0x002000 == 0 {
                                        if insn & 0x004000 == 0 {
                                            if insn & 0x10000000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xff20fc00 == 0x2e200000 {
                                                        return Some(UADDL_Vd_Vn_Vm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                } else {
                                                    if insn & 0xff20fc00 == 0x6e200000 {
                                                        return Some(UADDL2_Vd_Vn_Vm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                }
                                            } else {
                                                if insn & 0x010000 == 0 {
                                                    if insn & 0x020000 == 0 {
                                                        if insn & 0x040000 == 0 {
                                                            if insn & 0x080000 == 0 {
                                                                if insn & 0x100000 == 0 {
                                                                    if insn & 0x7ffffc00
                                                                        == 0x1ee00000
                                                                    {
                                                                        return Some (FCVTNS_Rd_Fn :: make_opcode (insn)) ;
                                                                    }
                                                                    if insn & 0x7f3ffc00
                                                                        == 0x1e200000
                                                                    {
                                                                        return Some (FCVTNS_Rd_W_Fn_S_D :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0x7ffffc00
                                                                        == 0x1ef00000
                                                                    {
                                                                        return Some (FCVTMS_Rd_Fn :: make_opcode (insn)) ;
                                                                    }
                                                                    if insn & 0x7f3ffc00
                                                                        == 0x1e300000
                                                                    {
                                                                        return Some (FCVTMS_Rd_W_Fn_S_D :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0x100000 == 0 {
                                                                    if insn & 0x7ffffc00
                                                                        == 0x1ee80000
                                                                    {
                                                                        return Some (FCVTPS_Rd_Fn :: make_opcode (insn)) ;
                                                                    }
                                                                    if insn & 0x7f3ffc00
                                                                        == 0x1e280000
                                                                    {
                                                                        return Some (FCVTPS_Rd_W_Fn_S_D :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0x7ffffc00
                                                                        == 0x1ef80000
                                                                    {
                                                                        return Some (FCVTZS_Rd_Fn :: make_opcode (insn)) ;
                                                                    }
                                                                    if insn & 0x7f3ffc00
                                                                        == 0x1e380000
                                                                    {
                                                                        return Some (FCVTZS_Rd_W_Fn_S_D :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x7ffffc00 == 0x1ee40000 {
                                                                return Some(
                                                                    FCVTAS_Rd_Fn::make_opcode(insn),
                                                                );
                                                            }
                                                            if insn & 0x7f3ffc00 == 0x1e240000 {
                                                                return Some(
                                                                    FCVTAS_Rd_W_Fn_S_D::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x040000 == 0 {
                                                            if insn & 0x7ffffc00 == 0x1ee20000 {
                                                                return Some(
                                                                    SCVTF_Fd_Rn::make_opcode(insn),
                                                                );
                                                            }
                                                            if insn & 0x7f3ffc00 == 0x1e220000 {
                                                                return Some(
                                                                    SCVTF_Fd_S_D_Rn_W::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0x080000 == 0 {
                                                                if insn & 0x7ffffc00 == 0x1ee60000 {
                                                                    return Some(
                                                                        FMOV_Rd_Fn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                                if insn & 0x7f3ffc00 == 0x1e260000 {
                                                                    return Some (FMOV_Rd_W_Fn_S_S :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0x100000 == 0 {
                                                                    if insn & 0xfffffc00
                                                                        == 0x9eae0000
                                                                    {
                                                                        return Some (FMOV_Rd_VnD1 :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xfffffc00
                                                                        == 0x1e7e0000
                                                                    {
                                                                        return Some (FJCVTZS_Rd_Fn :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x020000 == 0 {
                                                        if insn & 0x040000 == 0 {
                                                            if insn & 0x080000 == 0 {
                                                                if insn & 0x100000 == 0 {
                                                                    if insn & 0x7ffffc00
                                                                        == 0x1ee10000
                                                                    {
                                                                        return Some (FCVTNU_Rd_Fn :: make_opcode (insn)) ;
                                                                    }
                                                                    if insn & 0x7f3ffc00
                                                                        == 0x1e210000
                                                                    {
                                                                        return Some (FCVTNU_Rd_W_Fn_S_D :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0x7ffffc00
                                                                        == 0x1ef10000
                                                                    {
                                                                        return Some (FCVTMU_Rd_Fn :: make_opcode (insn)) ;
                                                                    }
                                                                    if insn & 0x7f3ffc00
                                                                        == 0x1e310000
                                                                    {
                                                                        return Some (FCVTMU_Rd_W_Fn_S_D :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0x100000 == 0 {
                                                                    if insn & 0x7ffffc00
                                                                        == 0x1ee90000
                                                                    {
                                                                        return Some (FCVTPU_Rd_Fn :: make_opcode (insn)) ;
                                                                    }
                                                                    if insn & 0x7f3ffc00
                                                                        == 0x1e290000
                                                                    {
                                                                        return Some (FCVTPU_Rd_W_Fn_S_D :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0x7ffffc00
                                                                        == 0x1ef90000
                                                                    {
                                                                        return Some (FCVTZU_Rd_Fn :: make_opcode (insn)) ;
                                                                    }
                                                                    if insn & 0x7f3ffc00
                                                                        == 0x1e390000
                                                                    {
                                                                        return Some (FCVTZU_Rd_W_Fn_S_D :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x7ffffc00 == 0x1ee50000 {
                                                                return Some(
                                                                    FCVTAU_Rd_Fn::make_opcode(insn),
                                                                );
                                                            }
                                                            if insn & 0x7f3ffc00 == 0x1e250000 {
                                                                return Some(
                                                                    FCVTAU_Rd_W_Fn_S_D::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x040000 == 0 {
                                                            if insn & 0x7ffffc00 == 0x1ee30000 {
                                                                return Some(
                                                                    UCVTF_Fd_Rn::make_opcode(insn),
                                                                );
                                                            }
                                                            if insn & 0x7f3ffc00 == 0x1e230000 {
                                                                return Some(
                                                                    UCVTF_Fd_S_D_Rn_W::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0x080000 == 0 {
                                                                if insn & 0x7ffffc00 == 0x1ee70000 {
                                                                    return Some(
                                                                        FMOV_Fd_Rn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                                if insn & 0x7f3ffc00 == 0x1e270000 {
                                                                    return Some (FMOV_Fd_S_S_Rn_W :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xfffffc00 == 0x9eaf0000 {
                                                                    return Some(
                                                                        FMOV_VdD1_Rn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x10000000 == 0 {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x40000000 == 0 {
                                                        if insn & 0xff20fc00 == 0xe204000 {
                                                            return Some(
                                                                ADDHN_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xff20fc00 == 0x4e204000 {
                                                            return Some(
                                                                ADDHN2_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x40000000 == 0 {
                                                        if insn & 0xff20fc00 == 0xe20c000 {
                                                            return Some(
                                                                SMULL_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xff20fc00 == 0x4e20c000 {
                                                            return Some(
                                                                SMULL2_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x020000 == 0 {
                                                    if insn & 0xfffffc00 == 0x1ee04000 {
                                                        return Some(FMOV_Fd_Fn::make_opcode(insn));
                                                    }
                                                    if insn & 0xff3ffc00 == 0x1e204000 {
                                                        return Some(
                                                            FMOV_Fd_S_S_Fn_S_S::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xfffffc00 == 0x1e634000 {
                                                        return Some(BFCVT_Fd_Fn::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                    if insn & 0xff3e7c00 == 0x1e224000 {
                                                        return Some(FCVT_Fd_Fn::make_opcode(insn));
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x004000 == 0 {
                                            if insn & 0x000008 == 0 {
                                                if insn & 0x000010 == 0 {
                                                    if insn & 0xffe0fc1f == 0x1ee02000 {
                                                        return Some(FCMP_Fn_Fm::make_opcode(insn));
                                                    }
                                                    if insn & 0xff20fc1f == 0x1e202000 {
                                                        return Some(
                                                            FCMP_Fn_S_S_Fm_S_S::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc1f == 0x1ee02010 {
                                                        return Some(FCMPE_Fn_Fm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                    if insn & 0xff20fc1f == 0x1e202010 {
                                                        return Some(
                                                            FCMPE_Fn_S_S_Fm_S_S::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0x000010 == 0 {
                                                    if insn & 0xffe0fc1f == 0x1ee02008 {
                                                        return Some(FCMP_Fn_FPIMM0::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                    if insn & 0xff20fc1f == 0x1e202008 {
                                                        return Some(
                                                            FCMP_Fn_S_S_FPIMM0::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc1f == 0x1ee02018 {
                                                        return Some(FCMPE_Fn_FPIMM0::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                    if insn & 0xff20fc1f == 0x1e202018 {
                                                        return Some(
                                                            FCMPE_Fn_S_S_FPIMM0::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xff20fc00 == 0xe206000 {
                                                    return Some(SUBHN_Vd_Vn_Vm::make_opcode(insn));
                                                }
                                            } else {
                                                if insn & 0xff20fc00 == 0x4e206000 {
                                                    return Some(SUBHN2_Vd_Vn_Vm::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x10000000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xff20fc00 == 0x2e201000 {
                                                return Some(UADDW_Vd_Vn_Vm::make_opcode(insn));
                                            }
                                        } else {
                                            if insn & 0xff20fc00 == 0x6e201000 {
                                                return Some(UADDW2_Vd_Vn_Vm::make_opcode(insn));
                                            }
                                        }
                                    } else {
                                        if insn & 0xffe01fe0 == 0x1ee01000 {
                                            return Some(FMOV_Fd_FPIMM::make_opcode(insn));
                                        }
                                        if insn & 0xff201fe0 == 0x1e201000 {
                                            return Some(FMOV_Fd_S_S_FPIMM::make_opcode(insn));
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x001000 == 0 {
                                    if insn & 0x002000 == 0 {
                                        if insn & 0x004000 == 0 {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0xffe0fc00 == 0x1ee00800 {
                                                    return Some(FMUL_Fd_Fn_Fm::make_opcode(insn));
                                                }
                                                if insn & 0xff20fc00 == 0x1e200800 {
                                                    return Some(
                                                        FMUL_Fd_S_S_Fn_S_S_Fm_S_S::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            } else {
                                                if insn & 0x10000000 == 0 {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0xbf3ffc00 == 0xe208800 {
                                                            return Some(
                                                                CMGT_Vd_Vn_IMM0::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xbf3ffc00 == 0x2e208800 {
                                                            return Some(
                                                                CMGE_Vd_Vn_IMM0::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0xff3ffc00 == 0x5e208800 {
                                                            return Some(
                                                                CMGT_Sd_Sn_IMM0::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xff3ffc00 == 0x7e208800 {
                                                            return Some(
                                                                CMGE_Sd_Sn_IMM0::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0x20000000 == 0 {
                                                    if insn & 0xbf3ffc00 == 0xe204800 {
                                                        return Some(CLS_Vd_Vn::make_opcode(insn));
                                                    }
                                                } else {
                                                    if insn & 0xbf3ffc00 == 0x2e204800 {
                                                        return Some(CLZ_Vd_Vn::make_opcode(insn));
                                                    }
                                                }
                                            } else {
                                                if insn & 0x010000 == 0 {
                                                    if insn & 0x080000 == 0 {
                                                        if insn & 0x10000000 == 0 {
                                                            if insn & 0x20000000 == 0 {
                                                                if insn & 0xbfbffc00 == 0xea0c800 {
                                                                    return Some (FCMGT_Vd_Vn_FPIMM0 :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xbfbffc00 == 0x2ea0c800 {
                                                                    return Some (FCMGE_Vd_Vn_FPIMM0 :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x20000000 == 0 {
                                                                if insn & 0xffbffc00 == 0x5ea0c800 {
                                                                    return Some (FCMGT_Sd_Sn_FPIMM0 :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xffbffc00 == 0x7ea0c800 {
                                                                    return Some (FCMGE_Sd_Sn_FPIMM0 :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x10000000 == 0 {
                                                            if insn & 0x20000000 == 0 {
                                                                if insn & 0xbffffc00 == 0xef8c800 {
                                                                    return Some (FCMGT_Vd_V_4H_Vn_V_4H_FPIMM0 :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xbffffc00 == 0x2ef8c800 {
                                                                    return Some (FCMGE_Vd_V_4H_Vn_V_4H_FPIMM0 :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x20000000 == 0 {
                                                                if insn & 0xfffffc00 == 0x5ef8c800 {
                                                                    return Some (FCMGT_Sd_S_H_Sn_S_H_FPIMM0 :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xfffffc00 == 0x7ef8c800 {
                                                                    return Some (FCMGE_Sd_S_H_Sn_S_H_FPIMM0 :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x080000 == 0 {
                                                        if insn & 0x10000000 == 0 {
                                                            if insn & 0x20000000 == 0 {
                                                                if insn & 0xbfbffc00 == 0xe21c800 {
                                                                    return Some(
                                                                        FCVTAS_Vd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            } else {
                                                                if insn & 0xbfbffc00 == 0x2e21c800 {
                                                                    return Some(
                                                                        FCVTAU_Vd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x20000000 == 0 {
                                                                if insn & 0xffbffc00 == 0x5e21c800 {
                                                                    return Some(
                                                                        FCVTAS_Sd_Sn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            } else {
                                                                if insn & 0xffbffc00 == 0x7e21c800 {
                                                                    return Some(
                                                                        FCVTAU_Sd_Sn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x10000000 == 0 {
                                                            if insn & 0x20000000 == 0 {
                                                                if insn & 0xbffffc00 == 0xe79c800 {
                                                                    return Some (FCVTAS_Vd_V_4H_Vn_V_4H :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xbffffc00 == 0x2e79c800 {
                                                                    return Some (FCVTAU_Vd_V_4H_Vn_V_4H :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x20000000 == 0 {
                                                                if insn & 0xfffffc00 == 0x5e79c800 {
                                                                    return Some (FCVTAS_Sd_S_H_Sn_S_H :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xfffffc00 == 0x7e79c800 {
                                                                    return Some (FCVTAU_Sd_S_H_Sn_S_H :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x004000 == 0 {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0x10000000 == 0 {
                                                    if insn & 0xbf3ffc00 == 0x2e202800 {
                                                        return Some(UADDLP_Vd_Vn::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0x1ee02800 {
                                                        return Some(FADD_Fd_Fn_Fm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                    if insn & 0xff20fc00 == 0x1e202800 {
                                                        return Some(
                                                            FADD_Fd_S_S_Fn_S_S_Fm_S_S::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0x010000 == 0 {
                                                    if insn & 0x10000000 == 0 {
                                                        if insn & 0xbf3ffc00 == 0xe20a800 {
                                                            return Some(
                                                                CMLT_Vd_Vn_IMM0::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xff3ffc00 == 0x5e20a800 {
                                                            return Some(
                                                                CMLT_Sd_Sn_IMM0::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x080000 == 0 {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x10000000 == 0 {
                                                                if insn & 0x20000000 == 0 {
                                                                    if insn & 0xbfbffc00
                                                                        == 0xe21a800
                                                                    {
                                                                        return Some (FCVTNS_Vd_Vn :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xbfbffc00
                                                                        == 0x2e21a800
                                                                    {
                                                                        return Some (FCVTNU_Vd_Vn :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0x20000000 == 0 {
                                                                    if insn & 0xffbffc00
                                                                        == 0x5e21a800
                                                                    {
                                                                        return Some (FCVTNS_Sd_Sn :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffbffc00
                                                                        == 0x7e21a800
                                                                    {
                                                                        return Some (FCVTNU_Sd_Sn :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x10000000 == 0 {
                                                                if insn & 0x20000000 == 0 {
                                                                    if insn & 0xbfbffc00
                                                                        == 0xea1a800
                                                                    {
                                                                        return Some (FCVTPS_Vd_Vn :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xbfbffc00
                                                                        == 0x2ea1a800
                                                                    {
                                                                        return Some (FCVTPU_Vd_Vn :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0x20000000 == 0 {
                                                                    if insn & 0xffbffc00
                                                                        == 0x5ea1a800
                                                                    {
                                                                        return Some (FCVTPS_Sd_Sn :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffbffc00
                                                                        == 0x7ea1a800
                                                                    {
                                                                        return Some (FCVTPU_Sd_Sn :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x10000000 == 0 {
                                                                if insn & 0x20000000 == 0 {
                                                                    if insn & 0xbffffc00
                                                                        == 0xe79a800
                                                                    {
                                                                        return Some (FCVTNS_Vd_V_4H_Vn_V_4H :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xbffffc00
                                                                        == 0x2e79a800
                                                                    {
                                                                        return Some (FCVTNU_Vd_V_4H_Vn_V_4H :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0x20000000 == 0 {
                                                                    if insn & 0xfffffc00
                                                                        == 0x5e79a800
                                                                    {
                                                                        return Some (FCVTNS_Sd_S_H_Sn_S_H :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xfffffc00
                                                                        == 0x7e79a800
                                                                    {
                                                                        return Some (FCVTNU_Sd_S_H_Sn_S_H :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x10000000 == 0 {
                                                                if insn & 0x20000000 == 0 {
                                                                    if insn & 0xbffffc00
                                                                        == 0xef9a800
                                                                    {
                                                                        return Some (FCVTPS_Vd_V_4H_Vn_V_4H :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xbffffc00
                                                                        == 0x2ef9a800
                                                                    {
                                                                        return Some (FCVTPU_Vd_V_4H_Vn_V_4H :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0x20000000 == 0 {
                                                                    if insn & 0xfffffc00
                                                                        == 0x5ef9a800
                                                                    {
                                                                        return Some (FCVTPS_Sd_S_H_Sn_S_H :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xfffffc00
                                                                        == 0x7ef9a800
                                                                    {
                                                                        return Some (FCVTPU_Sd_S_H_Sn_S_H :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0x800000 == 0 {
                                                    if insn & 0x10000000 == 0 {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xffbffc00 == 0xe216800 {
                                                                    return Some(
                                                                        FCVTN_Vd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            } else {
                                                                if insn & 0xffbffc00 == 0x4e216800 {
                                                                    return Some(
                                                                        FCVTN2_Vd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xfffffc00 == 0x2e616800 {
                                                                    return Some(
                                                                        FCVTXN_Vd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            } else {
                                                                if insn & 0xfffffc00 == 0x6e616800 {
                                                                    return Some(
                                                                        FCVTXN2_Vd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0xffbffc00 == 0x7e216800 {
                                                            return Some(
                                                                FCVTXN_Sd_Sn::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x40000000 == 0 {
                                                        if insn & 0xfffffc00 == 0xea16800 {
                                                            return Some(
                                                                BFCVTN_Vd_Vn::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xfffffc00 == 0x4ea16800 {
                                                            return Some(
                                                                BFCVTN2_Vd_Vn::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x080000 == 0 {
                                                    if insn & 0x10000000 == 0 {
                                                        if insn & 0xbfbffc00 == 0xea0e800 {
                                                            return Some(
                                                                FCMLT_Vd_Vn_FPIMM0::make_opcode(
                                                                    insn,
                                                                ),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xffbffc00 == 0x5ea0e800 {
                                                            return Some(
                                                                FCMLT_Sd_Sn_FPIMM0::make_opcode(
                                                                    insn,
                                                                ),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x10000000 == 0 {
                                                        if insn & 0xbffffc00 == 0xef8e800 {
                                                            return Some (FCMLT_Vd_V_4H_Vn_V_4H_FPIMM0 :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0xfffffc00 == 0x5ef8e800 {
                                                            return Some (FCMLT_Sd_S_H_Sn_S_H_FPIMM0 :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x002000 == 0 {
                                        if insn & 0x004000 == 0 {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0xffe0fc00 == 0x1ee01800 {
                                                    return Some(FDIV_Fd_Fn_Fm::make_opcode(insn));
                                                }
                                                if insn & 0xff20fc00 == 0x1e201800 {
                                                    return Some(
                                                        FDIV_Fd_S_S_Fn_S_S_Fm_S_S::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            } else {
                                                if insn & 0x10000000 == 0 {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0xbf3ffc00 == 0xe209800 {
                                                            return Some(
                                                                CMEQ_Vd_Vn_IMM0::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xbf3ffc00 == 0x2e209800 {
                                                            return Some(
                                                                CMLE_Vd_Vn_IMM0::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0xff3ffc00 == 0x5e209800 {
                                                            return Some(
                                                                CMEQ_Sd_Sn_IMM0::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xff3ffc00 == 0x7e209800 {
                                                            return Some(
                                                                CMLE_Sd_Sn_IMM0::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0xbf3ffc00 == 0xe205800 {
                                                    return Some(CNT_Vd_Vn::make_opcode(insn));
                                                }
                                            } else {
                                                if insn & 0x010000 == 0 {
                                                    if insn & 0x080000 == 0 {
                                                        if insn & 0x100000 == 0 {
                                                            if insn & 0x10000000 == 0 {
                                                                if insn & 0x20000000 == 0 {
                                                                    if insn & 0xbfbffc00
                                                                        == 0xea0d800
                                                                    {
                                                                        return Some (FCMEQ_Vd_Vn_FPIMM0 :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xbfbffc00
                                                                        == 0x2ea0d800
                                                                    {
                                                                        return Some (FCMLE_Vd_Vn_FPIMM0 :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0x20000000 == 0 {
                                                                    if insn & 0xffbffc00
                                                                        == 0x5ea0d800
                                                                    {
                                                                        return Some (FCMEQ_Sd_Sn_FPIMM0 :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffbffc00
                                                                        == 0x7ea0d800
                                                                    {
                                                                        return Some (FCMLE_Sd_Sn_FPIMM0 :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x20000000 == 0 {
                                                                if insn & 0xfffffc00 == 0x5e30d800 {
                                                                    return Some(
                                                                        FADDP_Sd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            } else {
                                                                if insn & 0xffbffc00 == 0x7e30d800 {
                                                                    return Some (FADDP_Sd_S_S_Vn_V_2S :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x10000000 == 0 {
                                                            if insn & 0x20000000 == 0 {
                                                                if insn & 0xbffffc00 == 0xef8d800 {
                                                                    return Some (FCMEQ_Vd_V_4H_Vn_V_4H_FPIMM0 :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xbffffc00 == 0x2ef8d800 {
                                                                    return Some (FCMLE_Vd_V_4H_Vn_V_4H_FPIMM0 :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x20000000 == 0 {
                                                                if insn & 0xfffffc00 == 0x5ef8d800 {
                                                                    return Some (FCMEQ_Sd_S_H_Sn_S_H_FPIMM0 :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xfffffc00 == 0x7ef8d800 {
                                                                    return Some (FCMLE_Sd_S_H_Sn_S_H_FPIMM0 :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x080000 == 0 {
                                                        if insn & 0x10000000 == 0 {
                                                            if insn & 0x20000000 == 0 {
                                                                if insn & 0xbfbffc00 == 0xe21d800 {
                                                                    return Some(
                                                                        SCVTF_Vd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            } else {
                                                                if insn & 0xbfbffc00 == 0x2e21d800 {
                                                                    return Some(
                                                                        UCVTF_Vd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x20000000 == 0 {
                                                                if insn & 0xffbffc00 == 0x5e21d800 {
                                                                    return Some(
                                                                        SCVTF_Sd_Sn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            } else {
                                                                if insn & 0xffbffc00 == 0x7e21d800 {
                                                                    return Some(
                                                                        UCVTF_Sd_Sn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x10000000 == 0 {
                                                            if insn & 0x20000000 == 0 {
                                                                if insn & 0xbffffc00 == 0xe79d800 {
                                                                    return Some (SCVTF_Vd_V_4H_Vn_V_4H :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xbffffc00 == 0x2e79d800 {
                                                                    return Some (UCVTF_Vd_V_4H_Vn_V_4H :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x20000000 == 0 {
                                                                if insn & 0xfffffc00 == 0x5e79d800 {
                                                                    return Some (SCVTF_Sd_S_H_Sn_S_H :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xfffffc00 == 0x7e79d800 {
                                                                    return Some (UCVTF_Sd_S_H_Sn_S_H :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x004000 == 0 {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0x10000000 == 0 {
                                                    if insn & 0xbf3ffc00 == 0x2e303800 {
                                                        return Some(UADDLV_Fd_Vn::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0x1ee03800 {
                                                        return Some(FSUB_Fd_Fn_Fm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                    if insn & 0xff20fc00 == 0x1e203800 {
                                                        return Some(
                                                            FSUB_Fd_S_S_Fn_S_S_Fm_S_S::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0x010000 == 0 {
                                                    if insn & 0x10000000 == 0 {
                                                        if insn & 0xbf3ffc00 == 0x2e20b800 {
                                                            return Some(NEG_Vd_Vn::make_opcode(
                                                                insn,
                                                            ));
                                                        }
                                                    } else {
                                                        if insn & 0xff3ffc00 == 0x7e20b800 {
                                                            return Some(NEG_Sd_Sn::make_opcode(
                                                                insn,
                                                            ));
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x080000 == 0 {
                                                        if insn & 0x100000 == 0 {
                                                            if insn & 0x800000 == 0 {
                                                                if insn & 0x10000000 == 0 {
                                                                    if insn & 0x20000000 == 0 {
                                                                        if insn & 0xbfbffc00
                                                                            == 0xe21b800
                                                                        {
                                                                            return Some (FCVTMS_Vd_Vn :: make_opcode (insn)) ;
                                                                        }
                                                                    } else {
                                                                        if insn & 0xbfbffc00
                                                                            == 0x2e21b800
                                                                        {
                                                                            return Some (FCVTMU_Vd_Vn :: make_opcode (insn)) ;
                                                                        }
                                                                    }
                                                                } else {
                                                                    if insn & 0x20000000 == 0 {
                                                                        if insn & 0xffbffc00
                                                                            == 0x5e21b800
                                                                        {
                                                                            return Some (FCVTMS_Sd_Sn :: make_opcode (insn)) ;
                                                                        }
                                                                    } else {
                                                                        if insn & 0xffbffc00
                                                                            == 0x7e21b800
                                                                        {
                                                                            return Some (FCVTMU_Sd_Sn :: make_opcode (insn)) ;
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0x10000000 == 0 {
                                                                    if insn & 0x20000000 == 0 {
                                                                        if insn & 0xbfbffc00
                                                                            == 0xea1b800
                                                                        {
                                                                            return Some (FCVTZS_Vd_Vn :: make_opcode (insn)) ;
                                                                        }
                                                                    } else {
                                                                        if insn & 0xbfbffc00
                                                                            == 0x2ea1b800
                                                                        {
                                                                            return Some (FCVTZU_Vd_Vn :: make_opcode (insn)) ;
                                                                        }
                                                                    }
                                                                } else {
                                                                    if insn & 0x20000000 == 0 {
                                                                        if insn & 0xffbffc00
                                                                            == 0x5ea1b800
                                                                        {
                                                                            return Some (FCVTZS_Sd_Sn :: make_opcode (insn)) ;
                                                                        }
                                                                    } else {
                                                                        if insn & 0xffbffc00
                                                                            == 0x7ea1b800
                                                                        {
                                                                            return Some (FCVTZU_Sd_Sn :: make_opcode (insn)) ;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x10000000 == 0 {
                                                                if insn & 0xbf3ffc00 == 0xe31b800 {
                                                                    return Some(
                                                                        ADDV_Fd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            } else {
                                                                if insn & 0xff3ffc00 == 0x5e31b800 {
                                                                    return Some(
                                                                        ADDP_Sd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x10000000 == 0 {
                                                                if insn & 0x20000000 == 0 {
                                                                    if insn & 0xbffffc00
                                                                        == 0xe79b800
                                                                    {
                                                                        return Some (FCVTMS_Vd_V_4H_Vn_V_4H :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xbffffc00
                                                                        == 0x2e79b800
                                                                    {
                                                                        return Some (FCVTMU_Vd_V_4H_Vn_V_4H :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0x20000000 == 0 {
                                                                    if insn & 0xfffffc00
                                                                        == 0x5e79b800
                                                                    {
                                                                        return Some (FCVTMS_Sd_S_H_Sn_S_H :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xfffffc00
                                                                        == 0x7e79b800
                                                                    {
                                                                        return Some (FCVTMU_Sd_S_H_Sn_S_H :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x10000000 == 0 {
                                                                if insn & 0x20000000 == 0 {
                                                                    if insn & 0xbffffc00
                                                                        == 0xef9b800
                                                                    {
                                                                        return Some (FCVTZS_Vd_V_4H_Vn_V_4H :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xbffffc00
                                                                        == 0x2ef9b800
                                                                    {
                                                                        return Some (FCVTZU_Vd_V_4H_Vn_V_4H :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0x20000000 == 0 {
                                                                    if insn & 0xfffffc00
                                                                        == 0x5ef9b800
                                                                    {
                                                                        return Some (FCVTZS_Sd_S_H_Sn_S_H :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xfffffc00
                                                                        == 0x7ef9b800
                                                                    {
                                                                        return Some (FCVTZU_Sd_S_H_Sn_S_H :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xffbffc00 == 0xe217800 {
                                                    return Some(FCVTL_Vd_Vn::make_opcode(insn));
                                                }
                                            } else {
                                                if insn & 0xffbffc00 == 0x4e217800 {
                                                    return Some(FCVTL2_Vd_Vn::make_opcode(insn));
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            if insn & 0x000800 == 0 {
                                if insn & 0x10000000 == 0 {
                                    if insn & 0x001000 == 0 {
                                        if insn & 0x002000 == 0 {
                                            if insn & 0x20000000 == 0 {
                                                if insn & 0xbf20fc00 == 0xe208400 {
                                                    return Some(ADD_Vd_Vn_Vm::make_opcode(insn));
                                                }
                                            } else {
                                                if insn & 0xbf20fc00 == 0x2e208400 {
                                                    return Some(SUB_Vd_Vn_Vm::make_opcode(insn));
                                                }
                                            }
                                        } else {
                                            if insn & 0x800000 == 0 {
                                                if insn & 0x20000000 == 0 {
                                                    if insn & 0xbfa0fc00 == 0xe20e400 {
                                                        return Some (FCMEQ_Vd_V_2S_Vn_V_2S_Vm_V_2S :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0xbfa0fc00 == 0x2e20e400 {
                                                        return Some (FCMGE_Vd_V_2S_Vn_V_2S_Vm_V_2S :: make_opcode (insn)) ;
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbfa0fc00 == 0x2ea0e400 {
                                                    return Some(
                                                        FCMGT_Vd_V_2S_Vn_V_2S_Vm_V_2S::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x002000 == 0 {
                                            if insn & 0x800000 == 0 {
                                                if insn & 0x20000000 == 0 {
                                                    if insn & 0xbfa0fc00 == 0xe20d400 {
                                                        return Some (FADD_Vd_V_2S_Vn_V_2S_Vm_V_2S :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0xbfa0fc00 == 0x2e20d400 {
                                                        return Some (FADDP_Vd_V_2S_Vn_V_2S_Vm_V_2S :: make_opcode (insn)) ;
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbfa0fc00 == 0xea0d400 {
                                                    return Some(
                                                        FSUB_Vd_V_2S_Vn_V_2S_Vm_V_2S::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            }
                                        } else {
                                            if insn & 0x20000000 == 0 {
                                                if insn & 0xbf20fc00 == 0xe203400 {
                                                    return Some(CMGT_Vd_Vn_Vm::make_opcode(insn));
                                                }
                                            } else {
                                                if insn & 0xbf20fc00 == 0x2e203400 {
                                                    return Some(CMHI_Vd_Vn_Vm::make_opcode(insn));
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x20000000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0x000010 == 0 {
                                                if insn & 0xffe00c10 == 0x1ee00400 {
                                                    return Some(
                                                        FCCMP_Fn_Fm_NZCV_COND::make_opcode(insn),
                                                    );
                                                }
                                                if insn & 0xff200c10 == 0x1e200400 {
                                                    return Some(
                                                        FCCMP_Fn_S_S_Fm_S_S_NZCV_COND::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            } else {
                                                if insn & 0xffe00c10 == 0x1ee00410 {
                                                    return Some(
                                                        FCCMPE_Fn_Fm_NZCV_COND::make_opcode(insn),
                                                    );
                                                }
                                                if insn & 0xff200c10 == 0x1e200410 {
                                                    return Some(
                                                        FCCMPE_Fn_S_S_Fm_S_S_NZCV_COND::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            }
                                        } else {
                                            if insn & 0x001000 == 0 {
                                                if insn & 0x002000 == 0 {
                                                    if insn & 0xffe0fc00 == 0x5ee08400 {
                                                        return Some(ADD_Sd_Sn_Sm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                } else {
                                                    if insn & 0xffa0fc00 == 0x5e20e400 {
                                                        return Some(
                                                            FCMEQ_Sd_S_S_Sn_S_S_Sm_S_S::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0xffe0fc00 == 0x5ee03400 {
                                                    return Some(CMGT_Sd_Sn_Sm::make_opcode(insn));
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x001000 == 0 {
                                            if insn & 0x002000 == 0 {
                                                if insn & 0xffe0fc00 == 0x7ee08400 {
                                                    return Some(SUB_Sd_Sn_Sm::make_opcode(insn));
                                                }
                                            } else {
                                                if insn & 0x800000 == 0 {
                                                    if insn & 0xffa0fc00 == 0x7e20e400 {
                                                        return Some(
                                                            FCMGE_Sd_S_S_Sn_S_S_Sm_S_S::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xffa0fc00 == 0x7ea0e400 {
                                                        return Some(
                                                            FCMGT_Sd_S_S_Sn_S_S_Sm_S_S::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0xffe0fc00 == 0x7ee03400 {
                                                return Some(CMHI_Sd_Sn_Sm::make_opcode(insn));
                                            }
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x10000000 == 0 {
                                    if insn & 0x001000 == 0 {
                                        if insn & 0x20000000 == 0 {
                                            if insn & 0xbf20fc00 == 0xe208c00 {
                                                return Some(CMTST_Vd_Vn_Vm::make_opcode(insn));
                                            }
                                        } else {
                                            if insn & 0xbf20fc00 == 0x2e208c00 {
                                                return Some(CMEQ_Vd_Vn_Vm::make_opcode(insn));
                                            }
                                        }
                                    } else {
                                        if insn & 0x002000 == 0 {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x400000 == 0 {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x20000000 == 0 {
                                                                if insn & 0xbfe0fc00 == 0xe201c00 {
                                                                    return Some(
                                                                        AND_Vd_Vn_Vm::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0x2e201c00 {
                                                                    return Some(
                                                                        EOR_Vd_Vn_Vm::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x20000000 == 0 {
                                                                if insn & 0xbfe0fc00 == 0xea01c00 {
                                                                    return Some(
                                                                        ORR_Vd_Vn_Vm::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0x2ea01c00 {
                                                                    return Some(
                                                                        BIT_Vd_Vn_Vm::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0xbfe0fc00 == 0xe601c00 {
                                                                return Some(
                                                                    BIC_Vd_Vn_Vm::make_opcode(insn),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0x20000000 == 0 {
                                                                if insn & 0xbfe0fc00 == 0xee01c00 {
                                                                    return Some(
                                                                        ORN_Vd_Vn_Vm::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0x2ee01c00 {
                                                                    return Some(
                                                                        BIF_Vd_Vn_Vm::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xbf20fc00 == 0xe209c00 {
                                                        return Some(MUL_Vd_Vn_Vm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                }
                                            } else {
                                                if insn & 0x20000000 == 0 {
                                                    if insn & 0xbfa0fc00 == 0xe20dc00 {
                                                        return Some (FMULX_Vd_V_2S_Vn_V_2S_Vm_V_2S :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0xbfa0fc00 == 0x2e20dc00 {
                                                        return Some (FMUL_Vd_V_2S_Vn_V_2S_Vm_V_2S :: make_opcode (insn)) ;
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0xbf20fc00 == 0xe203c00 {
                                                            return Some(
                                                                CMGE_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xbf20fc00 == 0x2e203c00 {
                                                            return Some(
                                                                CMHS_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xbf20fc00 == 0xe20bc00 {
                                                        return Some(ADDP_Vd_Vn_Vm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbfa0fc00 == 0x2e20fc00 {
                                                    return Some(
                                                        FDIV_Vd_V_2S_Vn_V_2S_Vm_V_2S::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x20000000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xffe00c00 == 0x1ee00c00 {
                                                return Some(FCSEL_Fd_Fn_Fm_COND::make_opcode(
                                                    insn,
                                                ));
                                            }
                                            if insn & 0xff200c00 == 0x1e200c00 {
                                                return Some(
                                                    FCSEL_Fd_S_S_Fn_S_S_Fm_S_S_COND::make_opcode(
                                                        insn,
                                                    ),
                                                );
                                            }
                                        } else {
                                            if insn & 0x001000 == 0 {
                                                if insn & 0xffe0fc00 == 0x5ee08c00 {
                                                    return Some(CMTST_Sd_Sn_Sm::make_opcode(insn));
                                                }
                                            } else {
                                                if insn & 0x002000 == 0 {
                                                    if insn & 0xffa0fc00 == 0x5e20dc00 {
                                                        return Some(
                                                            FMULX_Sd_S_S_Sn_S_S_Sm_S_S::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0x5ee03c00 {
                                                        return Some(CMGE_Sd_Sn_Sm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x001000 == 0 {
                                            if insn & 0xffe0fc00 == 0x7ee08c00 {
                                                return Some(CMEQ_Sd_Sn_Sm::make_opcode(insn));
                                            }
                                        } else {
                                            if insn & 0xffe0fc00 == 0x7ee03c00 {
                                                return Some(CMHS_Sd_Sn_Sm::make_opcode(insn));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if insn & 0x008000 == 0 {
                        if insn & 0x000400 == 0 {
                            if insn & 0xbf009400 == 0x2f001000 {
                                return Some(FCMLA_Vd_Vn_Em_IMM_ROT2::make_opcode(insn));
                            }
                        } else {
                            if insn & 0x001000 == 0 {
                                if insn & 0x20000000 == 0 {
                                    if insn & 0xbff89c00 == 0xf000400 {
                                        return Some(MOVI_Vd_SIMD_IMM_SFT::make_opcode(insn));
                                    }
                                } else {
                                    if insn & 0xbff89c00 == 0x2f000400 {
                                        return Some(MVNI_Vd_SIMD_IMM_SFT::make_opcode(insn));
                                    }
                                }
                            } else {
                                if insn & 0x20000000 == 0 {
                                    if insn & 0xbff89c00 == 0xf001400 {
                                        return Some(ORR_Vd_SIMD_IMM_SFT::make_opcode(insn));
                                    }
                                } else {
                                    if insn & 0xbff89c00 == 0x2f001400 {
                                        return Some(BIC_Vd_SIMD_IMM_SFT::make_opcode(insn));
                                    }
                                }
                            }
                        }
                    } else {
                        if insn & 0x10000000 == 0 {
                            if insn & 0x000400 == 0 {
                                if insn & 0x001000 == 0 {
                                    if insn & 0x002000 == 0 {
                                        if insn & 0xbf00f400 == 0xf008000 {
                                            return Some(MUL_Vd_Vn_Em16::make_opcode(insn));
                                        }
                                    } else {
                                        if insn & 0x004000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xff00f400 == 0xf00a000 {
                                                    return Some(SMULL_Vd_Vn_Em16::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            } else {
                                                if insn & 0xff00f400 == 0x4f00a000 {
                                                    return Some(SMULL2_Vd_Vn_Em16::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        } else {
                                            if insn & 0xbf00f400 == 0xf00e000 {
                                                return Some(SDOT_Vd_Vn_Em::make_opcode(insn));
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x002000 == 0 {
                                        if insn & 0x800000 == 0 {
                                            if insn & 0x20000000 == 0 {
                                                if insn & 0xbfc0f400 == 0xf009000 {
                                                    return Some(FMUL_Vd_Vn_Em16::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            } else {
                                                if insn & 0xbfc0f400 == 0x2f009000 {
                                                    return Some(FMULX_Vd_Vn_Em16::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        } else {
                                            if insn & 0x20000000 == 0 {
                                                if insn & 0xbf80f400 == 0xf809000 {
                                                    return Some(FMUL_Vd_Vn_Em::make_opcode(insn));
                                                }
                                            } else {
                                                if insn & 0xbf80f400 == 0x2f809000 {
                                                    return Some(FMULX_Vd_Vn_Em::make_opcode(insn));
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x800000 == 0 {
                                            if insn & 0xbfc0f400 == 0xf40f000 {
                                                return Some(BFDOT_Vd_Vn_Em::make_opcode(insn));
                                            }
                                        } else {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xffc0f400 == 0xfc0f000 {
                                                    return Some(BFMLALB_Vd_Vn_Em16::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            } else {
                                                if insn & 0xffc0f400 == 0x4fc0f000 {
                                                    return Some(BFMLALT_Vd_Vn_Em16::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x000800 == 0 {
                                    if insn & 0x004000 == 0 {
                                        if insn & 0x001000 == 0 {
                                            if insn & 0x20000000 == 0 {
                                                if insn & 0xbff8dc00 == 0xf008400 {
                                                    return Some(
                                                        MOVI_Vd_V_4H_SIMD_IMM_SFT_LSL::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            } else {
                                                if insn & 0xbff8dc00 == 0x2f008400 {
                                                    return Some(
                                                        MVNI_Vd_V_4H_SIMD_IMM_SFT_LSL::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            }
                                        } else {
                                            if insn & 0x20000000 == 0 {
                                                if insn & 0xbff8dc00 == 0xf009400 {
                                                    return Some(
                                                        ORR_Vd_V_4H_SIMD_IMM_SFT_LSL::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            } else {
                                                if insn & 0xbff8dc00 == 0x2f009400 {
                                                    return Some(
                                                        BIC_Vd_V_4H_SIMD_IMM_SFT_LSL::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x002000 == 0 {
                                            if insn & 0x20000000 == 0 {
                                                if insn & 0xbff8ec00 == 0xf00c400 {
                                                    return Some(
                                                        MOVI_Vd_V_2S_SIMD_IMM_SFT_MSL::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            } else {
                                                if insn & 0xbff8ec00 == 0x2f00c400 {
                                                    return Some(
                                                        MVNI_Vd_V_2S_SIMD_IMM_SFT_MSL::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            }
                                        } else {
                                            if insn & 0x001000 == 0 {
                                                if insn & 0x20000000 == 0 {
                                                    if insn & 0xbff8fc00 == 0xf00e400 {
                                                        return Some (MOVI_Vd_V_8B_SIMD_IMM_SFT_LSL :: make_opcode (insn)) ;
                                                    }
                                                    if insn & 0xbf90fc00 == 0xf10e400 {
                                                        return Some(
                                                            SCVTF_Vd_Vn_IMM_VLSR::make_opcode(insn),
                                                        );
                                                    }
                                                    if insn & 0xbf80fc00 == 0xf00e400 {
                                                        return Some (SCVTF_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0xfff8fc00 == 0x2f00e400 {
                                                        return Some(
                                                            MOVI_Sd_SIMD_IMM::make_opcode(insn),
                                                        );
                                                    }
                                                    if insn & 0xfff8fc00 == 0x6f00e400 {
                                                        return Some(
                                                            MOVI_Vd_SIMD_IMM::make_opcode(insn),
                                                        );
                                                    }
                                                    if insn & 0xbf90fc00 == 0x2f10e400 {
                                                        return Some(
                                                            UCVTF_Vd_Vn_IMM_VLSR::make_opcode(insn),
                                                        );
                                                    }
                                                    if insn & 0xbf80fc00 == 0x2f00e400 {
                                                        return Some (UCVTF_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S :: make_opcode (insn)) ;
                                                    }
                                                }
                                            } else {
                                                if insn & 0x20000000 == 0 {
                                                    if insn & 0xbff8fc00 == 0xf00f400 {
                                                        return Some(
                                                            FMOV_Vd_SIMD_FPIMM::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xfff8fc00 == 0x6f00f400 {
                                                        return Some(
                                                            FMOV_Vd_V_2D_SIMD_FPIMM::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x20000000 == 0 {
                                        if insn & 0xbff8fc00 == 0xf00fc00 {
                                            return Some(FMOV_Vd_V_4H_SIMD_FPIMM::make_opcode(
                                                insn,
                                            ));
                                        }
                                        if insn & 0xbf90fc00 == 0xf10fc00 {
                                            return Some(FCVTZS_Vd_Vn_IMM_VLSR::make_opcode(insn));
                                        }
                                        if insn & 0xbf80fc00 == 0xf00fc00 {
                                            return Some(
                                                FCVTZS_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S::make_opcode(
                                                    insn,
                                                ),
                                            );
                                        }
                                    } else {
                                        if insn & 0xbf90fc00 == 0x2f10fc00 {
                                            return Some(FCVTZU_Vd_Vn_IMM_VLSR::make_opcode(insn));
                                        }
                                        if insn & 0xbf80fc00 == 0x2f00fc00 {
                                            return Some(
                                                FCVTZU_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S::make_opcode(
                                                    insn,
                                                ),
                                            );
                                        }
                                    }
                                }
                            }
                        } else {
                            if insn & 0x20000000 == 0 {
                                if insn & 0x40000000 == 0 {
                                    if insn & 0xffe08000 == 0x1fc08000 {
                                        return Some(FMSUB_Fd_Fn_Fm_Fa::make_opcode(insn));
                                    }
                                    if insn & 0xff208000 == 0x1f008000 {
                                        return Some(
                                            FMSUB_Fd_S_S_Fn_S_S_Fm_S_S_Fa_S_S::make_opcode(insn),
                                        );
                                    }
                                } else {
                                    if insn & 0x000400 == 0 {
                                        if insn & 0x800000 == 0 {
                                            if insn & 0xffc0f400 == 0x5f009000 {
                                                return Some(FMUL_Sd_Sn_Em16::make_opcode(insn));
                                            }
                                        } else {
                                            if insn & 0xff80f400 == 0x5f809000 {
                                                return Some(FMUL_Sd_Sn_Em::make_opcode(insn));
                                            }
                                        }
                                    } else {
                                        if insn & 0x000800 == 0 {
                                            if insn & 0xff90fc00 == 0x5f10e400 {
                                                return Some(SCVTF_Sd_Sn_IMM_VLSR::make_opcode(
                                                    insn,
                                                ));
                                            }
                                            if insn & 0xff80fc00 == 0x5f00e400 {
                                                return Some(
                                                    SCVTF_Sd_S_S_Sn_S_S_IMM_VLSR_S_S::make_opcode(
                                                        insn,
                                                    ),
                                                );
                                            }
                                        } else {
                                            if insn & 0xff90fc00 == 0x5f10fc00 {
                                                return Some(FCVTZS_Sd_Sn_IMM_VLSR::make_opcode(
                                                    insn,
                                                ));
                                            }
                                            if insn & 0xff80fc00 == 0x5f00fc00 {
                                                return Some(
                                                    FCVTZS_Sd_S_S_Sn_S_S_IMM_VLSR_S_S::make_opcode(
                                                        insn,
                                                    ),
                                                );
                                            }
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x000400 == 0 {
                                    if insn & 0x800000 == 0 {
                                        if insn & 0xffc0f400 == 0x7f009000 {
                                            return Some(FMULX_Sd_Sn_Em16::make_opcode(insn));
                                        }
                                    } else {
                                        if insn & 0xff80f400 == 0x7f809000 {
                                            return Some(FMULX_Sd_Sn_Em::make_opcode(insn));
                                        }
                                    }
                                } else {
                                    if insn & 0x000800 == 0 {
                                        if insn & 0xff90fc00 == 0x7f10e400 {
                                            return Some(UCVTF_Sd_Sn_IMM_VLSR::make_opcode(insn));
                                        }
                                        if insn & 0xff80fc00 == 0x7f00e400 {
                                            return Some(
                                                UCVTF_Sd_S_S_Sn_S_S_IMM_VLSR_S_S::make_opcode(insn),
                                            );
                                        }
                                    } else {
                                        if insn & 0xff90fc00 == 0x7f10fc00 {
                                            return Some(FCVTZU_Sd_Sn_IMM_VLSR::make_opcode(insn));
                                        }
                                        if insn & 0xff80fc00 == 0x7f00fc00 {
                                            return Some(
                                                FCVTZU_Sd_S_S_Sn_S_S_IMM_VLSR_S_S::make_opcode(
                                                    insn,
                                                ),
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}
