#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
impl InsnOpcode for CONDBRANCH {
    fn definition(&self) -> &'static Insn {
        match self {
            CONDBRANCH::BC__ADDR_PCREL19(opcode) => opcode.definition(),
            CONDBRANCH::B__ADDR_PCREL19(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            CONDBRANCH::BC__ADDR_PCREL19(opcode) => opcode.bits(),
            CONDBRANCH::B__ADDR_PCREL19(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for CONDCMP_IMM {
    fn definition(&self) -> &'static Insn {
        match self {
            CONDCMP_IMM::CCMN_Rn_CCMP_IMM_NZCV_COND(opcode) => opcode.definition(),
            CONDCMP_IMM::CCMP_Rn_CCMP_IMM_NZCV_COND(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            CONDCMP_IMM::CCMN_Rn_CCMP_IMM_NZCV_COND(opcode) => opcode.bits(),
            CONDCMP_IMM::CCMP_Rn_CCMP_IMM_NZCV_COND(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for CONDCMP_REG {
    fn definition(&self) -> &'static Insn {
        match self {
            CONDCMP_REG::CCMN_Rn_Rm_NZCV_COND(opcode) => opcode.definition(),
            CONDCMP_REG::CCMP_Rn_Rm_NZCV_COND(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            CONDCMP_REG::CCMN_Rn_Rm_NZCV_COND(opcode) => opcode.bits(),
            CONDCMP_REG::CCMP_Rn_Rm_NZCV_COND(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for CONDSEL {
    fn definition(&self) -> &'static Insn {
        match self {
            CONDSEL::CSEL_Rd_Rn_Rm_COND(opcode) => opcode.definition(),
            CONDSEL::CSINC_Rd_Rn_Rm_COND(opcode) => opcode.definition(),
            CONDSEL::CSINV_Rd_Rn_Rm_COND(opcode) => opcode.definition(),
            CONDSEL::CSNEG_Rd_Rn_Rm_COND(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            CONDSEL::CSEL_Rd_Rn_Rm_COND(opcode) => opcode.bits(),
            CONDSEL::CSINC_Rd_Rn_Rm_COND(opcode) => opcode.bits(),
            CONDSEL::CSINV_Rd_Rn_Rm_COND(opcode) => opcode.bits(),
            CONDSEL::CSNEG_Rd_Rn_Rm_COND(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for DP_1SRC {
    fn definition(&self) -> &'static Insn {
        match self {
            DP_1SRC::AUTDA_Rd_Rn_SP(opcode) => opcode.definition(),
            DP_1SRC::AUTDB_Rd_Rn_SP(opcode) => opcode.definition(),
            DP_1SRC::AUTDZA_Rd(opcode) => opcode.definition(),
            DP_1SRC::AUTDZB_Rd(opcode) => opcode.definition(),
            DP_1SRC::AUTIA_Rd_Rn_SP(opcode) => opcode.definition(),
            DP_1SRC::AUTIB_Rd_Rn_SP(opcode) => opcode.definition(),
            DP_1SRC::AUTIZA_Rd(opcode) => opcode.definition(),
            DP_1SRC::AUTIZB_Rd(opcode) => opcode.definition(),
            DP_1SRC::CLS_Rd_Rn(opcode) => opcode.definition(),
            DP_1SRC::CLZ_Rd_Rn(opcode) => opcode.definition(),
            DP_1SRC::PACDA_Rd_Rn_SP(opcode) => opcode.definition(),
            DP_1SRC::PACDB_Rd_Rn_SP(opcode) => opcode.definition(),
            DP_1SRC::PACDZA_Rd(opcode) => opcode.definition(),
            DP_1SRC::PACDZB_Rd(opcode) => opcode.definition(),
            DP_1SRC::PACIA_Rd_Rn_SP(opcode) => opcode.definition(),
            DP_1SRC::PACIB_Rd_Rn_SP(opcode) => opcode.definition(),
            DP_1SRC::PACIZA_Rd(opcode) => opcode.definition(),
            DP_1SRC::PACIZB_Rd(opcode) => opcode.definition(),
            DP_1SRC::RBIT_Rd_Rn(opcode) => opcode.definition(),
            DP_1SRC::REV16_Rd_Rn(opcode) => opcode.definition(),
            DP_1SRC::REV32_Rd_Rn(opcode) => opcode.definition(),
            DP_1SRC::REV_Rd_Rn(opcode) => opcode.definition(),
            DP_1SRC::REV_Rd_X_Rn_X(opcode) => opcode.definition(),
            DP_1SRC::XPACD_Rd(opcode) => opcode.definition(),
            DP_1SRC::XPACI_Rd(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            DP_1SRC::AUTDA_Rd_Rn_SP(opcode) => opcode.bits(),
            DP_1SRC::AUTDB_Rd_Rn_SP(opcode) => opcode.bits(),
            DP_1SRC::AUTDZA_Rd(opcode) => opcode.bits(),
            DP_1SRC::AUTDZB_Rd(opcode) => opcode.bits(),
            DP_1SRC::AUTIA_Rd_Rn_SP(opcode) => opcode.bits(),
            DP_1SRC::AUTIB_Rd_Rn_SP(opcode) => opcode.bits(),
            DP_1SRC::AUTIZA_Rd(opcode) => opcode.bits(),
            DP_1SRC::AUTIZB_Rd(opcode) => opcode.bits(),
            DP_1SRC::CLS_Rd_Rn(opcode) => opcode.bits(),
            DP_1SRC::CLZ_Rd_Rn(opcode) => opcode.bits(),
            DP_1SRC::PACDA_Rd_Rn_SP(opcode) => opcode.bits(),
            DP_1SRC::PACDB_Rd_Rn_SP(opcode) => opcode.bits(),
            DP_1SRC::PACDZA_Rd(opcode) => opcode.bits(),
            DP_1SRC::PACDZB_Rd(opcode) => opcode.bits(),
            DP_1SRC::PACIA_Rd_Rn_SP(opcode) => opcode.bits(),
            DP_1SRC::PACIB_Rd_Rn_SP(opcode) => opcode.bits(),
            DP_1SRC::PACIZA_Rd(opcode) => opcode.bits(),
            DP_1SRC::PACIZB_Rd(opcode) => opcode.bits(),
            DP_1SRC::RBIT_Rd_Rn(opcode) => opcode.bits(),
            DP_1SRC::REV16_Rd_Rn(opcode) => opcode.bits(),
            DP_1SRC::REV32_Rd_Rn(opcode) => opcode.bits(),
            DP_1SRC::REV_Rd_Rn(opcode) => opcode.bits(),
            DP_1SRC::REV_Rd_X_Rn_X(opcode) => opcode.bits(),
            DP_1SRC::XPACD_Rd(opcode) => opcode.bits(),
            DP_1SRC::XPACI_Rd(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for DP_2SRC {
    fn definition(&self) -> &'static Insn {
        match self {
            DP_2SRC::ASRV_Rd_Rn_Rm(opcode) => opcode.definition(),
            DP_2SRC::CRC32B_Rd_Rn_Rm(opcode) => opcode.definition(),
            DP_2SRC::CRC32CB_Rd_Rn_Rm(opcode) => opcode.definition(),
            DP_2SRC::CRC32CH_Rd_Rn_Rm(opcode) => opcode.definition(),
            DP_2SRC::CRC32CW_Rd_Rn_Rm(opcode) => opcode.definition(),
            DP_2SRC::CRC32CX_Rd_Rn_Rm(opcode) => opcode.definition(),
            DP_2SRC::CRC32H_Rd_Rn_Rm(opcode) => opcode.definition(),
            DP_2SRC::CRC32W_Rd_Rn_Rm(opcode) => opcode.definition(),
            DP_2SRC::CRC32X_Rd_Rn_Rm(opcode) => opcode.definition(),
            DP_2SRC::GMI_Rd_Rn_SP_Rm(opcode) => opcode.definition(),
            DP_2SRC::IRG_Rd_SP_Rn_SP_Rm(opcode) => opcode.definition(),
            DP_2SRC::LSLV_Rd_Rn_Rm(opcode) => opcode.definition(),
            DP_2SRC::LSRV_Rd_Rn_Rm(opcode) => opcode.definition(),
            DP_2SRC::PACGA_Rd_Rn_Rm_SP(opcode) => opcode.definition(),
            DP_2SRC::RORV_Rd_Rn_Rm(opcode) => opcode.definition(),
            DP_2SRC::SDIV_Rd_Rn_Rm(opcode) => opcode.definition(),
            DP_2SRC::SUBPS_Rd_Rn_SP_Rm_SP(opcode) => opcode.definition(),
            DP_2SRC::SUBP_Rd_Rn_SP_Rm_SP(opcode) => opcode.definition(),
            DP_2SRC::UDIV_Rd_Rn_Rm(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            DP_2SRC::ASRV_Rd_Rn_Rm(opcode) => opcode.bits(),
            DP_2SRC::CRC32B_Rd_Rn_Rm(opcode) => opcode.bits(),
            DP_2SRC::CRC32CB_Rd_Rn_Rm(opcode) => opcode.bits(),
            DP_2SRC::CRC32CH_Rd_Rn_Rm(opcode) => opcode.bits(),
            DP_2SRC::CRC32CW_Rd_Rn_Rm(opcode) => opcode.bits(),
            DP_2SRC::CRC32CX_Rd_Rn_Rm(opcode) => opcode.bits(),
            DP_2SRC::CRC32H_Rd_Rn_Rm(opcode) => opcode.bits(),
            DP_2SRC::CRC32W_Rd_Rn_Rm(opcode) => opcode.bits(),
            DP_2SRC::CRC32X_Rd_Rn_Rm(opcode) => opcode.bits(),
            DP_2SRC::GMI_Rd_Rn_SP_Rm(opcode) => opcode.bits(),
            DP_2SRC::IRG_Rd_SP_Rn_SP_Rm(opcode) => opcode.bits(),
            DP_2SRC::LSLV_Rd_Rn_Rm(opcode) => opcode.bits(),
            DP_2SRC::LSRV_Rd_Rn_Rm(opcode) => opcode.bits(),
            DP_2SRC::PACGA_Rd_Rn_Rm_SP(opcode) => opcode.bits(),
            DP_2SRC::RORV_Rd_Rn_Rm(opcode) => opcode.bits(),
            DP_2SRC::SDIV_Rd_Rn_Rm(opcode) => opcode.bits(),
            DP_2SRC::SUBPS_Rd_Rn_SP_Rm_SP(opcode) => opcode.bits(),
            DP_2SRC::SUBP_Rd_Rn_SP_Rm_SP(opcode) => opcode.bits(),
            DP_2SRC::UDIV_Rd_Rn_Rm(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for DP_3SRC {
    fn definition(&self) -> &'static Insn {
        match self {
            DP_3SRC::MADD_Rd_Rn_Rm_Ra(opcode) => opcode.definition(),
            DP_3SRC::MSUB_Rd_Rn_Rm_Ra(opcode) => opcode.definition(),
            DP_3SRC::SMADDL_Rd_Rn_Rm_Ra(opcode) => opcode.definition(),
            DP_3SRC::SMSUBL_Rd_Rn_Rm_Ra(opcode) => opcode.definition(),
            DP_3SRC::SMULH_Rd_Rn_Rm(opcode) => opcode.definition(),
            DP_3SRC::UMADDL_Rd_Rn_Rm_Ra(opcode) => opcode.definition(),
            DP_3SRC::UMSUBL_Rd_Rn_Rm_Ra(opcode) => opcode.definition(),
            DP_3SRC::UMULH_Rd_Rn_Rm(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            DP_3SRC::MADD_Rd_Rn_Rm_Ra(opcode) => opcode.bits(),
            DP_3SRC::MSUB_Rd_Rn_Rm_Ra(opcode) => opcode.bits(),
            DP_3SRC::SMADDL_Rd_Rn_Rm_Ra(opcode) => opcode.bits(),
            DP_3SRC::SMSUBL_Rd_Rn_Rm_Ra(opcode) => opcode.bits(),
            DP_3SRC::SMULH_Rd_Rn_Rm(opcode) => opcode.bits(),
            DP_3SRC::UMADDL_Rd_Rn_Rm_Ra(opcode) => opcode.bits(),
            DP_3SRC::UMSUBL_Rd_Rn_Rm_Ra(opcode) => opcode.bits(),
            DP_3SRC::UMULH_Rd_Rn_Rm(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for FLOAT2FIX {
    fn definition(&self) -> &'static Insn {
        match self {
            FLOAT2FIX::FCVTZS_Rd_Fn_FBITS(opcode) => opcode.definition(),
            FLOAT2FIX::FCVTZS_Rd_W_Fn_S_D_FBITS_imm_1_32(opcode) => opcode.definition(),
            FLOAT2FIX::FCVTZU_Rd_Fn_FBITS(opcode) => opcode.definition(),
            FLOAT2FIX::FCVTZU_Rd_W_Fn_S_D_FBITS_imm_1_32(opcode) => opcode.definition(),
            FLOAT2FIX::SCVTF_Fd_Rn_FBITS(opcode) => opcode.definition(),
            FLOAT2FIX::SCVTF_Fd_S_D_Rn_W_FBITS_imm_1_32(opcode) => opcode.definition(),
            FLOAT2FIX::UCVTF_Fd_Rn_FBITS(opcode) => opcode.definition(),
            FLOAT2FIX::UCVTF_Fd_S_D_Rn_W_FBITS_imm_1_32(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            FLOAT2FIX::FCVTZS_Rd_Fn_FBITS(opcode) => opcode.bits(),
            FLOAT2FIX::FCVTZS_Rd_W_Fn_S_D_FBITS_imm_1_32(opcode) => opcode.bits(),
            FLOAT2FIX::FCVTZU_Rd_Fn_FBITS(opcode) => opcode.bits(),
            FLOAT2FIX::FCVTZU_Rd_W_Fn_S_D_FBITS_imm_1_32(opcode) => opcode.bits(),
            FLOAT2FIX::SCVTF_Fd_Rn_FBITS(opcode) => opcode.bits(),
            FLOAT2FIX::SCVTF_Fd_S_D_Rn_W_FBITS_imm_1_32(opcode) => opcode.bits(),
            FLOAT2FIX::UCVTF_Fd_Rn_FBITS(opcode) => opcode.bits(),
            FLOAT2FIX::UCVTF_Fd_S_D_Rn_W_FBITS_imm_1_32(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for FLOAT2INT {
    fn definition(&self) -> &'static Insn {
        match self {
            FLOAT2INT::FCVTAS_Rd_Fn(opcode) => opcode.definition(),
            FLOAT2INT::FCVTAS_Rd_W_Fn_S_D(opcode) => opcode.definition(),
            FLOAT2INT::FCVTAU_Rd_Fn(opcode) => opcode.definition(),
            FLOAT2INT::FCVTAU_Rd_W_Fn_S_D(opcode) => opcode.definition(),
            FLOAT2INT::FCVTMS_Rd_Fn(opcode) => opcode.definition(),
            FLOAT2INT::FCVTMS_Rd_W_Fn_S_D(opcode) => opcode.definition(),
            FLOAT2INT::FCVTMU_Rd_Fn(opcode) => opcode.definition(),
            FLOAT2INT::FCVTMU_Rd_W_Fn_S_D(opcode) => opcode.definition(),
            FLOAT2INT::FCVTNS_Rd_Fn(opcode) => opcode.definition(),
            FLOAT2INT::FCVTNS_Rd_W_Fn_S_D(opcode) => opcode.definition(),
            FLOAT2INT::FCVTNU_Rd_Fn(opcode) => opcode.definition(),
            FLOAT2INT::FCVTNU_Rd_W_Fn_S_D(opcode) => opcode.definition(),
            FLOAT2INT::FCVTPS_Rd_Fn(opcode) => opcode.definition(),
            FLOAT2INT::FCVTPS_Rd_W_Fn_S_D(opcode) => opcode.definition(),
            FLOAT2INT::FCVTPU_Rd_Fn(opcode) => opcode.definition(),
            FLOAT2INT::FCVTPU_Rd_W_Fn_S_D(opcode) => opcode.definition(),
            FLOAT2INT::FCVTZS_Rd_Fn(opcode) => opcode.definition(),
            FLOAT2INT::FCVTZS_Rd_W_Fn_S_D(opcode) => opcode.definition(),
            FLOAT2INT::FCVTZU_Rd_Fn(opcode) => opcode.definition(),
            FLOAT2INT::FCVTZU_Rd_W_Fn_S_D(opcode) => opcode.definition(),
            FLOAT2INT::FJCVTZS_Rd_Fn(opcode) => opcode.definition(),
            FLOAT2INT::FMOV_Fd_Rn(opcode) => opcode.definition(),
            FLOAT2INT::FMOV_Fd_S_S_Rn_W(opcode) => opcode.definition(),
            FLOAT2INT::FMOV_Rd_Fn(opcode) => opcode.definition(),
            FLOAT2INT::FMOV_Rd_VnD1(opcode) => opcode.definition(),
            FLOAT2INT::FMOV_Rd_W_Fn_S_S(opcode) => opcode.definition(),
            FLOAT2INT::FMOV_VdD1_Rn(opcode) => opcode.definition(),
            FLOAT2INT::SCVTF_Fd_Rn(opcode) => opcode.definition(),
            FLOAT2INT::SCVTF_Fd_S_D_Rn_W(opcode) => opcode.definition(),
            FLOAT2INT::UCVTF_Fd_Rn(opcode) => opcode.definition(),
            FLOAT2INT::UCVTF_Fd_S_D_Rn_W(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            FLOAT2INT::FCVTAS_Rd_Fn(opcode) => opcode.bits(),
            FLOAT2INT::FCVTAS_Rd_W_Fn_S_D(opcode) => opcode.bits(),
            FLOAT2INT::FCVTAU_Rd_Fn(opcode) => opcode.bits(),
            FLOAT2INT::FCVTAU_Rd_W_Fn_S_D(opcode) => opcode.bits(),
            FLOAT2INT::FCVTMS_Rd_Fn(opcode) => opcode.bits(),
            FLOAT2INT::FCVTMS_Rd_W_Fn_S_D(opcode) => opcode.bits(),
            FLOAT2INT::FCVTMU_Rd_Fn(opcode) => opcode.bits(),
            FLOAT2INT::FCVTMU_Rd_W_Fn_S_D(opcode) => opcode.bits(),
            FLOAT2INT::FCVTNS_Rd_Fn(opcode) => opcode.bits(),
            FLOAT2INT::FCVTNS_Rd_W_Fn_S_D(opcode) => opcode.bits(),
            FLOAT2INT::FCVTNU_Rd_Fn(opcode) => opcode.bits(),
            FLOAT2INT::FCVTNU_Rd_W_Fn_S_D(opcode) => opcode.bits(),
            FLOAT2INT::FCVTPS_Rd_Fn(opcode) => opcode.bits(),
            FLOAT2INT::FCVTPS_Rd_W_Fn_S_D(opcode) => opcode.bits(),
            FLOAT2INT::FCVTPU_Rd_Fn(opcode) => opcode.bits(),
            FLOAT2INT::FCVTPU_Rd_W_Fn_S_D(opcode) => opcode.bits(),
            FLOAT2INT::FCVTZS_Rd_Fn(opcode) => opcode.bits(),
            FLOAT2INT::FCVTZS_Rd_W_Fn_S_D(opcode) => opcode.bits(),
            FLOAT2INT::FCVTZU_Rd_Fn(opcode) => opcode.bits(),
            FLOAT2INT::FCVTZU_Rd_W_Fn_S_D(opcode) => opcode.bits(),
            FLOAT2INT::FJCVTZS_Rd_Fn(opcode) => opcode.bits(),
            FLOAT2INT::FMOV_Fd_Rn(opcode) => opcode.bits(),
            FLOAT2INT::FMOV_Fd_S_S_Rn_W(opcode) => opcode.bits(),
            FLOAT2INT::FMOV_Rd_Fn(opcode) => opcode.bits(),
            FLOAT2INT::FMOV_Rd_VnD1(opcode) => opcode.bits(),
            FLOAT2INT::FMOV_Rd_W_Fn_S_S(opcode) => opcode.bits(),
            FLOAT2INT::FMOV_VdD1_Rn(opcode) => opcode.bits(),
            FLOAT2INT::SCVTF_Fd_Rn(opcode) => opcode.bits(),
            FLOAT2INT::SCVTF_Fd_S_D_Rn_W(opcode) => opcode.bits(),
            FLOAT2INT::UCVTF_Fd_Rn(opcode) => opcode.bits(),
            FLOAT2INT::UCVTF_Fd_S_D_Rn_W(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for FLOATCCMP {
    fn definition(&self) -> &'static Insn {
        match self {
            FLOATCCMP::FCCMPE_Fn_Fm_NZCV_COND(opcode) => opcode.definition(),
            FLOATCCMP::FCCMPE_Fn_S_S_Fm_S_S_NZCV_COND(opcode) => opcode.definition(),
            FLOATCCMP::FCCMP_Fn_Fm_NZCV_COND(opcode) => opcode.definition(),
            FLOATCCMP::FCCMP_Fn_S_S_Fm_S_S_NZCV_COND(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            FLOATCCMP::FCCMPE_Fn_Fm_NZCV_COND(opcode) => opcode.bits(),
            FLOATCCMP::FCCMPE_Fn_S_S_Fm_S_S_NZCV_COND(opcode) => opcode.bits(),
            FLOATCCMP::FCCMP_Fn_Fm_NZCV_COND(opcode) => opcode.bits(),
            FLOATCCMP::FCCMP_Fn_S_S_Fm_S_S_NZCV_COND(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for FLOATCMP {
    fn definition(&self) -> &'static Insn {
        match self {
            FLOATCMP::FCMPE_Fn_FPIMM0(opcode) => opcode.definition(),
            FLOATCMP::FCMPE_Fn_Fm(opcode) => opcode.definition(),
            FLOATCMP::FCMPE_Fn_S_S_FPIMM0(opcode) => opcode.definition(),
            FLOATCMP::FCMPE_Fn_S_S_Fm_S_S(opcode) => opcode.definition(),
            FLOATCMP::FCMP_Fn_FPIMM0(opcode) => opcode.definition(),
            FLOATCMP::FCMP_Fn_Fm(opcode) => opcode.definition(),
            FLOATCMP::FCMP_Fn_S_S_FPIMM0(opcode) => opcode.definition(),
            FLOATCMP::FCMP_Fn_S_S_Fm_S_S(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            FLOATCMP::FCMPE_Fn_FPIMM0(opcode) => opcode.bits(),
            FLOATCMP::FCMPE_Fn_Fm(opcode) => opcode.bits(),
            FLOATCMP::FCMPE_Fn_S_S_FPIMM0(opcode) => opcode.bits(),
            FLOATCMP::FCMPE_Fn_S_S_Fm_S_S(opcode) => opcode.bits(),
            FLOATCMP::FCMP_Fn_FPIMM0(opcode) => opcode.bits(),
            FLOATCMP::FCMP_Fn_Fm(opcode) => opcode.bits(),
            FLOATCMP::FCMP_Fn_S_S_FPIMM0(opcode) => opcode.bits(),
            FLOATCMP::FCMP_Fn_S_S_Fm_S_S(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for FLOATIMM {
    fn definition(&self) -> &'static Insn {
        match self {
            FLOATIMM::FMOV_Fd_FPIMM(opcode) => opcode.definition(),
            FLOATIMM::FMOV_Fd_S_S_FPIMM(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            FLOATIMM::FMOV_Fd_FPIMM(opcode) => opcode.bits(),
            FLOATIMM::FMOV_Fd_S_S_FPIMM(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for FLOATSEL {
    fn definition(&self) -> &'static Insn {
        match self {
            FLOATSEL::FCSEL_Fd_Fn_Fm_COND(opcode) => opcode.definition(),
            FLOATSEL::FCSEL_Fd_S_S_Fn_S_S_Fm_S_S_COND(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            FLOATSEL::FCSEL_Fd_Fn_Fm_COND(opcode) => opcode.bits(),
            FLOATSEL::FCSEL_Fd_S_S_Fn_S_S_Fm_S_S_COND(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LDSTEXCL {
    fn definition(&self) -> &'static Insn {
        match self {
            LDSTEXCL::LDAPRB_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDAPRH_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDAPR_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDARB_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDARH_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDAR_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDAXP_Rt_Rt2_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDAXRB_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDAXRH_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDAXR_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDGM_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDLARB_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDLARH_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDLAR_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDXP_Rt_Rt2_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDXRB_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDXRH_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDXR_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STGM_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STLLRB_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STLLRH_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STLLR_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STLRB_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STLRH_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STLR_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STLXP_Rs_Rt_Rt2_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STLXRB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STLXRH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STLXR_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STXP_Rs_Rt_Rt2_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STXRB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STXRH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STXR_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STZGM_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LDSTEXCL::LDAPRB_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDAPRH_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDAPR_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDARB_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDARH_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDAR_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDAXP_Rt_Rt2_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDAXRB_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDAXRH_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDAXR_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDGM_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDLARB_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDLARH_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDLAR_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDXP_Rt_Rt2_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDXRB_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDXRH_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDXR_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STGM_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STLLRB_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STLLRH_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STLLR_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STLRB_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STLRH_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STLR_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STLXP_Rs_Rt_Rt2_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STLXRB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STLXRH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STLXR_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STXP_Rs_Rt_Rt2_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STXRB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STXRH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STXR_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STZGM_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LDSTNAPAIR_OFFS {
    fn definition(&self) -> &'static Insn {
        match self {
            LDSTNAPAIR_OFFS::LDNP_Ft_Ft2_ADDR_SIMM7(opcode) => opcode.definition(),
            LDSTNAPAIR_OFFS::LDNP_Rt_Rt2_ADDR_SIMM7(opcode) => opcode.definition(),
            LDSTNAPAIR_OFFS::STNP_Ft_Ft2_ADDR_SIMM7(opcode) => opcode.definition(),
            LDSTNAPAIR_OFFS::STNP_Rt_Rt2_ADDR_SIMM7(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LDSTNAPAIR_OFFS::LDNP_Ft_Ft2_ADDR_SIMM7(opcode) => opcode.bits(),
            LDSTNAPAIR_OFFS::LDNP_Rt_Rt2_ADDR_SIMM7(opcode) => opcode.bits(),
            LDSTNAPAIR_OFFS::STNP_Ft_Ft2_ADDR_SIMM7(opcode) => opcode.bits(),
            LDSTNAPAIR_OFFS::STNP_Rt_Rt2_ADDR_SIMM7(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LDSTPAIR_INDEXED {
    fn definition(&self) -> &'static Insn {
        match self {
            LDSTPAIR_INDEXED::LDPSW_Rt_X_Rt2_X_ADDR_SIMM7_S_S(opcode) => opcode.definition(),
            LDSTPAIR_INDEXED::LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S(opcode) => opcode.definition(),
            LDSTPAIR_INDEXED::LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S(opcode) => opcode.definition(),
            LDSTPAIR_INDEXED::STGP_Rt_X_Rt2_X_ADDR_SIMM11_imm_tag(opcode) => opcode.definition(),
            LDSTPAIR_INDEXED::STP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S(opcode) => opcode.definition(),
            LDSTPAIR_INDEXED::STP_Rt_W_Rt2_W_ADDR_SIMM7_S_S(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LDSTPAIR_INDEXED::LDPSW_Rt_X_Rt2_X_ADDR_SIMM7_S_S(opcode) => opcode.bits(),
            LDSTPAIR_INDEXED::LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S(opcode) => opcode.bits(),
            LDSTPAIR_INDEXED::LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S(opcode) => opcode.bits(),
            LDSTPAIR_INDEXED::STGP_Rt_X_Rt2_X_ADDR_SIMM11_imm_tag(opcode) => opcode.bits(),
            LDSTPAIR_INDEXED::STP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S(opcode) => opcode.bits(),
            LDSTPAIR_INDEXED::STP_Rt_W_Rt2_W_ADDR_SIMM7_S_S(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LDSTPAIR_OFF {
    fn definition(&self) -> &'static Insn {
        match self {
            LDSTPAIR_OFF::LDPSW_Rt_Rt2_ADDR_SIMM7(opcode) => opcode.definition(),
            LDSTPAIR_OFF::LDP_Ft_Ft2_ADDR_SIMM7(opcode) => opcode.definition(),
            LDSTPAIR_OFF::LDP_Rt_Rt2_ADDR_SIMM7(opcode) => opcode.definition(),
            LDSTPAIR_OFF::STGP_Rt_Rt2_ADDR_SIMM11(opcode) => opcode.definition(),
            LDSTPAIR_OFF::STP_Ft_Ft2_ADDR_SIMM7(opcode) => opcode.definition(),
            LDSTPAIR_OFF::STP_Rt_Rt2_ADDR_SIMM7(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LDSTPAIR_OFF::LDPSW_Rt_Rt2_ADDR_SIMM7(opcode) => opcode.bits(),
            LDSTPAIR_OFF::LDP_Ft_Ft2_ADDR_SIMM7(opcode) => opcode.bits(),
            LDSTPAIR_OFF::LDP_Rt_Rt2_ADDR_SIMM7(opcode) => opcode.bits(),
            LDSTPAIR_OFF::STGP_Rt_Rt2_ADDR_SIMM11(opcode) => opcode.bits(),
            LDSTPAIR_OFF::STP_Ft_Ft2_ADDR_SIMM7(opcode) => opcode.bits(),
            LDSTPAIR_OFF::STP_Rt_Rt2_ADDR_SIMM7(opcode) => opcode.bits(),
        }
    }
}
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
            LDST_IMM9::ST2G_Rt_SP_X_ADDR_SIMM13_imm_tag(opcode) => opcode.definition(),
            LDST_IMM9::STG_Rt_SP_X_ADDR_SIMM13_imm_tag(opcode) => opcode.definition(),
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
            LDST_IMM9::ST2G_Rt_SP_X_ADDR_SIMM13_imm_tag(opcode) => opcode.bits(),
            LDST_IMM9::STG_Rt_SP_X_ADDR_SIMM13_imm_tag(opcode) => opcode.bits(),
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
            LDST_POS::PRFM_PRFOP_ADDR_UIMM12(opcode) => opcode.definition(),
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
            LDST_POS::PRFM_PRFOP_ADDR_UIMM12(opcode) => opcode.bits(),
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
            LDST_REGOFF::PRFM_PRFOP_ADDR_REGOFF(opcode) => opcode.definition(),
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
            LDST_REGOFF::PRFM_PRFOP_ADDR_REGOFF(opcode) => opcode.bits(),
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
            LDST_UNPRIV::LDTRB_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNPRIV::LDTRH_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNPRIV::LDTRSB_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNPRIV::LDTRSH_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNPRIV::LDTRSW_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNPRIV::LDTR_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNPRIV::STTRB_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNPRIV::STTRH_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNPRIV::STTR_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LDST_UNPRIV::LDTRB_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNPRIV::LDTRH_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNPRIV::LDTRSB_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNPRIV::LDTRSH_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNPRIV::LDTRSW_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNPRIV::LDTR_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNPRIV::STTRB_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNPRIV::STTRH_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNPRIV::STTR_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LDST_UNSCALED {
    fn definition(&self) -> &'static Insn {
        match self {
            LDST_UNSCALED::LDAPURB_Rt_ADDR_OFFSET(opcode) => opcode.definition(),
            LDST_UNSCALED::LDAPURH_Rt_ADDR_OFFSET(opcode) => opcode.definition(),
            LDST_UNSCALED::LDAPURSB_Rt_ADDR_OFFSET(opcode) => opcode.definition(),
            LDST_UNSCALED::LDAPURSB_Rt_W_ADDR_OFFSET(opcode) => opcode.definition(),
            LDST_UNSCALED::LDAPURSH_Rt_ADDR_OFFSET(opcode) => opcode.definition(),
            LDST_UNSCALED::LDAPURSH_Rt_W_ADDR_OFFSET(opcode) => opcode.definition(),
            LDST_UNSCALED::LDAPURSW_Rt_ADDR_OFFSET(opcode) => opcode.definition(),
            LDST_UNSCALED::LDAPUR_Rt_ADDR_OFFSET(opcode) => opcode.definition(),
            LDST_UNSCALED::LDAPUR_Rt_X_ADDR_OFFSET(opcode) => opcode.definition(),
            LDST_UNSCALED::LDG_Rt_ADDR_SIMM13(opcode) => opcode.definition(),
            LDST_UNSCALED::LDURB_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::LDURH_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::LDURSB_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::LDURSH_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::LDURSW_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::LDUR_Ft_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::LDUR_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::PRFUM_PRFOP_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::ST2G_Rt_SP_ADDR_SIMM13(opcode) => opcode.definition(),
            LDST_UNSCALED::STG_Rt_SP_ADDR_SIMM13(opcode) => opcode.definition(),
            LDST_UNSCALED::STLURB_Rt_ADDR_OFFSET(opcode) => opcode.definition(),
            LDST_UNSCALED::STLURH_Rt_ADDR_OFFSET(opcode) => opcode.definition(),
            LDST_UNSCALED::STLUR_Rt_ADDR_OFFSET(opcode) => opcode.definition(),
            LDST_UNSCALED::STLUR_Rt_X_ADDR_OFFSET(opcode) => opcode.definition(),
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
            LDST_UNSCALED::LDAPURB_Rt_ADDR_OFFSET(opcode) => opcode.bits(),
            LDST_UNSCALED::LDAPURH_Rt_ADDR_OFFSET(opcode) => opcode.bits(),
            LDST_UNSCALED::LDAPURSB_Rt_ADDR_OFFSET(opcode) => opcode.bits(),
            LDST_UNSCALED::LDAPURSB_Rt_W_ADDR_OFFSET(opcode) => opcode.bits(),
            LDST_UNSCALED::LDAPURSH_Rt_ADDR_OFFSET(opcode) => opcode.bits(),
            LDST_UNSCALED::LDAPURSH_Rt_W_ADDR_OFFSET(opcode) => opcode.bits(),
            LDST_UNSCALED::LDAPURSW_Rt_ADDR_OFFSET(opcode) => opcode.bits(),
            LDST_UNSCALED::LDAPUR_Rt_ADDR_OFFSET(opcode) => opcode.bits(),
            LDST_UNSCALED::LDAPUR_Rt_X_ADDR_OFFSET(opcode) => opcode.bits(),
            LDST_UNSCALED::LDG_Rt_ADDR_SIMM13(opcode) => opcode.bits(),
            LDST_UNSCALED::LDURB_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::LDURH_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::LDURSB_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::LDURSH_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::LDURSW_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::LDUR_Ft_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::LDUR_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::PRFUM_PRFOP_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::ST2G_Rt_SP_ADDR_SIMM13(opcode) => opcode.bits(),
            LDST_UNSCALED::STG_Rt_SP_ADDR_SIMM13(opcode) => opcode.bits(),
            LDST_UNSCALED::STLURB_Rt_ADDR_OFFSET(opcode) => opcode.bits(),
            LDST_UNSCALED::STLURH_Rt_ADDR_OFFSET(opcode) => opcode.bits(),
            LDST_UNSCALED::STLUR_Rt_ADDR_OFFSET(opcode) => opcode.bits(),
            LDST_UNSCALED::STLUR_Rt_X_ADDR_OFFSET(opcode) => opcode.bits(),
            LDST_UNSCALED::STURB_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::STURH_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::STUR_Ft_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::STUR_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::STZ2G_Rt_SP_ADDR_SIMM13(opcode) => opcode.bits(),
            LDST_UNSCALED::STZG_Rt_SP_ADDR_SIMM13(opcode) => opcode.bits(),
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
            LOG_SHIFT::EON_Rd_Rn_Rm_SFT(opcode) => opcode.definition(),
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
            LOG_SHIFT::EON_Rd_Rn_Rm_SFT(opcode) => opcode.bits(),
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
            Operation::ADDSUB_CARRY(class) => class.definition(),
            Operation::ADDSUB_EXT(class) => class.definition(),
            Operation::ADDSUB_IMM(class) => class.definition(),
            Operation::ADDSUB_SHIFT(class) => class.definition(),
            Operation::ASIMDALL(class) => class.definition(),
            Operation::ASIMDDIFF(class) => class.definition(),
            Operation::ASIMDELEM(class) => class.definition(),
            Operation::ASIMDEXT(class) => class.definition(),
            Operation::ASIMDIMM(class) => class.definition(),
            Operation::ASIMDINS(class) => class.definition(),
            Operation::ASIMDMISC(class) => class.definition(),
            Operation::ASIMDPERM(class) => class.definition(),
            Operation::ASIMDSAME(class) => class.definition(),
            Operation::ASIMDSHF(class) => class.definition(),
            Operation::ASIMDTBL(class) => class.definition(),
            Operation::ASISDDIFF(class) => class.definition(),
            Operation::ASISDELEM(class) => class.definition(),
            Operation::ASISDLSE(class) => class.definition(),
            Operation::ASISDLSEP(class) => class.definition(),
            Operation::ASISDLSO(class) => class.definition(),
            Operation::ASISDLSOP(class) => class.definition(),
            Operation::ASISDMISC(class) => class.definition(),
            Operation::ASISDONE(class) => class.definition(),
            Operation::ASISDPAIR(class) => class.definition(),
            Operation::ASISDSAME(class) => class.definition(),
            Operation::ASISDSHF(class) => class.definition(),
            Operation::BITFIELD(class) => class.definition(),
            Operation::BRANCH_IMM(class) => class.definition(),
            Operation::BRANCH_REG(class) => class.definition(),
            Operation::COMPBRANCH(class) => class.definition(),
            Operation::CONDBRANCH(class) => class.definition(),
            Operation::CONDCMP_IMM(class) => class.definition(),
            Operation::CONDCMP_REG(class) => class.definition(),
            Operation::CONDSEL(class) => class.definition(),
            Operation::DP_1SRC(class) => class.definition(),
            Operation::DP_2SRC(class) => class.definition(),
            Operation::DP_3SRC(class) => class.definition(),
            Operation::FLOAT2FIX(class) => class.definition(),
            Operation::FLOAT2INT(class) => class.definition(),
            Operation::FLOATCCMP(class) => class.definition(),
            Operation::FLOATCMP(class) => class.definition(),
            Operation::FLOATIMM(class) => class.definition(),
            Operation::FLOATSEL(class) => class.definition(),
            Operation::LDSTEXCL(class) => class.definition(),
            Operation::LDSTNAPAIR_OFFS(class) => class.definition(),
            Operation::LDSTPAIR_INDEXED(class) => class.definition(),
            Operation::LDSTPAIR_OFF(class) => class.definition(),
            Operation::LDST_IMM10(class) => class.definition(),
            Operation::LDST_IMM9(class) => class.definition(),
            Operation::LDST_POS(class) => class.definition(),
            Operation::LDST_REGOFF(class) => class.definition(),
            Operation::LDST_UNPRIV(class) => class.definition(),
            Operation::LDST_UNSCALED(class) => class.definition(),
            Operation::LOG_IMM(class) => class.definition(),
            Operation::LOG_SHIFT(class) => class.definition(),
            Operation::MOVEWIDE(class) => class.definition(),
            Operation::PCRELADDR(class) => class.definition(),
            Operation::TESTBRANCH(class) => class.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            Operation::ADDSUB_CARRY(class) => class.bits(),
            Operation::ADDSUB_EXT(class) => class.bits(),
            Operation::ADDSUB_IMM(class) => class.bits(),
            Operation::ADDSUB_SHIFT(class) => class.bits(),
            Operation::ASIMDALL(class) => class.bits(),
            Operation::ASIMDDIFF(class) => class.bits(),
            Operation::ASIMDELEM(class) => class.bits(),
            Operation::ASIMDEXT(class) => class.bits(),
            Operation::ASIMDIMM(class) => class.bits(),
            Operation::ASIMDINS(class) => class.bits(),
            Operation::ASIMDMISC(class) => class.bits(),
            Operation::ASIMDPERM(class) => class.bits(),
            Operation::ASIMDSAME(class) => class.bits(),
            Operation::ASIMDSHF(class) => class.bits(),
            Operation::ASIMDTBL(class) => class.bits(),
            Operation::ASISDDIFF(class) => class.bits(),
            Operation::ASISDELEM(class) => class.bits(),
            Operation::ASISDLSE(class) => class.bits(),
            Operation::ASISDLSEP(class) => class.bits(),
            Operation::ASISDLSO(class) => class.bits(),
            Operation::ASISDLSOP(class) => class.bits(),
            Operation::ASISDMISC(class) => class.bits(),
            Operation::ASISDONE(class) => class.bits(),
            Operation::ASISDPAIR(class) => class.bits(),
            Operation::ASISDSAME(class) => class.bits(),
            Operation::ASISDSHF(class) => class.bits(),
            Operation::BITFIELD(class) => class.bits(),
            Operation::BRANCH_IMM(class) => class.bits(),
            Operation::BRANCH_REG(class) => class.bits(),
            Operation::COMPBRANCH(class) => class.bits(),
            Operation::CONDBRANCH(class) => class.bits(),
            Operation::CONDCMP_IMM(class) => class.bits(),
            Operation::CONDCMP_REG(class) => class.bits(),
            Operation::CONDSEL(class) => class.bits(),
            Operation::DP_1SRC(class) => class.bits(),
            Operation::DP_2SRC(class) => class.bits(),
            Operation::DP_3SRC(class) => class.bits(),
            Operation::FLOAT2FIX(class) => class.bits(),
            Operation::FLOAT2INT(class) => class.bits(),
            Operation::FLOATCCMP(class) => class.bits(),
            Operation::FLOATCMP(class) => class.bits(),
            Operation::FLOATIMM(class) => class.bits(),
            Operation::FLOATSEL(class) => class.bits(),
            Operation::LDSTEXCL(class) => class.bits(),
            Operation::LDSTNAPAIR_OFFS(class) => class.bits(),
            Operation::LDSTPAIR_INDEXED(class) => class.bits(),
            Operation::LDSTPAIR_OFF(class) => class.bits(),
            Operation::LDST_IMM10(class) => class.bits(),
            Operation::LDST_IMM9(class) => class.bits(),
            Operation::LDST_POS(class) => class.bits(),
            Operation::LDST_REGOFF(class) => class.bits(),
            Operation::LDST_UNPRIV(class) => class.bits(),
            Operation::LDST_UNSCALED(class) => class.bits(),
            Operation::LOG_IMM(class) => class.bits(),
            Operation::LOG_SHIFT(class) => class.bits(),
            Operation::MOVEWIDE(class) => class.bits(),
            Operation::PCRELADDR(class) => class.bits(),
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
                    if insn & 0x80000000 == 0 {
                        if insn & 0x9f000000 == 0x10000000 {
                            return Some(ADR_Rd_ADDR_PCREL21::make_opcode(insn));
                        }
                    } else {
                        if insn & 0x9f000000 == 0x90000000 {
                            return Some(ADRP_Rd_ADDR_ADRP::make_opcode(insn));
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
                                return Some(ADDG_Rd_SP_Rn_SP_UIMM10_UIMM4_ADDG::make_opcode(insn));
                            }
                        } else {
                            if insn & 0xffc0c000 == 0xd1800000 {
                                return Some(SUBG_Rd_SP_Rn_SP_UIMM10_UIMM4_ADDG::make_opcode(insn));
                            }
                        }
                    }
                }
            } else {
                if insn & 0x800000 == 0 {
                    if insn & 0x1000000 == 0 {
                        if insn & 0x10000000 == 0 {
                            if insn & 0x400000 == 0 {
                                if insn & 0x20000000 == 0 {
                                    if insn & 0x008000 == 0 {
                                        if insn & 0x200000 == 0 {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xffe0fc00 == 0x8007c00 {
                                                        return Some(
                                                            STXRB_Rs_Rt_ADDR_SIMPLE::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0x48007c00 {
                                                        return Some(
                                                            STXRH_Rs_Rt_ADDR_SIMPLE::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbfe0fc00 == 0x88007c00 {
                                                    return Some(
                                                        STXR_Rs_Rt_ADDR_SIMPLE::make_opcode(insn),
                                                    );
                                                }
                                            }
                                        } else {
                                            if insn & 0xbfe08000 == 0x88200000 {
                                                return Some(
                                                    STXP_Rs_Rt_Rt2_ADDR_SIMPLE::make_opcode(insn),
                                                );
                                            }
                                        }
                                    } else {
                                        if insn & 0x200000 == 0 {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xffe0fc00 == 0x800fc00 {
                                                        return Some(
                                                            STLXRB_Rs_Rt_ADDR_SIMPLE::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0x4800fc00 {
                                                        return Some(
                                                            STLXRH_Rs_Rt_ADDR_SIMPLE::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbfe0fc00 == 0x8800fc00 {
                                                    return Some(
                                                        STLXR_Rs_Rt_ADDR_SIMPLE::make_opcode(insn),
                                                    );
                                                }
                                            }
                                        } else {
                                            if insn & 0xbfe08000 == 0x88208000 {
                                                return Some(
                                                    STLXP_Rs_Rt_Rt2_ADDR_SIMPLE::make_opcode(insn),
                                                );
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x7fc00000 == 0x28000000 {
                                        return Some(STNP_Rt_Rt2_ADDR_SIMM7::make_opcode(insn));
                                    }
                                }
                            } else {
                                if insn & 0x20000000 == 0 {
                                    if insn & 0x008000 == 0 {
                                        if insn & 0x200000 == 0 {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xfffffc00 == 0x85f7c00 {
                                                        return Some(
                                                            LDXRB_Rt_ADDR_SIMPLE::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xfffffc00 == 0x485f7c00 {
                                                        return Some(
                                                            LDXRH_Rt_ADDR_SIMPLE::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbffffc00 == 0x885f7c00 {
                                                    return Some(LDXR_Rt_ADDR_SIMPLE::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        } else {
                                            if insn & 0xbfff8000 == 0x887f0000 {
                                                return Some(LDXP_Rt_Rt2_ADDR_SIMPLE::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    } else {
                                        if insn & 0x200000 == 0 {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xfffffc00 == 0x85ffc00 {
                                                        return Some(
                                                            LDAXRB_Rt_ADDR_SIMPLE::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xfffffc00 == 0x485ffc00 {
                                                        return Some(
                                                            LDAXRH_Rt_ADDR_SIMPLE::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbffffc00 == 0x885ffc00 {
                                                    return Some(
                                                        LDAXR_Rt_ADDR_SIMPLE::make_opcode(insn),
                                                    );
                                                }
                                            }
                                        } else {
                                            if insn & 0xbfff8000 == 0x887f8000 {
                                                return Some(
                                                    LDAXP_Rt_Rt2_ADDR_SIMPLE::make_opcode(insn),
                                                );
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x7fc00000 == 0x28400000 {
                                        return Some(LDNP_Rt_Rt2_ADDR_SIMM7::make_opcode(insn));
                                    }
                                }
                            }
                        } else {
                            if insn & 0x000400 == 0 {
                                if insn & 0x000800 == 0 {
                                    if insn & 0x400000 == 0 {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xffe00c00 == 0x38000000 {
                                                    return Some(STURB_Rt_ADDR_SIMM9::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            } else {
                                                if insn & 0xffe00c00 == 0x78000000 {
                                                    return Some(STURH_Rt_ADDR_SIMM9::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        } else {
                                            if insn & 0xbfe00c00 == 0xb8000000 {
                                                return Some(STUR_Rt_ADDR_SIMM9::make_opcode(insn));
                                            }
                                        }
                                    } else {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xffe00c00 == 0x38400000 {
                                                    return Some(LDURB_Rt_ADDR_SIMM9::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            } else {
                                                if insn & 0xffe00c00 == 0x78400000 {
                                                    return Some(LDURH_Rt_ADDR_SIMM9::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        } else {
                                            if insn & 0xbfe00c00 == 0xb8400000 {
                                                return Some(LDUR_Rt_ADDR_SIMM9::make_opcode(insn));
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x200000 == 0 {
                                        if insn & 0x400000 == 0 {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x38000800 {
                                                        return Some(
                                                            STTRB_Rt_ADDR_SIMM9::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x78000800 {
                                                        return Some(
                                                            STTRH_Rt_ADDR_SIMM9::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbfe00c00 == 0xb8000800 {
                                                    return Some(STTR_Rt_ADDR_SIMM9::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        } else {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x38400800 {
                                                        return Some(
                                                            LDTRB_Rt_ADDR_SIMM9::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x78400800 {
                                                        return Some(
                                                            LDTRH_Rt_ADDR_SIMM9::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbfe00c00 == 0xb8400800 {
                                                    return Some(LDTR_Rt_ADDR_SIMM9::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x400000 == 0 {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x38200800 {
                                                        return Some(
                                                            STRB_Rt_ADDR_REGOFF::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x78200800 {
                                                        return Some(
                                                            STRH_Rt_ADDR_REGOFF::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbfe00c00 == 0xb8200800 {
                                                    return Some(STR_Rt_ADDR_REGOFF::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        } else {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x38600800 {
                                                        return Some(
                                                            LDRB_Rt_ADDR_REGOFF::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x78600800 {
                                                        return Some(
                                                            LDRH_Rt_ADDR_REGOFF::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbfe00c00 == 0xb8600800 {
                                                    return Some(LDR_Rt_ADDR_REGOFF::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x200000 == 0 {
                                    if insn & 0x400000 == 0 {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xffe00400 == 0x38000400 {
                                                    return Some(STRB_Rt_ADDR_SIMM9::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            } else {
                                                if insn & 0xffe00400 == 0x78000400 {
                                                    return Some(STRH_Rt_ADDR_SIMM9::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        } else {
                                            if insn & 0xbfe00400 == 0xb8000400 {
                                                return Some(STR_Rt_ADDR_SIMM9::make_opcode(insn));
                                            }
                                        }
                                    } else {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xffe00400 == 0x38400400 {
                                                    return Some(LDRB_Rt_ADDR_SIMM9::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            } else {
                                                if insn & 0xffe00400 == 0x78400400 {
                                                    return Some(LDRH_Rt_ADDR_SIMM9::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        } else {
                                            if insn & 0xbfe00400 == 0xb8400400 {
                                                return Some(LDR_Rt_ADDR_SIMM9::make_opcode(insn));
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0xffa00400 == 0xf8200400 {
                                        return Some(LDRAA_Rt_ADDR_SIMM10::make_opcode(insn));
                                    }
                                }
                            }
                        }
                    } else {
                        if insn & 0x400000 == 0 {
                            if insn & 0x10000000 == 0 {
                                if insn & 0x40000000 == 0 {
                                    if insn & 0x7fc00000 == 0x29000000 {
                                        return Some(STP_Rt_Rt2_ADDR_SIMM7::make_opcode(insn));
                                    }
                                } else {
                                    if insn & 0xffc00000 == 0x69000000 {
                                        return Some(STGP_Rt_Rt2_ADDR_SIMM11::make_opcode(insn));
                                    }
                                }
                            } else {
                                if insn & 0x20000000 == 0 {
                                    if insn & 0x000400 == 0 {
                                        if insn & 0x000800 == 0 {
                                            if insn & 0x200000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0x80000000 == 0 {
                                                        if insn & 0xffe00c00 == 0x19000000 {
                                                            return Some(
                                                                STLURB_Rt_ADDR_OFFSET::make_opcode(
                                                                    insn,
                                                                ),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xffe00c00 == 0x99000000 {
                                                            return Some(
                                                                STLUR_Rt_ADDR_OFFSET::make_opcode(
                                                                    insn,
                                                                ),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x80000000 == 0 {
                                                        if insn & 0xffe00c00 == 0x59000000 {
                                                            return Some(
                                                                STLURH_Rt_ADDR_OFFSET::make_opcode(
                                                                    insn,
                                                                ),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xffe00c00 == 0xd9000000 {
                                                            return Some(
                                                                STLUR_Rt_X_ADDR_OFFSET::make_opcode(
                                                                    insn,
                                                                ),
                                                            );
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0xfffffc00 == 0xd9200000 {
                                                    return Some(
                                                        STZGM_Rt_ADDR_SIMPLE::make_opcode(insn),
                                                    );
                                                }
                                            }
                                        } else {
                                            if insn & 0xffe00c00 == 0xd9200800 {
                                                return Some(STG_Rt_SP_ADDR_SIMM13::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    } else {
                                        if insn & 0xffe00400 == 0xd9200400 {
                                            return Some(
                                                STG_Rt_SP_X_ADDR_SIMM13_imm_tag::make_opcode(insn),
                                            );
                                        }
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
                            }
                        } else {
                            if insn & 0x10000000 == 0 {
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
                                if insn & 0x20000000 == 0 {
                                    if insn & 0x000400 == 0 {
                                        if insn & 0x000800 == 0 {
                                            if insn & 0x200000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0x80000000 == 0 {
                                                        if insn & 0xffe00c00 == 0x19400000 {
                                                            return Some(
                                                                LDAPURB_Rt_ADDR_OFFSET::make_opcode(
                                                                    insn,
                                                                ),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xffe00c00 == 0x99400000 {
                                                            return Some(
                                                                LDAPUR_Rt_ADDR_OFFSET::make_opcode(
                                                                    insn,
                                                                ),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x80000000 == 0 {
                                                        if insn & 0xffe00c00 == 0x59400000 {
                                                            return Some(
                                                                LDAPURH_Rt_ADDR_OFFSET::make_opcode(
                                                                    insn,
                                                                ),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xffe00c00 == 0xd9400000 {
                                                            return Some (LDAPUR_Rt_X_ADDR_OFFSET :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0xffe00c00 == 0xd9600000 {
                                                    return Some(LDG_Rt_ADDR_SIMM13::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        } else {
                                            if insn & 0xffe00c00 == 0xd9600800 {
                                                return Some(STZG_Rt_SP_ADDR_SIMM13::make_opcode(
                                                    insn,
                                                ));
                                            }
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
                        }
                    }
                } else {
                    if insn & 0x10000000 == 0 {
                        if insn & 0x400000 == 0 {
                            if insn & 0x20000000 == 0 {
                                if insn & 0x008000 == 0 {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xfffffc00 == 0x89f7c00 {
                                                return Some(STLLRB_Rt_ADDR_SIMPLE::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        } else {
                                            if insn & 0xfffffc00 == 0x489f7c00 {
                                                return Some(STLLRH_Rt_ADDR_SIMPLE::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    } else {
                                        if insn & 0xbffffc00 == 0x889f7c00 {
                                            return Some(STLLR_Rt_ADDR_SIMPLE::make_opcode(insn));
                                        }
                                    }
                                } else {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xfffffc00 == 0x89ffc00 {
                                                return Some(STLRB_Rt_ADDR_SIMPLE::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        } else {
                                            if insn & 0xfffffc00 == 0x489ffc00 {
                                                return Some(STLRH_Rt_ADDR_SIMPLE::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    } else {
                                        if insn & 0xbffffc00 == 0x889ffc00 {
                                            return Some(STLR_Rt_ADDR_SIMPLE::make_opcode(insn));
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x40000000 == 0 {
                                    if insn & 0x7ec00000 == 0x28800000 {
                                        return Some(STP_Rt_W_Rt2_W_ADDR_SIMM7_S_S::make_opcode(
                                            insn,
                                        ));
                                    }
                                } else {
                                    if insn & 0xfec00000 == 0x68800000 {
                                        return Some(
                                            STGP_Rt_X_Rt2_X_ADDR_SIMM11_imm_tag::make_opcode(insn),
                                        );
                                    }
                                }
                            }
                        } else {
                            if insn & 0x20000000 == 0 {
                                if insn & 0x008000 == 0 {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xfffffc00 == 0x8df7c00 {
                                                return Some(LDLARB_Rt_ADDR_SIMPLE::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        } else {
                                            if insn & 0xfffffc00 == 0x48df7c00 {
                                                return Some(LDLARH_Rt_ADDR_SIMPLE::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    } else {
                                        if insn & 0xbffffc00 == 0x88df7c00 {
                                            return Some(LDLAR_Rt_ADDR_SIMPLE::make_opcode(insn));
                                        }
                                    }
                                } else {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xfffffc00 == 0x8dffc00 {
                                                return Some(LDARB_Rt_ADDR_SIMPLE::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        } else {
                                            if insn & 0xfffffc00 == 0x48dffc00 {
                                                return Some(LDARH_Rt_ADDR_SIMPLE::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    } else {
                                        if insn & 0xbffffc00 == 0x88dffc00 {
                                            return Some(LDAR_Rt_ADDR_SIMPLE::make_opcode(insn));
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x40000000 == 0 {
                                    if insn & 0x7ec00000 == 0x28c00000 {
                                        return Some(LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S::make_opcode(
                                            insn,
                                        ));
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
                            if insn & 0x000400 == 0 {
                                if insn & 0x000800 == 0 {
                                    if insn & 0x200000 == 0 {
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
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0xffa00c00 == 0x78800000 {
                                                    return Some(
                                                        LDURSH_Rt_ADDR_SIMM9::make_opcode(insn),
                                                    );
                                                }
                                            } else {
                                                if insn & 0xffe00c00 == 0xf8800000 {
                                                    return Some(
                                                        PRFUM_PRFOP_ADDR_SIMM9::make_opcode(insn),
                                                    );
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xfffffc00 == 0x38bfc000 {
                                                    return Some(
                                                        LDAPRB_Rt_ADDR_SIMPLE::make_opcode(insn),
                                                    );
                                                }
                                            } else {
                                                if insn & 0xfffffc00 == 0x78bfc000 {
                                                    return Some(
                                                        LDAPRH_Rt_ADDR_SIMPLE::make_opcode(insn),
                                                    );
                                                }
                                            }
                                        } else {
                                            if insn & 0xbffffc00 == 0xb8bfc000 {
                                                return Some(LDAPR_Rt_ADDR_SIMPLE::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x200000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0xffa00c00 == 0x38800800 {
                                                    return Some(
                                                        LDTRSB_Rt_ADDR_SIMM9::make_opcode(insn),
                                                    );
                                                }
                                            } else {
                                                if insn & 0xffe00c00 == 0xb8800800 {
                                                    return Some(
                                                        LDTRSW_Rt_ADDR_SIMM9::make_opcode(insn),
                                                    );
                                                }
                                            }
                                        } else {
                                            if insn & 0xffa00c00 == 0x78800800 {
                                                return Some(LDTRSH_Rt_ADDR_SIMM9::make_opcode(
                                                    insn,
                                                ));
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
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0xffa00c00 == 0x78a00800 {
                                                    return Some(
                                                        LDRSH_Rt_ADDR_REGOFF::make_opcode(insn),
                                                    );
                                                }
                                            } else {
                                                if insn & 0xffe00c00 == 0xf8a00800 {
                                                    return Some(
                                                        PRFM_PRFOP_ADDR_REGOFF::make_opcode(insn),
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x200000 == 0 {
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
                                            return Some(LDRSH_Rt_ADDR_SIMM9::make_opcode(insn));
                                        }
                                    }
                                } else {
                                    if insn & 0xffa00400 == 0xf8a00400 {
                                        return Some(LDRAB_Rt_ADDR_SIMM10::make_opcode(insn));
                                    }
                                }
                            }
                        } else {
                            if insn & 0x20000000 == 0 {
                                if insn & 0x000400 == 0 {
                                    if insn & 0x000800 == 0 {
                                        if insn & 0x200000 == 0 {
                                            if insn & 0x400000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0x80000000 == 0 {
                                                        if insn & 0xffe00c00 == 0x19800000 {
                                                            return Some (LDAPURSB_Rt_ADDR_OFFSET :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0xffe00c00 == 0x99800000 {
                                                            return Some (LDAPURSW_Rt_ADDR_OFFSET :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x59800000 {
                                                        return Some(
                                                            LDAPURSH_Rt_ADDR_OFFSET::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x19c00000 {
                                                        return Some(
                                                            LDAPURSB_Rt_W_ADDR_OFFSET::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x59c00000 {
                                                        return Some(
                                                            LDAPURSH_Rt_W_ADDR_OFFSET::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x400000 == 0 {
                                                if insn & 0xfffffc00 == 0xd9a00000 {
                                                    return Some(STGM_Rt_ADDR_SIMPLE::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            } else {
                                                if insn & 0xfffffc00 == 0xd9e00000 {
                                                    return Some(LDGM_Rt_ADDR_SIMPLE::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x400000 == 0 {
                                            if insn & 0xffe00c00 == 0xd9a00800 {
                                                return Some(ST2G_Rt_SP_ADDR_SIMM13::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        } else {
                                            if insn & 0xffe00c00 == 0xd9e00800 {
                                                return Some(STZ2G_Rt_SP_ADDR_SIMM13::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x400000 == 0 {
                                        if insn & 0xffe00400 == 0xd9a00400 {
                                            return Some(
                                                ST2G_Rt_SP_X_ADDR_SIMM13_imm_tag::make_opcode(insn),
                                            );
                                        }
                                    } else {
                                        if insn & 0xffe00400 == 0xd9e00400 {
                                            return Some(
                                                STZ2G_Rt_SP_X_ADDR_SIMM13_imm_tag::make_opcode(
                                                    insn,
                                                ),
                                            );
                                        }
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
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0xff800000 == 0x79800000 {
                                            return Some(LDRSH_Rt_ADDR_UIMM12::make_opcode(insn));
                                        }
                                    } else {
                                        if insn & 0xffc00000 == 0xf9800000 {
                                            return Some(PRFM_PRFOP_ADDR_UIMM12::make_opcode(insn));
                                        }
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
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0x7fe0fc00 == 0x1a000000 {
                                                        return Some(ADC_Rd_Rn_Rm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                } else {
                                                    if insn & 0x7fe0fc00 == 0x5a000000 {
                                                        return Some(SBC_Rd_Rn_Rm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                }
                                            } else {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0x7fe0fc00 == 0x3a000000 {
                                                        return Some(ADCS_Rd_Rn_Rm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                } else {
                                                    if insn & 0x7fe0fc00 == 0x7a000000 {
                                                        return Some(SBCS_Rd_Rn_Rm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
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
                                                    if insn & 0x004000 == 0 {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xffe0fc00 == 0x9ac00000 {
                                                                    return Some (SUBP_Rd_Rn_SP_Rm_SP :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0x010000 == 0 {
                                                                    if insn & 0x7ffffc00
                                                                        == 0x5ac00000
                                                                    {
                                                                        return Some(
                                                                            RBIT_Rd_Rn::make_opcode(
                                                                                insn,
                                                                            ),
                                                                        );
                                                                    }
                                                                } else {
                                                                    if insn & 0xfffffc00
                                                                        == 0xdac10000
                                                                    {
                                                                        return Some (PACIA_Rd_Rn_SP :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xffe0fc00 == 0xbac00000 {
                                                                return Some (SUBPS_Rd_Rn_SP_Rm_SP :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x40000000 == 0 {
                                                            if insn & 0xffe0fc00 == 0x1ac04000 {
                                                                return Some(
                                                                    CRC32B_Rd_Rn_Rm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xffffffe0 == 0xdac143e0 {
                                                                return Some(
                                                                    XPACI_Rd::make_opcode(insn),
                                                                );
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x40000000 == 0 {
                                                        if insn & 0x7fe0fc00 == 0x1ac02000 {
                                                            return Some(
                                                                LSLV_Rd_Rn_Rm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xffffffe0 == 0xdac123e0 {
                                                            return Some(PACIZA_Rd::make_opcode(
                                                                insn,
                                                            ));
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x002000 == 0 {
                                                    if insn & 0x004000 == 0 {
                                                        if insn & 0x40000000 == 0 {
                                                            if insn & 0xffe0fc00 == 0x9ac01000 {
                                                                return Some(
                                                                    IRG_Rd_SP_Rn_SP_Rm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0x010000 == 0 {
                                                                if insn & 0x7ffffc00 == 0x5ac01000 {
                                                                    return Some(
                                                                        CLZ_Rd_Rn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            } else {
                                                                if insn & 0xfffffc00 == 0xdac11000 {
                                                                    return Some(
                                                                        AUTIA_Rd_Rn_SP::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0xffe0fc00 == 0x1ac05000 {
                                                            return Some(
                                                                CRC32CB_Rd_Rn_Rm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x40000000 == 0 {
                                                        if insn & 0xffe0fc00 == 0x9ac03000 {
                                                            return Some(
                                                                PACGA_Rd_Rn_Rm_SP::make_opcode(
                                                                    insn,
                                                                ),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xffffffe0 == 0xdac133e0 {
                                                            return Some(AUTIZA_Rd::make_opcode(
                                                                insn,
                                                            ));
                                                        }
                                                    }
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
                                            if insn & 0x002000 == 0 {
                                                if insn & 0x004000 == 0 {
                                                    if insn & 0x40000000 == 0 {
                                                        if insn & 0x7fe0fc00 == 0x1ac00800 {
                                                            return Some(
                                                                UDIV_Rd_Rn_Rm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0x010000 == 0 {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0xfffffc00 == 0x5ac00800 {
                                                                    return Some(
                                                                        REV_Rd_Rn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            } else {
                                                                if insn & 0xfffffc00 == 0xdac00800 {
                                                                    return Some(
                                                                        REV32_Rd_Rn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xfffffc00 == 0xdac10800 {
                                                                return Some(
                                                                    PACDA_Rd_Rn_SP::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0x1ac04800 {
                                                        return Some(CRC32W_Rd_Rn_Rm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                }
                                            } else {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0x7fe0fc00 == 0x1ac02800 {
                                                        return Some(ASRV_Rd_Rn_Rm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                } else {
                                                    if insn & 0xffffffe0 == 0xdac12be0 {
                                                        return Some(PACDZA_Rd::make_opcode(insn));
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x002000 == 0 {
                                                if insn & 0x004000 == 0 {
                                                    if insn & 0xfffffc00 == 0xdac11800 {
                                                        return Some(AUTDA_Rd_Rn_SP::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0x1ac05800 {
                                                        return Some(
                                                            CRC32CW_Rd_Rn_Rm::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0xffffffe0 == 0xdac13be0 {
                                                    return Some(AUTDZA_Rd::make_opcode(insn));
                                                }
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
                                            if insn & 0x002000 == 0 {
                                                if insn & 0x004000 == 0 {
                                                    if insn & 0x010000 == 0 {
                                                        if insn & 0x7ffffc00 == 0x5ac00400 {
                                                            return Some(REV16_Rd_Rn::make_opcode(
                                                                insn,
                                                            ));
                                                        }
                                                    } else {
                                                        if insn & 0xfffffc00 == 0xdac10400 {
                                                            return Some(
                                                                PACIB_Rd_Rn_SP::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x40000000 == 0 {
                                                        if insn & 0xffe0fc00 == 0x1ac04400 {
                                                            return Some(
                                                                CRC32H_Rd_Rn_Rm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xffffffe0 == 0xdac147e0 {
                                                            return Some(XPACD_Rd::make_opcode(
                                                                insn,
                                                            ));
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0x7fe0fc00 == 0x1ac02400 {
                                                        return Some(LSRV_Rd_Rn_Rm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                } else {
                                                    if insn & 0xffffffe0 == 0xdac127e0 {
                                                        return Some(PACIZB_Rd::make_opcode(insn));
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x002000 == 0 {
                                                if insn & 0x004000 == 0 {
                                                    if insn & 0x40000000 == 0 {
                                                        if insn & 0xffe0fc00 == 0x9ac01400 {
                                                            return Some(
                                                                GMI_Rd_Rn_SP_Rm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0x010000 == 0 {
                                                            if insn & 0x7ffffc00 == 0x5ac01400 {
                                                                return Some(
                                                                    CLS_Rd_Rn::make_opcode(insn),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xfffffc00 == 0xdac11400 {
                                                                return Some(
                                                                    AUTIB_Rd_Rn_SP::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0x1ac05400 {
                                                        return Some(
                                                            CRC32CH_Rd_Rn_Rm::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0xffffffe0 == 0xdac137e0 {
                                                    return Some(AUTIZB_Rd::make_opcode(insn));
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x001000 == 0 {
                                        if insn & 0x002000 == 0 {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0x7fe0fc00 == 0x1ac00c00 {
                                                        return Some(SDIV_Rd_Rn_Rm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                } else {
                                                    if insn & 0x010000 == 0 {
                                                        if insn & 0xfffffc00 == 0xdac00c00 {
                                                            return Some(
                                                                REV_Rd_X_Rn_X::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xfffffc00 == 0xdac10c00 {
                                                            return Some(
                                                                PACDB_Rd_Rn_SP::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0xffe0fc00 == 0x9ac04c00 {
                                                    return Some(CRC32X_Rd_Rn_Rm::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        } else {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0x7fe0fc00 == 0x1ac02c00 {
                                                    return Some(RORV_Rd_Rn_Rm::make_opcode(insn));
                                                }
                                            } else {
                                                if insn & 0xffffffe0 == 0xdac12fe0 {
                                                    return Some(PACDZB_Rd::make_opcode(insn));
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x002000 == 0 {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0xfffffc00 == 0xdac11c00 {
                                                    return Some(AUTDB_Rd_Rn_SP::make_opcode(insn));
                                                }
                                            } else {
                                                if insn & 0xffe0fc00 == 0x9ac05c00 {
                                                    return Some(CRC32CX_Rd_Rn_Rm::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        } else {
                                            if insn & 0xffffffe0 == 0xdac13fe0 {
                                                return Some(AUTDZB_Rd::make_opcode(insn));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        if insn & 0x20000000 == 0 {
                            if insn & 0x40000000 == 0 {
                                if insn & 0x7f200000 == 0xa200000 {
                                    return Some(BIC_Rd_Rn_Rm_SFT::make_opcode(insn));
                                }
                            } else {
                                if insn & 0x7f200000 == 0x4a200000 {
                                    return Some(EON_Rd_Rn_Rm_SFT::make_opcode(insn));
                                }
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
                                    if insn & 0x800000 == 0 {
                                        if insn & 0xffe0fc00 == 0x9b407c00 {
                                            return Some(SMULH_Rd_Rn_Rm::make_opcode(insn));
                                        }
                                    } else {
                                        if insn & 0xffe0fc00 == 0x9bc07c00 {
                                            return Some(UMULH_Rd_Rn_Rm::make_opcode(insn));
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x7fe08000 == 0x1b008000 {
                                    return Some(MSUB_Rd_Rn_Rm_Ra::make_opcode(insn));
                                }
                            }
                        }
                    } else {
                        if insn & 0x800000 == 0 {
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
                                if insn & 0x008000 == 0 {
                                    if insn & 0xffe08000 == 0x9b200000 {
                                        return Some(SMADDL_Rd_Rn_Rm_Ra::make_opcode(insn));
                                    }
                                } else {
                                    if insn & 0xffe08000 == 0x9b208000 {
                                        return Some(SMSUBL_Rd_Rn_Rm_Ra::make_opcode(insn));
                                    }
                                }
                            }
                        } else {
                            if insn & 0x008000 == 0 {
                                if insn & 0xffe08000 == 0x9ba00000 {
                                    return Some(UMADDL_Rd_Rn_Rm_Ra::make_opcode(insn));
                                }
                            } else {
                                if insn & 0xffe08000 == 0x9ba08000 {
                                    return Some(UMSUBL_Rd_Rn_Rm_Ra::make_opcode(insn));
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        if insn & 0x8000000 == 0 {
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
                    if insn & 0x1000000 == 0 {
                        if insn & 0x000010 == 0 {
                            if insn & 0x2000000 == 0 {
                                if insn & 0xff000010 == 0x54000000 {
                                    return Some(B__ADDR_PCREL19::make_opcode(insn));
                                }
                            } else {
                                if insn & 0x200000 == 0 {
                                    if insn & 0x400000 == 0 {
                                        if insn & 0x800000 == 0 {
                                            if insn & 0xfffffc1f == 0xd61f0000 {
                                                return Some(BR_Rn::make_opcode(insn));
                                            }
                                        } else {
                                            if insn == 0xd69f03e0 {
                                                return Some(ERET::make_opcode(insn));
                                            }
                                        }
                                    } else {
                                        if insn & 0xfffffc1f == 0xd65f0000 {
                                            return Some(RET_Rn::make_opcode(insn));
                                        }
                                    }
                                } else {
                                    if insn & 0x800000 == 0 {
                                        if insn & 0xfffffc1f == 0xd63f0000 {
                                            return Some(BLR_Rn::make_opcode(insn));
                                        }
                                    } else {
                                        if insn == 0xd6bf03e0 {
                                            return Some(DRPS::make_opcode(insn));
                                        }
                                    }
                                }
                            }
                        } else {
                            if insn & 0x2000000 == 0 {
                                if insn & 0xff000010 == 0x54000010 {
                                    return Some(BC__ADDR_PCREL19::make_opcode(insn));
                                }
                            } else {
                                if insn & 0x000400 == 0 {
                                    if insn & 0x200000 == 0 {
                                        if insn & 0x400000 == 0 {
                                            if insn & 0x800000 == 0 {
                                                if insn & 0xfffffc1f == 0xd61f081f {
                                                    return Some(BRAAZ_Rn::make_opcode(insn));
                                                }
                                            } else {
                                                if insn == 0xd69f0bff {
                                                    return Some(ERETAA::make_opcode(insn));
                                                }
                                            }
                                        } else {
                                            if insn == 0xd65f0bff {
                                                return Some(RETAA::make_opcode(insn));
                                            }
                                        }
                                    } else {
                                        if insn & 0xfffffc1f == 0xd63f081f {
                                            return Some(BLRAAZ_Rn::make_opcode(insn));
                                        }
                                    }
                                } else {
                                    if insn & 0x200000 == 0 {
                                        if insn & 0x400000 == 0 {
                                            if insn & 0x800000 == 0 {
                                                if insn & 0xfffffc1f == 0xd61f0c1f {
                                                    return Some(BRABZ_Rn::make_opcode(insn));
                                                }
                                            } else {
                                                if insn == 0xd69f0fff {
                                                    return Some(ERETAB::make_opcode(insn));
                                                }
                                            }
                                        } else {
                                            if insn == 0xd65f0fff {
                                                return Some(RETAB::make_opcode(insn));
                                            }
                                        }
                                    } else {
                                        if insn & 0xfffffc1f == 0xd63f0c1f {
                                            return Some(BLRABZ_Rn::make_opcode(insn));
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        if insn & 0x000400 == 0 {
                            if insn & 0x200000 == 0 {
                                if insn & 0xfffffc00 == 0xd71f0800 {
                                    return Some(BRAA_Rn_Rd_SP::make_opcode(insn));
                                }
                            } else {
                                if insn & 0xfffffc00 == 0xd73f0800 {
                                    return Some(BLRAA_Rn_Rd_SP::make_opcode(insn));
                                }
                            }
                        } else {
                            if insn & 0x200000 == 0 {
                                if insn & 0xfffffc00 == 0xd71f0c00 {
                                    return Some(BRAB_Rn_Rd_SP::make_opcode(insn));
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
        } else {
            if insn & 0x2000000 == 0 {
                if insn & 0x400000 == 0 {
                    if insn & 0x10000000 == 0 {
                        if insn & 0x800000 == 0 {
                            if insn & 0x1000000 == 0 {
                                if insn & 0x20000000 == 0 {
                                    if insn & 0xbfff0000 == 0xc000000 {
                                        return Some(ST4_LVt_SIMD_ADDR_SIMPLE::make_opcode(insn));
                                    }
                                    if insn & 0xbfff0000 == 0xc000000 {
                                        return Some(ST4_LVt_SIMD_ADDR_SIMPLE::make_opcode(insn));
                                    }
                                    if insn & 0xbfff0000 == 0xc000000 {
                                        return Some(ST4_LVt_SIMD_ADDR_SIMPLE::make_opcode(insn));
                                    }
                                    if insn & 0xbfff0000 == 0xc000000 {
                                        return Some(ST4_LVt_SIMD_ADDR_SIMPLE::make_opcode(insn));
                                    }
                                } else {
                                    if insn & 0x3fc00000 == 0x2c000000 {
                                        return Some(STNP_Ft_Ft2_ADDR_SIMM7::make_opcode(insn));
                                    }
                                }
                            } else {
                                if insn & 0x20000000 == 0 {
                                    if insn & 0x002000 == 0 {
                                        if insn & 0x200000 == 0 {
                                            if insn & 0xbfff2000 == 0xd000000 {
                                                return Some(
                                                    ST1_LEt_SIMD_ADDR_SIMPLE::make_opcode(insn),
                                                );
                                            }
                                        } else {
                                            if insn & 0xbfff2000 == 0xd200000 {
                                                return Some(
                                                    ST2_LEt_SIMD_ADDR_SIMPLE::make_opcode(insn),
                                                );
                                            }
                                        }
                                    } else {
                                        if insn & 0x200000 == 0 {
                                            if insn & 0xbfff2000 == 0xd002000 {
                                                return Some(
                                                    ST3_LEt_SIMD_ADDR_SIMPLE::make_opcode(insn),
                                                );
                                            }
                                        } else {
                                            if insn & 0xbfff2000 == 0xd202000 {
                                                return Some(
                                                    ST4_LEt_SIMD_ADDR_SIMPLE::make_opcode(insn),
                                                );
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x3fc00000 == 0x2d000000 {
                                        return Some(STP_Ft_Ft2_ADDR_SIMM7::make_opcode(insn));
                                    }
                                }
                            }
                        } else {
                            if insn & 0x20000000 == 0 {
                                if insn & 0x200000 == 0 {
                                    if insn & 0x1000000 == 0 {
                                        if insn & 0xbfe00000 == 0xc800000 {
                                            return Some(ST4_LVt_SIMD_ADDR_POST::make_opcode(insn));
                                        }
                                        if insn & 0xbfe00000 == 0xc800000 {
                                            return Some(ST4_LVt_SIMD_ADDR_POST::make_opcode(insn));
                                        }
                                        if insn & 0xbfe00000 == 0xc800000 {
                                            return Some(ST4_LVt_SIMD_ADDR_POST::make_opcode(insn));
                                        }
                                        if insn & 0xbfe00000 == 0xc800000 {
                                            return Some(ST4_LVt_SIMD_ADDR_POST::make_opcode(insn));
                                        }
                                    } else {
                                        if insn & 0x002000 == 0 {
                                            if insn & 0xbfe02000 == 0xd800000 {
                                                return Some(ST1_LEt_SIMD_ADDR_POST::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        } else {
                                            if insn & 0xbfe02000 == 0xd802000 {
                                                return Some(ST3_LEt_SIMD_ADDR_POST::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x002000 == 0 {
                                        if insn & 0xbfe02000 == 0xda00000 {
                                            return Some(ST2_LEt_SIMD_ADDR_POST::make_opcode(insn));
                                        }
                                    } else {
                                        if insn & 0xbfe02000 == 0xda02000 {
                                            return Some(ST4_LEt_SIMD_ADDR_POST::make_opcode(insn));
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x3ec00000 == 0x2c800000 {
                                    return Some(STP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S::make_opcode(
                                        insn,
                                    ));
                                }
                            }
                        }
                    } else {
                        if insn & 0x1000000 == 0 {
                            if insn & 0x000400 == 0 {
                                if insn & 0x000800 == 0 {
                                    if insn & 0x3f600c00 == 0x3c000000 {
                                        return Some(STUR_Ft_ADDR_SIMM9::make_opcode(insn));
                                    }
                                } else {
                                    if insn & 0x3f600c00 == 0x3c200800 {
                                        return Some(STR_Ft_ADDR_REGOFF::make_opcode(insn));
                                    }
                                }
                            } else {
                                if insn & 0x3f600400 == 0x3c000400 {
                                    return Some(STR_Ft_ADDR_SIMM9::make_opcode(insn));
                                }
                            }
                        } else {
                            if insn & 0x3f400000 == 0x3d000000 {
                                return Some(STR_Ft_ADDR_UIMM12::make_opcode(insn));
                            }
                        }
                    }
                } else {
                    if insn & 0x10000000 == 0 {
                        if insn & 0x800000 == 0 {
                            if insn & 0x1000000 == 0 {
                                if insn & 0x20000000 == 0 {
                                    if insn & 0xbfff0000 == 0xc400000 {
                                        return Some(LD4_LVt_SIMD_ADDR_SIMPLE::make_opcode(insn));
                                    }
                                    if insn & 0xbfff0000 == 0xc400000 {
                                        return Some(LD4_LVt_SIMD_ADDR_SIMPLE::make_opcode(insn));
                                    }
                                    if insn & 0xbfff0000 == 0xc400000 {
                                        return Some(LD4_LVt_SIMD_ADDR_SIMPLE::make_opcode(insn));
                                    }
                                    if insn & 0xbfff0000 == 0xc400000 {
                                        return Some(LD4_LVt_SIMD_ADDR_SIMPLE::make_opcode(insn));
                                    }
                                } else {
                                    if insn & 0x3fc00000 == 0x2c400000 {
                                        return Some(LDNP_Ft_Ft2_ADDR_SIMM7::make_opcode(insn));
                                    }
                                }
                            } else {
                                if insn & 0x20000000 == 0 {
                                    if insn & 0x002000 == 0 {
                                        if insn & 0x200000 == 0 {
                                            if insn & 0xbffff000 == 0xd40c000 {
                                                return Some(
                                                    LD1R_LVt_AL_SIMD_ADDR_SIMPLE::make_opcode(insn),
                                                );
                                            }
                                            if insn & 0xbfff2000 == 0xd400000 {
                                                return Some(
                                                    LD1_LEt_SIMD_ADDR_SIMPLE::make_opcode(insn),
                                                );
                                            }
                                        } else {
                                            if insn & 0xbffff000 == 0xd60c000 {
                                                return Some(
                                                    LD2R_LVt_AL_SIMD_ADDR_SIMPLE::make_opcode(insn),
                                                );
                                            }
                                            if insn & 0xbfff2000 == 0xd600000 {
                                                return Some(
                                                    LD2_LEt_SIMD_ADDR_SIMPLE::make_opcode(insn),
                                                );
                                            }
                                        }
                                    } else {
                                        if insn & 0x200000 == 0 {
                                            if insn & 0xbffff000 == 0xd40e000 {
                                                return Some(
                                                    LD3R_LVt_AL_SIMD_ADDR_SIMPLE::make_opcode(insn),
                                                );
                                            }
                                            if insn & 0xbfff2000 == 0xd402000 {
                                                return Some(
                                                    LD3_LEt_SIMD_ADDR_SIMPLE::make_opcode(insn),
                                                );
                                            }
                                        } else {
                                            if insn & 0xbffff000 == 0xd60e000 {
                                                return Some(
                                                    LD4R_LVt_AL_SIMD_ADDR_SIMPLE::make_opcode(insn),
                                                );
                                            }
                                            if insn & 0xbfff2000 == 0xd602000 {
                                                return Some(
                                                    LD4_LEt_SIMD_ADDR_SIMPLE::make_opcode(insn),
                                                );
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x3fc00000 == 0x2d400000 {
                                        return Some(LDP_Ft_Ft2_ADDR_SIMM7::make_opcode(insn));
                                    }
                                }
                            }
                        } else {
                            if insn & 0x20000000 == 0 {
                                if insn & 0x200000 == 0 {
                                    if insn & 0x1000000 == 0 {
                                        if insn & 0xbfe00000 == 0xcc00000 {
                                            return Some(LD4_LVt_SIMD_ADDR_POST::make_opcode(insn));
                                        }
                                        if insn & 0xbfe00000 == 0xcc00000 {
                                            return Some(LD4_LVt_SIMD_ADDR_POST::make_opcode(insn));
                                        }
                                        if insn & 0xbfe00000 == 0xcc00000 {
                                            return Some(LD4_LVt_SIMD_ADDR_POST::make_opcode(insn));
                                        }
                                        if insn & 0xbfe00000 == 0xcc00000 {
                                            return Some(LD4_LVt_SIMD_ADDR_POST::make_opcode(insn));
                                        }
                                    } else {
                                        if insn & 0x002000 == 0 {
                                            if insn & 0xbfe0f000 == 0xdc0c000 {
                                                return Some(
                                                    LD1R_LVt_AL_SIMD_ADDR_POST::make_opcode(insn),
                                                );
                                            }
                                            if insn & 0xbfe02000 == 0xdc00000 {
                                                return Some(LD1_LEt_SIMD_ADDR_POST::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        } else {
                                            if insn & 0xbfe0f000 == 0xdc0e000 {
                                                return Some(
                                                    LD3R_LVt_AL_SIMD_ADDR_POST::make_opcode(insn),
                                                );
                                            }
                                            if insn & 0xbfe02000 == 0xdc02000 {
                                                return Some(LD3_LEt_SIMD_ADDR_POST::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x002000 == 0 {
                                        if insn & 0xbfe0f000 == 0xde0c000 {
                                            return Some(LD2R_LVt_AL_SIMD_ADDR_POST::make_opcode(
                                                insn,
                                            ));
                                        }
                                        if insn & 0xbfe02000 == 0xde00000 {
                                            return Some(LD2_LEt_SIMD_ADDR_POST::make_opcode(insn));
                                        }
                                    } else {
                                        if insn & 0xbfe0f000 == 0xde0e000 {
                                            return Some(LD4R_LVt_AL_SIMD_ADDR_POST::make_opcode(
                                                insn,
                                            ));
                                        }
                                        if insn & 0xbfe02000 == 0xde02000 {
                                            return Some(LD4_LEt_SIMD_ADDR_POST::make_opcode(insn));
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x3ec00000 == 0x2cc00000 {
                                    return Some(LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S::make_opcode(
                                        insn,
                                    ));
                                }
                            }
                        }
                    } else {
                        if insn & 0x1000000 == 0 {
                            if insn & 0x000400 == 0 {
                                if insn & 0x000800 == 0 {
                                    if insn & 0x3f600c00 == 0x3c400000 {
                                        return Some(LDUR_Ft_ADDR_SIMM9::make_opcode(insn));
                                    }
                                } else {
                                    if insn & 0x3f600c00 == 0x3c600800 {
                                        return Some(LDR_Ft_ADDR_REGOFF::make_opcode(insn));
                                    }
                                }
                            } else {
                                if insn & 0x3f600400 == 0x3c400400 {
                                    return Some(LDR_Ft_ADDR_SIMM9::make_opcode(insn));
                                }
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
                            if insn & 0x000400 == 0 {
                                if insn & 0x20000000 == 0 {
                                    if insn & 0x000800 == 0 {
                                        if insn & 0x001000 == 0 {
                                            if insn & 0xbfe09c00 == 0xe000000 {
                                                return Some(TBL_Vd_LVn_Vm::make_opcode(insn));
                                            }
                                        } else {
                                            if insn & 0xbfe09c00 == 0xe001000 {
                                                return Some(TBX_Vd_LVn_Vm::make_opcode(insn));
                                            }
                                        }
                                    } else {
                                        if insn & 0x001000 == 0 {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0xbf20fc00 == 0xe002800 {
                                                    return Some(TRN1_Vd_Vn_Vm::make_opcode(insn));
                                                }
                                            } else {
                                                if insn & 0xbf20fc00 == 0xe006800 {
                                                    return Some(TRN2_Vd_Vn_Vm::make_opcode(insn));
                                                }
                                            }
                                        } else {
                                            if insn & 0x002000 == 0 {
                                                if insn & 0x004000 == 0 {
                                                    if insn & 0xbf20fc00 == 0xe001800 {
                                                        return Some(UZP1_Vd_Vn_Vm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                } else {
                                                    if insn & 0xbf20fc00 == 0xe005800 {
                                                        return Some(UZP2_Vd_Vn_Vm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                }
                                            } else {
                                                if insn & 0x004000 == 0 {
                                                    if insn & 0xbf20fc00 == 0xe003800 {
                                                        return Some(ZIP1_Vd_Vn_Vm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                } else {
                                                    if insn & 0xbf20fc00 == 0xe007800 {
                                                        return Some(ZIP2_Vd_Vn_Vm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0xbfe08400 == 0x2e000000 {
                                        return Some(EXT_Vd_Vn_Vm_IDX::make_opcode(insn));
                                    }
                                }
                            } else {
                                if insn & 0x008000 == 0 {
                                    if insn & 0x400000 == 0 {
                                        if insn & 0x20000000 == 0 {
                                            if insn & 0x000800 == 0 {
                                                if insn & 0xbfe0fc00 == 0xe000400 {
                                                    return Some(DUP_Vd_En::make_opcode(insn));
                                                }
                                            } else {
                                                if insn & 0x001000 == 0 {
                                                    if insn & 0x002000 == 0 {
                                                        if insn & 0xbfe0fc00 == 0xe000c00 {
                                                            return Some(DUP_Vd_Rn::make_opcode(
                                                                insn,
                                                            ));
                                                        }
                                                    } else {
                                                        if insn & 0xbfe0fc00 == 0xe002c00 {
                                                            return Some(SMOV_Rd_En::make_opcode(
                                                                insn,
                                                            ));
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x002000 == 0 {
                                                        if insn & 0xffe0fc00 == 0x4e001c00 {
                                                            return Some(INS_Ed_Rn::make_opcode(
                                                                insn,
                                                            ));
                                                        }
                                                    } else {
                                                        if insn & 0xbfe0fc00 == 0xe003c00 {
                                                            return Some(UMOV_Rd_En::make_opcode(
                                                                insn,
                                                            ));
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0xffe08400 == 0x6e000400 {
                                                return Some(INS_Ed_En::make_opcode(insn));
                                            }
                                        }
                                    } else {
                                        if insn & 0x000800 == 0 {
                                            if insn & 0x001000 == 0 {
                                                if insn & 0x002000 == 0 {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0xbfe0fc00 == 0xe400400 {
                                                                return Some(
                                                                    FMAXNM_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xbfe0fc00 == 0x2e400400 {
                                                                return Some(
                                                                    FMAXNMP_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0xbfe0fc00 == 0xec00400 {
                                                                return Some(
                                                                    FMINNM_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xbfe0fc00 == 0x2ec00400 {
                                                                return Some(
                                                                    FMINNMP_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0xbfe0fc00 == 0xe402400 {
                                                                return Some(
                                                                    FCMEQ_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xbfe0fc00 == 0x2e402400 {
                                                                return Some(
                                                                    FCMGE_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0xbfe0fc00 == 0x2ec02400 {
                                                            return Some(
                                                                FCMGT_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x002000 == 0 {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0xbfe0fc00 == 0xe401400 {
                                                                return Some(
                                                                    FADD_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xbfe0fc00 == 0x2e401400 {
                                                                return Some(
                                                                    FADDP_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0xbfe0fc00 == 0xec01400 {
                                                                return Some(
                                                                    FSUB_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xbfe0fc00 == 0x2ec01400 {
                                                                return Some(
                                                                    FABD_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0xbfe0fc00 == 0xe403400 {
                                                                return Some(
                                                                    FMAX_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xbfe0fc00 == 0x2e403400 {
                                                                return Some(
                                                                    FMAXP_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0xbfe0fc00 == 0xec03400 {
                                                                return Some(
                                                                    FMIN_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xbfe0fc00 == 0x2ec03400 {
                                                                return Some(
                                                                    FMINP_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x001000 == 0 {
                                                if insn & 0x002000 == 0 {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0xbfe0fc00 == 0xe400c00 {
                                                            return Some(
                                                                FMLA_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xbfe0fc00 == 0xec00c00 {
                                                            return Some(
                                                                FMLS_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0xbfe0fc00 == 0x2e402c00 {
                                                            return Some(
                                                                FACGE_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xbfe0fc00 == 0x2ec02c00 {
                                                            return Some(
                                                                FACGT_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x002000 == 0 {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0xbfe0fc00 == 0xe401c00 {
                                                            return Some(
                                                                FMULX_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xbfe0fc00 == 0x2e401c00 {
                                                            return Some(
                                                                FMUL_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0xbfe0fc00 == 0xe403c00 {
                                                                return Some(
                                                                    FRECPS_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xbfe0fc00 == 0x2e403c00 {
                                                                return Some(
                                                                    FDIV_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0xbfe0fc00 == 0xec03c00 {
                                                            return Some(
                                                                FRSQRTS_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x002000 == 0 {
                                        if insn & 0x004000 == 0 {
                                            if insn & 0x000800 == 0 {
                                                if insn & 0xbf20fc00 == 0x2e008400 {
                                                    return Some(SQRDMLAH_Vd_Vn_Vm::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            } else {
                                                if insn & 0xbf20fc00 == 0x2e008c00 {
                                                    return Some(SQRDMLSH_Vd_Vn_Vm::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        } else {
                                            if insn & 0xbf20e400 == 0x2e00c400 {
                                                return Some(FCMLA_Vd_Vn_Vm_IMM_ROT1::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    } else {
                                        if insn & 0xbf20ec00 == 0x2e00e400 {
                                            return Some(FCADD_Vd_Vn_Vm_IMM_ROT3::make_opcode(
                                                insn,
                                            ));
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
                                        if insn & 0x002000 == 0 {
                                            if insn & 0xffe0fc00 == 0x5e000400 {
                                                return Some(DUP_Sd_En::make_opcode(insn));
                                            }
                                        } else {
                                            if insn & 0xffe0fc00 == 0x5e402400 {
                                                return Some(FCMEQ_Sd_Sn_Sm::make_opcode(insn));
                                            }
                                        }
                                    } else {
                                        if insn & 0x002000 == 0 {
                                            if insn & 0xffe0fc00 == 0x5e401c00 {
                                                return Some(FMULX_Sd_Sn_Sm::make_opcode(insn));
                                            }
                                        } else {
                                            if insn & 0x800000 == 0 {
                                                if insn & 0xffe0fc00 == 0x5e403c00 {
                                                    return Some(FRECPS_Sd_Sn_Sm::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            } else {
                                                if insn & 0xffe0fc00 == 0x5ec03c00 {
                                                    return Some(FRSQRTS_Sd_Sn_Sm::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x000800 == 0 {
                                    if insn & 0x001000 == 0 {
                                        if insn & 0x002000 == 0 {
                                            if insn & 0xff20fc00 == 0x7e008400 {
                                                return Some(SQRDMLAH_Sd_Sn_Sm::make_opcode(insn));
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
                                    } else {
                                        if insn & 0xffe0fc00 == 0x7ec01400 {
                                            return Some(FABD_Sd_Sn_Sm::make_opcode(insn));
                                        }
                                    }
                                } else {
                                    if insn & 0x002000 == 0 {
                                        if insn & 0xff20fc00 == 0x7e008c00 {
                                            return Some(SQRDMLSH_Sd_Sn_Sm::make_opcode(insn));
                                        }
                                    } else {
                                        if insn & 0x800000 == 0 {
                                            if insn & 0xffe0fc00 == 0x7e402c00 {
                                                return Some(FACGE_Sd_Sn_Sm::make_opcode(insn));
                                            }
                                        } else {
                                            if insn & 0xffe0fc00 == 0x7ec02c00 {
                                                return Some(FACGT_Sd_Sn_Sm::make_opcode(insn));
                                            }
                                        }
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
                                            if insn & 0x008000 == 0 {
                                                if insn & 0x10000000 == 0 {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0x40000000 == 0 {
                                                            if insn & 0xff20fc00 == 0xe200000 {
                                                                return Some(
                                                                    SADDL_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xff20fc00 == 0x4e200000 {
                                                                return Some(
                                                                    SADDL2_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x40000000 == 0 {
                                                            if insn & 0xff20fc00 == 0x2e200000 {
                                                                return Some(
                                                                    UADDL_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xff20fc00 == 0x6e200000 {
                                                                return Some(
                                                                    UADDL2_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
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
                                                                        FCVTAS_Rd_Fn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                                if insn & 0x7f3ffc00 == 0x1e240000 {
                                                                    return Some (FCVTAS_Rd_W_Fn_S_D :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x040000 == 0 {
                                                                if insn & 0x7ffffc00 == 0x1ee20000 {
                                                                    return Some(
                                                                        SCVTF_Fd_Rn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                                if insn & 0x7f3ffc00 == 0x1e220000 {
                                                                    return Some (SCVTF_Fd_S_D_Rn_W :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0x080000 == 0 {
                                                                    if insn & 0x7ffffc00
                                                                        == 0x1ee60000
                                                                    {
                                                                        return Some(
                                                                            FMOV_Rd_Fn::make_opcode(
                                                                                insn,
                                                                            ),
                                                                        );
                                                                    }
                                                                    if insn & 0x7f3ffc00
                                                                        == 0x1e260000
                                                                    {
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
                                                                        FCVTAU_Rd_Fn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                                if insn & 0x7f3ffc00 == 0x1e250000 {
                                                                    return Some (FCVTAU_Rd_W_Fn_S_D :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x040000 == 0 {
                                                                if insn & 0x7ffffc00 == 0x1ee30000 {
                                                                    return Some(
                                                                        UCVTF_Fd_Rn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                                if insn & 0x7f3ffc00 == 0x1e230000 {
                                                                    return Some (UCVTF_Fd_S_D_Rn_W :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0x080000 == 0 {
                                                                    if insn & 0x7ffffc00
                                                                        == 0x1ee70000
                                                                    {
                                                                        return Some(
                                                                            FMOV_Fd_Rn::make_opcode(
                                                                                insn,
                                                                            ),
                                                                        );
                                                                    }
                                                                    if insn & 0x7f3ffc00
                                                                        == 0x1e270000
                                                                    {
                                                                        return Some (FMOV_Fd_S_S_Rn_W :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xfffffc00
                                                                        == 0x9eaf0000
                                                                    {
                                                                        return Some (FMOV_VdD1_Rn :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x20000000 == 0 {
                                                    if insn & 0x40000000 == 0 {
                                                        if insn & 0xff20fc00 == 0xe208000 {
                                                            return Some(
                                                                SMLAL_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xff20fc00 == 0x4e208000 {
                                                            return Some(
                                                                SMLAL2_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x40000000 == 0 {
                                                        if insn & 0xff20fc00 == 0x2e208000 {
                                                            return Some(
                                                                UMLAL_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xff20fc00 == 0x6e208000 {
                                                            return Some(
                                                                UMLAL2_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0x20000000 == 0 {
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
                                                        if insn & 0xff20fc00 == 0x2e204000 {
                                                            return Some(
                                                                RADDHN_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xff20fc00 == 0x6e204000 {
                                                            return Some(
                                                                RADDHN2_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x20000000 == 0 {
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
                                                } else {
                                                    if insn & 0x40000000 == 0 {
                                                        if insn & 0xff20fc00 == 0x2e20c000 {
                                                            return Some(
                                                                UMULL_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xff20fc00 == 0x6e20c000 {
                                                            return Some(
                                                                UMULL2_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x004000 == 0 {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0x10000000 == 0 {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0x40000000 == 0 {
                                                            if insn & 0xff20fc00 == 0xe202000 {
                                                                return Some(
                                                                    SSUBL_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xff20fc00 == 0x4e202000 {
                                                                return Some(
                                                                    SSUBL2_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x40000000 == 0 {
                                                            if insn & 0xff20fc00 == 0x2e202000 {
                                                                return Some(
                                                                    USUBL_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xff20fc00 == 0x6e202000 {
                                                                return Some(
                                                                    USUBL2_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x000008 == 0 {
                                                        if insn & 0x000010 == 0 {
                                                            if insn & 0xffe0fc1f == 0x1ee02000 {
                                                                return Some(
                                                                    FCMP_Fn_Fm::make_opcode(insn),
                                                                );
                                                            }
                                                            if insn & 0xff20fc1f == 0x1e202000 {
                                                                return Some(
                                                                    FCMP_Fn_S_S_Fm_S_S::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xffe0fc1f == 0x1ee02010 {
                                                                return Some(
                                                                    FCMPE_Fn_Fm::make_opcode(insn),
                                                                );
                                                            }
                                                            if insn & 0xff20fc1f == 0x1e202010 {
                                                                return Some (FCMPE_Fn_S_S_Fm_S_S :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x000010 == 0 {
                                                            if insn & 0xffe0fc1f == 0x1ee02008 {
                                                                return Some(
                                                                    FCMP_Fn_FPIMM0::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                            if insn & 0xff20fc1f == 0x1e202008 {
                                                                return Some(
                                                                    FCMP_Fn_S_S_FPIMM0::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xffe0fc1f == 0x1ee02018 {
                                                                return Some(
                                                                    FCMPE_Fn_FPIMM0::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                            if insn & 0xff20fc1f == 0x1e202018 {
                                                                return Some (FCMPE_Fn_S_S_FPIMM0 :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x20000000 == 0 {
                                                    if insn & 0x40000000 == 0 {
                                                        if insn & 0xff20fc00 == 0xe20a000 {
                                                            return Some(
                                                                SMLSL_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xff20fc00 == 0x4e20a000 {
                                                            return Some(
                                                                SMLSL2_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x40000000 == 0 {
                                                        if insn & 0xff20fc00 == 0x2e20a000 {
                                                            return Some(
                                                                UMLSL_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xff20fc00 == 0x6e20a000 {
                                                            return Some(
                                                                UMLSL2_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0x20000000 == 0 {
                                                    if insn & 0x40000000 == 0 {
                                                        if insn & 0xff20fc00 == 0xe206000 {
                                                            return Some(
                                                                SUBHN_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xff20fc00 == 0x4e206000 {
                                                            return Some(
                                                                SUBHN2_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x40000000 == 0 {
                                                        if insn & 0xff20fc00 == 0x2e206000 {
                                                            return Some(
                                                                RSUBHN_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xff20fc00 == 0x6e206000 {
                                                            return Some(
                                                                RSUBHN2_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0x40000000 == 0 {
                                                        if insn & 0xffe0fc00 == 0xe20e000 {
                                                            return Some(
                                                                PMULL_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xffe0fc00 == 0x4e20e000 {
                                                            return Some(
                                                                PMULL2_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x40000000 == 0 {
                                                        if insn & 0xffe0fc00 == 0xee0e000 {
                                                            return Some (PMULL_Vd_V_1Q_Vn_V_1D_Vm_V_1D :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0xffe0fc00 == 0x4ee0e000 {
                                                            return Some (PMULL2_Vd_V_1Q_Vn_V_2D_Vm_V_2D :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x10000000 == 0 {
                                        if insn & 0x002000 == 0 {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0x40000000 == 0 {
                                                            if insn & 0xff20fc00 == 0xe201000 {
                                                                return Some(
                                                                    SADDW_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xff20fc00 == 0x4e201000 {
                                                                return Some(
                                                                    SADDW2_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x40000000 == 0 {
                                                            if insn & 0xff20fc00 == 0x2e201000 {
                                                                return Some(
                                                                    UADDW_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xff20fc00 == 0x6e201000 {
                                                                return Some(
                                                                    UADDW2_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x40000000 == 0 {
                                                        if insn & 0xff20fc00 == 0xe209000 {
                                                            return Some(
                                                                SQDMLAL_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xff20fc00 == 0x4e209000 {
                                                            return Some(
                                                                SQDMLAL2_Vd_Vn_Vm::make_opcode(
                                                                    insn,
                                                                ),
                                                            );
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0x40000000 == 0 {
                                                            if insn & 0xff20fc00 == 0xe205000 {
                                                                return Some(
                                                                    SABAL_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xff20fc00 == 0x4e205000 {
                                                                return Some(
                                                                    SABAL2_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x40000000 == 0 {
                                                            if insn & 0xff20fc00 == 0x2e205000 {
                                                                return Some(
                                                                    UABAL_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xff20fc00 == 0x6e205000 {
                                                                return Some(
                                                                    UABAL2_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x40000000 == 0 {
                                                        if insn & 0xff20fc00 == 0xe20d000 {
                                                            return Some(
                                                                SQDMULL_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xff20fc00 == 0x4e20d000 {
                                                            return Some(
                                                                SQDMULL2_Vd_Vn_Vm::make_opcode(
                                                                    insn,
                                                                ),
                                                            );
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0x40000000 == 0 {
                                                            if insn & 0xff20fc00 == 0xe203000 {
                                                                return Some(
                                                                    SSUBW_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xff20fc00 == 0x4e203000 {
                                                                return Some(
                                                                    SSUBW2_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x40000000 == 0 {
                                                            if insn & 0xff20fc00 == 0x2e203000 {
                                                                return Some(
                                                                    USUBW_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xff20fc00 == 0x6e203000 {
                                                                return Some(
                                                                    USUBW2_Vd_Vn_Vm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x40000000 == 0 {
                                                        if insn & 0xff20fc00 == 0xe20b000 {
                                                            return Some(
                                                                SQDMLSL_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xff20fc00 == 0x4e20b000 {
                                                            return Some(
                                                                SQDMLSL2_Vd_Vn_Vm::make_opcode(
                                                                    insn,
                                                                ),
                                                            );
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x20000000 == 0 {
                                                    if insn & 0x40000000 == 0 {
                                                        if insn & 0xff20fc00 == 0xe207000 {
                                                            return Some(
                                                                SABDL_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xff20fc00 == 0x4e207000 {
                                                            return Some(
                                                                SABDL2_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x40000000 == 0 {
                                                        if insn & 0xff20fc00 == 0x2e207000 {
                                                            return Some(
                                                                UABDL_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xff20fc00 == 0x6e207000 {
                                                            return Some(
                                                                UABDL2_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xffe01fe0 == 0x1ee01000 {
                                                return Some(FMOV_Fd_FPIMM::make_opcode(insn));
                                            }
                                            if insn & 0xff201fe0 == 0x1e201000 {
                                                return Some(FMOV_Fd_S_S_FPIMM::make_opcode(insn));
                                            }
                                        } else {
                                            if insn & 0x002000 == 0 {
                                                if insn & 0x004000 == 0 {
                                                    if insn & 0xff20fc00 == 0x5e209000 {
                                                        return Some(
                                                            SQDMLAL_Sd_Sn_Sm::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xff20fc00 == 0x5e20d000 {
                                                        return Some(
                                                            SQDMULL_Sd_Sn_Sm::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0xff20fc00 == 0x5e20b000 {
                                                    return Some(SQDMLSL_Sd_Sn_Sm::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x001000 == 0 {
                                    if insn & 0x002000 == 0 {
                                        if insn & 0x004000 == 0 {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0x20000000 == 0 {
                                                    if insn & 0xbf3ffc00 == 0xe200800 {
                                                        return Some(REV64_Vd_Vn::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                } else {
                                                    if insn & 0xbf3ffc00 == 0x2e200800 {
                                                        return Some(REV32_Vd_Vn::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                }
                                            } else {
                                                if insn & 0x010000 == 0 {
                                                    if insn & 0x10000000 == 0 {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0xbf3ffc00 == 0xe208800 {
                                                                return Some(
                                                                    CMGT_Vd_Vn_IMM0::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xbf3ffc00 == 0x2e208800 {
                                                                return Some(
                                                                    CMGE_Vd_Vn_IMM0::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0xff3ffc00 == 0x5e208800 {
                                                                return Some(
                                                                    CMGT_Sd_Sn_IMM0::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xff3ffc00 == 0x7e208800 {
                                                                return Some(
                                                                    CMGE_Sd_Sn_IMM0::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x080000 == 0 {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x20000000 == 0 {
                                                                if insn & 0xbfbffc00 == 0xe218800 {
                                                                    return Some(
                                                                        FRINTN_Vd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            } else {
                                                                if insn & 0xbfbffc00 == 0x2e218800 {
                                                                    return Some(
                                                                        FRINTA_Vd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xbfbffc00 == 0xea18800 {
                                                                return Some(
                                                                    FRINTP_Vd_Vn::make_opcode(insn),
                                                                );
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x20000000 == 0 {
                                                                if insn & 0xbffffc00 == 0xe798800 {
                                                                    return Some (FRINTN_Vd_V_4H_Vn_V_4H :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xbffffc00 == 0x2e798800 {
                                                                    return Some (FRINTA_Vd_V_4H_Vn_V_4H :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xbffffc00 == 0xef98800 {
                                                                return Some (FRINTP_Vd_V_4H_Vn_V_4H :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0x010000 == 0 {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0xbf3ffc00 == 0xe204800 {
                                                            return Some(CLS_Vd_Vn::make_opcode(
                                                                insn,
                                                            ));
                                                        }
                                                    } else {
                                                        if insn & 0xbf3ffc00 == 0x2e204800 {
                                                            return Some(CLZ_Vd_Vn::make_opcode(
                                                                insn,
                                                            ));
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x10000000 == 0 {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xff3ffc00 == 0xe214800 {
                                                                    return Some(
                                                                        SQXTN_Vd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            } else {
                                                                if insn & 0xff3ffc00 == 0x4e214800 {
                                                                    return Some(
                                                                        SQXTN2_Vd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xff3ffc00 == 0x2e214800 {
                                                                    return Some(
                                                                        UQXTN_Vd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            } else {
                                                                if insn & 0xff3ffc00 == 0x6e214800 {
                                                                    return Some(
                                                                        UQXTN2_Vd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0xff3ffc00 == 0x5e214800 {
                                                                return Some(
                                                                    SQXTN_Sd_Sn::make_opcode(insn),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xff3ffc00 == 0x7e214800 {
                                                                return Some(
                                                                    UQXTN_Sd_Sn::make_opcode(insn),
                                                                );
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x010000 == 0 {
                                                    if insn & 0x080000 == 0 {
                                                        if insn & 0x100000 == 0 {
                                                            if insn & 0x10000000 == 0 {
                                                                if insn & 0x20000000 == 0 {
                                                                    if insn & 0xbfbffc00
                                                                        == 0xea0c800
                                                                    {
                                                                        return Some (FCMGT_Vd_Vn_FPIMM0 :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xbfbffc00
                                                                        == 0x2ea0c800
                                                                    {
                                                                        return Some (FCMGE_Vd_Vn_FPIMM0 :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0x20000000 == 0 {
                                                                    if insn & 0xffbffc00
                                                                        == 0x5ea0c800
                                                                    {
                                                                        return Some (FCMGT_Sd_Sn_FPIMM0 :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffbffc00
                                                                        == 0x7ea0c800
                                                                    {
                                                                        return Some (FCMGE_Sd_Sn_FPIMM0 :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x800000 == 0 {
                                                                if insn & 0x10000000 == 0 {
                                                                    if insn & 0x20000000 == 0 {
                                                                        if insn & 0xbffffc00
                                                                            == 0xe30c800
                                                                        {
                                                                            return Some (FMAXNMV_Fd_Vn :: make_opcode (insn)) ;
                                                                        }
                                                                    } else {
                                                                        if insn & 0xbfbffc00
                                                                            == 0x2e30c800
                                                                        {
                                                                            return Some (FMAXNMV_Fd_S_S_Vn_V_4S :: make_opcode (insn)) ;
                                                                        }
                                                                    }
                                                                } else {
                                                                    if insn & 0x20000000 == 0 {
                                                                        if insn & 0xfffffc00
                                                                            == 0x5e30c800
                                                                        {
                                                                            return Some (FMAXNMP_Sd_Vn :: make_opcode (insn)) ;
                                                                        }
                                                                    } else {
                                                                        if insn & 0xffbffc00
                                                                            == 0x7e30c800
                                                                        {
                                                                            return Some (FMAXNMP_Sd_S_S_Vn_V_2S :: make_opcode (insn)) ;
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0x10000000 == 0 {
                                                                    if insn & 0x20000000 == 0 {
                                                                        if insn & 0xbffffc00
                                                                            == 0xeb0c800
                                                                        {
                                                                            return Some (FMINNMV_Fd_Vn :: make_opcode (insn)) ;
                                                                        }
                                                                    } else {
                                                                        if insn & 0xbfbffc00
                                                                            == 0x2eb0c800
                                                                        {
                                                                            return Some (FMINNMV_Fd_S_S_Vn_V_4S :: make_opcode (insn)) ;
                                                                        }
                                                                    }
                                                                } else {
                                                                    if insn & 0x20000000 == 0 {
                                                                        if insn & 0xfffffc00
                                                                            == 0x5eb0c800
                                                                        {
                                                                            return Some (FMINNMP_Sd_Vn :: make_opcode (insn)) ;
                                                                        }
                                                                    } else {
                                                                        if insn & 0xffbffc00
                                                                            == 0x7eb0c800
                                                                        {
                                                                            return Some (FMINNMP_Sd_S_S_Vn_V_2S :: make_opcode (insn)) ;
                                                                        }
                                                                    }
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
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x10000000 == 0 {
                                                                if insn & 0x20000000 == 0 {
                                                                    if insn & 0xbfbffc00
                                                                        == 0xe21c800
                                                                    {
                                                                        return Some (FCVTAS_Vd_Vn :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xbfbffc00
                                                                        == 0x2e21c800
                                                                    {
                                                                        return Some (FCVTAU_Vd_Vn :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0x20000000 == 0 {
                                                                    if insn & 0xffbffc00
                                                                        == 0x5e21c800
                                                                    {
                                                                        return Some (FCVTAS_Sd_Sn :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffbffc00
                                                                        == 0x7e21c800
                                                                    {
                                                                        return Some (FCVTAU_Sd_Sn :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x20000000 == 0 {
                                                                if insn & 0xbfbffc00 == 0xea1c800 {
                                                                    return Some(
                                                                        URECPE_Vd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            } else {
                                                                if insn & 0xbfbffc00 == 0x2ea1c800 {
                                                                    return Some(
                                                                        URSQRTE_Vd_Vn::make_opcode(
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
                                                if insn & 0x010000 == 0 {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0xbf3ffc00 == 0xe202800 {
                                                            return Some(
                                                                SADDLP_Vd_Vn::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xbf3ffc00 == 0x2e202800 {
                                                            return Some(
                                                                UADDLP_Vd_Vn::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x10000000 == 0 {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xff3ffc00 == 0xe212800 {
                                                                    return Some(
                                                                        XTN_Vd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            } else {
                                                                if insn & 0xff3ffc00 == 0x4e212800 {
                                                                    return Some(
                                                                        XTN2_Vd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xff3ffc00 == 0x2e212800 {
                                                                    return Some(
                                                                        SQXTUN_Vd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            } else {
                                                                if insn & 0xff3ffc00 == 0x6e212800 {
                                                                    return Some(
                                                                        SQXTUN2_Vd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0xff3ffc00 == 0x7e212800 {
                                                            return Some(
                                                                SQXTUN_Sd_Sn::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x010000 == 0 {
                                                    if insn & 0x100000 == 0 {
                                                        if insn & 0x10000000 == 0 {
                                                            if insn & 0xbf3ffc00 == 0xe20a800 {
                                                                return Some(
                                                                    CMLT_Vd_Vn_IMM0::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xff3ffc00 == 0x5e20a800 {
                                                                return Some(
                                                                    CMLT_Sd_Sn_IMM0::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0xbf3ffc00 == 0xe30a800 {
                                                                return Some(
                                                                    SMAXV_Fd_Vn::make_opcode(insn),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xbf3ffc00 == 0x2e30a800 {
                                                                return Some(
                                                                    UMAXV_Fd_Vn::make_opcode(insn),
                                                                );
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x080000 == 0 {
                                                        if insn & 0x100000 == 0 {
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
                                                            if insn & 0x20000000 == 0 {
                                                                if insn & 0xbf3ffc00 == 0xe31a800 {
                                                                    return Some(
                                                                        SMINV_Fd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            } else {
                                                                if insn & 0xbf3ffc00 == 0x2e31a800 {
                                                                    return Some(
                                                                        UMINV_Fd_Vn::make_opcode(
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
                                                if insn & 0x010000 == 0 {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0xbf3ffc00 == 0xe206800 {
                                                            return Some(
                                                                SADALP_Vd_Vn::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xbf3ffc00 == 0x2e206800 {
                                                            return Some(
                                                                UADALP_Vd_Vn::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                } else {
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
                                                }
                                            } else {
                                                if insn & 0x010000 == 0 {
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
                                                } else {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0xbfbffc00 == 0xe21e800 {
                                                            return Some(
                                                                FRINT32Z_Vd_Vn::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xbfbffc00 == 0x2e21e800 {
                                                            return Some(
                                                                FRINT32X_Vd_Vn::make_opcode(insn),
                                                            );
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
                                                if insn & 0xbf3ffc00 == 0xe201800 {
                                                    return Some(REV16_Vd_Vn::make_opcode(insn));
                                                }
                                            } else {
                                                if insn & 0x010000 == 0 {
                                                    if insn & 0x10000000 == 0 {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0xbf3ffc00 == 0xe209800 {
                                                                return Some(
                                                                    CMEQ_Vd_Vn_IMM0::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xbf3ffc00 == 0x2e209800 {
                                                                return Some(
                                                                    CMLE_Vd_Vn_IMM0::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0xff3ffc00 == 0x5e209800 {
                                                                return Some(
                                                                    CMEQ_Sd_Sn_IMM0::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xff3ffc00 == 0x7e209800 {
                                                                return Some(
                                                                    CMLE_Sd_Sn_IMM0::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x080000 == 0 {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x20000000 == 0 {
                                                                if insn & 0xbfbffc00 == 0xe219800 {
                                                                    return Some(
                                                                        FRINTM_Vd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            } else {
                                                                if insn & 0xbfbffc00 == 0x2e219800 {
                                                                    return Some(
                                                                        FRINTX_Vd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x20000000 == 0 {
                                                                if insn & 0xbfbffc00 == 0xea19800 {
                                                                    return Some(
                                                                        FRINTZ_Vd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            } else {
                                                                if insn & 0xbfbffc00 == 0x2ea19800 {
                                                                    return Some(
                                                                        FRINTI_Vd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x20000000 == 0 {
                                                                if insn & 0xbffffc00 == 0xe799800 {
                                                                    return Some (FRINTM_Vd_V_4H_Vn_V_4H :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xbffffc00 == 0x2e799800 {
                                                                    return Some (FRINTX_Vd_V_4H_Vn_V_4H :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x20000000 == 0 {
                                                                if insn & 0xbffffc00 == 0xef99800 {
                                                                    return Some (FRINTZ_Vd_V_4H_Vn_V_4H :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xbffffc00 == 0x2ef99800 {
                                                                    return Some (FRINTI_Vd_V_4H_Vn_V_4H :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0x20000000 == 0 {
                                                    if insn & 0xbf3ffc00 == 0xe205800 {
                                                        return Some(CNT_Vd_Vn::make_opcode(insn));
                                                    }
                                                } else {
                                                    if insn & 0x400000 == 0 {
                                                        if insn & 0xbffffc00 == 0x2e205800 {
                                                            return Some(NOT_Vd_Vn::make_opcode(
                                                                insn,
                                                            ));
                                                        }
                                                    } else {
                                                        if insn & 0xbffffc00 == 0x2e605800 {
                                                            return Some(RBIT_Vd_Vn::make_opcode(
                                                                insn,
                                                            ));
                                                        }
                                                    }
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
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x10000000 == 0 {
                                                                if insn & 0x20000000 == 0 {
                                                                    if insn & 0xbfbffc00
                                                                        == 0xe21d800
                                                                    {
                                                                        return Some (SCVTF_Vd_Vn :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xbfbffc00
                                                                        == 0x2e21d800
                                                                    {
                                                                        return Some (UCVTF_Vd_Vn :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0x20000000 == 0 {
                                                                    if insn & 0xffbffc00
                                                                        == 0x5e21d800
                                                                    {
                                                                        return Some (SCVTF_Sd_Sn :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffbffc00
                                                                        == 0x7e21d800
                                                                    {
                                                                        return Some (UCVTF_Sd_Sn :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x10000000 == 0 {
                                                                if insn & 0x20000000 == 0 {
                                                                    if insn & 0xbfbffc00
                                                                        == 0xea1d800
                                                                    {
                                                                        return Some (FRECPE_Vd_Vn :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xbfbffc00
                                                                        == 0x2ea1d800
                                                                    {
                                                                        return Some (FRSQRTE_Vd_Vn :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0x20000000 == 0 {
                                                                    if insn & 0xffbffc00
                                                                        == 0x5ea1d800
                                                                    {
                                                                        return Some (FRECPE_Sd_Sn :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffbffc00
                                                                        == 0x7ea1d800
                                                                    {
                                                                        return Some (FRSQRTE_Sd_Sn :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x10000000 == 0 {
                                                                if insn & 0x20000000 == 0 {
                                                                    if insn & 0xbffffc00
                                                                        == 0xe79d800
                                                                    {
                                                                        return Some (SCVTF_Vd_V_4H_Vn_V_4H :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xbffffc00
                                                                        == 0x2e79d800
                                                                    {
                                                                        return Some (UCVTF_Vd_V_4H_Vn_V_4H :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0x20000000 == 0 {
                                                                    if insn & 0xfffffc00
                                                                        == 0x5e79d800
                                                                    {
                                                                        return Some (SCVTF_Sd_S_H_Sn_S_H :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xfffffc00
                                                                        == 0x7e79d800
                                                                    {
                                                                        return Some (UCVTF_Sd_S_H_Sn_S_H :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x10000000 == 0 {
                                                                if insn & 0x20000000 == 0 {
                                                                    if insn & 0xbffffc00
                                                                        == 0xef9d800
                                                                    {
                                                                        return Some (FRECPE_Vd_V_4H_Vn_V_4H :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xbffffc00
                                                                        == 0x2ef9d800
                                                                    {
                                                                        return Some (FRSQRTE_Vd_V_4H_Vn_V_4H :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0x20000000 == 0 {
                                                                    if insn & 0xfffffc00
                                                                        == 0x5ef9d800
                                                                    {
                                                                        return Some (FRECPE_Sd_S_H_Sn_S_H :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xfffffc00
                                                                        == 0x7ef9d800
                                                                    {
                                                                        return Some (FRSQRTE_Sd_S_H_Sn_S_H :: make_opcode (insn)) ;
                                                                    }
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
                                                if insn & 0x010000 == 0 {
                                                    if insn & 0x100000 == 0 {
                                                        if insn & 0x10000000 == 0 {
                                                            if insn & 0x20000000 == 0 {
                                                                if insn & 0xbf3ffc00 == 0xe203800 {
                                                                    return Some(
                                                                        SUQADD_Vd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            } else {
                                                                if insn & 0xbf3ffc00 == 0x2e203800 {
                                                                    return Some(
                                                                        USQADD_Vd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x20000000 == 0 {
                                                                if insn & 0xff3ffc00 == 0x5e203800 {
                                                                    return Some(
                                                                        SUQADD_Sd_Sn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            } else {
                                                                if insn & 0xff3ffc00 == 0x7e203800 {
                                                                    return Some(
                                                                        USQADD_Sd_Sn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0xbf3ffc00 == 0xe303800 {
                                                                return Some(
                                                                    SADDLV_Fd_Vn::make_opcode(insn),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xbf3ffc00 == 0x2e303800 {
                                                                return Some(
                                                                    UADDLV_Fd_Vn::make_opcode(insn),
                                                                );
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x40000000 == 0 {
                                                        if insn & 0xff3ffc00 == 0x2e213800 {
                                                            return Some(
                                                                SHLL_Vd_Vn_SHLL_IMM::make_opcode(
                                                                    insn,
                                                                ),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xff3ffc00 == 0x6e213800 {
                                                            return Some(
                                                                SHLL2_Vd_Vn_SHLL_IMM::make_opcode(
                                                                    insn,
                                                                ),
                                                            );
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x010000 == 0 {
                                                    if insn & 0x10000000 == 0 {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0xbf3ffc00 == 0xe20b800 {
                                                                return Some(
                                                                    ABS_Vd_Vn::make_opcode(insn),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xbf3ffc00 == 0x2e20b800 {
                                                                return Some(
                                                                    NEG_Vd_Vn::make_opcode(insn),
                                                                );
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0xff3ffc00 == 0x5e20b800 {
                                                                return Some(
                                                                    ABS_Sd_Sn::make_opcode(insn),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xff3ffc00 == 0x7e20b800 {
                                                                return Some(
                                                                    NEG_Sd_Sn::make_opcode(insn),
                                                                );
                                                            }
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
                                            if insn & 0x008000 == 0 {
                                                if insn & 0x010000 == 0 {
                                                    if insn & 0x10000000 == 0 {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0xbf3ffc00 == 0xe207800 {
                                                                return Some(
                                                                    SQABS_Vd_Vn::make_opcode(insn),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xbf3ffc00 == 0x2e207800 {
                                                                return Some(
                                                                    SQNEG_Vd_Vn::make_opcode(insn),
                                                                );
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0xff3ffc00 == 0x5e207800 {
                                                                return Some(
                                                                    SQABS_Sd_Sn::make_opcode(insn),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xff3ffc00 == 0x7e207800 {
                                                                return Some(
                                                                    SQNEG_Sd_Sn::make_opcode(insn),
                                                                );
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x40000000 == 0 {
                                                        if insn & 0xffbffc00 == 0xe217800 {
                                                            return Some(FCVTL_Vd_Vn::make_opcode(
                                                                insn,
                                                            ));
                                                        }
                                                    } else {
                                                        if insn & 0xffbffc00 == 0x4e217800 {
                                                            return Some(
                                                                FCVTL2_Vd_Vn::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x010000 == 0 {
                                                    if insn & 0x080000 == 0 {
                                                        if insn & 0x100000 == 0 {
                                                            if insn & 0x20000000 == 0 {
                                                                if insn & 0xbfbffc00 == 0xea0f800 {
                                                                    return Some(
                                                                        FABS_Vd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            } else {
                                                                if insn & 0xbfbffc00 == 0x2ea0f800 {
                                                                    return Some(
                                                                        FNEG_Vd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x800000 == 0 {
                                                                if insn & 0x10000000 == 0 {
                                                                    if insn & 0x20000000 == 0 {
                                                                        if insn & 0xbffffc00
                                                                            == 0xe30f800
                                                                        {
                                                                            return Some (FMAXV_Fd_Vn :: make_opcode (insn)) ;
                                                                        }
                                                                    } else {
                                                                        if insn & 0xbfbffc00
                                                                            == 0x2e30f800
                                                                        {
                                                                            return Some (FMAXV_Fd_S_S_Vn_V_4S :: make_opcode (insn)) ;
                                                                        }
                                                                    }
                                                                } else {
                                                                    if insn & 0x20000000 == 0 {
                                                                        if insn & 0xfffffc00
                                                                            == 0x5e30f800
                                                                        {
                                                                            return Some (FMAXP_Sd_Vn :: make_opcode (insn)) ;
                                                                        }
                                                                    } else {
                                                                        if insn & 0xffbffc00
                                                                            == 0x7e30f800
                                                                        {
                                                                            return Some (FMAXP_Sd_S_S_Vn_V_2S :: make_opcode (insn)) ;
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0x10000000 == 0 {
                                                                    if insn & 0x20000000 == 0 {
                                                                        if insn & 0xbffffc00
                                                                            == 0xeb0f800
                                                                        {
                                                                            return Some (FMINV_Fd_Vn :: make_opcode (insn)) ;
                                                                        }
                                                                    } else {
                                                                        if insn & 0xbfbffc00
                                                                            == 0x2eb0f800
                                                                        {
                                                                            return Some (FMINV_Fd_S_S_Vn_V_4S :: make_opcode (insn)) ;
                                                                        }
                                                                    }
                                                                } else {
                                                                    if insn & 0x20000000 == 0 {
                                                                        if insn & 0xfffffc00
                                                                            == 0x5eb0f800
                                                                        {
                                                                            return Some (FMINP_Sd_Vn :: make_opcode (insn)) ;
                                                                        }
                                                                    } else {
                                                                        if insn & 0xffbffc00
                                                                            == 0x7eb0f800
                                                                        {
                                                                            return Some (FMINP_Sd_S_S_Vn_V_2S :: make_opcode (insn)) ;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0xbffffc00 == 0xef8f800 {
                                                                return Some (FABS_Vd_V_4H_Vn_V_4H :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xbffffc00 == 0x2ef8f800 {
                                                                return Some (FNEG_Vd_V_4H_Vn_V_4H :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x080000 == 0 {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x20000000 == 0 {
                                                                if insn & 0xbfbffc00 == 0xe21f800 {
                                                                    return Some(
                                                                        FRINT64Z_Vd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            } else {
                                                                if insn & 0xbfbffc00 == 0x2e21f800 {
                                                                    return Some(
                                                                        FRINT64X_Vd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x10000000 == 0 {
                                                                if insn & 0xbfbffc00 == 0x2ea1f800 {
                                                                    return Some(
                                                                        FSQRT_Vd_Vn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            } else {
                                                                if insn & 0xffbffc00 == 0x5ea1f800 {
                                                                    return Some(
                                                                        FRECPX_Sd_Sn::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x10000000 == 0 {
                                                            if insn & 0xbffffc00 == 0x2ef9f800 {
                                                                return Some (FSQRT_Vd_V_4H_Vn_V_4H :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xfffffc00 == 0x5ef9f800 {
                                                                return Some (FRECPX_Sd_S_H_Sn_S_H :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    }
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
                                            if insn & 0x004000 == 0 {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0xbf20fc00 == 0xe200400 {
                                                            return Some(
                                                                SHADD_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xbf20fc00 == 0x2e200400 {
                                                            return Some(
                                                                UHADD_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0xbf20fc00 == 0xe208400 {
                                                            return Some(
                                                                ADD_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xbf20fc00 == 0x2e208400 {
                                                            return Some(
                                                                SUB_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0xbf20fc00 == 0xe204400 {
                                                            return Some(
                                                                SSHL_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xbf20fc00 == 0x2e204400 {
                                                            return Some(
                                                                USHL_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0xbfa0fc00 == 0xe20c400 {
                                                                return Some (FMAXNM_Vd_V_2S_Vn_V_2S_Vm_V_2S :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xbfa0fc00 == 0x2e20c400 {
                                                                return Some (FMAXNMP_Vd_V_2S_Vn_V_2S_Vm_V_2S :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0xbfa0fc00 == 0xea0c400 {
                                                                return Some (FMINNM_Vd_V_2S_Vn_V_2S_Vm_V_2S :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xbfa0fc00 == 0x2ea0c400 {
                                                                return Some (FMINNMP_Vd_V_2S_Vn_V_2S_Vm_V_2S :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0xbf20fc00 == 0xe202400 {
                                                            return Some(
                                                                SHSUB_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xbf20fc00 == 0x2e202400 {
                                                            return Some(
                                                                UHSUB_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0xbf20fc00 == 0xe20a400 {
                                                            return Some(
                                                                SMAXP_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xbf20fc00 == 0x2e20a400 {
                                                            return Some(
                                                                UMAXP_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0xbf20fc00 == 0xe206400 {
                                                            return Some(
                                                                SMAX_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xbf20fc00 == 0x2e206400 {
                                                            return Some(
                                                                UMAX_Vd_Vn_Vm::make_opcode(insn),
                                                            );
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
                                                            return Some (FCMGT_Vd_V_2S_Vn_V_2S_Vm_V_2S :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x002000 == 0 {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0xbf20fc00 == 0xe201400 {
                                                            return Some(
                                                                SRHADD_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xbf20fc00 == 0x2e201400 {
                                                            return Some(
                                                                URHADD_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0xbf20fc00 == 0xe209400 {
                                                            return Some(
                                                                MLA_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xbf20fc00 == 0x2e209400 {
                                                            return Some(
                                                                MLS_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0xbf20fc00 == 0xe205400 {
                                                            return Some(
                                                                SRSHL_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xbf20fc00 == 0x2e205400 {
                                                            return Some(
                                                                URSHL_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                } else {
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
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0xbfa0fc00 == 0xea0d400 {
                                                                return Some (FSUB_Vd_V_2S_Vn_V_2S_Vm_V_2S :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xbfa0fc00 == 0x2ea0d400 {
                                                                return Some (FABD_Vd_V_2S_Vn_V_2S_Vm_V_2S :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0xbf20fc00 == 0xe203400 {
                                                            return Some(
                                                                CMGT_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xbf20fc00 == 0x2e203400 {
                                                            return Some(
                                                                CMHI_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0xbf20fc00 == 0xe20b400 {
                                                            return Some(
                                                                SQDMULH_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xbf20fc00 == 0x2e20b400 {
                                                            return Some(
                                                                SQRDMULH_Vd_Vn_Vm::make_opcode(
                                                                    insn,
                                                                ),
                                                            );
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0xbf20fc00 == 0xe207400 {
                                                            return Some(
                                                                SABD_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xbf20fc00 == 0x2e207400 {
                                                            return Some(
                                                                UABD_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0xbfa0fc00 == 0xe20f400 {
                                                                return Some (FMAX_Vd_V_2S_Vn_V_2S_Vm_V_2S :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xbfa0fc00 == 0x2e20f400 {
                                                                return Some (FMAXP_Vd_V_2S_Vn_V_2S_Vm_V_2S :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0xbfa0fc00 == 0xea0f400 {
                                                                return Some (FMIN_Vd_V_2S_Vn_V_2S_Vm_V_2S :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xbfa0fc00 == 0x2ea0f400 {
                                                                return Some (FMINP_Vd_V_2S_Vn_V_2S_Vm_V_2S :: make_opcode (insn)) ;
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
                                                    if insn & 0x004000 == 0 {
                                                        if insn & 0xffe0fc00 == 0x5ee08400 {
                                                            return Some(
                                                                ADD_Sd_Sn_Sm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xffe0fc00 == 0x5ee04400 {
                                                            return Some(
                                                                SSHL_Sd_Sn_Sm::make_opcode(insn),
                                                            );
                                                        }
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
                                                if insn & 0x002000 == 0 {
                                                    if insn & 0xffe0fc00 == 0x5ee05400 {
                                                        return Some(SRSHL_Sd_Sn_Sm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                } else {
                                                    if insn & 0x008000 == 0 {
                                                        if insn & 0xffe0fc00 == 0x5ee03400 {
                                                            return Some(
                                                                CMGT_Sd_Sn_Sm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xff20fc00 == 0x5e20b400 {
                                                            return Some(
                                                                SQDMULH_Sd_Sn_Sm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x001000 == 0 {
                                            if insn & 0x002000 == 0 {
                                                if insn & 0x004000 == 0 {
                                                    if insn & 0xffe0fc00 == 0x7ee08400 {
                                                        return Some(SUB_Sd_Sn_Sm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0x7ee04400 {
                                                        return Some(USHL_Sd_Sn_Sm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
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
                                            if insn & 0x002000 == 0 {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0xffe0fc00 == 0x7ee05400 {
                                                        return Some(URSHL_Sd_Sn_Sm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                } else {
                                                    if insn & 0xffa0fc00 == 0x7ea0d400 {
                                                        return Some(
                                                            FABD_Sd_S_S_Sn_S_S_Sm_S_S::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0xffe0fc00 == 0x7ee03400 {
                                                        return Some(CMHI_Sd_Sn_Sm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                } else {
                                                    if insn & 0xff20fc00 == 0x7e20b400 {
                                                        return Some(
                                                            SQRDMULH_Sd_Sn_Sm::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x10000000 == 0 {
                                    if insn & 0x001000 == 0 {
                                        if insn & 0x002000 == 0 {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0xbf20fc00 == 0xe200c00 {
                                                            return Some(
                                                                SQADD_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xbf20fc00 == 0x2e200c00 {
                                                            return Some(
                                                                UQADD_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0xbf20fc00 == 0xe208c00 {
                                                            return Some(
                                                                CMTST_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xbf20fc00 == 0x2e208c00 {
                                                            return Some(
                                                                CMEQ_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0xbf20fc00 == 0xe204c00 {
                                                            return Some(
                                                                SQSHL_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xbf20fc00 == 0x2e204c00 {
                                                            return Some(
                                                                UQSHL_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0xbfa0fc00 == 0xe20cc00 {
                                                                return Some (FMLA_Vd_V_2S_Vn_V_2S_Vm_V_2S :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xffa0fc00 == 0x2e20cc00 {
                                                                    return Some (FMLAL2_Vd_Vn_Vm :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xffa0fc00 == 0x6e20cc00 {
                                                                    return Some (FMLAL2_Vd_V_4S_Vn_V_4H_Vm_V_4H :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0xbfa0fc00 == 0xea0cc00 {
                                                                return Some (FMLS_Vd_V_2S_Vn_V_2S_Vm_V_2S :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xffa0fc00 == 0x2ea0cc00 {
                                                                    return Some (FMLSL2_Vd_Vn_Vm :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xffa0fc00 == 0x6ea0cc00 {
                                                                    return Some (FMLSL2_Vd_V_4S_Vn_V_4H_Vm_V_4H :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0xbf20fc00 == 0xe202c00 {
                                                            return Some(
                                                                SQSUB_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xbf20fc00 == 0x2e202c00 {
                                                            return Some(
                                                                UQSUB_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0xbf20fc00 == 0xe20ac00 {
                                                            return Some(
                                                                SMINP_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xbf20fc00 == 0x2e20ac00 {
                                                            return Some(
                                                                UMINP_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0xbf20fc00 == 0xe206c00 {
                                                            return Some(
                                                                SMIN_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xbf20fc00 == 0x2e206c00 {
                                                            return Some(
                                                                UMIN_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xffa0fc00 == 0xe20ec00 {
                                                                    return Some(
                                                                        FMLAL_Vd_Vn_Vm::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            } else {
                                                                if insn & 0xffa0fc00 == 0x4e20ec00 {
                                                                    return Some (FMLAL_Vd_V_4S_Vn_V_4H_Vm_V_4H :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xbfa0fc00 == 0x2e20ec00 {
                                                                return Some (FACGE_Vd_V_2S_Vn_V_2S_Vm_V_2S :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xffa0fc00 == 0xea0ec00 {
                                                                    return Some(
                                                                        FMLSL_Vd_Vn_Vm::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            } else {
                                                                if insn & 0xffa0fc00 == 0x4ea0ec00 {
                                                                    return Some (FMLSL_Vd_V_4S_Vn_V_4H_Vm_V_4H :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xbfa0fc00 == 0x2ea0ec00 {
                                                                return Some (FACGT_Vd_V_2S_Vn_V_2S_Vm_V_2S :: make_opcode (insn)) ;
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
                                                            if insn & 0x20000000 == 0 {
                                                                if insn & 0xbfe0fc00 == 0xe601c00 {
                                                                    return Some(
                                                                        BIC_Vd_Vn_Vm::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0x2e601c00 {
                                                                    return Some(
                                                                        BSL_Vd_Vn_Vm::make_opcode(
                                                                            insn,
                                                                        ),
                                                                    );
                                                                }
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
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0xbf20fc00 == 0xe209c00 {
                                                            return Some(
                                                                MUL_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xbf20fc00 == 0x2e209c00 {
                                                            return Some(
                                                                PMUL_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0xbf20fc00 == 0xe205c00 {
                                                            return Some(
                                                                SQRSHL_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xbf20fc00 == 0x2e205c00 {
                                                            return Some(
                                                                UQRSHL_Vd_Vn_Vm::make_opcode(insn),
                                                            );
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
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x20000000 == 0 {
                                                        if insn & 0xbf20fc00 == 0xe207c00 {
                                                            return Some(
                                                                SABA_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xbf20fc00 == 0x2e207c00 {
                                                            return Some(
                                                                UABA_Vd_Vn_Vm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0x20000000 == 0 {
                                                            if insn & 0xbfa0fc00 == 0xe20fc00 {
                                                                return Some (FRECPS_Vd_V_2S_Vn_V_2S_Vm_V_2S :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xbfa0fc00 == 0x2e20fc00 {
                                                                return Some (FDIV_Vd_V_2S_Vn_V_2S_Vm_V_2S :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0xbfa0fc00 == 0xea0fc00 {
                                                            return Some (FRSQRTS_Vd_V_2S_Vn_V_2S_Vm_V_2S :: make_opcode (insn)) ;
                                                        }
                                                    }
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
                                                if insn & 0x002000 == 0 {
                                                    if insn & 0x004000 == 0 {
                                                        if insn & 0x008000 == 0 {
                                                            if insn & 0xff20fc00 == 0x5e200c00 {
                                                                return Some(
                                                                    SQADD_Sd_Sn_Sm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        } else {
                                                            if insn & 0xffe0fc00 == 0x5ee08c00 {
                                                                return Some(
                                                                    CMTST_Sd_Sn_Sm::make_opcode(
                                                                        insn,
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0xff20fc00 == 0x5e204c00 {
                                                            return Some(
                                                                SQSHL_Sd_Sn_Sm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xff20fc00 == 0x5e202c00 {
                                                        return Some(SQSUB_Sd_Sn_Sm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                }
                                            } else {
                                                if insn & 0x002000 == 0 {
                                                    if insn & 0x008000 == 0 {
                                                        if insn & 0xff20fc00 == 0x5e205c00 {
                                                            return Some(
                                                                SQRSHL_Sd_Sn_Sm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xffa0fc00 == 0x5e20dc00 {
                                                            return Some (FMULX_Sd_S_S_Sn_S_S_Sm_S_S :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x004000 == 0 {
                                                        if insn & 0xffe0fc00 == 0x5ee03c00 {
                                                            return Some(
                                                                CMGE_Sd_Sn_Sm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0xffa0fc00 == 0x5e20fc00 {
                                                                return Some (FRECPS_Sd_S_S_Sn_S_S_Sm_S_S :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xffa0fc00 == 0x5ea0fc00 {
                                                                return Some (FRSQRTS_Sd_S_S_Sn_S_S_Sm_S_S :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x001000 == 0 {
                                            if insn & 0x002000 == 0 {
                                                if insn & 0x004000 == 0 {
                                                    if insn & 0x008000 == 0 {
                                                        if insn & 0xff20fc00 == 0x7e200c00 {
                                                            return Some(
                                                                UQADD_Sd_Sn_Sm::make_opcode(insn),
                                                            );
                                                        }
                                                    } else {
                                                        if insn & 0xffe0fc00 == 0x7ee08c00 {
                                                            return Some(
                                                                CMEQ_Sd_Sn_Sm::make_opcode(insn),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xff20fc00 == 0x7e204c00 {
                                                        return Some(UQSHL_Sd_Sn_Sm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                }
                                            } else {
                                                if insn & 0x004000 == 0 {
                                                    if insn & 0xff20fc00 == 0x7e202c00 {
                                                        return Some(UQSUB_Sd_Sn_Sm::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                } else {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0xffa0fc00 == 0x7e20ec00 {
                                                            return Some (FACGE_Sd_S_S_Sn_S_S_Sm_S_S :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0xffa0fc00 == 0x7ea0ec00 {
                                                            return Some (FACGT_Sd_S_S_Sn_S_S_Sm_S_S :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x002000 == 0 {
                                                if insn & 0xff20fc00 == 0x7e205c00 {
                                                    return Some(UQRSHL_Sd_Sn_Sm::make_opcode(
                                                        insn,
                                                    ));
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
                    }
                } else {
                    if insn & 0x000400 == 0 {
                        if insn & 0x001000 == 0 {
                            if insn & 0x002000 == 0 {
                                if insn & 0x004000 == 0 {
                                    if insn & 0x008000 == 0 {
                                        if insn & 0x20000000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xffc0f400 == 0xf800000 {
                                                    return Some(FMLAL_Vd_Vn_Em16::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            } else {
                                                if insn & 0xffc0f400 == 0x4f800000 {
                                                    return Some(
                                                        FMLAL_Vd_V_4S_Vn_V_4H_Em16_S_H::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            }
                                        } else {
                                            if insn & 0xbf00f400 == 0x2f000000 {
                                                return Some(MLA_Vd_Vn_Em16::make_opcode(insn));
                                            }
                                        }
                                    } else {
                                        if insn & 0x20000000 == 0 {
                                            if insn & 0xbf00f400 == 0xf008000 {
                                                return Some(MUL_Vd_Vn_Em16::make_opcode(insn));
                                            }
                                        } else {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xffc0f400 == 0x2f808000 {
                                                    return Some(FMLAL2_Vd_Vn_Em16::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            } else {
                                                if insn & 0xffc0f400 == 0x6f808000 {
                                                    return Some (FMLAL2_Vd_V_4S_Vn_V_4H_Em16_S_H :: make_opcode (insn)) ;
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x008000 == 0 {
                                        if insn & 0x20000000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xffc0f400 == 0xf804000 {
                                                    return Some(FMLSL_Vd_Vn_Em16::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            } else {
                                                if insn & 0xffc0f400 == 0x4f804000 {
                                                    return Some(
                                                        FMLSL_Vd_V_4S_Vn_V_4H_Em16_S_H::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            }
                                        } else {
                                            if insn & 0xbf00f400 == 0x2f004000 {
                                                return Some(MLS_Vd_Vn_Em16::make_opcode(insn));
                                            }
                                        }
                                    } else {
                                        if insn & 0x10000000 == 0 {
                                            if insn & 0x20000000 == 0 {
                                                if insn & 0xbf00f400 == 0xf00c000 {
                                                    return Some(SQDMULH_Vd_Vn_Em16::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            } else {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xffc0f400 == 0x2f80c000 {
                                                        return Some(
                                                            FMLSL2_Vd_Vn_Em16::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xffc0f400 == 0x6f80c000 {
                                                        return Some (FMLSL2_Vd_V_4S_Vn_V_4H_Em16_S_H :: make_opcode (insn)) ;
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0xff00f400 == 0x5f00c000 {
                                                return Some(SQDMULH_Sd_Sn_Em16::make_opcode(insn));
                                            }
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x004000 == 0 {
                                    if insn & 0x008000 == 0 {
                                        if insn & 0x20000000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xff00f400 == 0xf002000 {
                                                    return Some(SMLAL_Vd_Vn_Em16::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            } else {
                                                if insn & 0xff00f400 == 0x4f002000 {
                                                    return Some(SMLAL2_Vd_Vn_Em16::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        } else {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xff00f400 == 0x2f002000 {
                                                    return Some(UMLAL_Vd_Vn_Em16::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            } else {
                                                if insn & 0xff00f400 == 0x6f002000 {
                                                    return Some(UMLAL2_Vd_Vn_Em16::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x20000000 == 0 {
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
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xff00f400 == 0x2f00a000 {
                                                    return Some(UMULL_Vd_Vn_Em16::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            } else {
                                                if insn & 0xff00f400 == 0x6f00a000 {
                                                    return Some(UMULL2_Vd_Vn_Em16::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x20000000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xff00f400 == 0xf006000 {
                                                return Some(SMLSL_Vd_Vn_Em16::make_opcode(insn));
                                            }
                                        } else {
                                            if insn & 0xff00f400 == 0x4f006000 {
                                                return Some(SMLSL2_Vd_Vn_Em16::make_opcode(insn));
                                            }
                                        }
                                    } else {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xff00f400 == 0x2f006000 {
                                                return Some(UMLSL_Vd_Vn_Em16::make_opcode(insn));
                                            }
                                        } else {
                                            if insn & 0xff00f400 == 0x6f006000 {
                                                return Some(UMLSL2_Vd_Vn_Em16::make_opcode(insn));
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            if insn & 0x008000 == 0 {
                                if insn & 0x10000000 == 0 {
                                    if insn & 0x20000000 == 0 {
                                        if insn & 0x002000 == 0 {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0x800000 == 0 {
                                                    if insn & 0xbfc0f400 == 0xf001000 {
                                                        return Some(FMLA_Vd_Vn_Em16::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                } else {
                                                    if insn & 0xbf80f400 == 0xf801000 {
                                                        return Some(FMLA_Vd_Vn_Em::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                }
                                            } else {
                                                if insn & 0x800000 == 0 {
                                                    if insn & 0xbfc0f400 == 0xf005000 {
                                                        return Some(FMLS_Vd_Vn_Em16::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                } else {
                                                    if insn & 0xbf80f400 == 0xf805000 {
                                                        return Some(FMLS_Vd_Vn_Em::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xff00f400 == 0xf003000 {
                                                        return Some(
                                                            SQDMLAL_Vd_Vn_Em16::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xff00f400 == 0x4f003000 {
                                                        return Some(
                                                            SQDMLAL2_Vd_Vn_Em16::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xff00f400 == 0xf007000 {
                                                        return Some(
                                                            SQDMLSL_Vd_Vn_Em16::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xff00f400 == 0x4f007000 {
                                                        return Some(
                                                            SQDMLSL2_Vd_Vn_Em16::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0xbf009400 == 0x2f001000 {
                                            return Some(FCMLA_Vd_Vn_Em_IMM_ROT2::make_opcode(
                                                insn,
                                            ));
                                        }
                                    }
                                } else {
                                    if insn & 0x002000 == 0 {
                                        if insn & 0x004000 == 0 {
                                            if insn & 0x800000 == 0 {
                                                if insn & 0xffc0f400 == 0x5f001000 {
                                                    return Some(FMLA_Sd_Sn_Em16::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            } else {
                                                if insn & 0xff80f400 == 0x5f801000 {
                                                    return Some(FMLA_Sd_Sn_Em::make_opcode(insn));
                                                }
                                            }
                                        } else {
                                            if insn & 0x800000 == 0 {
                                                if insn & 0xffc0f400 == 0x5f005000 {
                                                    return Some(FMLS_Sd_Sn_Em16::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            } else {
                                                if insn & 0xff80f400 == 0x5f805000 {
                                                    return Some(FMLS_Sd_Sn_Em::make_opcode(insn));
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x004000 == 0 {
                                            if insn & 0xff00f400 == 0x5f003000 {
                                                return Some(SQDMLAL_Sd_Sn_Em16::make_opcode(insn));
                                            }
                                        } else {
                                            if insn & 0xff00f400 == 0x5f007000 {
                                                return Some(SQDMLSL_Sd_Sn_Em16::make_opcode(insn));
                                            }
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x002000 == 0 {
                                    if insn & 0x004000 == 0 {
                                        if insn & 0x800000 == 0 {
                                            if insn & 0x10000000 == 0 {
                                                if insn & 0x20000000 == 0 {
                                                    if insn & 0xbfc0f400 == 0xf009000 {
                                                        return Some(FMUL_Vd_Vn_Em16::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                } else {
                                                    if insn & 0xbfc0f400 == 0x2f009000 {
                                                        return Some(
                                                            FMULX_Vd_Vn_Em16::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0x20000000 == 0 {
                                                    if insn & 0xffc0f400 == 0x5f009000 {
                                                        return Some(FMUL_Sd_Sn_Em16::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                } else {
                                                    if insn & 0xffc0f400 == 0x7f009000 {
                                                        return Some(
                                                            FMULX_Sd_Sn_Em16::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x10000000 == 0 {
                                                if insn & 0x20000000 == 0 {
                                                    if insn & 0xbf80f400 == 0xf809000 {
                                                        return Some(FMUL_Vd_Vn_Em::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                } else {
                                                    if insn & 0xbf80f400 == 0x2f809000 {
                                                        return Some(FMULX_Vd_Vn_Em::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                }
                                            } else {
                                                if insn & 0x20000000 == 0 {
                                                    if insn & 0xff80f400 == 0x5f809000 {
                                                        return Some(FMUL_Sd_Sn_Em::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                } else {
                                                    if insn & 0xff80f400 == 0x7f809000 {
                                                        return Some(FMULX_Sd_Sn_Em::make_opcode(
                                                            insn,
                                                        ));
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x10000000 == 0 {
                                            if insn & 0x20000000 == 0 {
                                                if insn & 0xbf00f400 == 0xf00d000 {
                                                    return Some(SQRDMULH_Vd_Vn_Em16::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            } else {
                                                if insn & 0xbf00f400 == 0x2f00d000 {
                                                    return Some(SQRDMLAH_Vd_Vn_Em16::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        } else {
                                            if insn & 0x20000000 == 0 {
                                                if insn & 0xff00f400 == 0x5f00d000 {
                                                    return Some(SQRDMULH_Sd_Sn_Em16::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            } else {
                                                if insn & 0xff00f400 == 0x7f00d000 {
                                                    return Some(SQRDMLAH_Sd_Sn_Em16::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x004000 == 0 {
                                        if insn & 0x10000000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xff00f400 == 0xf00b000 {
                                                    return Some(SQDMULL_Vd_Vn_Em16::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            } else {
                                                if insn & 0xff00f400 == 0x4f00b000 {
                                                    return Some(SQDMULL2_Vd_Vn_Em16::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        } else {
                                            if insn & 0xff00f400 == 0x5f00b000 {
                                                return Some(SQDMULL_Sd_Sn_Em16::make_opcode(insn));
                                            }
                                        }
                                    } else {
                                        if insn & 0x10000000 == 0 {
                                            if insn & 0xbf00f400 == 0x2f00f000 {
                                                return Some(SQRDMLSH_Vd_Vn_Em16::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        } else {
                                            if insn & 0xff00f400 == 0x7f00f000 {
                                                return Some(SQRDMLSH_Sd_Sn_Em16::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        if insn & 0x000800 == 0 {
                            if insn & 0x008000 == 0 {
                                if insn & 0x001000 == 0 {
                                    if insn & 0x10000000 == 0 {
                                        if insn & 0x20000000 == 0 {
                                            if insn & 0xbff89c00 == 0xf000400 {
                                                return Some(MOVI_Vd_SIMD_IMM_SFT::make_opcode(
                                                    insn,
                                                ));
                                            }
                                            if insn & 0xbf80fc00 == 0xf000400 {
                                                return Some(SSHR_Vd_Vn_IMM_VLSR::make_opcode(
                                                    insn,
                                                ));
                                            }
                                            if insn & 0xbf80fc00 == 0xf002400 {
                                                return Some(SRSHR_Vd_Vn_IMM_VLSR::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        } else {
                                            if insn & 0xbff89c00 == 0x2f000400 {
                                                return Some(MVNI_Vd_SIMD_IMM_SFT::make_opcode(
                                                    insn,
                                                ));
                                            }
                                            if insn & 0xbf80fc00 == 0x2f000400 {
                                                return Some(USHR_Vd_Vn_IMM_VLSR::make_opcode(
                                                    insn,
                                                ));
                                            }
                                            if insn & 0xbf80fc00 == 0x2f002400 {
                                                return Some(URSHR_Vd_Vn_IMM_VLSR::make_opcode(
                                                    insn,
                                                ));
                                            }
                                            if insn & 0xbf80fc00 == 0x2f004400 {
                                                return Some(SRI_Vd_Vn_IMM_VLSR::make_opcode(insn));
                                            }
                                            if insn & 0xbf80fc00 == 0x2f006400 {
                                                return Some(SQSHLU_Vd_Vn_IMM_VLSL::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    } else {
                                        if insn & 0x002000 == 0 {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0x20000000 == 0 {
                                                    if insn & 0xff80fc00 == 0x5f000400 {
                                                        return Some(
                                                            SSHR_Sd_Sn_IMM_VLSR::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xff80fc00 == 0x7f000400 {
                                                        return Some(
                                                            USHR_Sd_Sn_IMM_VLSR::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0xff80fc00 == 0x7f004400 {
                                                    return Some(SRI_Sd_Sn_IMM_VLSR::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        } else {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0x20000000 == 0 {
                                                    if insn & 0xff80fc00 == 0x5f002400 {
                                                        return Some(
                                                            SRSHR_Sd_Sn_IMM_VLSR::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xff80fc00 == 0x7f002400 {
                                                        return Some(
                                                            URSHR_Sd_Sn_IMM_VLSR::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0xff80fc00 == 0x7f006400 {
                                                    return Some(
                                                        SQSHLU_Sd_Sn_IMM_VLSL::make_opcode(insn),
                                                    );
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x10000000 == 0 {
                                        if insn & 0x20000000 == 0 {
                                            if insn & 0xbff89c00 == 0xf001400 {
                                                return Some(ORR_Vd_SIMD_IMM_SFT::make_opcode(
                                                    insn,
                                                ));
                                            }
                                            if insn & 0xbf80fc00 == 0xf001400 {
                                                return Some(SSRA_Vd_Vn_IMM_VLSR::make_opcode(
                                                    insn,
                                                ));
                                            }
                                            if insn & 0xbf80fc00 == 0xf003400 {
                                                return Some(SRSRA_Vd_Vn_IMM_VLSR::make_opcode(
                                                    insn,
                                                ));
                                            }
                                            if insn & 0xbf80fc00 == 0xf005400 {
                                                return Some(SHL_Vd_Vn_IMM_VLSL::make_opcode(insn));
                                            }
                                            if insn & 0xbf80fc00 == 0xf007400 {
                                                return Some(SQSHL_Vd_Vn_IMM_VLSL::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        } else {
                                            if insn & 0xbff89c00 == 0x2f001400 {
                                                return Some(BIC_Vd_SIMD_IMM_SFT::make_opcode(
                                                    insn,
                                                ));
                                            }
                                            if insn & 0xbf80fc00 == 0x2f001400 {
                                                return Some(USRA_Vd_Vn_IMM_VLSR::make_opcode(
                                                    insn,
                                                ));
                                            }
                                            if insn & 0xbf80fc00 == 0x2f003400 {
                                                return Some(URSRA_Vd_Vn_IMM_VLSR::make_opcode(
                                                    insn,
                                                ));
                                            }
                                            if insn & 0xbf80fc00 == 0x2f005400 {
                                                return Some(SLI_Vd_Vn_IMM_VLSL::make_opcode(insn));
                                            }
                                            if insn & 0xbf80fc00 == 0x2f007400 {
                                                return Some(UQSHL_Vd_Vn_IMM_VLSL::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    } else {
                                        if insn & 0x002000 == 0 {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0x20000000 == 0 {
                                                    if insn & 0xff80fc00 == 0x5f001400 {
                                                        return Some(
                                                            SSRA_Sd_Sn_IMM_VLSR::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xff80fc00 == 0x7f001400 {
                                                        return Some(
                                                            USRA_Sd_Sn_IMM_VLSR::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0x20000000 == 0 {
                                                    if insn & 0xff80fc00 == 0x5f005400 {
                                                        return Some(
                                                            SHL_Sd_Sn_IMM_VLSL::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xff80fc00 == 0x7f005400 {
                                                        return Some(
                                                            SLI_Sd_Sn_IMM_VLSL::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0x20000000 == 0 {
                                                    if insn & 0xff80fc00 == 0x5f003400 {
                                                        return Some(
                                                            SRSRA_Sd_Sn_IMM_VLSR::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xff80fc00 == 0x7f003400 {
                                                        return Some(
                                                            URSRA_Sd_Sn_IMM_VLSR::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0x20000000 == 0 {
                                                    if insn & 0xff80fc00 == 0x5f007400 {
                                                        return Some(
                                                            SQSHL_Sd_Sn_IMM_VLSL::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xff80fc00 == 0x7f007400 {
                                                        return Some(
                                                            UQSHL_Sd_Sn_IMM_VLSL::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x004000 == 0 {
                                    if insn & 0x001000 == 0 {
                                        if insn & 0x10000000 == 0 {
                                            if insn & 0x20000000 == 0 {
                                                if insn & 0xbff8dc00 == 0xf008400 {
                                                    return Some(
                                                        MOVI_Vd_V_4H_SIMD_IMM_SFT_LSL::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                                if insn & 0xff80fc00 == 0xf008400 {
                                                    return Some(SHRN_Vd_Vn_IMM_VLSR::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                                if insn & 0xff80fc00 == 0x4f008400 {
                                                    return Some(
                                                        SHRN2_Vd_Vn_IMM_VLSR::make_opcode(insn),
                                                    );
                                                }
                                                if insn & 0xff80fc00 == 0xf00a400 {
                                                    return Some(
                                                        SSHLL_Vd_Vn_IMM_VLSL::make_opcode(insn),
                                                    );
                                                }
                                                if insn & 0xff80fc00 == 0x4f00a400 {
                                                    return Some(
                                                        SSHLL2_Vd_Vn_IMM_VLSL::make_opcode(insn),
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
                                                if insn & 0xff80fc00 == 0x2f008400 {
                                                    return Some(
                                                        SQSHRUN_Vd_Vn_IMM_VLSR::make_opcode(insn),
                                                    );
                                                }
                                                if insn & 0xff80fc00 == 0x6f008400 {
                                                    return Some(
                                                        SQSHRUN2_Vd_Vn_IMM_VLSR::make_opcode(insn),
                                                    );
                                                }
                                                if insn & 0xff80fc00 == 0x2f00a400 {
                                                    return Some(
                                                        USHLL_Vd_Vn_IMM_VLSL::make_opcode(insn),
                                                    );
                                                }
                                                if insn & 0xff80fc00 == 0x6f00a400 {
                                                    return Some(
                                                        USHLL2_Vd_Vn_IMM_VLSL::make_opcode(insn),
                                                    );
                                                }
                                            }
                                        } else {
                                            if insn & 0xff80fc00 == 0x7f008400 {
                                                return Some(SQSHRUN_Sd_Sn_IMM_VLSR::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    } else {
                                        if insn & 0x10000000 == 0 {
                                            if insn & 0x20000000 == 0 {
                                                if insn & 0xbff8dc00 == 0xf009400 {
                                                    return Some(
                                                        ORR_Vd_V_4H_SIMD_IMM_SFT_LSL::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                                if insn & 0xff80fc00 == 0xf009400 {
                                                    return Some(
                                                        SQSHRN_Vd_Vn_IMM_VLSR::make_opcode(insn),
                                                    );
                                                }
                                                if insn & 0xff80fc00 == 0x4f009400 {
                                                    return Some(
                                                        SQSHRN2_Vd_Vn_IMM_VLSR::make_opcode(insn),
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
                                                if insn & 0xff80fc00 == 0x2f009400 {
                                                    return Some(
                                                        UQSHRN_Vd_Vn_IMM_VLSR::make_opcode(insn),
                                                    );
                                                }
                                                if insn & 0xff80fc00 == 0x6f009400 {
                                                    return Some(
                                                        UQSHRN2_Vd_Vn_IMM_VLSR::make_opcode(insn),
                                                    );
                                                }
                                            }
                                        } else {
                                            if insn & 0x20000000 == 0 {
                                                if insn & 0xff80fc00 == 0x5f009400 {
                                                    return Some(
                                                        SQSHRN_Sd_Sn_IMM_VLSR::make_opcode(insn),
                                                    );
                                                }
                                            } else {
                                                if insn & 0xff80fc00 == 0x7f009400 {
                                                    return Some(
                                                        UQSHRN_Sd_Sn_IMM_VLSR::make_opcode(insn),
                                                    );
                                                }
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
                                            if insn & 0x10000000 == 0 {
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
                                                    if insn & 0xff90fc00 == 0x5f10e400 {
                                                        return Some(
                                                            SCVTF_Sd_Sn_IMM_VLSR::make_opcode(insn),
                                                        );
                                                    }
                                                    if insn & 0xff80fc00 == 0x5f00e400 {
                                                        return Some (SCVTF_Sd_S_S_Sn_S_S_IMM_VLSR_S_S :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0xff90fc00 == 0x7f10e400 {
                                                        return Some(
                                                            UCVTF_Sd_Sn_IMM_VLSR::make_opcode(insn),
                                                        );
                                                    }
                                                    if insn & 0xff80fc00 == 0x7f00e400 {
                                                        return Some (UCVTF_Sd_S_S_Sn_S_S_IMM_VLSR_S_S :: make_opcode (insn)) ;
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x20000000 == 0 {
                                                if insn & 0xbff8fc00 == 0xf00f400 {
                                                    return Some(FMOV_Vd_SIMD_FPIMM::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            } else {
                                                if insn & 0xfff8fc00 == 0x6f00f400 {
                                                    return Some(
                                                        FMOV_Vd_V_2D_SIMD_FPIMM::make_opcode(insn),
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            if insn & 0x001000 == 0 {
                                if insn & 0x10000000 == 0 {
                                    if insn & 0x20000000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xff80fc00 == 0xf008c00 {
                                                return Some(RSHRN_Vd_Vn_IMM_VLSR::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        } else {
                                            if insn & 0xff80fc00 == 0x4f008c00 {
                                                return Some(RSHRN2_Vd_Vn_IMM_VLSR::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    } else {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xff80fc00 == 0x2f008c00 {
                                                return Some(SQRSHRUN_Vd_Vn_IMM_VLSR::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        } else {
                                            if insn & 0xff80fc00 == 0x6f008c00 {
                                                return Some(
                                                    SQRSHRUN2_Vd_Vn_IMM_VLSR::make_opcode(insn),
                                                );
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0xff80fc00 == 0x7f008c00 {
                                        return Some(SQRSHRUN_Sd_Sn_IMM_VLSR::make_opcode(insn));
                                    }
                                }
                            } else {
                                if insn & 0x002000 == 0 {
                                    if insn & 0x10000000 == 0 {
                                        if insn & 0x20000000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xff80fc00 == 0xf009c00 {
                                                    return Some(
                                                        SQRSHRN_Vd_Vn_IMM_VLSR::make_opcode(insn),
                                                    );
                                                }
                                            } else {
                                                if insn & 0xff80fc00 == 0x4f009c00 {
                                                    return Some(
                                                        SQRSHRN2_Vd_Vn_IMM_VLSR::make_opcode(insn),
                                                    );
                                                }
                                            }
                                        } else {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xff80fc00 == 0x2f009c00 {
                                                    return Some(
                                                        UQRSHRN_Vd_Vn_IMM_VLSR::make_opcode(insn),
                                                    );
                                                }
                                            } else {
                                                if insn & 0xff80fc00 == 0x6f009c00 {
                                                    return Some(
                                                        UQRSHRN2_Vd_Vn_IMM_VLSR::make_opcode(insn),
                                                    );
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x20000000 == 0 {
                                            if insn & 0xff80fc00 == 0x5f009c00 {
                                                return Some(SQRSHRN_Sd_Sn_IMM_VLSR::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        } else {
                                            if insn & 0xff80fc00 == 0x7f009c00 {
                                                return Some(UQRSHRN_Sd_Sn_IMM_VLSR::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x10000000 == 0 {
                                        if insn & 0x20000000 == 0 {
                                            if insn & 0xbff8fc00 == 0xf00fc00 {
                                                return Some(FMOV_Vd_V_4H_SIMD_FPIMM::make_opcode(
                                                    insn,
                                                ));
                                            }
                                            if insn & 0xbf90fc00 == 0xf10fc00 {
                                                return Some(FCVTZS_Vd_Vn_IMM_VLSR::make_opcode(
                                                    insn,
                                                ));
                                            }
                                            if insn & 0xbf80fc00 == 0xf00fc00 {
                                                return Some (FCVTZS_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S :: make_opcode (insn)) ;
                                            }
                                        } else {
                                            if insn & 0xbf90fc00 == 0x2f10fc00 {
                                                return Some(FCVTZU_Vd_Vn_IMM_VLSR::make_opcode(
                                                    insn,
                                                ));
                                            }
                                            if insn & 0xbf80fc00 == 0x2f00fc00 {
                                                return Some (FCVTZU_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S :: make_opcode (insn)) ;
                                            }
                                        }
                                    } else {
                                        if insn & 0x20000000 == 0 {
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
                                        } else {
                                            if insn & 0xff90fc00 == 0x7f10fc00 {
                                                return Some(FCVTZU_Sd_Sn_IMM_VLSR::make_opcode(
                                                    insn,
                                                ));
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
    }
    None
}
