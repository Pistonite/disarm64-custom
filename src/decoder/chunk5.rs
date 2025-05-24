#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SDOT_SME_ZA_array_off3_0_S_S_SME_Znx2_S_H_SME_Zmx2_S_H {
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
pub struct SDOT_SME_ZA_array_off3_0_S_S_SME_Znx4_S_H_SME_Zmx4_S_H {
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
pub struct SDOT_SVE_Zd_SVE_Zn_SVE_Zm_16 {
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
pub struct SDOT_SVE_Zd_S_S_SVE_Zn_S_H_SVE_Zm_16_S_H {
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
pub struct SDOT_SVE_Zd_SVE_Zn_SVE_Zm3_19_INDEX {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(3)]
    pub sve_imm3: u32,
    #[bits(2)]
    pub imm2_19: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SDOT_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX {
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
pub struct SDOT_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX {
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
pub struct SDOT_Vd_Vn_Vm {
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
pub struct SDOT_Vd_Vn_Em {
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
pub struct SMADDL_Rd_Rn_Rm_Ra {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(5)]
    pub ra: u32,
    #[bits(1)]
    pub _op_15: u32,
    #[bits(5)]
    pub rm: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SMULH_Rd_Rn_Rm {
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
pub struct SMULH_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
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
pub struct SMULH_SVE_Zd_SVE_Zn_SVE_Zm_16 {
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
pub struct SMULL_Vd_Vn_Vm {
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
pub struct SMULL_Vd_Vn_Em16 {
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
pub struct SMULL2_Vd_Vn_Vm {
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
pub struct SMULL2_Vd_Vn_Em16 {
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
pub struct SMULLB_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(1)]
    pub _op_10: u32,
    #[bits(1)]
    pub sve_i3l: u32,
    #[bits(4)]
    pub _op_12: u32,
    #[bits(3)]
    pub sve_imm3: u32,
    #[bits(2)]
    pub sve_i3h2: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SMULLB_SVE_Zd_SVE_Zn_SVE_Zm4_11_INDEX {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(1)]
    pub _op_10: u32,
    #[bits(1)]
    pub sve_i3l: u32,
    #[bits(4)]
    pub _op_12: u32,
    #[bits(4)]
    pub sve_imm4: u32,
    #[bits(1)]
    pub sve_i2h: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SMULLB_SVE_Zd_SVE_Zn_SVE_Zm_16 {
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
pub struct SMULLT_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(1)]
    pub _op_10: u32,
    #[bits(1)]
    pub sve_i3l: u32,
    #[bits(4)]
    pub _op_12: u32,
    #[bits(3)]
    pub sve_imm3: u32,
    #[bits(2)]
    pub sve_i3h2: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SMULLT_SVE_Zd_SVE_Zn_SVE_Zm4_11_INDEX {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(1)]
    pub _op_10: u32,
    #[bits(1)]
    pub sve_i3l: u32,
    #[bits(4)]
    pub _op_12: u32,
    #[bits(4)]
    pub sve_imm4: u32,
    #[bits(1)]
    pub sve_i2h: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SMULLT_SVE_Zd_SVE_Zn_SVE_Zm_16 {
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
pub struct STP_Rt_Rt2_ADDR_SIMM7 {
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
pub struct STP_Rt_W_Rt2_W_ADDR_SIMM7_S_S {
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
pub struct STP_Ft_Ft2_ADDR_SIMM7 {
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
