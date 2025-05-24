#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FCVTPU_Sd_S_H_Sn_S_H {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FCVTX_SVE_Zd_SVE_Pg3_SVE_Zn {
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
pub struct FCVTXN_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FCVTXN_Sd_Sn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FCVTXN2_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FCVTXNT_SVE_Zd_SVE_Pg3_SVE_Zn {
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
pub struct FCVTZS_SME_Zdnx2_SME_Znx2 {
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
pub struct FCVTZS_SME_Zdnx4_SME_Znx4 {
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
pub struct FCVTZS_SVE_Zd_SVE_Pg3_SVE_Zn {
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
pub struct FCVTZS_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_S {
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
pub struct FCVTZS_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_H {
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
pub struct FCVTZS_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_S {
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
pub struct FCVTZS_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_H {
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
pub struct FCVTZS_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_H {
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
pub struct FCVTZS_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_D {
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
pub struct FCVTZS_Rd_Fn_FBITS {
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
pub struct FCVTZS_Rd_W_Fn_S_D_FBITS_imm_1_32 {
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
pub struct FCVTZS_Rd_Fn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FCVTZS_Rd_W_Fn_S_D {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FCVTZS_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FCVTZS_Sd_Sn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FCVTZS_Vd_V_4H_Vn_V_4H {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FCVTZS_Sd_S_H_Sn_S_H {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FCVTZS_Vd_Vn_IMM_VLSR {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FCVTZS_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FCVTZS_Sd_Sn_IMM_VLSR {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FCVTZS_Sd_S_S_Sn_S_S_IMM_VLSR_S_S {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FCVTZU_SME_Zdnx2_SME_Znx2 {
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
pub struct FCVTZU_SME_Zdnx4_SME_Znx4 {
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
pub struct FCVTZU_SVE_Zd_SVE_Pg3_SVE_Zn {
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
pub struct FCVTZU_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_S {
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
pub struct FCVTZU_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_H {
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
pub struct FCVTZU_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_S {
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
pub struct FCVTZU_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_H {
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
pub struct FCVTZU_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_H {
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
pub struct FCVTZU_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_D {
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
pub struct FCVTZU_Rd_Fn_FBITS {
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
pub struct FCVTZU_Rd_W_Fn_S_D_FBITS_imm_1_32 {
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
pub struct FCVTZU_Rd_Fn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FCVTZU_Rd_W_Fn_S_D {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FCVTZU_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FCVTZU_Sd_Sn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FCVTZU_Vd_V_4H_Vn_V_4H {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FCVTZU_Sd_S_H_Sn_S_H {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FCVTZU_Vd_Vn_IMM_VLSR {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FCVTZU_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FCVTZU_Sd_Sn_IMM_VLSR {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FCVTZU_Sd_S_S_Sn_S_S_IMM_VLSR_S_S {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FDIV_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
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
pub struct FDIV_Vd_Vn_Vm {
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
pub struct FDIV_Fd_Fn_Fm {
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
pub struct FDIV_Fd_S_S_Fn_S_S_Fm_S_S {
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
pub struct FDIV_Vd_V_2S_Vn_V_2S_Vm_V_2S {
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
pub struct FDIVR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
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
pub struct FDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2 {
    #[bits(3)]
    pub imm3_0: u32,
    #[bits(3)]
    pub _op_3: u32,
    #[bits(4)]
    pub sme_zn2: u32,
    #[bits(2)]
    pub imm2_10: u32,
    #[bits(1)]
    pub _op_12: u32,
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
pub struct FDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX2 {
    #[bits(3)]
    pub imm3_0: u32,
    #[bits(4)]
    pub _op_3: u32,
    #[bits(3)]
    pub sme_zn4: u32,
    #[bits(2)]
    pub imm2_10: u32,
    #[bits(1)]
    pub _op_12: u32,
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
pub struct FDOT_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm {
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
pub struct FDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H {
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
pub struct FDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2 {
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
pub struct FDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4 {
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
pub struct FDOT_SVE_Zd_SVE_Zn_SVE_Zm_16 {
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
pub struct FDOT_SVE_Zd_SVE_Zn_SVE_Zm3_19_INDEX {
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
pub struct FDUP_SVE_Zd_SVE_FPIMM8 {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(8)]
    pub sve_imm8: u32,
    #[bits(19)]
    pub _op_13: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FEXPA_SVE_Zd_SVE_Zn {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FJCVTZS_Rd_Fn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FMOPA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16 {
    #[bits(2)]
    pub sme_zada_2b: u32,
    #[bits(3)]
    pub _op_2: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(3)]
    pub sme_pm: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FMOPA_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16 {
    #[bits(3)]
    pub sme_zada_3b: u32,
    #[bits(2)]
    pub _op_3: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(3)]
    pub sme_pm: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FMOPA_SME_ZAda_2b_S_S_SVE_Pg3_P_M_SME_Pm_P_M_SVE_Zn_S_H_SVE_Zm_16_S_H {
    #[bits(2)]
    pub sme_zada_2b: u32,
    #[bits(3)]
    pub _op_2: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(3)]
    pub sme_pm: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FMOPS_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16 {
    #[bits(2)]
    pub sme_zada_2b: u32,
    #[bits(3)]
    pub _op_2: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(3)]
    pub sme_pm: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FMOPS_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16 {
    #[bits(3)]
    pub sme_zada_3b: u32,
    #[bits(2)]
    pub _op_3: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(3)]
    pub sme_pm: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FMOPS_SME_ZAda_2b_S_S_SVE_Pg3_P_M_SME_Pm_P_M_SVE_Zn_S_H_SVE_Zm_16_S_H {
    #[bits(2)]
    pub sme_zada_2b: u32,
    #[bits(3)]
    pub _op_2: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(3)]
    pub sme_pm: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FMOV_Rd_Fn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FMOV_Rd_W_Fn_S_S {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FMOV_Rd_VnD1 {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FMOV_Fd_Rn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FMOV_Fd_S_S_Rn_W {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FMOV_VdD1_Rn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FMOV_Fd_Fn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FMOV_Fd_S_S_Fn_S_S {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FMOV_Fd_FPIMM {
    #[bits(5)]
    pub rd: u32,
    #[bits(8)]
    pub _op_5: u32,
    #[bits(8)]
    pub imm8: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FMOV_Fd_S_S_FPIMM {
    #[bits(5)]
    pub rd: u32,
    #[bits(8)]
    pub _op_5: u32,
    #[bits(8)]
    pub imm8: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FMOV_Vd_SIMD_FPIMM {
    #[bits(5)]
    pub rd: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FMOV_Vd_V_2D_SIMD_FPIMM {
    #[bits(5)]
    pub rd: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FMOV_Vd_V_4H_SIMD_FPIMM {
    #[bits(5)]
    pub rd: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FMSB_SVE_Zd_SVE_Pg3_SVE_Zm_5_SVE_Za_16 {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zm_5: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(3)]
    pub _op_13: u32,
    #[bits(5)]
    pub sve_za_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FMSUB_Fd_Fn_Fm_Fa {
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
pub struct FMSUB_Fd_S_S_Fn_S_S_Fm_S_S_Fa_S_S {
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
pub struct FMUL_SVE_Zd_SVE_Zn_SVE_Zm3_22_INDEX {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(1)]
    pub _op_21: u32,
    #[bits(1)]
    pub sve_i3h: u32,
    #[bits(9)]
    pub _op_23: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FMUL_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX {
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
pub struct FMUL_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX {
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
pub struct FMUL_SVE_Zd_SVE_Zn_SVE_Zm_16 {
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
pub struct FMUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
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
pub struct FMUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_TWO {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(1)]
    pub sve_i1: u32,
    #[bits(4)]
    pub _op_6: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(19)]
    pub _op_13: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FMUL_Vd_Vn_Vm {
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
pub struct FMUL_Fd_Fn_Fm {
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
pub struct FMUL_Fd_S_S_Fn_S_S_Fm_S_S {
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
pub struct FMUL_Vd_V_2S_Vn_V_2S_Vm_V_2S {
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
pub struct FMUL_Vd_Vn_Em16 {
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
pub struct FMUL_Vd_Vn_Em {
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
pub struct FMUL_Sd_Sn_Em16 {
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
pub struct FMUL_Sd_Sn_Em {
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
pub struct FMULX_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
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
pub struct FMULX_Vd_Vn_Vm {
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
pub struct FMULX_Sd_Sn_Sm {
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
pub struct FMULX_Vd_V_2S_Vn_V_2S_Vm_V_2S {
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
pub struct FMULX_Sd_S_S_Sn_S_S_Sm_S_S {
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
pub struct FMULX_Vd_Vn_Em16 {
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
pub struct FMULX_Vd_Vn_Em {
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
pub struct FMULX_Sd_Sn_Em16 {
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
pub struct FMULX_Sd_Sn_Em {
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
pub struct FSUB_SME_ZA_array_off3_0_SME_Znx2 {
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
pub struct FSUB_SME_ZA_array_off3_0_SME_Znx4 {
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
pub struct FSUB_SVE_Zd_SVE_Zn_SVE_Zm_16 {
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
pub struct FSUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
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
pub struct FSUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_ONE {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(1)]
    pub sve_i1: u32,
    #[bits(4)]
    pub _op_6: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(19)]
    pub _op_13: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FSUB_Vd_Vn_Vm {
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
pub struct FSUB_Fd_Fn_Fm {
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
pub struct FSUB_Fd_S_S_Fn_S_S_Fm_S_S {
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
pub struct FSUB_Vd_V_2S_Vn_V_2S_Vm_V_2S {
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
pub struct FSUBR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
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
pub struct FSUBR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_I1_HALF_ONE {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(1)]
    pub sve_i1: u32,
    #[bits(4)]
    pub _op_6: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(19)]
    pub _op_13: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAR_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDARB_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDARH_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDP_Rt_Rt2_ADDR_SIMM7 {
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
pub struct LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S {
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
pub struct LDP_Ft_Ft2_ADDR_SIMM7 {
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
pub struct LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S {
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
pub struct LDPSW_Rt_Rt2_ADDR_SIMM7 {
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
pub struct LDPSW_Rt_X_Rt2_X_ADDR_SIMM7_S_S {
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
pub struct LDR_SME_ZA_array_off4_SME_ADDR_RI_U4xVL {
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
pub struct LDR_SME_ZT0_SIMD_ADDR_SIMPLE {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDR_Rt_ADDR_PCREL19 {
    #[bits(5)]
    pub rt: u32,
    #[bits(19)]
    pub imm19: u32,
    #[bits(8)]
    pub _op_24: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDR_Rt_ADDR_REGOFF {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDR_Rt_ADDR_SIMM9 {
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
pub struct LDR_Rt_ADDR_UIMM12 {
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
pub struct LDR_SVE_Pt_SVE_ADDR_RI_S9xVL {
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
pub struct LDR_SVE_PNt_SVE_ADDR_RI_S9xVL {
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
pub struct LDR_SVE_Zt_SVE_ADDR_RI_S9xVL {
    #[bits(5)]
    pub sve_zt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDR_Ft_ADDR_PCREL19 {
    #[bits(5)]
    pub rt: u32,
    #[bits(19)]
    pub imm19: u32,
    #[bits(8)]
    pub _op_24: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDR_Ft_ADDR_REGOFF {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDR_Ft_ADDR_SIMM9 {
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
pub struct LDR_Ft_ADDR_UIMM12 {
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
pub struct LDRAA_Rt_ADDR_SIMM10 {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(1)]
    pub _op_10: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(1)]
    pub _op_21: u32,
    #[bits(1)]
    pub s_imm10: u32,
    #[bits(9)]
    pub _op_23: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDRAB_Rt_ADDR_SIMM10 {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(1)]
    pub _op_10: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(1)]
    pub _op_21: u32,
    #[bits(1)]
    pub s_imm10: u32,
    #[bits(9)]
    pub _op_23: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDRB_Rt_ADDR_REGOFF {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDRB_Rt_ADDR_SIMM9 {
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
pub struct LDRB_Rt_ADDR_UIMM12 {
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
pub struct LDRH_Rt_ADDR_REGOFF {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDRH_Rt_ADDR_SIMM9 {
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
pub struct LDRH_Rt_ADDR_UIMM12 {
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
pub struct LDRSB_Rt_ADDR_REGOFF {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDRSB_Rt_ADDR_SIMM9 {
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
pub struct LDRSB_Rt_ADDR_UIMM12 {
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
pub struct LDRSH_Rt_ADDR_REGOFF {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDRSH_Rt_ADDR_SIMM9 {
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
pub struct LDRSH_Rt_ADDR_UIMM12 {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(12)]
    pub imm12: u32,
    #[bits(10)]
    pub _op_22: u32,
}
