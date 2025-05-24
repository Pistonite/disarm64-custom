#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub _op_5: u32,
    #[bits(5)]
    pub rt2: u32,
    #[bits(7)]
    pub imm7: u32,
    #[bits(2)]
    pub _op_22: u32,
    #[bits(1)]
    pub index2: u32,
    #[bits(7)]
    pub _op_25: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STR_SME_ZA_array_off4_SME_ADDR_RI_U4xVL {
    #[bits(4)]
    pub imm4_0: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(3)]
    pub _op_10: u32,
    #[bits(2)]
    pub sme_rv: u32,
    #[bits(17)]
    pub _op_15: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STR_SME_ZT0_SIMD_ADDR_SIMPLE {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STR_Rt_ADDR_REGOFF {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STR_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STR_Rt_ADDR_UIMM12 {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(12)]
    pub imm12: u32,
    #[bits(10)]
    pub _op_22: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STR_SVE_Pt_SVE_ADDR_RI_S9xVL {
    #[bits(4)]
    pub sve_pt: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STR_SVE_PNt_SVE_ADDR_RI_S9xVL {
    #[bits(4)]
    pub sve_pt: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STR_SVE_Zt_SVE_ADDR_RI_S9xVL {
    #[bits(5)]
    pub sve_zt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STR_Ft_ADDR_REGOFF {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STR_Ft_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STR_Ft_ADDR_UIMM12 {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(12)]
    pub imm12: u32,
    #[bits(10)]
    pub _op_22: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STRB_Rt_ADDR_REGOFF {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STRB_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STRB_Rt_ADDR_UIMM12 {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(12)]
    pub imm12: u32,
    #[bits(10)]
    pub _op_22: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STRH_Rt_ADDR_REGOFF {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STRH_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STRH_Rt_ADDR_UIMM12 {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(12)]
    pub imm12: u32,
    #[bits(10)]
    pub _op_22: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STTR_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STTRB_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STTRH_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STUR_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STUR_Ft_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STURB_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STURH_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STXP_Rs_Rt_Rt2_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub _op_5: u32,
    #[bits(5)]
    pub rt2: u32,
    #[bits(1)]
    pub _op_15: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STXR_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STXRB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STXRH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STZ2G_Rt_SP_ADDR_SIMM13 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STZ2G_Rt_SP_X_ADDR_SIMM13_imm_tag {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STZG_Rt_SP_ADDR_SIMM13 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STZG_Rt_SP_X_ADDR_SIMM13_imm_tag {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STZGM_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SUB_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm {
    #[bits(3)]
    pub imm3_0: u32,
    #[bits(2)]
    pub _op_3: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub _op_10: u32,
    #[bits(2)]
    pub sme_rv: u32,
    #[bits(1)]
    pub _op_15: u32,
    #[bits(4)]
    pub sme_zm: u32,
    #[bits(12)]
    pub _op_20: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SUB_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_S_SME_Zm_S_S {
    #[bits(3)]
    pub imm3_0: u32,
    #[bits(2)]
    pub _op_3: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub _op_10: u32,
    #[bits(2)]
    pub sme_rv: u32,
    #[bits(1)]
    pub _op_15: u32,
    #[bits(4)]
    pub sme_zm: u32,
    #[bits(12)]
    pub _op_20: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SUB_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2 {
    #[bits(3)]
    pub imm3_0: u32,
    #[bits(3)]
    pub _op_3: u32,
    #[bits(4)]
    pub sme_zn2: u32,
    #[bits(3)]
    pub _op_10: u32,
    #[bits(2)]
    pub sme_rv: u32,
    #[bits(2)]
    pub _op_15: u32,
    #[bits(4)]
    pub sme_zm2: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SUB_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4 {
    #[bits(3)]
    pub imm3_0: u32,
    #[bits(4)]
    pub _op_3: u32,
    #[bits(3)]
    pub sme_zn4: u32,
    #[bits(3)]
    pub _op_10: u32,
    #[bits(2)]
    pub sme_rv: u32,
    #[bits(3)]
    pub _op_15: u32,
    #[bits(3)]
    pub sme_zm4: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SUB_SME_ZA_array_off3_0_SME_Znx2 {
    #[bits(3)]
    pub imm3_0: u32,
    #[bits(3)]
    pub _op_3: u32,
    #[bits(4)]
    pub sme_zn2: u32,
    #[bits(3)]
    pub _op_10: u32,
    #[bits(2)]
    pub sme_rv: u32,
    #[bits(17)]
    pub _op_15: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SUB_SME_ZA_array_off3_0_SME_Znx4 {
    #[bits(3)]
    pub imm3_0: u32,
    #[bits(4)]
    pub _op_3: u32,
    #[bits(3)]
    pub sme_zn4: u32,
    #[bits(3)]
    pub _op_10: u32,
    #[bits(2)]
    pub sme_rv: u32,
    #[bits(17)]
    pub _op_15: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SUB_Rd_SP_Rn_SP_AIMM {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(12)]
    pub imm12: u32,
    #[bits(2)]
    pub shift: u32,
    #[bits(8)]
    pub _op_24: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SUB_Rd_Rn_Rm_SFT {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SUB_Rd_SP_Rn_SP_Rm_EXT {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zm_5: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(19)]
    pub _op_13: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SUB_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SUB_SVE_Zd_SVE_Zd_SVE_AIMM {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(9)]
    pub sve_imm9: u32,
    #[bits(18)]
    pub _op_14: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SUB_Vd_Vn_Vm {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub rm: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SUB_Sd_Sn_Sm {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub rm: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SUBG_Rd_SP_Rn_SP_UIMM10_UIMM4_ADDG {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(4)]
    pub imm4_10: u32,
    #[bits(2)]
    pub _op_14: u32,
    #[bits(6)]
    pub immr: u32,
    #[bits(10)]
    pub _op_22: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SUBHN_Vd_Vn_Vm {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub rm: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SUBHN2_Vd_Vn_Vm {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub rm: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SUBHNB_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SUBHNT_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SUBP_Rd_Rn_SP_Rm_SP {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub rm: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SUBPS_Rd_Rn_SP_Rm_SP {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub rm: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SUBR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zm_5: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(19)]
    pub _op_13: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SUBR_SVE_Zd_SVE_Zd_SVE_AIMM {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(9)]
    pub sve_imm9: u32,
    #[bits(18)]
    pub _op_14: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SUBS_Rd_Rn_SP_AIMM {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(12)]
    pub imm12: u32,
    #[bits(2)]
    pub shift: u32,
    #[bits(8)]
    pub _op_24: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SUBS_Rd_Rn_Rm_SFT {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SUBS_Rd_Rn_SP_Rm_EXT {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SXTB_SVE_Zd_SVE_Pg3_SVE_Zn {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(19)]
    pub _op_13: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SXTH_SVE_Zd_SVE_Pg3_SVE_Zn {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(19)]
    pub _op_13: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SXTW_SVE_Zd_SVE_Pg3_SVE_Zn {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(19)]
    pub _op_13: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TBL_SVE_Zd_SVE_ZnxN_SVE_Zm_16 {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TBL_SVE_Zd_S_B_SVE_ZnxN_S_B_SVE_Zm_16_S_B {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TBL_Vd_LVn_Vm {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub rm: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TBNZ_Rt_BIT_NUM_ADDR_PCREL14 {
    #[bits(5)]
    pub rt: u32,
    #[bits(14)]
    pub imm14: u32,
    #[bits(5)]
    pub b40: u32,
    #[bits(7)]
    pub _op_24: u32,
    #[bits(1)]
    pub b5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TBX_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TBX_Vd_LVn_Vm {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub rm: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TBZ_Rt_BIT_NUM_ADDR_PCREL14 {
    #[bits(5)]
    pub rt: u32,
    #[bits(14)]
    pub imm14: u32,
    #[bits(5)]
    pub b40: u32,
    #[bits(7)]
    pub _op_24: u32,
    #[bits(1)]
    pub b5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TRN1_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TRN1_SVE_Pd_SVE_Pn_SVE_Pm {
    #[bits(4)]
    pub sve_pd: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(4)]
    pub sve_pn: u32,
    #[bits(7)]
    pub _op_9: u32,
    #[bits(4)]
    pub sve_pm: u32,
    #[bits(12)]
    pub _op_20: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TRN1_SVE_Zd_S_B_SVE_Zn_S_B_SVE_Zm_16_S_B {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TRN1_Vd_Vn_Vm {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub rm: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TRN2_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TRN2_SVE_Pd_SVE_Pn_SVE_Pm {
    #[bits(4)]
    pub sve_pd: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(4)]
    pub sve_pn: u32,
    #[bits(7)]
    pub _op_9: u32,
    #[bits(4)]
    pub sve_pm: u32,
    #[bits(12)]
    pub _op_20: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TRN2_SVE_Zd_S_B_SVE_Zn_S_B_SVE_Zm_16_S_B {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TRN2_Vd_Vn_Vm {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub rm: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TSTART_Rd {
    #[bits(5)]
    pub rd: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TTEST_Rd {
    #[bits(5)]
    pub rd: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UADDL_Vd_Vn_Vm {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub rm: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UADDL2_Vd_Vn_Vm {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub rm: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UADDLB_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UADDLP_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UADDLT_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UADDLV_Fd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UADDV_SVE_Vd_SVE_Pg3_SVE_Zn {
    #[bits(5)]
    pub sve_vd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(19)]
    pub _op_13: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UADDW_Vd_Vn_Vm {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub rm: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UADDW2_Vd_Vn_Vm {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub rm: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UADDWB_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UADDWT_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UBFM_Rd_Rn_IMMR_IMMS {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(6)]
    pub imm6_10: u32,
    #[bits(6)]
    pub immr: u32,
    #[bits(10)]
    pub _op_22: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UCLAMP_SME_Zdnx2_SVE_Zn_SVE_Zm_16 {
    #[bits(1)]
    pub _op_0: u32,
    #[bits(4)]
    pub sme_zdn2: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UCLAMP_SME_Zdnx4_SVE_Zn_SVE_Zm_16 {
    #[bits(2)]
    pub _op_0: u32,
    #[bits(3)]
    pub sme_zdn4: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UCLAMP_SVE_Zd_SVE_Zn_SVE_Zm_16 {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UCVTF_SME_Zdnx2_SME_Znx2 {
    #[bits(1)]
    pub _op_0: u32,
    #[bits(4)]
    pub sme_zdn2: u32,
    #[bits(1)]
    pub _op_5: u32,
    #[bits(4)]
    pub sme_zn2: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UCVTF_SME_Zdnx4_SME_Znx4 {
    #[bits(2)]
    pub _op_0: u32,
    #[bits(3)]
    pub sme_zdn4: u32,
    #[bits(2)]
    pub _op_5: u32,
    #[bits(3)]
    pub sme_zn4: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UCVTF_SVE_Zd_SVE_Pg3_SVE_Zn {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(19)]
    pub _op_13: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UCVTF_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_S {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(19)]
    pub _op_13: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_S {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(19)]
    pub _op_13: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UCVTF_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_D {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(19)]
    pub _op_13: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_H {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(19)]
    pub _op_13: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_D {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(19)]
    pub _op_13: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UCVTF_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_D {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(19)]
    pub _op_13: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UCVTF_Fd_Rn_FBITS {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(6)]
    pub scale: u32,
    #[bits(16)]
    pub _op_16: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UCVTF_Fd_S_D_Rn_W_FBITS_imm_1_32 {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(6)]
    pub scale: u32,
    #[bits(16)]
    pub _op_16: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UCVTF_Fd_Rn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UCVTF_Fd_S_D_Rn_W {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UCVTF_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UCVTF_Sd_Sn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UCVTF_Vd_V_4H_Vn_V_4H {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UCVTF_Sd_S_H_Sn_S_H {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UCVTF_Vd_Vn_IMM_VLSR {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UCVTF_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UCVTF_Sd_Sn_IMM_VLSR {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UCVTF_Sd_S_S_Sn_S_S_IMM_VLSR_S_S {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum AARCH64_MISC {
    TSTART_Rd(TSTART_Rd),
    TTEST_Rd(TTEST_Rd),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ADDSUB_CARRY {
    SBCS_Rd_Rn_Rm(SBCS_Rd_Rn_Rm),
    SBC_Rd_Rn_Rm(SBC_Rd_Rn_Rm),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ADDSUB_EXT {
    ADDS_Rd_Rn_SP_Rm_EXT(ADDS_Rd_Rn_SP_Rm_EXT),
    ADD_Rd_SP_Rn_SP_Rm_EXT(ADD_Rd_SP_Rn_SP_Rm_EXT),
    SUBS_Rd_Rn_SP_Rm_EXT(SUBS_Rd_Rn_SP_Rm_EXT),
    SUB_Rd_SP_Rn_SP_Rm_EXT(SUB_Rd_SP_Rn_SP_Rm_EXT),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ADDSUB_IMM {
    ADDG_Rd_SP_Rn_SP_UIMM10_UIMM4_ADDG(ADDG_Rd_SP_Rn_SP_UIMM10_UIMM4_ADDG),
    ADDS_Rd_Rn_SP_AIMM(ADDS_Rd_Rn_SP_AIMM),
    ADD_Rd_SP_Rn_SP_AIMM(ADD_Rd_SP_Rn_SP_AIMM),
    SUBG_Rd_SP_Rn_SP_UIMM10_UIMM4_ADDG(SUBG_Rd_SP_Rn_SP_UIMM10_UIMM4_ADDG),
    SUBS_Rd_Rn_SP_AIMM(SUBS_Rd_Rn_SP_AIMM),
    SUB_Rd_SP_Rn_SP_AIMM(SUB_Rd_SP_Rn_SP_AIMM),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ADDSUB_SHIFT {
    ADDS_Rd_Rn_Rm_SFT(ADDS_Rd_Rn_Rm_SFT),
    ADD_Rd_Rn_Rm_SFT(ADD_Rd_Rn_Rm_SFT),
    SUBS_Rd_Rn_Rm_SFT(SUBS_Rd_Rn_Rm_SFT),
    SUB_Rd_Rn_Rm_SFT(SUB_Rd_Rn_Rm_SFT),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ASIMDALL {
    ADDV_Fd_Vn(ADDV_Fd_Vn),
    UADDLV_Fd_Vn(UADDLV_Fd_Vn),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ASIMDDIFF {
    ADDHN2_Vd_Vn_Vm(ADDHN2_Vd_Vn_Vm),
    ADDHN_Vd_Vn_Vm(ADDHN_Vd_Vn_Vm),
    SMULL2_Vd_Vn_Vm(SMULL2_Vd_Vn_Vm),
    SMULL_Vd_Vn_Vm(SMULL_Vd_Vn_Vm),
    SUBHN2_Vd_Vn_Vm(SUBHN2_Vd_Vn_Vm),
    SUBHN_Vd_Vn_Vm(SUBHN_Vd_Vn_Vm),
    UADDL2_Vd_Vn_Vm(UADDL2_Vd_Vn_Vm),
    UADDL_Vd_Vn_Vm(UADDL_Vd_Vn_Vm),
    UADDW2_Vd_Vn_Vm(UADDW2_Vd_Vn_Vm),
    UADDW_Vd_Vn_Vm(UADDW_Vd_Vn_Vm),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ASIMDELEM {
    FCMLA_Vd_Vn_Em_IMM_ROT2(FCMLA_Vd_Vn_Em_IMM_ROT2),
    FMULX_Vd_Vn_Em(FMULX_Vd_Vn_Em),
    FMULX_Vd_Vn_Em16(FMULX_Vd_Vn_Em16),
    FMUL_Vd_Vn_Em(FMUL_Vd_Vn_Em),
    FMUL_Vd_Vn_Em16(FMUL_Vd_Vn_Em16),
    MUL_Vd_Vn_Em16(MUL_Vd_Vn_Em16),
    SMULL2_Vd_Vn_Em16(SMULL2_Vd_Vn_Em16),
    SMULL_Vd_Vn_Em16(SMULL_Vd_Vn_Em16),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ASIMDIMM {
    BIC_Vd_SIMD_IMM_SFT(BIC_Vd_SIMD_IMM_SFT),
    BIC_Vd_V_4H_SIMD_IMM_SFT_LSL(BIC_Vd_V_4H_SIMD_IMM_SFT_LSL),
    FMOV_Vd_SIMD_FPIMM(FMOV_Vd_SIMD_FPIMM),
    FMOV_Vd_V_2D_SIMD_FPIMM(FMOV_Vd_V_2D_SIMD_FPIMM),
    FMOV_Vd_V_4H_SIMD_FPIMM(FMOV_Vd_V_4H_SIMD_FPIMM),
    MOVI_Sd_SIMD_IMM(MOVI_Sd_SIMD_IMM),
    MOVI_Vd_SIMD_IMM(MOVI_Vd_SIMD_IMM),
    MOVI_Vd_SIMD_IMM_SFT(MOVI_Vd_SIMD_IMM_SFT),
    MOVI_Vd_V_2S_SIMD_IMM_SFT_MSL(MOVI_Vd_V_2S_SIMD_IMM_SFT_MSL),
    MOVI_Vd_V_4H_SIMD_IMM_SFT_LSL(MOVI_Vd_V_4H_SIMD_IMM_SFT_LSL),
    MOVI_Vd_V_8B_SIMD_IMM_SFT_LSL(MOVI_Vd_V_8B_SIMD_IMM_SFT_LSL),
    MVNI_Vd_SIMD_IMM_SFT(MVNI_Vd_SIMD_IMM_SFT),
    MVNI_Vd_V_2S_SIMD_IMM_SFT_MSL(MVNI_Vd_V_2S_SIMD_IMM_SFT_MSL),
    MVNI_Vd_V_4H_SIMD_IMM_SFT_LSL(MVNI_Vd_V_4H_SIMD_IMM_SFT_LSL),
    ORR_Vd_SIMD_IMM_SFT(ORR_Vd_SIMD_IMM_SFT),
    ORR_Vd_V_4H_SIMD_IMM_SFT_LSL(ORR_Vd_V_4H_SIMD_IMM_SFT_LSL),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ASIMDMISC {
    CLS_Vd_Vn(CLS_Vd_Vn),
    CLZ_Vd_Vn(CLZ_Vd_Vn),
    CMEQ_Vd_Vn_IMM0(CMEQ_Vd_Vn_IMM0),
    CMGE_Vd_Vn_IMM0(CMGE_Vd_Vn_IMM0),
    CMGT_Vd_Vn_IMM0(CMGT_Vd_Vn_IMM0),
    CMLE_Vd_Vn_IMM0(CMLE_Vd_Vn_IMM0),
    CMLT_Vd_Vn_IMM0(CMLT_Vd_Vn_IMM0),
    CNT_Vd_Vn(CNT_Vd_Vn),
    FCMEQ_Vd_V_4H_Vn_V_4H_FPIMM0(FCMEQ_Vd_V_4H_Vn_V_4H_FPIMM0),
    FCMEQ_Vd_Vn_FPIMM0(FCMEQ_Vd_Vn_FPIMM0),
    FCMGE_Vd_V_4H_Vn_V_4H_FPIMM0(FCMGE_Vd_V_4H_Vn_V_4H_FPIMM0),
    FCMGE_Vd_Vn_FPIMM0(FCMGE_Vd_Vn_FPIMM0),
    FCMGT_Vd_V_4H_Vn_V_4H_FPIMM0(FCMGT_Vd_V_4H_Vn_V_4H_FPIMM0),
    FCMGT_Vd_Vn_FPIMM0(FCMGT_Vd_Vn_FPIMM0),
    FCMLE_Vd_V_4H_Vn_V_4H_FPIMM0(FCMLE_Vd_V_4H_Vn_V_4H_FPIMM0),
    FCMLE_Vd_Vn_FPIMM0(FCMLE_Vd_Vn_FPIMM0),
    FCMLT_Vd_V_4H_Vn_V_4H_FPIMM0(FCMLT_Vd_V_4H_Vn_V_4H_FPIMM0),
    FCMLT_Vd_Vn_FPIMM0(FCMLT_Vd_Vn_FPIMM0),
    FCVTAS_Vd_V_4H_Vn_V_4H(FCVTAS_Vd_V_4H_Vn_V_4H),
    FCVTAS_Vd_Vn(FCVTAS_Vd_Vn),
    FCVTAU_Vd_V_4H_Vn_V_4H(FCVTAU_Vd_V_4H_Vn_V_4H),
    FCVTAU_Vd_Vn(FCVTAU_Vd_Vn),
    FCVTL2_Vd_Vn(FCVTL2_Vd_Vn),
    FCVTL_Vd_Vn(FCVTL_Vd_Vn),
    FCVTMS_Vd_V_4H_Vn_V_4H(FCVTMS_Vd_V_4H_Vn_V_4H),
    FCVTMS_Vd_Vn(FCVTMS_Vd_Vn),
    FCVTMU_Vd_V_4H_Vn_V_4H(FCVTMU_Vd_V_4H_Vn_V_4H),
    FCVTMU_Vd_Vn(FCVTMU_Vd_Vn),
    FCVTN2_Vd_Vn(FCVTN2_Vd_Vn),
    FCVTNS_Vd_V_4H_Vn_V_4H(FCVTNS_Vd_V_4H_Vn_V_4H),
    FCVTNS_Vd_Vn(FCVTNS_Vd_Vn),
    FCVTNU_Vd_V_4H_Vn_V_4H(FCVTNU_Vd_V_4H_Vn_V_4H),
    FCVTNU_Vd_Vn(FCVTNU_Vd_Vn),
    FCVTN_Vd_Vn(FCVTN_Vd_Vn),
    FCVTPS_Vd_V_4H_Vn_V_4H(FCVTPS_Vd_V_4H_Vn_V_4H),
    FCVTPS_Vd_Vn(FCVTPS_Vd_Vn),
    FCVTPU_Vd_V_4H_Vn_V_4H(FCVTPU_Vd_V_4H_Vn_V_4H),
    FCVTPU_Vd_Vn(FCVTPU_Vd_Vn),
    FCVTXN2_Vd_Vn(FCVTXN2_Vd_Vn),
    FCVTXN_Vd_Vn(FCVTXN_Vd_Vn),
    FCVTZS_Vd_V_4H_Vn_V_4H(FCVTZS_Vd_V_4H_Vn_V_4H),
    FCVTZS_Vd_Vn(FCVTZS_Vd_Vn),
    FCVTZU_Vd_V_4H_Vn_V_4H(FCVTZU_Vd_V_4H_Vn_V_4H),
    FCVTZU_Vd_Vn(FCVTZU_Vd_Vn),
    NEG_Vd_Vn(NEG_Vd_Vn),
    SCVTF_Vd_V_4H_Vn_V_4H(SCVTF_Vd_V_4H_Vn_V_4H),
    SCVTF_Vd_Vn(SCVTF_Vd_Vn),
    UADDLP_Vd_Vn(UADDLP_Vd_Vn),
    UCVTF_Vd_V_4H_Vn_V_4H(UCVTF_Vd_V_4H_Vn_V_4H),
    UCVTF_Vd_Vn(UCVTF_Vd_Vn),
}
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
    FDIV_Fd_Fn_Fm(FDIV_Fd_Fn_Fm),
    FDIV_Fd_S_S_Fn_S_S_Fm_S_S(FDIV_Fd_S_S_Fn_S_S_Fm_S_S),
    FMUL_Fd_Fn_Fm(FMUL_Fd_Fn_Fm),
    FMUL_Fd_S_S_Fn_S_S_Fm_S_S(FMUL_Fd_S_S_Fn_S_S_Fm_S_S),
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
