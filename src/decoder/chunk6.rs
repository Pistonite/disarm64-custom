#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ASIMDPERM {
    TRN1_Vd_Vn_Vm(TRN1_Vd_Vn_Vm),
    TRN2_Vd_Vn_Vm(TRN2_Vd_Vn_Vm),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ASIMDSAME {
    ADDP_Vd_Vn_Vm(ADDP_Vd_Vn_Vm),
    ADD_Vd_Vn_Vm(ADD_Vd_Vn_Vm),
    AND_Vd_Vn_Vm(AND_Vd_Vn_Vm),
    BIC_Vd_Vn_Vm(BIC_Vd_Vn_Vm),
    BIF_Vd_Vn_Vm(BIF_Vd_Vn_Vm),
    BIT_Vd_Vn_Vm(BIT_Vd_Vn_Vm),
    CMEQ_Vd_Vn_Vm(CMEQ_Vd_Vn_Vm),
    CMGE_Vd_Vn_Vm(CMGE_Vd_Vn_Vm),
    CMGT_Vd_Vn_Vm(CMGT_Vd_Vn_Vm),
    CMHI_Vd_Vn_Vm(CMHI_Vd_Vn_Vm),
    CMHS_Vd_Vn_Vm(CMHS_Vd_Vn_Vm),
    CMTST_Vd_Vn_Vm(CMTST_Vd_Vn_Vm),
    EOR_Vd_Vn_Vm(EOR_Vd_Vn_Vm),
    FADDP_Vd_V_2S_Vn_V_2S_Vm_V_2S(FADDP_Vd_V_2S_Vn_V_2S_Vm_V_2S),
    FADDP_Vd_Vn_Vm(FADDP_Vd_Vn_Vm),
    FADD_Vd_V_2S_Vn_V_2S_Vm_V_2S(FADD_Vd_V_2S_Vn_V_2S_Vm_V_2S),
    FADD_Vd_Vn_Vm(FADD_Vd_Vn_Vm),
    FCADD_Vd_Vn_Vm_IMM_ROT3(FCADD_Vd_Vn_Vm_IMM_ROT3),
    FCMEQ_Vd_V_2S_Vn_V_2S_Vm_V_2S(FCMEQ_Vd_V_2S_Vn_V_2S_Vm_V_2S),
    FCMEQ_Vd_Vn_Vm(FCMEQ_Vd_Vn_Vm),
    FCMGE_Vd_V_2S_Vn_V_2S_Vm_V_2S(FCMGE_Vd_V_2S_Vn_V_2S_Vm_V_2S),
    FCMGE_Vd_Vn_Vm(FCMGE_Vd_Vn_Vm),
    FCMGT_Vd_V_2S_Vn_V_2S_Vm_V_2S(FCMGT_Vd_V_2S_Vn_V_2S_Vm_V_2S),
    FCMGT_Vd_Vn_Vm(FCMGT_Vd_Vn_Vm),
    FCMLA_Vd_Vn_Vm_IMM_ROT1(FCMLA_Vd_Vn_Vm_IMM_ROT1),
    FDIV_Vd_V_2S_Vn_V_2S_Vm_V_2S(FDIV_Vd_V_2S_Vn_V_2S_Vm_V_2S),
    FDIV_Vd_Vn_Vm(FDIV_Vd_Vn_Vm),
    FMULX_Vd_V_2S_Vn_V_2S_Vm_V_2S(FMULX_Vd_V_2S_Vn_V_2S_Vm_V_2S),
    FMULX_Vd_Vn_Vm(FMULX_Vd_Vn_Vm),
    FMUL_Vd_V_2S_Vn_V_2S_Vm_V_2S(FMUL_Vd_V_2S_Vn_V_2S_Vm_V_2S),
    FMUL_Vd_Vn_Vm(FMUL_Vd_Vn_Vm),
    FSUB_Vd_V_2S_Vn_V_2S_Vm_V_2S(FSUB_Vd_V_2S_Vn_V_2S_Vm_V_2S),
    FSUB_Vd_Vn_Vm(FSUB_Vd_Vn_Vm),
    MUL_Vd_Vn_Vm(MUL_Vd_Vn_Vm),
    ORN_Vd_Vn_Vm(ORN_Vd_Vn_Vm),
    ORR_Vd_Vn_Vm(ORR_Vd_Vn_Vm),
    SUB_Vd_Vn_Vm(SUB_Vd_Vn_Vm),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ASIMDSHF {
    FCVTZS_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S(FCVTZS_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S),
    FCVTZS_Vd_Vn_IMM_VLSR(FCVTZS_Vd_Vn_IMM_VLSR),
    FCVTZU_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S(FCVTZU_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S),
    FCVTZU_Vd_Vn_IMM_VLSR(FCVTZU_Vd_Vn_IMM_VLSR),
    SCVTF_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S(SCVTF_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S),
    SCVTF_Vd_Vn_IMM_VLSR(SCVTF_Vd_Vn_IMM_VLSR),
    UCVTF_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S(UCVTF_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S),
    UCVTF_Vd_Vn_IMM_VLSR(UCVTF_Vd_Vn_IMM_VLSR),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ASIMDTBL {
    TBL_Vd_LVn_Vm(TBL_Vd_LVn_Vm),
    TBX_Vd_LVn_Vm(TBX_Vd_LVn_Vm),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ASISDELEM {
    FMULX_Sd_Sn_Em(FMULX_Sd_Sn_Em),
    FMULX_Sd_Sn_Em16(FMULX_Sd_Sn_Em16),
    FMUL_Sd_Sn_Em(FMUL_Sd_Sn_Em),
    FMUL_Sd_Sn_Em16(FMUL_Sd_Sn_Em16),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ASISDMISC {
    CMEQ_Sd_Sn_IMM0(CMEQ_Sd_Sn_IMM0),
    CMGE_Sd_Sn_IMM0(CMGE_Sd_Sn_IMM0),
    CMGT_Sd_Sn_IMM0(CMGT_Sd_Sn_IMM0),
    CMLE_Sd_Sn_IMM0(CMLE_Sd_Sn_IMM0),
    CMLT_Sd_Sn_IMM0(CMLT_Sd_Sn_IMM0),
    FCMEQ_Sd_S_H_Sn_S_H_FPIMM0(FCMEQ_Sd_S_H_Sn_S_H_FPIMM0),
    FCMEQ_Sd_Sn_FPIMM0(FCMEQ_Sd_Sn_FPIMM0),
    FCMGE_Sd_S_H_Sn_S_H_FPIMM0(FCMGE_Sd_S_H_Sn_S_H_FPIMM0),
    FCMGE_Sd_Sn_FPIMM0(FCMGE_Sd_Sn_FPIMM0),
    FCMGT_Sd_S_H_Sn_S_H_FPIMM0(FCMGT_Sd_S_H_Sn_S_H_FPIMM0),
    FCMGT_Sd_Sn_FPIMM0(FCMGT_Sd_Sn_FPIMM0),
    FCMLE_Sd_S_H_Sn_S_H_FPIMM0(FCMLE_Sd_S_H_Sn_S_H_FPIMM0),
    FCMLE_Sd_Sn_FPIMM0(FCMLE_Sd_Sn_FPIMM0),
    FCMLT_Sd_S_H_Sn_S_H_FPIMM0(FCMLT_Sd_S_H_Sn_S_H_FPIMM0),
    FCMLT_Sd_Sn_FPIMM0(FCMLT_Sd_Sn_FPIMM0),
    FCVTAS_Sd_S_H_Sn_S_H(FCVTAS_Sd_S_H_Sn_S_H),
    FCVTAS_Sd_Sn(FCVTAS_Sd_Sn),
    FCVTAU_Sd_S_H_Sn_S_H(FCVTAU_Sd_S_H_Sn_S_H),
    FCVTAU_Sd_Sn(FCVTAU_Sd_Sn),
    FCVTMS_Sd_S_H_Sn_S_H(FCVTMS_Sd_S_H_Sn_S_H),
    FCVTMS_Sd_Sn(FCVTMS_Sd_Sn),
    FCVTMU_Sd_S_H_Sn_S_H(FCVTMU_Sd_S_H_Sn_S_H),
    FCVTMU_Sd_Sn(FCVTMU_Sd_Sn),
    FCVTNS_Sd_S_H_Sn_S_H(FCVTNS_Sd_S_H_Sn_S_H),
    FCVTNS_Sd_Sn(FCVTNS_Sd_Sn),
    FCVTNU_Sd_S_H_Sn_S_H(FCVTNU_Sd_S_H_Sn_S_H),
    FCVTNU_Sd_Sn(FCVTNU_Sd_Sn),
    FCVTPS_Sd_S_H_Sn_S_H(FCVTPS_Sd_S_H_Sn_S_H),
    FCVTPS_Sd_Sn(FCVTPS_Sd_Sn),
    FCVTPU_Sd_S_H_Sn_S_H(FCVTPU_Sd_S_H_Sn_S_H),
    FCVTPU_Sd_Sn(FCVTPU_Sd_Sn),
    FCVTXN_Sd_Sn(FCVTXN_Sd_Sn),
    FCVTZS_Sd_S_H_Sn_S_H(FCVTZS_Sd_S_H_Sn_S_H),
    FCVTZS_Sd_Sn(FCVTZS_Sd_Sn),
    FCVTZU_Sd_S_H_Sn_S_H(FCVTZU_Sd_S_H_Sn_S_H),
    FCVTZU_Sd_Sn(FCVTZU_Sd_Sn),
    NEG_Sd_Sn(NEG_Sd_Sn),
    SCVTF_Sd_S_H_Sn_S_H(SCVTF_Sd_S_H_Sn_S_H),
    SCVTF_Sd_Sn(SCVTF_Sd_Sn),
    UCVTF_Sd_S_H_Sn_S_H(UCVTF_Sd_S_H_Sn_S_H),
    UCVTF_Sd_Sn(UCVTF_Sd_Sn),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ASISDPAIR {
    ADDP_Sd_Vn(ADDP_Sd_Vn),
    FADDP_Sd_S_S_Vn_V_2S(FADDP_Sd_S_S_Vn_V_2S),
    FADDP_Sd_Vn(FADDP_Sd_Vn),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ASISDSAME {
    ADD_Sd_Sn_Sm(ADD_Sd_Sn_Sm),
    CMEQ_Sd_Sn_Sm(CMEQ_Sd_Sn_Sm),
    CMGE_Sd_Sn_Sm(CMGE_Sd_Sn_Sm),
    CMGT_Sd_Sn_Sm(CMGT_Sd_Sn_Sm),
    CMHI_Sd_Sn_Sm(CMHI_Sd_Sn_Sm),
    CMHS_Sd_Sn_Sm(CMHS_Sd_Sn_Sm),
    CMTST_Sd_Sn_Sm(CMTST_Sd_Sn_Sm),
    FCMEQ_Sd_S_S_Sn_S_S_Sm_S_S(FCMEQ_Sd_S_S_Sn_S_S_Sm_S_S),
    FCMEQ_Sd_Sn_Sm(FCMEQ_Sd_Sn_Sm),
    FCMGE_Sd_S_S_Sn_S_S_Sm_S_S(FCMGE_Sd_S_S_Sn_S_S_Sm_S_S),
    FCMGE_Sd_Sn_Sm(FCMGE_Sd_Sn_Sm),
    FCMGT_Sd_S_S_Sn_S_S_Sm_S_S(FCMGT_Sd_S_S_Sn_S_S_Sm_S_S),
    FCMGT_Sd_Sn_Sm(FCMGT_Sd_Sn_Sm),
    FMULX_Sd_S_S_Sn_S_S_Sm_S_S(FMULX_Sd_S_S_Sn_S_S_Sm_S_S),
    FMULX_Sd_Sn_Sm(FMULX_Sd_Sn_Sm),
    SUB_Sd_Sn_Sm(SUB_Sd_Sn_Sm),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ASISDSHF {
    FCVTZS_Sd_S_S_Sn_S_S_IMM_VLSR_S_S(FCVTZS_Sd_S_S_Sn_S_S_IMM_VLSR_S_S),
    FCVTZS_Sd_Sn_IMM_VLSR(FCVTZS_Sd_Sn_IMM_VLSR),
    FCVTZU_Sd_S_S_Sn_S_S_IMM_VLSR_S_S(FCVTZU_Sd_S_S_Sn_S_S_IMM_VLSR_S_S),
    FCVTZU_Sd_Sn_IMM_VLSR(FCVTZU_Sd_Sn_IMM_VLSR),
    SCVTF_Sd_S_S_Sn_S_S_IMM_VLSR_S_S(SCVTF_Sd_S_S_Sn_S_S_IMM_VLSR_S_S),
    SCVTF_Sd_Sn_IMM_VLSR(SCVTF_Sd_Sn_IMM_VLSR),
    UCVTF_Sd_S_S_Sn_S_S_IMM_VLSR_S_S(UCVTF_Sd_S_S_Sn_S_S_IMM_VLSR_S_S),
    UCVTF_Sd_Sn_IMM_VLSR(UCVTF_Sd_Sn_IMM_VLSR),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum BFLOAT16 {
    BFCVTN2_Vd_Vn(BFCVTN2_Vd_Vn),
    BFCVTN_Vd_Vn(BFCVTN_Vd_Vn),
    BFCVT_Fd_Fn(BFCVT_Fd_Fn),
    BFDOT_Vd_Vn_Vm(BFDOT_Vd_Vn_Vm),
    BFMLALB_Vd_Vn_Em16(BFMLALB_Vd_Vn_Em16),
    BFMLALB_Vd_Vn_Vm(BFMLALB_Vd_Vn_Vm),
    BFMLALT_Vd_Vn_Em16(BFMLALT_Vd_Vn_Em16),
    BFMLALT_Vd_Vn_Vm(BFMLALT_Vd_Vn_Vm),
    BFMMLA_Vd_Vn_Vm(BFMMLA_Vd_Vn_Vm),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum BITFIELD {
    BFM_Rd_Rn_IMMR_IMMS(BFM_Rd_Rn_IMMR_IMMS),
    SBFM_Rd_Rn_IMMR_IMMS(SBFM_Rd_Rn_IMMR_IMMS),
    UBFM_Rd_Rn_IMMR_IMMS(UBFM_Rd_Rn_IMMR_IMMS),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum BRANCH_IMM {
    BL_ADDR_PCREL26(BL_ADDR_PCREL26),
    B_ADDR_PCREL26(B_ADDR_PCREL26),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum BRANCH_REG {
    BLRAAZ_Rn(BLRAAZ_Rn),
    BLRAA_Rn_Rd_SP(BLRAA_Rn_Rd_SP),
    BLRABZ_Rn(BLRABZ_Rn),
    BLRAB_Rn_Rd_SP(BLRAB_Rn_Rd_SP),
    BLR_Rn(BLR_Rn),
    BRAAZ_Rn(BRAAZ_Rn),
    BRAA_Rn_Rd_SP(BRAA_Rn_Rd_SP),
    BRABZ_Rn(BRABZ_Rn),
    BRAB_Rn_Rd_SP(BRAB_Rn_Rd_SP),
    BR_Rn(BR_Rn),
    RETAA(RETAA),
    RETAB(RETAB),
    RET_Rn(RET_Rn),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum COMPBRANCH {
    CBNZ_Rt_ADDR_PCREL19(CBNZ_Rt_ADDR_PCREL19),
    CBZ_Rt_ADDR_PCREL19(CBZ_Rt_ADDR_PCREL19),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum CONDCMP_IMM {
    CCMN_Rn_CCMP_IMM_NZCV_COND(CCMN_Rn_CCMP_IMM_NZCV_COND),
    CCMP_Rn_CCMP_IMM_NZCV_COND(CCMP_Rn_CCMP_IMM_NZCV_COND),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum CONDCMP_REG {
    CCMN_Rn_Rm_NZCV_COND(CCMN_Rn_Rm_NZCV_COND),
    CCMP_Rn_Rm_NZCV_COND(CCMP_Rn_Rm_NZCV_COND),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum CONDSEL {
    CSEL_Rd_Rn_Rm_COND(CSEL_Rd_Rn_Rm_COND),
    CSINC_Rd_Rn_Rm_COND(CSINC_Rd_Rn_Rm_COND),
    CSINV_Rd_Rn_Rm_COND(CSINV_Rd_Rn_Rm_COND),
    CSNEG_Rd_Rn_Rm_COND(CSNEG_Rd_Rn_Rm_COND),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum CRYPTOSHA3 {
    EOR3_Vd_Vn_Vm_Va(EOR3_Vd_Vn_Vm_Va),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum CSSC {
    CNT_Rd_Rn(CNT_Rd_Rn),
    CTZ_Rd_Rn(CTZ_Rd_Rn),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum DOTPRODUCT {
    BFDOT_Vd_Vn_Em(BFDOT_Vd_Vn_Em),
    SDOT_Vd_Vn_Em(SDOT_Vd_Vn_Em),
    SDOT_Vd_Vn_Vm(SDOT_Vd_Vn_Vm),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum DP_1SRC {
    CLS_Rd_Rn(CLS_Rd_Rn),
    CLZ_Rd_Rn(CLZ_Rd_Rn),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum DP_2SRC {
    ASRV_Rd_Rn_Rm(ASRV_Rd_Rn_Rm),
    LSLV_Rd_Rn_Rm(LSLV_Rd_Rn_Rm),
    LSRV_Rd_Rn_Rm(LSRV_Rd_Rn_Rm),
    SDIV_Rd_Rn_Rm(SDIV_Rd_Rn_Rm),
    SUBPS_Rd_Rn_SP_Rm_SP(SUBPS_Rd_Rn_SP_Rm_SP),
    SUBP_Rd_Rn_SP_Rm_SP(SUBP_Rd_Rn_SP_Rm_SP),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum DP_3SRC {
    MADD_Rd_Rn_Rm_Ra(MADD_Rd_Rn_Rm_Ra),
    MSUB_Rd_Rn_Rm_Ra(MSUB_Rd_Rn_Rm_Ra),
    SMADDL_Rd_Rn_Rm_Ra(SMADDL_Rd_Rn_Rm_Ra),
    SMULH_Rd_Rn_Rm(SMULH_Rd_Rn_Rm),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum EXCEPTION {
    BRK_EXCEPTION(BRK_EXCEPTION),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum FLOAT2FIX {
    FCVTZS_Rd_Fn_FBITS(FCVTZS_Rd_Fn_FBITS),
    FCVTZS_Rd_W_Fn_S_D_FBITS_imm_1_32(FCVTZS_Rd_W_Fn_S_D_FBITS_imm_1_32),
    FCVTZU_Rd_Fn_FBITS(FCVTZU_Rd_Fn_FBITS),
    FCVTZU_Rd_W_Fn_S_D_FBITS_imm_1_32(FCVTZU_Rd_W_Fn_S_D_FBITS_imm_1_32),
    SCVTF_Fd_Rn_FBITS(SCVTF_Fd_Rn_FBITS),
    SCVTF_Fd_S_D_Rn_W_FBITS_imm_1_32(SCVTF_Fd_S_D_Rn_W_FBITS_imm_1_32),
    UCVTF_Fd_Rn_FBITS(UCVTF_Fd_Rn_FBITS),
    UCVTF_Fd_S_D_Rn_W_FBITS_imm_1_32(UCVTF_Fd_S_D_Rn_W_FBITS_imm_1_32),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum FLOAT2INT {
    FCVTAS_Rd_Fn(FCVTAS_Rd_Fn),
    FCVTAS_Rd_W_Fn_S_D(FCVTAS_Rd_W_Fn_S_D),
    FCVTAU_Rd_Fn(FCVTAU_Rd_Fn),
    FCVTAU_Rd_W_Fn_S_D(FCVTAU_Rd_W_Fn_S_D),
    FCVTMS_Rd_Fn(FCVTMS_Rd_Fn),
    FCVTMS_Rd_W_Fn_S_D(FCVTMS_Rd_W_Fn_S_D),
    FCVTMU_Rd_Fn(FCVTMU_Rd_Fn),
    FCVTMU_Rd_W_Fn_S_D(FCVTMU_Rd_W_Fn_S_D),
    FCVTNS_Rd_Fn(FCVTNS_Rd_Fn),
    FCVTNS_Rd_W_Fn_S_D(FCVTNS_Rd_W_Fn_S_D),
    FCVTNU_Rd_Fn(FCVTNU_Rd_Fn),
    FCVTNU_Rd_W_Fn_S_D(FCVTNU_Rd_W_Fn_S_D),
    FCVTPS_Rd_Fn(FCVTPS_Rd_Fn),
    FCVTPS_Rd_W_Fn_S_D(FCVTPS_Rd_W_Fn_S_D),
    FCVTPU_Rd_Fn(FCVTPU_Rd_Fn),
    FCVTPU_Rd_W_Fn_S_D(FCVTPU_Rd_W_Fn_S_D),
    FCVTZS_Rd_Fn(FCVTZS_Rd_Fn),
    FCVTZS_Rd_W_Fn_S_D(FCVTZS_Rd_W_Fn_S_D),
    FCVTZU_Rd_Fn(FCVTZU_Rd_Fn),
    FCVTZU_Rd_W_Fn_S_D(FCVTZU_Rd_W_Fn_S_D),
    FJCVTZS_Rd_Fn(FJCVTZS_Rd_Fn),
    FMOV_Fd_Rn(FMOV_Fd_Rn),
    FMOV_Fd_S_S_Rn_W(FMOV_Fd_S_S_Rn_W),
    FMOV_Rd_Fn(FMOV_Rd_Fn),
    FMOV_Rd_VnD1(FMOV_Rd_VnD1),
    FMOV_Rd_W_Fn_S_S(FMOV_Rd_W_Fn_S_S),
    FMOV_VdD1_Rn(FMOV_VdD1_Rn),
    SCVTF_Fd_Rn(SCVTF_Fd_Rn),
    SCVTF_Fd_S_D_Rn_W(SCVTF_Fd_S_D_Rn_W),
    UCVTF_Fd_Rn(UCVTF_Fd_Rn),
    UCVTF_Fd_S_D_Rn_W(UCVTF_Fd_S_D_Rn_W),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum FLOATCCMP {
    FCCMPE_Fn_Fm_NZCV_COND(FCCMPE_Fn_Fm_NZCV_COND),
    FCCMPE_Fn_S_S_Fm_S_S_NZCV_COND(FCCMPE_Fn_S_S_Fm_S_S_NZCV_COND),
    FCCMP_Fn_Fm_NZCV_COND(FCCMP_Fn_Fm_NZCV_COND),
    FCCMP_Fn_S_S_Fm_S_S_NZCV_COND(FCCMP_Fn_S_S_Fm_S_S_NZCV_COND),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum FLOATCMP {
    FCMPE_Fn_FPIMM0(FCMPE_Fn_FPIMM0),
    FCMPE_Fn_Fm(FCMPE_Fn_Fm),
    FCMPE_Fn_S_S_FPIMM0(FCMPE_Fn_S_S_FPIMM0),
    FCMPE_Fn_S_S_Fm_S_S(FCMPE_Fn_S_S_Fm_S_S),
    FCMP_Fn_FPIMM0(FCMP_Fn_FPIMM0),
    FCMP_Fn_Fm(FCMP_Fn_Fm),
    FCMP_Fn_S_S_FPIMM0(FCMP_Fn_S_S_FPIMM0),
    FCMP_Fn_S_S_Fm_S_S(FCMP_Fn_S_S_Fm_S_S),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum FLOATDP1 {
    FCVT_Fd_Fn(FCVT_Fd_Fn),
    FMOV_Fd_Fn(FMOV_Fd_Fn),
    FMOV_Fd_S_S_Fn_S_S(FMOV_Fd_S_S_Fn_S_S),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum FLOATDP2 {
    FADD_Fd_Fn_Fm(FADD_Fd_Fn_Fm),
    FADD_Fd_S_S_Fn_S_S_Fm_S_S(FADD_Fd_S_S_Fn_S_S_Fm_S_S),
    FDIV_Fd_Fn_Fm(FDIV_Fd_Fn_Fm),
    FDIV_Fd_S_S_Fn_S_S_Fm_S_S(FDIV_Fd_S_S_Fn_S_S_Fm_S_S),
    FMUL_Fd_Fn_Fm(FMUL_Fd_Fn_Fm),
    FMUL_Fd_S_S_Fn_S_S_Fm_S_S(FMUL_Fd_S_S_Fn_S_S_Fm_S_S),
    FSUB_Fd_Fn_Fm(FSUB_Fd_Fn_Fm),
    FSUB_Fd_S_S_Fn_S_S_Fm_S_S(FSUB_Fd_S_S_Fn_S_S_Fm_S_S),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum FLOATDP3 {
    FMSUB_Fd_Fn_Fm_Fa(FMSUB_Fd_Fn_Fm_Fa),
    FMSUB_Fd_S_S_Fn_S_S_Fm_S_S_Fa_S_S(FMSUB_Fd_S_S_Fn_S_S_Fm_S_S_Fa_S_S),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum FLOATIMM {
    FMOV_Fd_FPIMM(FMOV_Fd_FPIMM),
    FMOV_Fd_S_S_FPIMM(FMOV_Fd_S_S_FPIMM),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum FLOATSEL {
    FCSEL_Fd_Fn_Fm_COND(FCSEL_Fd_Fn_Fm_COND),
    FCSEL_Fd_S_S_Fn_S_S_Fm_S_S_COND(FCSEL_Fd_S_S_Fn_S_S_Fm_S_S_COND),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum IC_SYSTEM {
    CFINV(CFINV),
    CHKFEAT_X16(CHKFEAT_X16),
    CLREX_UIMM4(CLREX_UIMM4),
    SB(SB),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LDSTEXCL {
    STXP_Rs_Rt_Rt2_ADDR_SIMPLE(STXP_Rs_Rt_Rt2_ADDR_SIMPLE),
    STXRB_Rs_Rt_ADDR_SIMPLE(STXRB_Rs_Rt_ADDR_SIMPLE),
    STXRH_Rs_Rt_ADDR_SIMPLE(STXRH_Rs_Rt_ADDR_SIMPLE),
    STXR_Rs_Rt_ADDR_SIMPLE(STXR_Rs_Rt_ADDR_SIMPLE),
    STZGM_Rt_ADDR_SIMPLE(STZGM_Rt_ADDR_SIMPLE),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LDSTPAIR_INDEXED {
    LDPSW_Rt_X_Rt2_X_ADDR_SIMM7_S_S(LDPSW_Rt_X_Rt2_X_ADDR_SIMM7_S_S),
    LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S(LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S),
    LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S(LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S),
    STP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S(STP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S),
    STP_Rt_W_Rt2_W_ADDR_SIMM7_S_S(STP_Rt_W_Rt2_W_ADDR_SIMM7_S_S),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LDSTPAIR_OFF {
    LDPSW_Rt_Rt2_ADDR_SIMM7(LDPSW_Rt_Rt2_ADDR_SIMM7),
    LDP_Ft_Ft2_ADDR_SIMM7(LDP_Ft_Ft2_ADDR_SIMM7),
    LDP_Rt_Rt2_ADDR_SIMM7(LDP_Rt_Rt2_ADDR_SIMM7),
    STP_Ft_Ft2_ADDR_SIMM7(STP_Ft_Ft2_ADDR_SIMM7),
    STP_Rt_Rt2_ADDR_SIMM7(STP_Rt_Rt2_ADDR_SIMM7),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LDST_IMM10 {
    LDRAA_Rt_ADDR_SIMM10(LDRAA_Rt_ADDR_SIMM10),
    LDRAB_Rt_ADDR_SIMM10(LDRAB_Rt_ADDR_SIMM10),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LDST_IMM9 {
    LDRB_Rt_ADDR_SIMM9(LDRB_Rt_ADDR_SIMM9),
    LDRH_Rt_ADDR_SIMM9(LDRH_Rt_ADDR_SIMM9),
    LDRSB_Rt_ADDR_SIMM9(LDRSB_Rt_ADDR_SIMM9),
    LDRSH_Rt_ADDR_SIMM9(LDRSH_Rt_ADDR_SIMM9),
    LDRSW_Rt_ADDR_SIMM9(LDRSW_Rt_ADDR_SIMM9),
    LDR_Ft_ADDR_SIMM9(LDR_Ft_ADDR_SIMM9),
    LDR_Rt_ADDR_SIMM9(LDR_Rt_ADDR_SIMM9),
    STRB_Rt_ADDR_SIMM9(STRB_Rt_ADDR_SIMM9),
    STRH_Rt_ADDR_SIMM9(STRH_Rt_ADDR_SIMM9),
    STR_Ft_ADDR_SIMM9(STR_Ft_ADDR_SIMM9),
    STR_Rt_ADDR_SIMM9(STR_Rt_ADDR_SIMM9),
    STZ2G_Rt_SP_X_ADDR_SIMM13_imm_tag(STZ2G_Rt_SP_X_ADDR_SIMM13_imm_tag),
    STZG_Rt_SP_X_ADDR_SIMM13_imm_tag(STZG_Rt_SP_X_ADDR_SIMM13_imm_tag),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LDST_POS {
    LDRB_Rt_ADDR_UIMM12(LDRB_Rt_ADDR_UIMM12),
    LDRH_Rt_ADDR_UIMM12(LDRH_Rt_ADDR_UIMM12),
    LDRSB_Rt_ADDR_UIMM12(LDRSB_Rt_ADDR_UIMM12),
    LDRSH_Rt_ADDR_UIMM12(LDRSH_Rt_ADDR_UIMM12),
    LDRSW_Rt_ADDR_UIMM12(LDRSW_Rt_ADDR_UIMM12),
    LDR_Ft_ADDR_UIMM12(LDR_Ft_ADDR_UIMM12),
    LDR_Rt_ADDR_UIMM12(LDR_Rt_ADDR_UIMM12),
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
    STRB_Rt_ADDR_REGOFF(STRB_Rt_ADDR_REGOFF),
    STRH_Rt_ADDR_REGOFF(STRH_Rt_ADDR_REGOFF),
    STR_Ft_ADDR_REGOFF(STR_Ft_ADDR_REGOFF),
    STR_Rt_ADDR_REGOFF(STR_Rt_ADDR_REGOFF),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LDST_UNPRIV {
    STTRB_Rt_ADDR_SIMM9(STTRB_Rt_ADDR_SIMM9),
    STTRH_Rt_ADDR_SIMM9(STTRH_Rt_ADDR_SIMM9),
    STTR_Rt_ADDR_SIMM9(STTR_Rt_ADDR_SIMM9),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LDST_UNSCALED {
    LDURB_Rt_ADDR_SIMM9(LDURB_Rt_ADDR_SIMM9),
    LDURH_Rt_ADDR_SIMM9(LDURH_Rt_ADDR_SIMM9),
    LDURSB_Rt_ADDR_SIMM9(LDURSB_Rt_ADDR_SIMM9),
    LDURSH_Rt_ADDR_SIMM9(LDURSH_Rt_ADDR_SIMM9),
    LDURSW_Rt_ADDR_SIMM9(LDURSW_Rt_ADDR_SIMM9),
    LDUR_Ft_ADDR_SIMM9(LDUR_Ft_ADDR_SIMM9),
    LDUR_Rt_ADDR_SIMM9(LDUR_Rt_ADDR_SIMM9),
    STURB_Rt_ADDR_SIMM9(STURB_Rt_ADDR_SIMM9),
    STURH_Rt_ADDR_SIMM9(STURH_Rt_ADDR_SIMM9),
    STUR_Ft_ADDR_SIMM9(STUR_Ft_ADDR_SIMM9),
    STUR_Rt_ADDR_SIMM9(STUR_Rt_ADDR_SIMM9),
    STZ2G_Rt_SP_ADDR_SIMM13(STZ2G_Rt_SP_ADDR_SIMM13),
    STZG_Rt_SP_ADDR_SIMM13(STZG_Rt_SP_ADDR_SIMM13),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LOADLIT {
    LDRSW_Rt_ADDR_PCREL19(LDRSW_Rt_ADDR_PCREL19),
    LDR_Ft_ADDR_PCREL19(LDR_Ft_ADDR_PCREL19),
    LDR_Rt_ADDR_PCREL19(LDR_Rt_ADDR_PCREL19),
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
pub enum SME2_MOV {
    MOVA_SME_ZA_array_off3_0_SME_Znx2(MOVA_SME_ZA_array_off3_0_SME_Znx2),
    MOVA_SME_ZA_array_off3_0_SME_Znx4(MOVA_SME_ZA_array_off3_0_SME_Znx4),
    MOVA_SME_Zdnx2_SME_ZA_array_off3_5(MOVA_SME_Zdnx2_SME_ZA_array_off3_5),
    MOVA_SME_Zdnx4_SME_ZA_array_off3_5(MOVA_SME_Zdnx4_SME_ZA_array_off3_5),
    MOV_SME_ZA_array_off3_0_SME_Znx2(MOV_SME_ZA_array_off3_0_SME_Znx2),
    MOV_SME_ZA_array_off3_0_SME_Znx4(MOV_SME_ZA_array_off3_0_SME_Znx4),
    MOV_SME_Zdnx2_SME_ZA_array_off3_5(MOV_SME_Zdnx2_SME_ZA_array_off3_5),
    MOV_SME_Zdnx4_SME_ZA_array_off3_5(MOV_SME_Zdnx4_SME_ZA_array_off3_5),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SME2_MOVAZ {
    MOVAZ_SME_Zdnx2_SME_ZA_array_vrsb_1(MOVAZ_SME_Zdnx2_SME_ZA_array_vrsb_1),
    MOVAZ_SME_Zdnx2_SME_ZA_array_vrsd_1(MOVAZ_SME_Zdnx2_SME_ZA_array_vrsd_1),
    MOVAZ_SME_Zdnx2_SME_ZA_array_vrsh_1(MOVAZ_SME_Zdnx2_SME_ZA_array_vrsh_1),
    MOVAZ_SME_Zdnx2_SME_ZA_array_vrss_1(MOVAZ_SME_Zdnx2_SME_ZA_array_vrss_1),
    MOVAZ_SME_Zdnx4_SME_ZA_array_vrsb_2(MOVAZ_SME_Zdnx4_SME_ZA_array_vrsb_2),
    MOVAZ_SME_Zdnx4_SME_ZA_array_vrsd_2(MOVAZ_SME_Zdnx4_SME_ZA_array_vrsd_2),
    MOVAZ_SME_Zdnx4_SME_ZA_array_vrsh_2(MOVAZ_SME_Zdnx4_SME_ZA_array_vrsh_2),
    MOVAZ_SME_Zdnx4_SME_ZA_array_vrss_2(MOVAZ_SME_Zdnx4_SME_ZA_array_vrss_2),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SME_FP_SD {
    FADD_SME_ZA_array_off3_0_SME_Znx2(FADD_SME_ZA_array_off3_0_SME_Znx2),
    FADD_SME_ZA_array_off3_0_SME_Znx4(FADD_SME_ZA_array_off3_0_SME_Znx4),
    FSUB_SME_ZA_array_off3_0_SME_Znx2(FSUB_SME_ZA_array_off3_0_SME_Znx2),
    FSUB_SME_ZA_array_off3_0_SME_Znx4(FSUB_SME_ZA_array_off3_0_SME_Znx4),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SME_INT_SD {
    ADD_SME_ZA_array_off3_0_SME_Znx2(ADD_SME_ZA_array_off3_0_SME_Znx2),
    ADD_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2(ADD_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2),
    ADD_SME_ZA_array_off3_0_SME_Znx4(ADD_SME_ZA_array_off3_0_SME_Znx4),
    ADD_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4(ADD_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4),
    ADD_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm(ADD_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm),
    ADD_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_S_SME_Zm_S_S(
        ADD_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_S_SME_Zm_S_S,
    ),
    SDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2(SDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2),
    SDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4(SDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4),
    SDOT_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm(SDOT_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm),
    SDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_B_SME_Zm_S_B(
        SDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_B_SME_Zm_S_B,
    ),
    SUB_SME_ZA_array_off3_0_SME_Znx2(SUB_SME_ZA_array_off3_0_SME_Znx2),
    SUB_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2(SUB_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2),
    SUB_SME_ZA_array_off3_0_SME_Znx4(SUB_SME_ZA_array_off3_0_SME_Znx4),
    SUB_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4(SUB_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4),
    SUB_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm(SUB_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm),
    SUB_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_S_SME_Zm_S_S(
        SUB_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_S_SME_Zm_S_S,
    ),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SME_LDR {
    LDR_SME_ZA_array_off4_SME_ADDR_RI_U4xVL(LDR_SME_ZA_array_off4_SME_ADDR_RI_U4xVL),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SME_MISC {
    ADDHA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn(ADDHA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn),
    ADDHA_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn(ADDHA_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn),
    ADDSPL_Rd_SP_SVE_Rn_SP_SVE_SIMM6(ADDSPL_Rd_SP_SVE_Rn_SP_SVE_SIMM6),
    ADDSVL_Rd_SP_SVE_Rn_SP_SVE_SIMM6(ADDSVL_Rd_SP_SVE_Rn_SP_SVE_SIMM6),
    ADDVA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn(ADDVA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn),
    ADDVA_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn(ADDVA_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn),
    BFCVTN_SVE_Zd_SME_Znx2(BFCVTN_SVE_Zd_SME_Znx2),
    BFCVT_SVE_Zd_SME_Znx2(BFCVT_SVE_Zd_SME_Znx2),
    BFDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2(
        BFDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2,
    ),
    BFDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2(BFDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2),
    BFDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX2(
        BFDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX2,
    ),
    BFDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4(BFDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4),
    BFDOT_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm(BFDOT_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm),
    BFDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H(
        BFDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H,
    ),
    BFMLAL_SME_ZA_array_off2x2_SME_Znx2_SME_Zm_INDEX3_2(
        BFMLAL_SME_ZA_array_off2x2_SME_Znx2_SME_Zm_INDEX3_2,
    ),
    BFMLAL_SME_ZA_array_off2x2_SME_Znx2_SME_Zmx2(BFMLAL_SME_ZA_array_off2x2_SME_Znx2_SME_Zmx2),
    BFMLAL_SME_ZA_array_off2x2_SME_Znx4_SME_Zm_INDEX3_2(
        BFMLAL_SME_ZA_array_off2x2_SME_Znx4_SME_Zm_INDEX3_2,
    ),
    BFMLAL_SME_ZA_array_off2x2_SME_Znx4_SME_Zmx4(BFMLAL_SME_ZA_array_off2x2_SME_Znx4_SME_Zmx4),
    BFMLAL_SME_ZA_array_off2x2_SVE_ZnxN_SME_Zm(BFMLAL_SME_ZA_array_off2x2_SVE_ZnxN_SME_Zm),
    BFMLAL_SME_ZA_array_off2x2_S_S_SVE_ZnxN_S_H_SME_Zm_S_H(
        BFMLAL_SME_ZA_array_off2x2_S_S_SVE_ZnxN_S_H_SME_Zm_S_H,
    ),
    BFMLAL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm(BFMLAL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm),
    BFMLAL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm_INDEX3_10(
        BFMLAL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm_INDEX3_10,
    ),
    BFMLSL_SME_ZA_array_off2x2_SME_Znx2_SME_Zm_INDEX3_2(
        BFMLSL_SME_ZA_array_off2x2_SME_Znx2_SME_Zm_INDEX3_2,
    ),
    BFMLSL_SME_ZA_array_off2x2_SME_Znx2_SME_Zmx2(BFMLSL_SME_ZA_array_off2x2_SME_Znx2_SME_Zmx2),
    BFMLSL_SME_ZA_array_off2x2_SME_Znx4_SME_Zm_INDEX3_2(
        BFMLSL_SME_ZA_array_off2x2_SME_Znx4_SME_Zm_INDEX3_2,
    ),
    BFMLSL_SME_ZA_array_off2x2_SME_Znx4_SME_Zmx4(BFMLSL_SME_ZA_array_off2x2_SME_Znx4_SME_Zmx4),
    BFMLSL_SME_ZA_array_off2x2_SVE_ZnxN_SME_Zm(BFMLSL_SME_ZA_array_off2x2_SVE_ZnxN_SME_Zm),
    BFMLSL_SME_ZA_array_off2x2_S_S_SVE_ZnxN_S_H_SME_Zm_S_H(
        BFMLSL_SME_ZA_array_off2x2_S_S_SVE_ZnxN_S_H_SME_Zm_S_H,
    ),
    BFMLSL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm(BFMLSL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm),
    BFMLSL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm_INDEX3_10(
        BFMLSL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm_INDEX3_10,
    ),
    BFMOPA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(
        BFMOPA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16,
    ),
    BFMOPS_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(
        BFMOPS_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16,
    ),
    BFVDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2(
        BFVDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2,
    ),
    FDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2(
        FDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2,
    ),
    FDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2(FDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2),
    FDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX2(
        FDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX2,
    ),
    FDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4(FDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4),
    FDOT_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm(FDOT_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm),
    FDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H(
        FDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H,
    ),
    FMOPA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(
        FMOPA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16,
    ),
    FMOPA_SME_ZAda_2b_S_S_SVE_Pg3_P_M_SME_Pm_P_M_SVE_Zn_S_H_SVE_Zm_16_S_H(
        FMOPA_SME_ZAda_2b_S_S_SVE_Pg3_P_M_SME_Pm_P_M_SVE_Zn_S_H_SVE_Zm_16_S_H,
    ),
    FMOPA_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(
        FMOPA_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16,
    ),
    FMOPS_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(
        FMOPS_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16,
    ),
    FMOPS_SME_ZAda_2b_S_S_SVE_Pg3_P_M_SME_Pm_P_M_SVE_Zn_S_H_SVE_Zm_16_S_H(
        FMOPS_SME_ZAda_2b_S_S_SVE_Pg3_P_M_SME_Pm_P_M_SVE_Zn_S_H_SVE_Zm_16_S_H,
    ),
    FMOPS_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16(
        FMOPS_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16,
    ),
    LDR_SME_ZT0_SIMD_ADDR_SIMPLE(LDR_SME_ZT0_SIMD_ADDR_SIMPLE),
    MOVT_Rt_SME_ZT0_INDEX(MOVT_Rt_SME_ZT0_INDEX),
    MOVT_SME_ZT0_INDEX_Rt(MOVT_SME_ZT0_INDEX_Rt),
    SDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX1(
        SDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX1,
    ),
    SDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2(
        SDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2,
    ),
    SDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX1(
        SDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX1,
    ),
    SDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX2(
        SDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX2,
    ),
    SDOT_SME_ZA_array_off3_0_S_S_SME_Znx2_S_B_SME_Zm_INDEX2_S_B(
        SDOT_SME_ZA_array_off3_0_S_S_SME_Znx2_S_B_SME_Zm_INDEX2_S_B,
    ),
    SDOT_SME_ZA_array_off3_0_S_S_SME_Znx2_S_H_SME_Zmx2_S_H(
        SDOT_SME_ZA_array_off3_0_S_S_SME_Znx2_S_H_SME_Zmx2_S_H,
    ),
    SDOT_SME_ZA_array_off3_0_S_S_SME_Znx4_S_B_SME_Zm_INDEX2_S_B(
        SDOT_SME_ZA_array_off3_0_S_S_SME_Znx4_S_B_SME_Zm_INDEX2_S_B,
    ),
    SDOT_SME_ZA_array_off3_0_S_S_SME_Znx4_S_H_SME_Zmx4_S_H(
        SDOT_SME_ZA_array_off3_0_S_S_SME_Znx4_S_H_SME_Zmx4_S_H,
    ),
    SDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H(
        SDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H,
    ),
    SDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H_c1701408(
        SDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H_c1701408,
    ),
    STR_SME_ZT0_SIMD_ADDR_SIMPLE(STR_SME_ZT0_SIMD_ADDR_SIMPLE),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SME_MOV {
    MOVA_SME_ZA_HV_idx_dest_SVE_Pg3_SVE_Zn(MOVA_SME_ZA_HV_idx_dest_SVE_Pg3_SVE_Zn),
    MOVA_SVE_Zd_SVE_Pg3_SME_ZA_HV_idx_src(MOVA_SVE_Zd_SVE_Pg3_SME_ZA_HV_idx_src),
    MOV_SME_ZA_HV_idx_dest_SVE_Pg3_SVE_Zn(MOV_SME_ZA_HV_idx_dest_SVE_Pg3_SVE_Zn),
    MOV_SVE_Zd_SVE_Pg3_SME_ZA_HV_idx_src(MOV_SVE_Zd_SVE_Pg3_SME_ZA_HV_idx_src),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SME_SIZE_22 {
    ADD_SME_Zdnx2_SME_Zdnx2_SME_Zm(ADD_SME_Zdnx2_SME_Zdnx2_SME_Zm),
    ADD_SME_Zdnx4_SME_Zdnx4_SME_Zm(ADD_SME_Zdnx4_SME_Zdnx4_SME_Zm),
    CNTP_Rd_SME_PNn_SME_VLxN_10(CNTP_Rd_SME_PNn_SME_VLxN_10),
    MOVA_SME_ZA_HV_idx_destxN_SME_Znx2(MOVA_SME_ZA_HV_idx_destxN_SME_Znx2),
    MOVA_SME_ZA_HV_idx_destxN_SME_Znx4(MOVA_SME_ZA_HV_idx_destxN_SME_Znx4),
    MOVA_SME_Zdnx2_SME_ZA_HV_idx_srcxN(MOVA_SME_Zdnx2_SME_ZA_HV_idx_srcxN),
    MOVA_SME_Zdnx4_SME_ZA_HV_idx_srcxN(MOVA_SME_Zdnx4_SME_ZA_HV_idx_srcxN),
    MOV_SME_ZA_HV_idx_destxN_SME_Znx2(MOV_SME_ZA_HV_idx_destxN_SME_Znx2),
    MOV_SME_ZA_HV_idx_destxN_SME_Znx4(MOV_SME_ZA_HV_idx_destxN_SME_Znx4),
    MOV_SME_Zdnx2_SME_ZA_HV_idx_srcxN(MOV_SME_Zdnx2_SME_ZA_HV_idx_srcxN),
    MOV_SME_Zdnx4_SME_ZA_HV_idx_srcxN(MOV_SME_Zdnx4_SME_ZA_HV_idx_srcxN),
    UCLAMP_SME_Zdnx2_SVE_Zn_SVE_Zm_16(UCLAMP_SME_Zdnx2_SVE_Zn_SVE_Zm_16),
    UCLAMP_SME_Zdnx4_SVE_Zn_SVE_Zm_16(UCLAMP_SME_Zdnx4_SVE_Zn_SVE_Zm_16),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SME_SIZE_22_HSD {
    FCLAMP_SME_Zdnx2_SVE_Zn_SVE_Zm_16(FCLAMP_SME_Zdnx2_SVE_Zn_SVE_Zm_16),
    FCLAMP_SME_Zdnx4_SVE_Zn_SVE_Zm_16(FCLAMP_SME_Zdnx4_SVE_Zn_SVE_Zm_16),
    FCLAMP_SVE_Zd_SVE_Zn_SVE_Zm_16(FCLAMP_SVE_Zd_SVE_Zn_SVE_Zm_16),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SME_STR {
    STR_SME_ZA_array_off4_SME_ADDR_RI_U4xVL(STR_SME_ZA_array_off4_SME_ADDR_RI_U4xVL),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SVE2_URQVS {
    ADDQV_Vd_SVE_Pg3_SVE_Zn(ADDQV_Vd_SVE_Pg3_SVE_Zn),
    ANDQV_Vd_SVE_Pg3_SVE_Zn(ANDQV_Vd_SVE_Pg3_SVE_Zn),
    EORQV_Vd_SVE_Pg3_SVE_Zn(EORQV_Vd_SVE_Pg3_SVE_Zn),
    FADDQV_Vd_SVE_Pg3_SVE_Zn(FADDQV_Vd_SVE_Pg3_SVE_Zn),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SVE_LIMM {
    AND_SVE_Zd_SVE_Zd_SVE_LIMM(AND_SVE_Zd_SVE_Zd_SVE_LIMM),
    EOR_SVE_Zd_SVE_Zd_SVE_LIMM(EOR_SVE_Zd_SVE_Zd_SVE_LIMM),
    ORR_SVE_Zd_SVE_Zd_SVE_LIMM(ORR_SVE_Zd_SVE_Zd_SVE_LIMM),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SVE_MISC {
    ADDPL_Rd_SP_SVE_Rn_SP_SVE_SIMM6(ADDPL_Rd_SP_SVE_Rn_SP_SVE_SIMM6),
    ADDVL_Rd_SP_SVE_Rn_SP_SVE_SIMM6(ADDVL_Rd_SP_SVE_Rn_SP_SVE_SIMM6),
    ADR_SVE_Zd_SVE_ADDR_ZZ_SXTW(ADR_SVE_Zd_SVE_ADDR_ZZ_SXTW),
    ADR_SVE_Zd_SVE_ADDR_ZZ_UXTW(ADR_SVE_Zd_SVE_ADDR_ZZ_UXTW),
    ANDS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(ANDS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm),
    AND_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(AND_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm),
    AND_SVE_Zd_SVE_Zn_SVE_Zm_16(AND_SVE_Zd_SVE_Zn_SVE_Zm_16),
    BFADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(BFADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    BFADD_SVE_Zd_SVE_Zn_SVE_Zm_16(BFADD_SVE_Zd_SVE_Zn_SVE_Zm_16),
    BFCLAMP_SVE_Zd_SVE_Zn_SVE_Zm_16(BFCLAMP_SVE_Zd_SVE_Zn_SVE_Zm_16),
    BFCVTNT_SVE_Zd_SVE_Pg3_SVE_Zn(BFCVTNT_SVE_Zd_SVE_Pg3_SVE_Zn),
    BFCVT_SVE_Zd_SVE_Pg3_SVE_Zn(BFCVT_SVE_Zd_SVE_Pg3_SVE_Zn),
    BFDOT_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX(BFDOT_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX),
    BFDOT_SVE_Zd_SVE_Zn_SVE_Zm_16(BFDOT_SVE_Zd_SVE_Zn_SVE_Zm_16),
    BFMAXNM_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(BFMAXNM_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    BFMAX_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(BFMAX_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    BFMINNM_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(BFMINNM_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    BFMIN_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(BFMIN_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    BFMLALB_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX(BFMLALB_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX),
    BFMLALB_SVE_Zd_SVE_Zn_SVE_Zm_16(BFMLALB_SVE_Zd_SVE_Zn_SVE_Zm_16),
    BFMLALT_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX(BFMLALT_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX),
    BFMLALT_SVE_Zd_SVE_Zn_SVE_Zm_16(BFMLALT_SVE_Zd_SVE_Zn_SVE_Zm_16),
    BFMLA_SVE_Zd_SVE_Pg3_SVE_Zn_SVE_Zm_16(BFMLA_SVE_Zd_SVE_Pg3_SVE_Zn_SVE_Zm_16),
    BFMLA_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX(BFMLA_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX),
    BFMLSLB_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX(BFMLSLB_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX),
    BFMLSLB_SVE_Zd_SVE_Zn_SVE_Zm_16(BFMLSLB_SVE_Zd_SVE_Zn_SVE_Zm_16),
    BFMLSLT_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX(BFMLSLT_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX),
    BFMLSLT_SVE_Zd_SVE_Zn_SVE_Zm_16(BFMLSLT_SVE_Zd_SVE_Zn_SVE_Zm_16),
    BFMLS_SVE_Zd_SVE_Pg3_SVE_Zn_SVE_Zm_16(BFMLS_SVE_Zd_SVE_Pg3_SVE_Zn_SVE_Zm_16),
    BFMLS_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX(BFMLS_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX),
    BFMMLA_SVE_Zd_SVE_Zn_SVE_Zm_16(BFMMLA_SVE_Zd_SVE_Zn_SVE_Zm_16),
    BFMUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(BFMUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    BFMUL_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX(BFMUL_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX),
    BFMUL_SVE_Zd_SVE_Zn_SVE_Zm_16(BFMUL_SVE_Zd_SVE_Zn_SVE_Zm_16),
    BFSUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(BFSUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    BFSUB_SVE_Zd_SVE_Zn_SVE_Zm_16(BFSUB_SVE_Zd_SVE_Zn_SVE_Zm_16),
    BICS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(BICS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm),
    BIC_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(BIC_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm),
    BIC_SVE_Zd_SVE_Zn_SVE_Zm_16(BIC_SVE_Zd_SVE_Zn_SVE_Zm_16),
    BRKAS_SVE_Pd_SVE_Pg4_10_SVE_Pn(BRKAS_SVE_Pd_SVE_Pg4_10_SVE_Pn),
    BRKBS_SVE_Pd_SVE_Pg4_10_SVE_Pn(BRKBS_SVE_Pd_SVE_Pg4_10_SVE_Pn),
    BRKNS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pd(BRKNS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pd),
    BRKN_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pd(BRKN_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pd),
    BRKPAS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(BRKPAS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm),
    BRKPA_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(BRKPA_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm),
    BRKPBS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(BRKPBS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm),
    BRKPB_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(BRKPB_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm),
    CDOT_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX_SVE_IMM_ROT2(CDOT_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX_SVE_IMM_ROT2),
    CDOT_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX_SVE_IMM_ROT2(CDOT_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX_SVE_IMM_ROT2),
    CMLA_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX_SVE_IMM_ROT2(CMLA_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX_SVE_IMM_ROT2),
    CMLA_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX_SVE_IMM_ROT2(CMLA_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX_SVE_IMM_ROT2),
    CNTB_Rd_SVE_PATTERN_SCALED(CNTB_Rd_SVE_PATTERN_SCALED),
    CNTD_Rd_SVE_PATTERN_SCALED(CNTD_Rd_SVE_PATTERN_SCALED),
    CNTH_Rd_SVE_PATTERN_SCALED(CNTH_Rd_SVE_PATTERN_SCALED),
    CNTW_Rd_SVE_PATTERN_SCALED(CNTW_Rd_SVE_PATTERN_SCALED),
    EOR3_SVE_Zd_SVE_Zd_SVE_Zm_16_SVE_Zn(EOR3_SVE_Zd_SVE_Zd_SVE_Zm_16_SVE_Zn),
    EORS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(EORS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm),
    EOR_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(EOR_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm),
    EOR_SVE_Zd_SVE_Zn_SVE_Zm_16(EOR_SVE_Zd_SVE_Zn_SVE_Zm_16),
    FCMLA_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX_SVE_IMM_ROT2(FCMLA_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX_SVE_IMM_ROT2),
    FCMLA_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX_SVE_IMM_ROT2(FCMLA_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX_SVE_IMM_ROT2),
    FCVTLT_SVE_Zd_SVE_Pg3_SVE_Zn(FCVTLT_SVE_Zd_SVE_Pg3_SVE_Zn),
    FCVTLT_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_S(FCVTLT_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_S),
    FCVTNT_SVE_Zd_SVE_Pg3_SVE_Zn(FCVTNT_SVE_Zd_SVE_Pg3_SVE_Zn),
    FCVTNT_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_D(FCVTNT_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_D),
    FCVTN_SVE_Zd_SME_Znx2(FCVTN_SVE_Zd_SME_Znx2),
    FCVTXNT_SVE_Zd_SVE_Pg3_SVE_Zn(FCVTXNT_SVE_Zd_SVE_Pg3_SVE_Zn),
    FCVTX_SVE_Zd_SVE_Pg3_SVE_Zn(FCVTX_SVE_Zd_SVE_Pg3_SVE_Zn),
    FCVTZS_SME_Zdnx2_SME_Znx2(FCVTZS_SME_Zdnx2_SME_Znx2),
    FCVTZS_SME_Zdnx4_SME_Znx4(FCVTZS_SME_Zdnx4_SME_Znx4),
    FCVTZS_SVE_Zd_SVE_Pg3_SVE_Zn(FCVTZS_SVE_Zd_SVE_Pg3_SVE_Zn),
    FCVTZS_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_D(FCVTZS_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_D),
    FCVTZS_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_H(FCVTZS_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_H),
    FCVTZS_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_S(FCVTZS_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_S),
    FCVTZS_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_H(FCVTZS_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_H),
    FCVTZS_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_H(FCVTZS_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_H),
    FCVTZS_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_S(FCVTZS_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_S),
    FCVTZU_SME_Zdnx2_SME_Znx2(FCVTZU_SME_Zdnx2_SME_Znx2),
    FCVTZU_SME_Zdnx4_SME_Znx4(FCVTZU_SME_Zdnx4_SME_Znx4),
    FCVTZU_SVE_Zd_SVE_Pg3_SVE_Zn(FCVTZU_SVE_Zd_SVE_Pg3_SVE_Zn),
    FCVTZU_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_D(FCVTZU_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_D),
    FCVTZU_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_H(FCVTZU_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_H),
    FCVTZU_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_S(FCVTZU_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_S),
    FCVTZU_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_H(FCVTZU_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_H),
    FCVTZU_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_H(FCVTZU_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_H),
    FCVTZU_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_S(FCVTZU_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_S),
    FCVT_SVE_Zd_SME_Znx2(FCVT_SVE_Zd_SME_Znx2),
    FCVT_SVE_Zd_SVE_Pg3_SVE_Zn(FCVT_SVE_Zd_SVE_Pg3_SVE_Zn),
    FCVT_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_H(FCVT_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_H),
    FCVT_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_S(FCVT_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_S),
    FCVT_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_D(FCVT_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_D),
    FCVT_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_D(FCVT_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_D),
    FCVT_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_H(FCVT_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_H),
    FDOT_SVE_Zd_SVE_Zn_SVE_Zm3_19_INDEX(FDOT_SVE_Zd_SVE_Zn_SVE_Zm3_19_INDEX),
    FDOT_SVE_Zd_SVE_Zn_SVE_Zm_16(FDOT_SVE_Zd_SVE_Zn_SVE_Zm_16),
    FMUL_SVE_Zd_SVE_Zn_SVE_Zm3_22_INDEX(FMUL_SVE_Zd_SVE_Zn_SVE_Zm3_22_INDEX),
    FMUL_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX(FMUL_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX),
    FMUL_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX(FMUL_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX),
    LDR_SVE_PNt_SVE_ADDR_RI_S9xVL(LDR_SVE_PNt_SVE_ADDR_RI_S9xVL),
    LDR_SVE_Pt_SVE_ADDR_RI_S9xVL(LDR_SVE_Pt_SVE_ADDR_RI_S9xVL),
    LDR_SVE_Zt_SVE_ADDR_RI_S9xVL(LDR_SVE_Zt_SVE_ADDR_RI_S9xVL),
    MOVPRFX_SVE_Zd_SVE_Zn(MOVPRFX_SVE_Zd_SVE_Zn),
    MUL_SVE_Zd_SVE_Zn_SVE_Zm3_22_INDEX(MUL_SVE_Zd_SVE_Zn_SVE_Zm3_22_INDEX),
    MUL_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX(MUL_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX),
    MUL_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX(MUL_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX),
    ORNS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(ORNS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm),
    ORN_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(ORN_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm),
    ORRS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(ORRS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm),
    ORR_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm(ORR_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm),
    ORR_SVE_Zd_SVE_Zn_SVE_Zm_16(ORR_SVE_Zd_SVE_Zn_SVE_Zm_16),
    SCVTF_SME_Zdnx2_SME_Znx2(SCVTF_SME_Zdnx2_SME_Znx2),
    SCVTF_SME_Zdnx4_SME_Znx4(SCVTF_SME_Zdnx4_SME_Znx4),
    SCVTF_SVE_Zd_SVE_Pg3_SVE_Zn(SCVTF_SVE_Zd_SVE_Pg3_SVE_Zn),
    SCVTF_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_D(SCVTF_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_D),
    SCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_D(SCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_D),
    SCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_H(SCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_H),
    SCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_S(SCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_S),
    SCVTF_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_D(SCVTF_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_D),
    SCVTF_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_S(SCVTF_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_S),
    SDOT_SVE_Zd_SVE_Zn_SVE_Zm3_19_INDEX(SDOT_SVE_Zd_SVE_Zn_SVE_Zm3_19_INDEX),
    SDOT_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX(SDOT_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX),
    SDOT_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX(SDOT_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX),
    SDOT_SVE_Zd_S_S_SVE_Zn_S_H_SVE_Zm_16_S_H(SDOT_SVE_Zd_S_S_SVE_Zn_S_H_SVE_Zm_16_S_H),
    SMULLB_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX(SMULLB_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX),
    SMULLB_SVE_Zd_SVE_Zn_SVE_Zm4_11_INDEX(SMULLB_SVE_Zd_SVE_Zn_SVE_Zm4_11_INDEX),
    SMULLT_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX(SMULLT_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX),
    SMULLT_SVE_Zd_SVE_Zn_SVE_Zm4_11_INDEX(SMULLT_SVE_Zd_SVE_Zn_SVE_Zm4_11_INDEX),
    STR_SVE_PNt_SVE_ADDR_RI_S9xVL(STR_SVE_PNt_SVE_ADDR_RI_S9xVL),
    STR_SVE_Pt_SVE_ADDR_RI_S9xVL(STR_SVE_Pt_SVE_ADDR_RI_S9xVL),
    STR_SVE_Zt_SVE_ADDR_RI_S9xVL(STR_SVE_Zt_SVE_ADDR_RI_S9xVL),
    SXTW_SVE_Zd_SVE_Pg3_SVE_Zn(SXTW_SVE_Zd_SVE_Pg3_SVE_Zn),
    TRN1_SVE_Zd_SVE_Zn_SVE_Zm_16(TRN1_SVE_Zd_SVE_Zn_SVE_Zm_16),
    TRN2_SVE_Zd_SVE_Zn_SVE_Zm_16(TRN2_SVE_Zd_SVE_Zn_SVE_Zm_16),
    UCVTF_SME_Zdnx2_SME_Znx2(UCVTF_SME_Zdnx2_SME_Znx2),
    UCVTF_SME_Zdnx4_SME_Znx4(UCVTF_SME_Zdnx4_SME_Znx4),
    UCVTF_SVE_Zd_SVE_Pg3_SVE_Zn(UCVTF_SVE_Zd_SVE_Pg3_SVE_Zn),
    UCVTF_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_D(UCVTF_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_D),
    UCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_D(UCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_D),
    UCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_H(UCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_H),
    UCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_S(UCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_S),
    UCVTF_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_D(UCVTF_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_D),
    UCVTF_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_S(UCVTF_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_S),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SVE_MOVPRFX {
    MOVPRFX_SVE_Zd_SVE_Pg3_SVE_Zn(MOVPRFX_SVE_Zd_SVE_Pg3_SVE_Zn),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SVE_PRED_ZM {
    BRKA_SVE_Pd_SVE_Pg4_10_SVE_Pn(BRKA_SVE_Pd_SVE_Pg4_10_SVE_Pn),
    BRKB_SVE_Pd_SVE_Pg4_10_SVE_Pn(BRKB_SVE_Pd_SVE_Pg4_10_SVE_Pn),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SVE_SHIFT_PRED {
    ASRD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHRIMM_PRED(ASRD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHRIMM_PRED),
    ASR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHRIMM_PRED(ASR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHRIMM_PRED),
    LSL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHLIMM_PRED(LSL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHLIMM_PRED),
    LSR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHRIMM_PRED(LSR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHRIMM_PRED),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SVE_SHIFT_UNPRED {
    ASR_SVE_Zd_SVE_Zn_SVE_SHRIMM_UNPRED(ASR_SVE_Zd_SVE_Zn_SVE_SHRIMM_UNPRED),
    LSL_SVE_Zd_SVE_Zn_SVE_SHLIMM_UNPRED(LSL_SVE_Zd_SVE_Zn_SVE_SHLIMM_UNPRED),
    LSR_SVE_Zd_SVE_Zn_SVE_SHRIMM_UNPRED(LSR_SVE_Zd_SVE_Zn_SVE_SHRIMM_UNPRED),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SVE_SIZE_BHS {
    ASR_SVE_Zd_SVE_Zn_SVE_Zm_16(ASR_SVE_Zd_SVE_Zn_SVE_Zm_16),
    ASR_SVE_Zd_S_B_SVE_Pg3_P_M_SVE_Zd_S_B_SVE_Zm_5_S_D(
        ASR_SVE_Zd_S_B_SVE_Pg3_P_M_SVE_Zd_S_B_SVE_Zm_5_S_D,
    ),
    CMPEQ_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(CMPEQ_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16),
    CMPGE_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(CMPGE_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16),
    CMPGT_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(CMPGT_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16),
    CMPHI_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_D(
        CMPHI_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_D,
    ),
    CMPHS_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_D(
        CMPHS_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_D,
    ),
    CMPLE_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(CMPLE_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16),
    CMPLO_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(CMPLO_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16),
    CMPLS_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(CMPLS_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16),
    CMPLT_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(CMPLT_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16),
    CMPNE_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(CMPNE_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16),
    LSL_SVE_Zd_SVE_Zn_SVE_Zm_16(LSL_SVE_Zd_SVE_Zn_SVE_Zm_16),
    LSL_SVE_Zd_S_B_SVE_Pg3_P_M_SVE_Zd_S_B_SVE_Zm_5_S_D(
        LSL_SVE_Zd_S_B_SVE_Pg3_P_M_SVE_Zd_S_B_SVE_Zm_5_S_D,
    ),
    LSR_SVE_Zd_SVE_Zn_SVE_Zm_16(LSR_SVE_Zd_SVE_Zn_SVE_Zm_16),
    LSR_SVE_Zd_S_B_SVE_Pg3_P_M_SVE_Zd_S_B_SVE_Zm_5_S_D(
        LSR_SVE_Zd_S_B_SVE_Pg3_P_M_SVE_Zd_S_B_SVE_Zm_5_S_D,
    ),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SVE_SIZE_BHSD {
    ADDP_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zn(ADDP_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zn),
    ADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(ADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    ADD_SVE_Zd_SVE_Zd_SVE_AIMM(ADD_SVE_Zd_SVE_Zd_SVE_AIMM),
    ADD_SVE_Zd_SVE_Zn_SVE_Zm_16(ADD_SVE_Zd_SVE_Zn_SVE_Zm_16),
    ANDV_SVE_Vd_SVE_Pg3_SVE_Zn(ANDV_SVE_Vd_SVE_Pg3_SVE_Zn),
    AND_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(AND_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    ASRR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(ASRR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    ASR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(ASR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    BIC_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(BIC_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    CLASTA_Rd_SVE_Pg3_Rd_SVE_Zm_5(CLASTA_Rd_SVE_Pg3_Rd_SVE_Zm_5),
    CLASTA_SVE_Vd_SVE_Pg3_SVE_Vd_SVE_Zm_5(CLASTA_SVE_Vd_SVE_Pg3_SVE_Vd_SVE_Zm_5),
    CLASTA_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(CLASTA_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    CLASTB_Rd_SVE_Pg3_Rd_SVE_Zm_5(CLASTB_Rd_SVE_Pg3_Rd_SVE_Zm_5),
    CLASTB_SVE_Vd_SVE_Pg3_SVE_Vd_SVE_Zm_5(CLASTB_SVE_Vd_SVE_Pg3_SVE_Vd_SVE_Zm_5),
    CLASTB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(CLASTB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    CLS_SVE_Zd_SVE_Pg3_SVE_Zn(CLS_SVE_Zd_SVE_Pg3_SVE_Zn),
    CLZ_SVE_Zd_SVE_Pg3_SVE_Zn(CLZ_SVE_Zd_SVE_Pg3_SVE_Zn),
    CMLA_SVE_Zd_SVE_Zn_SVE_Zm_16_SVE_IMM_ROT2(CMLA_SVE_Zd_SVE_Zn_SVE_Zm_16_SVE_IMM_ROT2),
    CMPEQ_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5(CMPEQ_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5),
    CMPEQ_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_B(
        CMPEQ_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_B,
    ),
    CMPGE_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5(CMPGE_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5),
    CMPGE_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_B(
        CMPGE_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_B,
    ),
    CMPGT_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5(CMPGT_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5),
    CMPGT_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_B(
        CMPGT_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_B,
    ),
    CMPHI_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_UIMM7(CMPHI_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_UIMM7),
    CMPHI_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(CMPHI_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16),
    CMPHS_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_UIMM7(CMPHS_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_UIMM7),
    CMPHS_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(CMPHS_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16),
    CMPLE_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5(CMPLE_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5),
    CMPLO_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_UIMM7(CMPLO_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_UIMM7),
    CMPLS_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_UIMM7(CMPLS_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_UIMM7),
    CMPLT_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5(CMPLT_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5),
    CMPNE_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5(CMPNE_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5),
    CMPNE_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_B(
        CMPNE_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_B,
    ),
    CNOT_SVE_Zd_SVE_Pg3_SVE_Zn(CNOT_SVE_Zd_SVE_Pg3_SVE_Zn),
    CNTP_Rd_SVE_Pg4_10_SVE_Pn(CNTP_Rd_SVE_Pg4_10_SVE_Pn),
    CNT_SVE_Zd_SVE_Pg3_SVE_Zn(CNT_SVE_Zd_SVE_Pg3_SVE_Zn),
    EORBT_SVE_Zd_SVE_Zn_SVE_Zm_16(EORBT_SVE_Zd_SVE_Zn_SVE_Zm_16),
    EORTB_SVE_Zd_SVE_Zn_SVE_Zm_16(EORTB_SVE_Zd_SVE_Zn_SVE_Zm_16),
    EORV_SVE_Vd_SVE_Pg3_SVE_Zn(EORV_SVE_Vd_SVE_Pg3_SVE_Zn),
    EOR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(EOR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    LSLR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(LSLR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    LSL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(LSL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    LSRR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(LSRR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    LSR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(LSR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    MUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(MUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    MUL_SVE_Zd_SVE_Zd_SVE_SIMM8(MUL_SVE_Zd_SVE_Zd_SVE_SIMM8),
    MUL_SVE_Zd_SVE_Zn_SVE_Zm_16(MUL_SVE_Zd_SVE_Zn_SVE_Zm_16),
    NEG_SVE_Zd_SVE_Pg3_SVE_Zn(NEG_SVE_Zd_SVE_Pg3_SVE_Zn),
    ORR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(ORR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    ORV_SVE_Vd_SVE_Pg3_SVE_Zn(ORV_SVE_Vd_SVE_Pg3_SVE_Zn),
    SMULH_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(SMULH_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    SMULH_SVE_Zd_SVE_Zn_SVE_Zm_16(SMULH_SVE_Zd_SVE_Zn_SVE_Zm_16),
    SUBR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(SUBR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    SUBR_SVE_Zd_SVE_Zd_SVE_AIMM(SUBR_SVE_Zd_SVE_Zd_SVE_AIMM),
    SUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(SUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    SUB_SVE_Zd_SVE_Zd_SVE_AIMM(SUB_SVE_Zd_SVE_Zd_SVE_AIMM),
    SUB_SVE_Zd_SVE_Zn_SVE_Zm_16(SUB_SVE_Zd_SVE_Zn_SVE_Zm_16),
    TBL_SVE_Zd_SVE_ZnxN_SVE_Zm_16(TBL_SVE_Zd_SVE_ZnxN_SVE_Zm_16),
    TBL_SVE_Zd_S_B_SVE_ZnxN_S_B_SVE_Zm_16_S_B(TBL_SVE_Zd_S_B_SVE_ZnxN_S_B_SVE_Zm_16_S_B),
    TBX_SVE_Zd_SVE_Zn_SVE_Zm_16(TBX_SVE_Zd_SVE_Zn_SVE_Zm_16),
    TRN1_SVE_Pd_SVE_Pn_SVE_Pm(TRN1_SVE_Pd_SVE_Pn_SVE_Pm),
    TRN1_SVE_Zd_S_B_SVE_Zn_S_B_SVE_Zm_16_S_B(TRN1_SVE_Zd_S_B_SVE_Zn_S_B_SVE_Zm_16_S_B),
    TRN2_SVE_Pd_SVE_Pn_SVE_Pm(TRN2_SVE_Pd_SVE_Pn_SVE_Pm),
    TRN2_SVE_Zd_S_B_SVE_Zn_S_B_SVE_Zm_16_S_B(TRN2_SVE_Zd_S_B_SVE_Zn_S_B_SVE_Zm_16_S_B),
    UADDV_SVE_Vd_SVE_Pg3_SVE_Zn(UADDV_SVE_Vd_SVE_Pg3_SVE_Zn),
    UCLAMP_SVE_Zd_SVE_Zn_SVE_Zm_16(UCLAMP_SVE_Zd_SVE_Zn_SVE_Zm_16),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SVE_SIZE_HSD {
    ADDHNB_SVE_Zd_SVE_Zn_SVE_Zm_16(ADDHNB_SVE_Zd_SVE_Zn_SVE_Zm_16),
    ADDHNT_SVE_Zd_SVE_Zn_SVE_Zm_16(ADDHNT_SVE_Zd_SVE_Zn_SVE_Zm_16),
    FADDA_SVE_Vd_SVE_Pg3_SVE_Vd_SVE_Zm_5(FADDA_SVE_Vd_SVE_Pg3_SVE_Vd_SVE_Zm_5),
    FADDP_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zn(FADDP_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zn),
    FADDV_SVE_Vd_SVE_Pg3_SVE_Zn(FADDV_SVE_Vd_SVE_Pg3_SVE_Zn),
    FADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_ONE(FADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_ONE),
    FADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(FADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    FADD_SVE_Zd_SVE_Zn_SVE_Zm_16(FADD_SVE_Zd_SVE_Zn_SVE_Zm_16),
    FCADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5_SVE_IMM_ROT1(
        FCADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5_SVE_IMM_ROT1,
    ),
    FCMEQ_SVE_Pd_SVE_Pg3_SVE_Zn_FPIMM0(FCMEQ_SVE_Pd_SVE_Pg3_SVE_Zn_FPIMM0),
    FCMEQ_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(FCMEQ_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16),
    FCMGE_SVE_Pd_SVE_Pg3_SVE_Zn_FPIMM0(FCMGE_SVE_Pd_SVE_Pg3_SVE_Zn_FPIMM0),
    FCMGE_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(FCMGE_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16),
    FCMGT_SVE_Pd_SVE_Pg3_SVE_Zn_FPIMM0(FCMGT_SVE_Pd_SVE_Pg3_SVE_Zn_FPIMM0),
    FCMGT_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(FCMGT_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16),
    FCMLA_SVE_Zd_SVE_Pg3_SVE_Zn_SVE_Zm_16_IMM_ROT2(FCMLA_SVE_Zd_SVE_Pg3_SVE_Zn_SVE_Zm_16_IMM_ROT2),
    FCMLE_SVE_Pd_SVE_Pg3_SVE_Zn_FPIMM0(FCMLE_SVE_Pd_SVE_Pg3_SVE_Zn_FPIMM0),
    FCMLT_SVE_Pd_SVE_Pg3_SVE_Zn_FPIMM0(FCMLT_SVE_Pd_SVE_Pg3_SVE_Zn_FPIMM0),
    FCMNE_SVE_Pd_SVE_Pg3_SVE_Zn_FPIMM0(FCMNE_SVE_Pd_SVE_Pg3_SVE_Zn_FPIMM0),
    FCMNE_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(FCMNE_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16),
    FCMUO_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16(FCMUO_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16),
    FCPY_SVE_Zd_SVE_Pg4_16_SVE_FPIMM8(FCPY_SVE_Zd_SVE_Pg4_16_SVE_FPIMM8),
    FDIVR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(FDIVR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    FDIV_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(FDIV_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    FDUP_SVE_Zd_SVE_FPIMM8(FDUP_SVE_Zd_SVE_FPIMM8),
    FEXPA_SVE_Zd_SVE_Zn(FEXPA_SVE_Zd_SVE_Zn),
    FMSB_SVE_Zd_SVE_Pg3_SVE_Zm_5_SVE_Za_16(FMSB_SVE_Zd_SVE_Pg3_SVE_Zm_5_SVE_Za_16),
    FMULX_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(FMULX_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    FMUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_TWO(FMUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_TWO),
    FMUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(FMUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    FMUL_SVE_Zd_SVE_Zn_SVE_Zm_16(FMUL_SVE_Zd_SVE_Zn_SVE_Zm_16),
    FSUBR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_ONE(FSUBR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_ONE),
    FSUBR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(FSUBR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    FSUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_ONE(FSUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_ONE),
    FSUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(FSUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    FSUB_SVE_Zd_SVE_Zn_SVE_Zm_16(FSUB_SVE_Zd_SVE_Zn_SVE_Zm_16),
    SMULLB_SVE_Zd_SVE_Zn_SVE_Zm_16(SMULLB_SVE_Zd_SVE_Zn_SVE_Zm_16),
    SMULLT_SVE_Zd_SVE_Zn_SVE_Zm_16(SMULLT_SVE_Zd_SVE_Zn_SVE_Zm_16),
    SUBHNB_SVE_Zd_SVE_Zn_SVE_Zm_16(SUBHNB_SVE_Zd_SVE_Zn_SVE_Zm_16),
    SUBHNT_SVE_Zd_SVE_Zn_SVE_Zm_16(SUBHNT_SVE_Zd_SVE_Zn_SVE_Zm_16),
    SXTB_SVE_Zd_SVE_Pg3_SVE_Zn(SXTB_SVE_Zd_SVE_Pg3_SVE_Zn),
    UADDLB_SVE_Zd_SVE_Zn_SVE_Zm_16(UADDLB_SVE_Zd_SVE_Zn_SVE_Zm_16),
    UADDLT_SVE_Zd_SVE_Zn_SVE_Zm_16(UADDLT_SVE_Zd_SVE_Zn_SVE_Zm_16),
    UADDWB_SVE_Zd_SVE_Zn_SVE_Zm_16(UADDWB_SVE_Zd_SVE_Zn_SVE_Zm_16),
    UADDWT_SVE_Zd_SVE_Zn_SVE_Zm_16(UADDWT_SVE_Zd_SVE_Zn_SVE_Zm_16),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SVE_SIZE_SD {
    ADR_SVE_Zd_SVE_ADDR_ZZ_LSL(ADR_SVE_Zd_SVE_ADDR_ZZ_LSL),
    CDOT_SVE_Zd_SVE_Zn_SVE_Zm_16_SVE_IMM_ROT2(CDOT_SVE_Zd_SVE_Zn_SVE_Zm_16_SVE_IMM_ROT2),
    CTERMEQ_Rn_Rm(CTERMEQ_Rn_Rm),
    CTERMNE_Rn_Rm(CTERMNE_Rn_Rm),
    SBCLB_SVE_Zd_SVE_Zn_SVE_Zm_16(SBCLB_SVE_Zd_SVE_Zn_SVE_Zm_16),
    SBCLT_SVE_Zd_SVE_Zn_SVE_Zm_16(SBCLT_SVE_Zd_SVE_Zn_SVE_Zm_16),
    SDIVR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(SDIVR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    SDIV_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(SDIV_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5),
    SDOT_SVE_Zd_SVE_Zn_SVE_Zm_16(SDOT_SVE_Zd_SVE_Zn_SVE_Zm_16),
    SXTH_SVE_Zd_SVE_Pg3_SVE_Zn(SXTH_SVE_Zd_SVE_Pg3_SVE_Zn),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TESTBRANCH {
    TBNZ_Rt_BIT_NUM_ADDR_PCREL14(TBNZ_Rt_BIT_NUM_ADDR_PCREL14),
    TBZ_Rt_BIT_NUM_ADDR_PCREL14(TBZ_Rt_BIT_NUM_ADDR_PCREL14),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Operation {
    AARCH64_MISC(AARCH64_MISC),
    ADDSUB_CARRY(ADDSUB_CARRY),
    ADDSUB_EXT(ADDSUB_EXT),
    ADDSUB_IMM(ADDSUB_IMM),
    ADDSUB_SHIFT(ADDSUB_SHIFT),
    ASIMDALL(ASIMDALL),
    ASIMDDIFF(ASIMDDIFF),
    ASIMDELEM(ASIMDELEM),
    ASIMDIMM(ASIMDIMM),
    ASIMDMISC(ASIMDMISC),
    ASIMDPERM(ASIMDPERM),
    ASIMDSAME(ASIMDSAME),
    ASIMDSHF(ASIMDSHF),
    ASIMDTBL(ASIMDTBL),
    ASISDELEM(ASISDELEM),
    ASISDMISC(ASISDMISC),
    ASISDPAIR(ASISDPAIR),
    ASISDSAME(ASISDSAME),
    ASISDSHF(ASISDSHF),
    BFLOAT16(BFLOAT16),
    BITFIELD(BITFIELD),
    BRANCH_IMM(BRANCH_IMM),
    BRANCH_REG(BRANCH_REG),
    COMPBRANCH(COMPBRANCH),
    CONDCMP_IMM(CONDCMP_IMM),
    CONDCMP_REG(CONDCMP_REG),
    CONDSEL(CONDSEL),
    CRYPTOSHA3(CRYPTOSHA3),
    CSSC(CSSC),
    DOTPRODUCT(DOTPRODUCT),
    DP_1SRC(DP_1SRC),
    DP_2SRC(DP_2SRC),
    DP_3SRC(DP_3SRC),
    EXCEPTION(EXCEPTION),
    FLOAT2FIX(FLOAT2FIX),
    FLOAT2INT(FLOAT2INT),
    FLOATCCMP(FLOATCCMP),
    FLOATCMP(FLOATCMP),
    FLOATDP1(FLOATDP1),
    FLOATDP2(FLOATDP2),
    FLOATDP3(FLOATDP3),
    FLOATIMM(FLOATIMM),
    FLOATSEL(FLOATSEL),
    IC_SYSTEM(IC_SYSTEM),
    LDSTEXCL(LDSTEXCL),
    LDSTPAIR_INDEXED(LDSTPAIR_INDEXED),
    LDSTPAIR_OFF(LDSTPAIR_OFF),
    LDST_IMM10(LDST_IMM10),
    LDST_IMM9(LDST_IMM9),
    LDST_POS(LDST_POS),
    LDST_REGOFF(LDST_REGOFF),
    LDST_UNPRIV(LDST_UNPRIV),
    LDST_UNSCALED(LDST_UNSCALED),
    LOADLIT(LOADLIT),
    LOG_IMM(LOG_IMM),
    LOG_SHIFT(LOG_SHIFT),
    MOVEWIDE(MOVEWIDE),
    PCRELADDR(PCRELADDR),
    SME2_MOV(SME2_MOV),
    SME2_MOVAZ(SME2_MOVAZ),
    SME_FP_SD(SME_FP_SD),
    SME_INT_SD(SME_INT_SD),
    SME_LDR(SME_LDR),
    SME_MISC(SME_MISC),
    SME_MOV(SME_MOV),
    SME_SIZE_22(SME_SIZE_22),
    SME_SIZE_22_HSD(SME_SIZE_22_HSD),
    SME_STR(SME_STR),
    SVE2_URQVS(SVE2_URQVS),
    SVE_LIMM(SVE_LIMM),
    SVE_MISC(SVE_MISC),
    SVE_MOVPRFX(SVE_MOVPRFX),
    SVE_PRED_ZM(SVE_PRED_ZM),
    SVE_SHIFT_PRED(SVE_SHIFT_PRED),
    SVE_SHIFT_UNPRED(SVE_SHIFT_UNPRED),
    SVE_SIZE_BHS(SVE_SIZE_BHS),
    SVE_SIZE_BHSD(SVE_SIZE_BHSD),
    SVE_SIZE_HSD(SVE_SIZE_HSD),
    SVE_SIZE_SD(SVE_SIZE_SD),
    TESTBRANCH(TESTBRANCH),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Opcode {
    pub mnemonic: Mnemonic,
    pub operation: Operation,
}
impl ADD_SME_Zdnx2_SME_Zdnx2_SME_Zm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "add",
        aliases: &[],
        opcode: 0xc120a300,
        mask: 0xff30ffe1,
        class: InsnClass::SME_SIZE_22,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_Zdnx2,
                class: InsnOperandClass::SVE_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zdn2,
                    lsb: 1,
                    width: 4,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_Zdnx2,
                class: InsnOperandClass::SVE_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zdn2,
                    lsb: 1,
                    width: 4,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_Zm,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zm,
                    lsb: 16,
                    width: 4,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#add,
            operation: Operation::SME_SIZE_22(SME_SIZE_22::ADD_SME_Zdnx2_SME_Zdnx2_SME_Zm(
                ADD_SME_Zdnx2_SME_Zdnx2_SME_Zm::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ADD_SME_Zdnx2_SME_Zdnx2_SME_Zm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ADD_SME_Zdnx4_SME_Zdnx4_SME_Zm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "add",
        aliases: &[],
        opcode: 0xc120ab00,
        mask: 0xff30ffe3,
        class: InsnClass::SME_SIZE_22,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_Zdnx4,
                class: InsnOperandClass::SVE_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zdn4,
                    lsb: 2,
                    width: 3,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_Zdnx4,
                class: InsnOperandClass::SVE_REGLIST,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zdn4,
                    lsb: 2,
                    width: 3,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_Zm,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zm,
                    lsb: 16,
                    width: 4,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#add,
            operation: Operation::SME_SIZE_22(SME_SIZE_22::ADD_SME_Zdnx4_SME_Zdnx4_SME_Zm(
                ADD_SME_Zdnx4_SME_Zdnx4_SME_Zm::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ADD_SME_Zdnx4_SME_Zdnx4_SME_Zm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ADD_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "add",
        aliases: &[],
        opcode: 0xc1201810,
        mask: 0xffb09c18,
        class: InsnClass::SME_INT_SD,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_off3_0,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_Rv,
                        lsb: 13,
                        width: 2,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm3_0,
                        lsb: 0,
                        width: 3,
                    },
                ],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_ZnxN,
                class: InsnOperandClass::SVE_REGLIST,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_Zm,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zm,
                    lsb: 16,
                    width: 4,
                }],
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
            mnemonic: Mnemonic::r#add,
            operation: Operation::SME_INT_SD(SME_INT_SD::ADD_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm(
                ADD_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ADD_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ADD_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_S_SME_Zm_S_S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "add",
        aliases: &[],
        opcode: 0xc1301810,
        mask: 0xffb09c18,
        class: InsnClass::SME_INT_SD,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_off3_0,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_Rv,
                        lsb: 13,
                        width: 2,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm3_0,
                        lsb: 0,
                        width: 3,
                    },
                ],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_ZnxN,
                class: InsnOperandClass::SVE_REGLIST,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_Zm,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zm,
                    lsb: 16,
                    width: 4,
                }],
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
            mnemonic: Mnemonic::r#add,
            operation: Operation::SME_INT_SD(
                SME_INT_SD::ADD_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_S_SME_Zm_S_S(
                    ADD_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_S_SME_Zm_S_S::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for ADD_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_S_SME_Zm_S_S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ADD_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "add",
        aliases: &[],
        opcode: 0xc1a01810,
        mask: 0xffa19c38,
        class: InsnClass::SME_INT_SD,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_off3_0,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_Rv,
                        lsb: 13,
                        width: 2,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm3_0,
                        lsb: 0,
                        width: 3,
                    },
                ],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_Znx2,
                class: InsnOperandClass::SVE_REGLIST,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zn2,
                    lsb: 6,
                    width: 4,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_Zmx2,
                class: InsnOperandClass::SVE_REGLIST,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zm2,
                    lsb: 17,
                    width: 4,
                }],
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
            mnemonic: Mnemonic::r#add,
            operation: Operation::SME_INT_SD(
                SME_INT_SD::ADD_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2(
                    ADD_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for ADD_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ADD_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "add",
        aliases: &[],
        opcode: 0xc1a11810,
        mask: 0xffa39c78,
        class: InsnClass::SME_INT_SD,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_off3_0,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_Rv,
                        lsb: 13,
                        width: 2,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm3_0,
                        lsb: 0,
                        width: 3,
                    },
                ],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_Znx4,
                class: InsnOperandClass::SVE_REGLIST,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zn4,
                    lsb: 7,
                    width: 3,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_Zmx4,
                class: InsnOperandClass::SVE_REGLIST,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zm4,
                    lsb: 18,
                    width: 3,
                }],
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
            mnemonic: Mnemonic::r#add,
            operation: Operation::SME_INT_SD(
                SME_INT_SD::ADD_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4(
                    ADD_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for ADD_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ADD_SME_ZA_array_off3_0_SME_Znx2 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "add",
        aliases: &[],
        opcode: 0xc1a01c10,
        mask: 0xffbf9c38,
        class: InsnClass::SME_INT_SD,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_off3_0,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_Rv,
                        lsb: 13,
                        width: 2,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm3_0,
                        lsb: 0,
                        width: 3,
                    },
                ],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_Znx2,
                class: InsnOperandClass::SVE_REGLIST,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zn2,
                    lsb: 6,
                    width: 4,
                }],
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
            mnemonic: Mnemonic::r#add,
            operation: Operation::SME_INT_SD(SME_INT_SD::ADD_SME_ZA_array_off3_0_SME_Znx2(
                ADD_SME_ZA_array_off3_0_SME_Znx2::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ADD_SME_ZA_array_off3_0_SME_Znx2 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ADD_SME_ZA_array_off3_0_SME_Znx4 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "add",
        aliases: &[],
        opcode: 0xc1a11c10,
        mask: 0xffbf9c78,
        class: InsnClass::SME_INT_SD,
        feature_set: InsnFeatureSet::SME2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SME_ZA_array_off3_0,
                class: InsnOperandClass::ZA_ACCESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::SME_Rv,
                        lsb: 13,
                        width: 2,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm3_0,
                        lsb: 0,
                        width: 3,
                    },
                ],
            },
            InsnOperand {
                kind: InsnOperandKind::SME_Znx4,
                class: InsnOperandClass::SVE_REGLIST,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SME_Zn4,
                    lsb: 7,
                    width: 3,
                }],
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
            mnemonic: Mnemonic::r#add,
            operation: Operation::SME_INT_SD(SME_INT_SD::ADD_SME_ZA_array_off3_0_SME_Znx4(
                ADD_SME_ZA_array_off3_0_SME_Znx4::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ADD_SME_ZA_array_off3_0_SME_Znx4 {
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
impl ADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "add",
        aliases: &[],
        opcode: 0x4000000,
        mask: 0xff3fe000,
        class: InsnClass::SVE_SIZE_BHSD,
        feature_set: InsnFeatureSet::SVE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SVE_Zd,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
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
                    InsnOperandQualifier::S_B,
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
                    InsnOperandQualifier::S_B,
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
            mnemonic: Mnemonic::r#add,
            operation: Operation::SVE_SIZE_BHSD(SVE_SIZE_BHSD::ADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5(
                ADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ADD_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "add",
        aliases: &[],
        opcode: 0x4200000,
        mask: 0xff20fc00,
        class: InsnClass::SVE_SIZE_BHSD,
        feature_set: InsnFeatureSet::SVE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::SVE_Zd,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
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
                kind: InsnOperandKind::SVE_Zn,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SVE_Zm_16,
                class: InsnOperandClass::SVE_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::SVE_Zm_16,
                    lsb: 16,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    pub(crate) fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#add,
            operation: Operation::SVE_SIZE_BHSD(SVE_SIZE_BHSD::ADD_SVE_Zd_SVE_Zn_SVE_Zm_16(
                ADD_SVE_Zd_SVE_Zn_SVE_Zm_16::from(bits),
            )),
        }
    }
}
