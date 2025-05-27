#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ASIMDSHF {
    FCVTZS_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S(FCVTZS_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S),
    FCVTZS_Vd_Vn_IMM_VLSR(FCVTZS_Vd_Vn_IMM_VLSR),
    FCVTZU_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S(FCVTZU_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S),
    FCVTZU_Vd_Vn_IMM_VLSR(FCVTZU_Vd_Vn_IMM_VLSR),
    RSHRN2_Vd_Vn_IMM_VLSR(RSHRN2_Vd_Vn_IMM_VLSR),
    RSHRN_Vd_Vn_IMM_VLSR(RSHRN_Vd_Vn_IMM_VLSR),
    SCVTF_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S(SCVTF_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S),
    SCVTF_Vd_Vn_IMM_VLSR(SCVTF_Vd_Vn_IMM_VLSR),
    SHL_Vd_Vn_IMM_VLSL(SHL_Vd_Vn_IMM_VLSL),
    SHRN2_Vd_Vn_IMM_VLSR(SHRN2_Vd_Vn_IMM_VLSR),
    SHRN_Vd_Vn_IMM_VLSR(SHRN_Vd_Vn_IMM_VLSR),
    SLI_Vd_Vn_IMM_VLSL(SLI_Vd_Vn_IMM_VLSL),
    SQRSHRN2_Vd_Vn_IMM_VLSR(SQRSHRN2_Vd_Vn_IMM_VLSR),
    SQRSHRN_Vd_Vn_IMM_VLSR(SQRSHRN_Vd_Vn_IMM_VLSR),
    SQRSHRUN2_Vd_Vn_IMM_VLSR(SQRSHRUN2_Vd_Vn_IMM_VLSR),
    SQRSHRUN_Vd_Vn_IMM_VLSR(SQRSHRUN_Vd_Vn_IMM_VLSR),
    SQSHLU_Vd_Vn_IMM_VLSL(SQSHLU_Vd_Vn_IMM_VLSL),
    SQSHL_Vd_Vn_IMM_VLSL(SQSHL_Vd_Vn_IMM_VLSL),
    SQSHRN2_Vd_Vn_IMM_VLSR(SQSHRN2_Vd_Vn_IMM_VLSR),
    SQSHRN_Vd_Vn_IMM_VLSR(SQSHRN_Vd_Vn_IMM_VLSR),
    SQSHRUN2_Vd_Vn_IMM_VLSR(SQSHRUN2_Vd_Vn_IMM_VLSR),
    SQSHRUN_Vd_Vn_IMM_VLSR(SQSHRUN_Vd_Vn_IMM_VLSR),
    SRI_Vd_Vn_IMM_VLSR(SRI_Vd_Vn_IMM_VLSR),
    SRSHR_Vd_Vn_IMM_VLSR(SRSHR_Vd_Vn_IMM_VLSR),
    SRSRA_Vd_Vn_IMM_VLSR(SRSRA_Vd_Vn_IMM_VLSR),
    SSHLL2_Vd_Vn_IMM_VLSL(SSHLL2_Vd_Vn_IMM_VLSL),
    SSHLL_Vd_Vn_IMM_VLSL(SSHLL_Vd_Vn_IMM_VLSL),
    SSHR_Vd_Vn_IMM_VLSR(SSHR_Vd_Vn_IMM_VLSR),
    SSRA_Vd_Vn_IMM_VLSR(SSRA_Vd_Vn_IMM_VLSR),
    UCVTF_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S(UCVTF_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S),
    UCVTF_Vd_Vn_IMM_VLSR(UCVTF_Vd_Vn_IMM_VLSR),
    UQRSHRN2_Vd_Vn_IMM_VLSR(UQRSHRN2_Vd_Vn_IMM_VLSR),
    UQRSHRN_Vd_Vn_IMM_VLSR(UQRSHRN_Vd_Vn_IMM_VLSR),
    UQSHL_Vd_Vn_IMM_VLSL(UQSHL_Vd_Vn_IMM_VLSL),
    UQSHRN2_Vd_Vn_IMM_VLSR(UQSHRN2_Vd_Vn_IMM_VLSR),
    UQSHRN_Vd_Vn_IMM_VLSR(UQSHRN_Vd_Vn_IMM_VLSR),
    URSHR_Vd_Vn_IMM_VLSR(URSHR_Vd_Vn_IMM_VLSR),
    URSRA_Vd_Vn_IMM_VLSR(URSRA_Vd_Vn_IMM_VLSR),
    USHLL2_Vd_Vn_IMM_VLSL(USHLL2_Vd_Vn_IMM_VLSL),
    USHLL_Vd_Vn_IMM_VLSL(USHLL_Vd_Vn_IMM_VLSL),
    USHR_Vd_Vn_IMM_VLSR(USHR_Vd_Vn_IMM_VLSR),
    USRA_Vd_Vn_IMM_VLSR(USRA_Vd_Vn_IMM_VLSR),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ASIMDTBL {
    TBL_Vd_LVn_Vm(TBL_Vd_LVn_Vm),
    TBX_Vd_LVn_Vm(TBX_Vd_LVn_Vm),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ASISDDIFF {
    SQDMLAL_Sd_Sn_Sm(SQDMLAL_Sd_Sn_Sm),
    SQDMLSL_Sd_Sn_Sm(SQDMLSL_Sd_Sn_Sm),
    SQDMULL_Sd_Sn_Sm(SQDMULL_Sd_Sn_Sm),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ASISDELEM {
    FMLA_Sd_Sn_Em(FMLA_Sd_Sn_Em),
    FMLA_Sd_Sn_Em16(FMLA_Sd_Sn_Em16),
    FMLS_Sd_Sn_Em(FMLS_Sd_Sn_Em),
    FMLS_Sd_Sn_Em16(FMLS_Sd_Sn_Em16),
    FMULX_Sd_Sn_Em(FMULX_Sd_Sn_Em),
    FMULX_Sd_Sn_Em16(FMULX_Sd_Sn_Em16),
    FMUL_Sd_Sn_Em(FMUL_Sd_Sn_Em),
    FMUL_Sd_Sn_Em16(FMUL_Sd_Sn_Em16),
    SQDMLAL_Sd_Sn_Em16(SQDMLAL_Sd_Sn_Em16),
    SQDMLSL_Sd_Sn_Em16(SQDMLSL_Sd_Sn_Em16),
    SQDMULH_Sd_Sn_Em16(SQDMULH_Sd_Sn_Em16),
    SQDMULL_Sd_Sn_Em16(SQDMULL_Sd_Sn_Em16),
    SQRDMLAH_Sd_Sn_Em16(SQRDMLAH_Sd_Sn_Em16),
    SQRDMLSH_Sd_Sn_Em16(SQRDMLSH_Sd_Sn_Em16),
    SQRDMULH_Sd_Sn_Em16(SQRDMULH_Sd_Sn_Em16),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ASISDLSE {
    LD1_LVt_SIMD_ADDR_SIMPLE(LD1_LVt_SIMD_ADDR_SIMPLE),
    LD2_LVt_SIMD_ADDR_SIMPLE(LD2_LVt_SIMD_ADDR_SIMPLE),
    LD3_LVt_SIMD_ADDR_SIMPLE(LD3_LVt_SIMD_ADDR_SIMPLE),
    LD4_LVt_SIMD_ADDR_SIMPLE(LD4_LVt_SIMD_ADDR_SIMPLE),
    ST1_LVt_SIMD_ADDR_SIMPLE(ST1_LVt_SIMD_ADDR_SIMPLE),
    ST2_LVt_SIMD_ADDR_SIMPLE(ST2_LVt_SIMD_ADDR_SIMPLE),
    ST3_LVt_SIMD_ADDR_SIMPLE(ST3_LVt_SIMD_ADDR_SIMPLE),
    ST4_LVt_SIMD_ADDR_SIMPLE(ST4_LVt_SIMD_ADDR_SIMPLE),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ASISDLSEP {
    LD1_LVt_SIMD_ADDR_POST(LD1_LVt_SIMD_ADDR_POST),
    LD2_LVt_SIMD_ADDR_POST(LD2_LVt_SIMD_ADDR_POST),
    LD3_LVt_SIMD_ADDR_POST(LD3_LVt_SIMD_ADDR_POST),
    LD4_LVt_SIMD_ADDR_POST(LD4_LVt_SIMD_ADDR_POST),
    ST1_LVt_SIMD_ADDR_POST(ST1_LVt_SIMD_ADDR_POST),
    ST2_LVt_SIMD_ADDR_POST(ST2_LVt_SIMD_ADDR_POST),
    ST3_LVt_SIMD_ADDR_POST(ST3_LVt_SIMD_ADDR_POST),
    ST4_LVt_SIMD_ADDR_POST(ST4_LVt_SIMD_ADDR_POST),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ASISDLSO {
    LD1R_LVt_AL_SIMD_ADDR_SIMPLE(LD1R_LVt_AL_SIMD_ADDR_SIMPLE),
    LD1_LEt_SIMD_ADDR_SIMPLE(LD1_LEt_SIMD_ADDR_SIMPLE),
    LD2R_LVt_AL_SIMD_ADDR_SIMPLE(LD2R_LVt_AL_SIMD_ADDR_SIMPLE),
    LD2_LEt_SIMD_ADDR_SIMPLE(LD2_LEt_SIMD_ADDR_SIMPLE),
    LD3R_LVt_AL_SIMD_ADDR_SIMPLE(LD3R_LVt_AL_SIMD_ADDR_SIMPLE),
    LD3_LEt_SIMD_ADDR_SIMPLE(LD3_LEt_SIMD_ADDR_SIMPLE),
    LD4R_LVt_AL_SIMD_ADDR_SIMPLE(LD4R_LVt_AL_SIMD_ADDR_SIMPLE),
    LD4_LEt_SIMD_ADDR_SIMPLE(LD4_LEt_SIMD_ADDR_SIMPLE),
    ST1_LEt_SIMD_ADDR_SIMPLE(ST1_LEt_SIMD_ADDR_SIMPLE),
    ST2_LEt_SIMD_ADDR_SIMPLE(ST2_LEt_SIMD_ADDR_SIMPLE),
    ST3_LEt_SIMD_ADDR_SIMPLE(ST3_LEt_SIMD_ADDR_SIMPLE),
    ST4_LEt_SIMD_ADDR_SIMPLE(ST4_LEt_SIMD_ADDR_SIMPLE),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ASISDLSOP {
    LD1R_LVt_AL_SIMD_ADDR_POST(LD1R_LVt_AL_SIMD_ADDR_POST),
    LD1_LEt_SIMD_ADDR_POST(LD1_LEt_SIMD_ADDR_POST),
    LD2R_LVt_AL_SIMD_ADDR_POST(LD2R_LVt_AL_SIMD_ADDR_POST),
    LD2_LEt_SIMD_ADDR_POST(LD2_LEt_SIMD_ADDR_POST),
    LD3R_LVt_AL_SIMD_ADDR_POST(LD3R_LVt_AL_SIMD_ADDR_POST),
    LD3_LEt_SIMD_ADDR_POST(LD3_LEt_SIMD_ADDR_POST),
    LD4R_LVt_AL_SIMD_ADDR_POST(LD4R_LVt_AL_SIMD_ADDR_POST),
    LD4_LEt_SIMD_ADDR_POST(LD4_LEt_SIMD_ADDR_POST),
    ST1_LEt_SIMD_ADDR_POST(ST1_LEt_SIMD_ADDR_POST),
    ST2_LEt_SIMD_ADDR_POST(ST2_LEt_SIMD_ADDR_POST),
    ST3_LEt_SIMD_ADDR_POST(ST3_LEt_SIMD_ADDR_POST),
    ST4_LEt_SIMD_ADDR_POST(ST4_LEt_SIMD_ADDR_POST),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ASISDMISC {
    ABS_Sd_Sn(ABS_Sd_Sn),
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
    FRECPE_Sd_S_H_Sn_S_H(FRECPE_Sd_S_H_Sn_S_H),
    FRECPE_Sd_Sn(FRECPE_Sd_Sn),
    FRECPX_Sd_S_H_Sn_S_H(FRECPX_Sd_S_H_Sn_S_H),
    FRECPX_Sd_Sn(FRECPX_Sd_Sn),
    FRSQRTE_Sd_S_H_Sn_S_H(FRSQRTE_Sd_S_H_Sn_S_H),
    FRSQRTE_Sd_Sn(FRSQRTE_Sd_Sn),
    NEG_Sd_Sn(NEG_Sd_Sn),
    SCVTF_Sd_S_H_Sn_S_H(SCVTF_Sd_S_H_Sn_S_H),
    SCVTF_Sd_Sn(SCVTF_Sd_Sn),
    SQABS_Sd_Sn(SQABS_Sd_Sn),
    SQNEG_Sd_Sn(SQNEG_Sd_Sn),
    SQXTN_Sd_Sn(SQXTN_Sd_Sn),
    SQXTUN_Sd_Sn(SQXTUN_Sd_Sn),
    SUQADD_Sd_Sn(SUQADD_Sd_Sn),
    UCVTF_Sd_S_H_Sn_S_H(UCVTF_Sd_S_H_Sn_S_H),
    UCVTF_Sd_Sn(UCVTF_Sd_Sn),
    UQXTN_Sd_Sn(UQXTN_Sd_Sn),
    USQADD_Sd_Sn(USQADD_Sd_Sn),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ASISDONE {
    DUP_Sd_En(DUP_Sd_En),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ASISDPAIR {
    ADDP_Sd_Vn(ADDP_Sd_Vn),
    FADDP_Sd_S_S_Vn_V_2S(FADDP_Sd_S_S_Vn_V_2S),
    FADDP_Sd_Vn(FADDP_Sd_Vn),
    FMAXNMP_Sd_S_S_Vn_V_2S(FMAXNMP_Sd_S_S_Vn_V_2S),
    FMAXNMP_Sd_Vn(FMAXNMP_Sd_Vn),
    FMAXP_Sd_S_S_Vn_V_2S(FMAXP_Sd_S_S_Vn_V_2S),
    FMAXP_Sd_Vn(FMAXP_Sd_Vn),
    FMINNMP_Sd_S_S_Vn_V_2S(FMINNMP_Sd_S_S_Vn_V_2S),
    FMINNMP_Sd_Vn(FMINNMP_Sd_Vn),
    FMINP_Sd_S_S_Vn_V_2S(FMINP_Sd_S_S_Vn_V_2S),
    FMINP_Sd_Vn(FMINP_Sd_Vn),
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
    FABD_Sd_S_S_Sn_S_S_Sm_S_S(FABD_Sd_S_S_Sn_S_S_Sm_S_S),
    FABD_Sd_Sn_Sm(FABD_Sd_Sn_Sm),
    FACGE_Sd_S_S_Sn_S_S_Sm_S_S(FACGE_Sd_S_S_Sn_S_S_Sm_S_S),
    FACGE_Sd_Sn_Sm(FACGE_Sd_Sn_Sm),
    FACGT_Sd_S_S_Sn_S_S_Sm_S_S(FACGT_Sd_S_S_Sn_S_S_Sm_S_S),
    FACGT_Sd_Sn_Sm(FACGT_Sd_Sn_Sm),
    FCMEQ_Sd_S_S_Sn_S_S_Sm_S_S(FCMEQ_Sd_S_S_Sn_S_S_Sm_S_S),
    FCMEQ_Sd_Sn_Sm(FCMEQ_Sd_Sn_Sm),
    FCMGE_Sd_S_S_Sn_S_S_Sm_S_S(FCMGE_Sd_S_S_Sn_S_S_Sm_S_S),
    FCMGE_Sd_Sn_Sm(FCMGE_Sd_Sn_Sm),
    FCMGT_Sd_S_S_Sn_S_S_Sm_S_S(FCMGT_Sd_S_S_Sn_S_S_Sm_S_S),
    FCMGT_Sd_Sn_Sm(FCMGT_Sd_Sn_Sm),
    FMULX_Sd_S_S_Sn_S_S_Sm_S_S(FMULX_Sd_S_S_Sn_S_S_Sm_S_S),
    FMULX_Sd_Sn_Sm(FMULX_Sd_Sn_Sm),
    FRECPS_Sd_S_S_Sn_S_S_Sm_S_S(FRECPS_Sd_S_S_Sn_S_S_Sm_S_S),
    FRECPS_Sd_Sn_Sm(FRECPS_Sd_Sn_Sm),
    FRSQRTS_Sd_S_S_Sn_S_S_Sm_S_S(FRSQRTS_Sd_S_S_Sn_S_S_Sm_S_S),
    FRSQRTS_Sd_Sn_Sm(FRSQRTS_Sd_Sn_Sm),
    SQADD_Sd_Sn_Sm(SQADD_Sd_Sn_Sm),
    SQDMULH_Sd_Sn_Sm(SQDMULH_Sd_Sn_Sm),
    SQRDMULH_Sd_Sn_Sm(SQRDMULH_Sd_Sn_Sm),
    SQRSHL_Sd_Sn_Sm(SQRSHL_Sd_Sn_Sm),
    SQSHL_Sd_Sn_Sm(SQSHL_Sd_Sn_Sm),
    SQSUB_Sd_Sn_Sm(SQSUB_Sd_Sn_Sm),
    SRSHL_Sd_Sn_Sm(SRSHL_Sd_Sn_Sm),
    SSHL_Sd_Sn_Sm(SSHL_Sd_Sn_Sm),
    SUB_Sd_Sn_Sm(SUB_Sd_Sn_Sm),
    UQADD_Sd_Sn_Sm(UQADD_Sd_Sn_Sm),
    UQRSHL_Sd_Sn_Sm(UQRSHL_Sd_Sn_Sm),
    UQSHL_Sd_Sn_Sm(UQSHL_Sd_Sn_Sm),
    UQSUB_Sd_Sn_Sm(UQSUB_Sd_Sn_Sm),
    URSHL_Sd_Sn_Sm(URSHL_Sd_Sn_Sm),
    USHL_Sd_Sn_Sm(USHL_Sd_Sn_Sm),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ASISDSHF {
    FCVTZS_Sd_S_S_Sn_S_S_IMM_VLSR_S_S(FCVTZS_Sd_S_S_Sn_S_S_IMM_VLSR_S_S),
    FCVTZS_Sd_Sn_IMM_VLSR(FCVTZS_Sd_Sn_IMM_VLSR),
    FCVTZU_Sd_S_S_Sn_S_S_IMM_VLSR_S_S(FCVTZU_Sd_S_S_Sn_S_S_IMM_VLSR_S_S),
    FCVTZU_Sd_Sn_IMM_VLSR(FCVTZU_Sd_Sn_IMM_VLSR),
    SCVTF_Sd_S_S_Sn_S_S_IMM_VLSR_S_S(SCVTF_Sd_S_S_Sn_S_S_IMM_VLSR_S_S),
    SCVTF_Sd_Sn_IMM_VLSR(SCVTF_Sd_Sn_IMM_VLSR),
    SHL_Sd_Sn_IMM_VLSL(SHL_Sd_Sn_IMM_VLSL),
    SLI_Sd_Sn_IMM_VLSL(SLI_Sd_Sn_IMM_VLSL),
    SQRSHRN_Sd_Sn_IMM_VLSR(SQRSHRN_Sd_Sn_IMM_VLSR),
    SQRSHRUN_Sd_Sn_IMM_VLSR(SQRSHRUN_Sd_Sn_IMM_VLSR),
    SQSHLU_Sd_Sn_IMM_VLSL(SQSHLU_Sd_Sn_IMM_VLSL),
    SQSHL_Sd_Sn_IMM_VLSL(SQSHL_Sd_Sn_IMM_VLSL),
    SQSHRN_Sd_Sn_IMM_VLSR(SQSHRN_Sd_Sn_IMM_VLSR),
    SQSHRUN_Sd_Sn_IMM_VLSR(SQSHRUN_Sd_Sn_IMM_VLSR),
    SRI_Sd_Sn_IMM_VLSR(SRI_Sd_Sn_IMM_VLSR),
    SRSHR_Sd_Sn_IMM_VLSR(SRSHR_Sd_Sn_IMM_VLSR),
    SRSRA_Sd_Sn_IMM_VLSR(SRSRA_Sd_Sn_IMM_VLSR),
    SSHR_Sd_Sn_IMM_VLSR(SSHR_Sd_Sn_IMM_VLSR),
    SSRA_Sd_Sn_IMM_VLSR(SSRA_Sd_Sn_IMM_VLSR),
    UCVTF_Sd_S_S_Sn_S_S_IMM_VLSR_S_S(UCVTF_Sd_S_S_Sn_S_S_IMM_VLSR_S_S),
    UCVTF_Sd_Sn_IMM_VLSR(UCVTF_Sd_Sn_IMM_VLSR),
    UQRSHRN_Sd_Sn_IMM_VLSR(UQRSHRN_Sd_Sn_IMM_VLSR),
    UQSHL_Sd_Sn_IMM_VLSL(UQSHL_Sd_Sn_IMM_VLSL),
    UQSHRN_Sd_Sn_IMM_VLSR(UQSHRN_Sd_Sn_IMM_VLSR),
    URSHR_Sd_Sn_IMM_VLSR(URSHR_Sd_Sn_IMM_VLSR),
    URSRA_Sd_Sn_IMM_VLSR(URSRA_Sd_Sn_IMM_VLSR),
    USHR_Sd_Sn_IMM_VLSR(USHR_Sd_Sn_IMM_VLSR),
    USRA_Sd_Sn_IMM_VLSR(USRA_Sd_Sn_IMM_VLSR),
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
    DRPS(DRPS),
    ERET(ERET),
    ERETAA(ERETAA),
    ERETAB(ERETAB),
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
pub enum CONDBRANCH {
    BC__ADDR_PCREL19(BC__ADDR_PCREL19),
    B__ADDR_PCREL19(B__ADDR_PCREL19),
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
pub enum DP_1SRC {
    AUTDA_Rd_Rn_SP(AUTDA_Rd_Rn_SP),
    AUTDB_Rd_Rn_SP(AUTDB_Rd_Rn_SP),
    AUTDZA_Rd(AUTDZA_Rd),
    AUTDZB_Rd(AUTDZB_Rd),
    AUTIA_Rd_Rn_SP(AUTIA_Rd_Rn_SP),
    AUTIB_Rd_Rn_SP(AUTIB_Rd_Rn_SP),
    AUTIZA_Rd(AUTIZA_Rd),
    AUTIZB_Rd(AUTIZB_Rd),
    CLS_Rd_Rn(CLS_Rd_Rn),
    CLZ_Rd_Rn(CLZ_Rd_Rn),
    PACDA_Rd_Rn_SP(PACDA_Rd_Rn_SP),
    PACDB_Rd_Rn_SP(PACDB_Rd_Rn_SP),
    PACDZA_Rd(PACDZA_Rd),
    PACDZB_Rd(PACDZB_Rd),
    PACIA_Rd_Rn_SP(PACIA_Rd_Rn_SP),
    PACIB_Rd_Rn_SP(PACIB_Rd_Rn_SP),
    PACIZA_Rd(PACIZA_Rd),
    PACIZB_Rd(PACIZB_Rd),
    RBIT_Rd_Rn(RBIT_Rd_Rn),
    REV16_Rd_Rn(REV16_Rd_Rn),
    REV32_Rd_Rn(REV32_Rd_Rn),
    REV_Rd_Rn(REV_Rd_Rn),
    REV_Rd_X_Rn_X(REV_Rd_X_Rn_X),
    XPACD_Rd(XPACD_Rd),
    XPACI_Rd(XPACI_Rd),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum DP_2SRC {
    ASRV_Rd_Rn_Rm(ASRV_Rd_Rn_Rm),
    CRC32B_Rd_Rn_Rm(CRC32B_Rd_Rn_Rm),
    CRC32CB_Rd_Rn_Rm(CRC32CB_Rd_Rn_Rm),
    CRC32CH_Rd_Rn_Rm(CRC32CH_Rd_Rn_Rm),
    CRC32CW_Rd_Rn_Rm(CRC32CW_Rd_Rn_Rm),
    CRC32CX_Rd_Rn_Rm(CRC32CX_Rd_Rn_Rm),
    CRC32H_Rd_Rn_Rm(CRC32H_Rd_Rn_Rm),
    CRC32W_Rd_Rn_Rm(CRC32W_Rd_Rn_Rm),
    CRC32X_Rd_Rn_Rm(CRC32X_Rd_Rn_Rm),
    GMI_Rd_Rn_SP_Rm(GMI_Rd_Rn_SP_Rm),
    IRG_Rd_SP_Rn_SP_Rm(IRG_Rd_SP_Rn_SP_Rm),
    LSLV_Rd_Rn_Rm(LSLV_Rd_Rn_Rm),
    LSRV_Rd_Rn_Rm(LSRV_Rd_Rn_Rm),
    PACGA_Rd_Rn_Rm_SP(PACGA_Rd_Rn_Rm_SP),
    RORV_Rd_Rn_Rm(RORV_Rd_Rn_Rm),
    SDIV_Rd_Rn_Rm(SDIV_Rd_Rn_Rm),
    SUBPS_Rd_Rn_SP_Rm_SP(SUBPS_Rd_Rn_SP_Rm_SP),
    SUBP_Rd_Rn_SP_Rm_SP(SUBP_Rd_Rn_SP_Rm_SP),
    UDIV_Rd_Rn_Rm(UDIV_Rd_Rn_Rm),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum DP_3SRC {
    MADD_Rd_Rn_Rm_Ra(MADD_Rd_Rn_Rm_Ra),
    MSUB_Rd_Rn_Rm_Ra(MSUB_Rd_Rn_Rm_Ra),
    SMADDL_Rd_Rn_Rm_Ra(SMADDL_Rd_Rn_Rm_Ra),
    SMSUBL_Rd_Rn_Rm_Ra(SMSUBL_Rd_Rn_Rm_Ra),
    SMULH_Rd_Rn_Rm(SMULH_Rd_Rn_Rm),
    UMADDL_Rd_Rn_Rm_Ra(UMADDL_Rd_Rn_Rm_Ra),
    UMSUBL_Rd_Rn_Rm_Ra(UMSUBL_Rd_Rn_Rm_Ra),
    UMULH_Rd_Rn_Rm(UMULH_Rd_Rn_Rm),
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
    FABS_Fd_Fn(FABS_Fd_Fn),
    FABS_Fd_S_S_Fn_S_S(FABS_Fd_S_S_Fn_S_S),
    FCVT_Fd_Fn(FCVT_Fd_Fn),
    FMOV_Fd_Fn(FMOV_Fd_Fn),
    FMOV_Fd_S_S_Fn_S_S(FMOV_Fd_S_S_Fn_S_S),
    FNEG_Fd_Fn(FNEG_Fd_Fn),
    FNEG_Fd_S_S_Fn_S_S(FNEG_Fd_S_S_Fn_S_S),
    FRINT32X_Fd_Fn(FRINT32X_Fd_Fn),
    FRINT32Z_Fd_Fn(FRINT32Z_Fd_Fn),
    FRINT64X_Fd_Fn(FRINT64X_Fd_Fn),
    FRINT64Z_Fd_Fn(FRINT64Z_Fd_Fn),
    FRINTA_Fd_Fn(FRINTA_Fd_Fn),
    FRINTA_Fd_S_S_Fn_S_S(FRINTA_Fd_S_S_Fn_S_S),
    FRINTI_Fd_Fn(FRINTI_Fd_Fn),
    FRINTI_Fd_S_S_Fn_S_S(FRINTI_Fd_S_S_Fn_S_S),
    FRINTM_Fd_Fn(FRINTM_Fd_Fn),
    FRINTM_Fd_S_S_Fn_S_S(FRINTM_Fd_S_S_Fn_S_S),
    FRINTN_Fd_Fn(FRINTN_Fd_Fn),
    FRINTN_Fd_S_S_Fn_S_S(FRINTN_Fd_S_S_Fn_S_S),
    FRINTP_Fd_Fn(FRINTP_Fd_Fn),
    FRINTP_Fd_S_S_Fn_S_S(FRINTP_Fd_S_S_Fn_S_S),
    FRINTX_Fd_Fn(FRINTX_Fd_Fn),
    FRINTX_Fd_S_S_Fn_S_S(FRINTX_Fd_S_S_Fn_S_S),
    FRINTZ_Fd_Fn(FRINTZ_Fd_Fn),
    FRINTZ_Fd_S_S_Fn_S_S(FRINTZ_Fd_S_S_Fn_S_S),
    FSQRT_Fd_Fn(FSQRT_Fd_Fn),
    FSQRT_Fd_S_S_Fn_S_S(FSQRT_Fd_S_S_Fn_S_S),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum FLOATDP2 {
    FADD_Fd_Fn_Fm(FADD_Fd_Fn_Fm),
    FADD_Fd_S_S_Fn_S_S_Fm_S_S(FADD_Fd_S_S_Fn_S_S_Fm_S_S),
    FDIV_Fd_Fn_Fm(FDIV_Fd_Fn_Fm),
    FDIV_Fd_S_S_Fn_S_S_Fm_S_S(FDIV_Fd_S_S_Fn_S_S_Fm_S_S),
    FMAXNM_Fd_Fn_Fm(FMAXNM_Fd_Fn_Fm),
    FMAXNM_Fd_S_S_Fn_S_S_Fm_S_S(FMAXNM_Fd_S_S_Fn_S_S_Fm_S_S),
    FMAX_Fd_Fn_Fm(FMAX_Fd_Fn_Fm),
    FMAX_Fd_S_S_Fn_S_S_Fm_S_S(FMAX_Fd_S_S_Fn_S_S_Fm_S_S),
    FMINNM_Fd_Fn_Fm(FMINNM_Fd_Fn_Fm),
    FMINNM_Fd_S_S_Fn_S_S_Fm_S_S(FMINNM_Fd_S_S_Fn_S_S_Fm_S_S),
    FMIN_Fd_Fn_Fm(FMIN_Fd_Fn_Fm),
    FMIN_Fd_S_S_Fn_S_S_Fm_S_S(FMIN_Fd_S_S_Fn_S_S_Fm_S_S),
    FMUL_Fd_Fn_Fm(FMUL_Fd_Fn_Fm),
    FMUL_Fd_S_S_Fn_S_S_Fm_S_S(FMUL_Fd_S_S_Fn_S_S_Fm_S_S),
    FNMUL_Fd_Fn_Fm(FNMUL_Fd_Fn_Fm),
    FNMUL_Fd_S_S_Fn_S_S_Fm_S_S(FNMUL_Fd_S_S_Fn_S_S_Fm_S_S),
    FSUB_Fd_Fn_Fm(FSUB_Fd_Fn_Fm),
    FSUB_Fd_S_S_Fn_S_S_Fm_S_S(FSUB_Fd_S_S_Fn_S_S_Fm_S_S),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum FLOATDP3 {
    FMADD_Fd_Fn_Fm_Fa(FMADD_Fd_Fn_Fm_Fa),
    FMADD_Fd_S_S_Fn_S_S_Fm_S_S_Fa_S_S(FMADD_Fd_S_S_Fn_S_S_Fm_S_S_Fa_S_S),
    FMSUB_Fd_Fn_Fm_Fa(FMSUB_Fd_Fn_Fm_Fa),
    FMSUB_Fd_S_S_Fn_S_S_Fm_S_S_Fa_S_S(FMSUB_Fd_S_S_Fn_S_S_Fm_S_S_Fa_S_S),
    FNMADD_Fd_Fn_Fm_Fa(FNMADD_Fd_Fn_Fm_Fa),
    FNMADD_Fd_S_S_Fn_S_S_Fm_S_S_Fa_S_S(FNMADD_Fd_S_S_Fn_S_S_Fm_S_S_Fa_S_S),
    FNMSUB_Fd_Fn_Fm_Fa(FNMSUB_Fd_Fn_Fm_Fa),
    FNMSUB_Fd_S_S_Fn_S_S_Fm_S_S_Fa_S_S(FNMSUB_Fd_S_S_Fn_S_S_Fm_S_S_Fa_S_S),
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
pub enum LDSTEXCL {
    LDAPRB_Rt_ADDR_SIMPLE(LDAPRB_Rt_ADDR_SIMPLE),
    LDAPRH_Rt_ADDR_SIMPLE(LDAPRH_Rt_ADDR_SIMPLE),
    LDAPR_Rt_ADDR_SIMPLE(LDAPR_Rt_ADDR_SIMPLE),
    LDARB_Rt_ADDR_SIMPLE(LDARB_Rt_ADDR_SIMPLE),
    LDARH_Rt_ADDR_SIMPLE(LDARH_Rt_ADDR_SIMPLE),
    LDAR_Rt_ADDR_SIMPLE(LDAR_Rt_ADDR_SIMPLE),
    LDAXP_Rt_Rt2_ADDR_SIMPLE(LDAXP_Rt_Rt2_ADDR_SIMPLE),
    LDAXRB_Rt_ADDR_SIMPLE(LDAXRB_Rt_ADDR_SIMPLE),
    LDAXRH_Rt_ADDR_SIMPLE(LDAXRH_Rt_ADDR_SIMPLE),
    LDAXR_Rt_ADDR_SIMPLE(LDAXR_Rt_ADDR_SIMPLE),
    LDGM_Rt_ADDR_SIMPLE(LDGM_Rt_ADDR_SIMPLE),
    LDLARB_Rt_ADDR_SIMPLE(LDLARB_Rt_ADDR_SIMPLE),
    LDLARH_Rt_ADDR_SIMPLE(LDLARH_Rt_ADDR_SIMPLE),
    LDLAR_Rt_ADDR_SIMPLE(LDLAR_Rt_ADDR_SIMPLE),
    LDXP_Rt_Rt2_ADDR_SIMPLE(LDXP_Rt_Rt2_ADDR_SIMPLE),
    LDXRB_Rt_ADDR_SIMPLE(LDXRB_Rt_ADDR_SIMPLE),
    LDXRH_Rt_ADDR_SIMPLE(LDXRH_Rt_ADDR_SIMPLE),
    LDXR_Rt_ADDR_SIMPLE(LDXR_Rt_ADDR_SIMPLE),
    STGM_Rt_ADDR_SIMPLE(STGM_Rt_ADDR_SIMPLE),
    STLLRB_Rt_ADDR_SIMPLE(STLLRB_Rt_ADDR_SIMPLE),
    STLLRH_Rt_ADDR_SIMPLE(STLLRH_Rt_ADDR_SIMPLE),
    STLLR_Rt_ADDR_SIMPLE(STLLR_Rt_ADDR_SIMPLE),
    STLRB_Rt_ADDR_SIMPLE(STLRB_Rt_ADDR_SIMPLE),
    STLRH_Rt_ADDR_SIMPLE(STLRH_Rt_ADDR_SIMPLE),
    STLR_Rt_ADDR_SIMPLE(STLR_Rt_ADDR_SIMPLE),
    STLXP_Rs_Rt_Rt2_ADDR_SIMPLE(STLXP_Rs_Rt_Rt2_ADDR_SIMPLE),
    STLXRB_Rs_Rt_ADDR_SIMPLE(STLXRB_Rs_Rt_ADDR_SIMPLE),
    STLXRH_Rs_Rt_ADDR_SIMPLE(STLXRH_Rs_Rt_ADDR_SIMPLE),
    STLXR_Rs_Rt_ADDR_SIMPLE(STLXR_Rs_Rt_ADDR_SIMPLE),
    STXP_Rs_Rt_Rt2_ADDR_SIMPLE(STXP_Rs_Rt_Rt2_ADDR_SIMPLE),
    STXRB_Rs_Rt_ADDR_SIMPLE(STXRB_Rs_Rt_ADDR_SIMPLE),
    STXRH_Rs_Rt_ADDR_SIMPLE(STXRH_Rs_Rt_ADDR_SIMPLE),
    STXR_Rs_Rt_ADDR_SIMPLE(STXR_Rs_Rt_ADDR_SIMPLE),
    STZGM_Rt_ADDR_SIMPLE(STZGM_Rt_ADDR_SIMPLE),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LDSTNAPAIR_OFFS {
    LDNP_Ft_Ft2_ADDR_SIMM7(LDNP_Ft_Ft2_ADDR_SIMM7),
    LDNP_Rt_Rt2_ADDR_SIMM7(LDNP_Rt_Rt2_ADDR_SIMM7),
    STNP_Ft_Ft2_ADDR_SIMM7(STNP_Ft_Ft2_ADDR_SIMM7),
    STNP_Rt_Rt2_ADDR_SIMM7(STNP_Rt_Rt2_ADDR_SIMM7),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LDSTPAIR_INDEXED {
    LDPSW_Rt_X_Rt2_X_ADDR_SIMM7_S_S(LDPSW_Rt_X_Rt2_X_ADDR_SIMM7_S_S),
    LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S(LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S),
    LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S(LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S),
    STGP_Rt_X_Rt2_X_ADDR_SIMM11_imm_tag(STGP_Rt_X_Rt2_X_ADDR_SIMM11_imm_tag),
    STP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S(STP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S),
    STP_Rt_W_Rt2_W_ADDR_SIMM7_S_S(STP_Rt_W_Rt2_W_ADDR_SIMM7_S_S),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LDSTPAIR_OFF {
    LDPSW_Rt_Rt2_ADDR_SIMM7(LDPSW_Rt_Rt2_ADDR_SIMM7),
    LDP_Ft_Ft2_ADDR_SIMM7(LDP_Ft_Ft2_ADDR_SIMM7),
    LDP_Rt_Rt2_ADDR_SIMM7(LDP_Rt_Rt2_ADDR_SIMM7),
    STGP_Rt_Rt2_ADDR_SIMM11(STGP_Rt_Rt2_ADDR_SIMM11),
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
    ST2G_Rt_SP_X_ADDR_SIMM13_imm_tag(ST2G_Rt_SP_X_ADDR_SIMM13_imm_tag),
    STG_Rt_SP_X_ADDR_SIMM13_imm_tag(STG_Rt_SP_X_ADDR_SIMM13_imm_tag),
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
    FLOATDP1(FLOATDP1),
    FLOATDP2(FLOATDP2),
    FLOATDP3(FLOATDP3),
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
