#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDRSW_Rt_ADDR_SIMM9 {
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
pub struct LDRSW_Rt_ADDR_UIMM12 {
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
pub struct LDUR_Rt_ADDR_SIMM9 {
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
pub struct LDUR_Ft_ADDR_SIMM9 {
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
pub struct LDURB_Rt_ADDR_SIMM9 {
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
pub struct LDURH_Rt_ADDR_SIMM9 {
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
pub struct LDURSB_Rt_ADDR_SIMM9 {
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
pub struct LDURSH_Rt_ADDR_SIMM9 {
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
pub struct LDURSW_Rt_ADDR_SIMM9 {
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
pub struct LSL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHLIMM_PRED {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_imm5: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(9)]
    pub _op_13: u32,
    #[bits(2)]
    pub sve_tszh: u32,
    #[bits(8)]
    pub _op_24: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LSL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
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
pub struct LSL_SVE_Zd_S_B_SVE_Pg3_P_M_SVE_Zd_S_B_SVE_Zm_5_S_D {
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
pub struct LSL_SVE_Zd_SVE_Zn_SVE_Zm_16 {
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
pub struct LSL_SVE_Zd_SVE_Zn_SVE_SHLIMM_UNPRED {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub imm5: u32,
    #[bits(1)]
    pub _op_21: u32,
    #[bits(2)]
    pub sve_tszh: u32,
    #[bits(8)]
    pub _op_24: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LSLR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
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
pub struct LSLV_Rd_Rn_Rm {
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
pub struct LSR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHRIMM_PRED {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_imm5: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(9)]
    pub _op_13: u32,
    #[bits(2)]
    pub sve_tszh: u32,
    #[bits(8)]
    pub _op_24: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LSR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
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
pub struct LSR_SVE_Zd_S_B_SVE_Pg3_P_M_SVE_Zd_S_B_SVE_Zm_5_S_D {
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
pub struct LSR_SVE_Zd_SVE_Zn_SVE_Zm_16 {
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
pub struct LSR_SVE_Zd_SVE_Zn_SVE_SHRIMM_UNPRED {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub imm5: u32,
    #[bits(1)]
    pub _op_21: u32,
    #[bits(2)]
    pub sve_tszh: u32,
    #[bits(8)]
    pub _op_24: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LSRR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
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
pub struct LSRV_Rd_Rn_Rm {
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
pub struct MADD_Rd_Rn_Rm_Ra {
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
pub struct MOV_SME_ZA_HV_idx_dest_SVE_Pg3_SVE_Zn {
    #[bits(4)]
    pub imm4_0: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(2)]
    pub sme_rv: u32,
    #[bits(1)]
    pub sme_v: u32,
    #[bits(1)]
    pub sme_q: u32,
    #[bits(5)]
    pub _op_17: u32,
    #[bits(2)]
    pub sme_size_22: u32,
    #[bits(8)]
    pub _op_24: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOV_SME_ZA_HV_idx_destxN_SME_Znx2 {
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
    #[bits(1)]
    pub sme_v: u32,
    #[bits(16)]
    pub _op_16: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOV_SME_ZA_array_off3_0_SME_Znx2 {
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
pub struct MOV_SME_ZA_HV_idx_destxN_SME_Znx4 {
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
    #[bits(1)]
    pub sme_v: u32,
    #[bits(16)]
    pub _op_16: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOV_SME_ZA_array_off3_0_SME_Znx4 {
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
pub struct MOV_SVE_Zd_SVE_Pg3_SME_ZA_HV_idx_src {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(4)]
    pub imm4_5: u32,
    #[bits(1)]
    pub _op_9: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(2)]
    pub sme_rv: u32,
    #[bits(1)]
    pub sme_v: u32,
    #[bits(1)]
    pub sme_q: u32,
    #[bits(5)]
    pub _op_17: u32,
    #[bits(2)]
    pub sme_size_22: u32,
    #[bits(8)]
    pub _op_24: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOV_SME_Zdnx2_SME_ZA_HV_idx_srcxN {
    #[bits(1)]
    pub _op_0: u32,
    #[bits(4)]
    pub sme_zdn2: u32,
    #[bits(3)]
    pub imm3_5: u32,
    #[bits(5)]
    pub _op_8: u32,
    #[bits(2)]
    pub sme_rv: u32,
    #[bits(1)]
    pub sme_v: u32,
    #[bits(16)]
    pub _op_16: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOV_SME_Zdnx2_SME_ZA_array_off3_5 {
    #[bits(1)]
    pub _op_0: u32,
    #[bits(4)]
    pub sme_zdn2: u32,
    #[bits(3)]
    pub imm3_5: u32,
    #[bits(5)]
    pub _op_8: u32,
    #[bits(2)]
    pub sme_rv: u32,
    #[bits(17)]
    pub _op_15: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOV_SME_Zdnx4_SME_ZA_HV_idx_srcxN {
    #[bits(2)]
    pub _op_0: u32,
    #[bits(3)]
    pub sme_zdn4: u32,
    #[bits(3)]
    pub imm3_5: u32,
    #[bits(5)]
    pub _op_8: u32,
    #[bits(2)]
    pub sme_rv: u32,
    #[bits(1)]
    pub sme_v: u32,
    #[bits(16)]
    pub _op_16: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOV_SME_Zdnx4_SME_ZA_array_off3_5 {
    #[bits(2)]
    pub _op_0: u32,
    #[bits(3)]
    pub sme_zdn4: u32,
    #[bits(3)]
    pub imm3_5: u32,
    #[bits(5)]
    pub _op_8: u32,
    #[bits(2)]
    pub sme_rv: u32,
    #[bits(17)]
    pub _op_15: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOVA_SME_ZA_HV_idx_dest_SVE_Pg3_SVE_Zn {
    #[bits(4)]
    pub imm4_0: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(2)]
    pub sme_rv: u32,
    #[bits(1)]
    pub sme_v: u32,
    #[bits(1)]
    pub sme_q: u32,
    #[bits(5)]
    pub _op_17: u32,
    #[bits(2)]
    pub sme_size_22: u32,
    #[bits(8)]
    pub _op_24: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOVA_SME_ZA_HV_idx_destxN_SME_Znx2 {
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
    #[bits(1)]
    pub sme_v: u32,
    #[bits(16)]
    pub _op_16: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOVA_SME_ZA_array_off3_0_SME_Znx2 {
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
pub struct MOVA_SME_ZA_HV_idx_destxN_SME_Znx4 {
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
    #[bits(1)]
    pub sme_v: u32,
    #[bits(16)]
    pub _op_16: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOVA_SME_ZA_array_off3_0_SME_Znx4 {
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
pub struct MOVA_SVE_Zd_SVE_Pg3_SME_ZA_HV_idx_src {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(4)]
    pub imm4_5: u32,
    #[bits(1)]
    pub _op_9: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(2)]
    pub sme_rv: u32,
    #[bits(1)]
    pub sme_v: u32,
    #[bits(1)]
    pub sme_q: u32,
    #[bits(5)]
    pub _op_17: u32,
    #[bits(2)]
    pub sme_size_22: u32,
    #[bits(8)]
    pub _op_24: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOVA_SME_Zdnx2_SME_ZA_HV_idx_srcxN {
    #[bits(1)]
    pub _op_0: u32,
    #[bits(4)]
    pub sme_zdn2: u32,
    #[bits(3)]
    pub imm3_5: u32,
    #[bits(5)]
    pub _op_8: u32,
    #[bits(2)]
    pub sme_rv: u32,
    #[bits(1)]
    pub sme_v: u32,
    #[bits(16)]
    pub _op_16: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOVA_SME_Zdnx2_SME_ZA_array_off3_5 {
    #[bits(1)]
    pub _op_0: u32,
    #[bits(4)]
    pub sme_zdn2: u32,
    #[bits(3)]
    pub imm3_5: u32,
    #[bits(5)]
    pub _op_8: u32,
    #[bits(2)]
    pub sme_rv: u32,
    #[bits(17)]
    pub _op_15: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOVA_SME_Zdnx4_SME_ZA_HV_idx_srcxN {
    #[bits(2)]
    pub _op_0: u32,
    #[bits(3)]
    pub sme_zdn4: u32,
    #[bits(3)]
    pub imm3_5: u32,
    #[bits(5)]
    pub _op_8: u32,
    #[bits(2)]
    pub sme_rv: u32,
    #[bits(1)]
    pub sme_v: u32,
    #[bits(16)]
    pub _op_16: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOVA_SME_Zdnx4_SME_ZA_array_off3_5 {
    #[bits(2)]
    pub _op_0: u32,
    #[bits(3)]
    pub sme_zdn4: u32,
    #[bits(3)]
    pub imm3_5: u32,
    #[bits(5)]
    pub _op_8: u32,
    #[bits(2)]
    pub sme_rv: u32,
    #[bits(17)]
    pub _op_15: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOVAZ_SME_Zdnx2_SME_ZA_array_vrsb_1 {
    #[bits(1)]
    pub _op_0: u32,
    #[bits(4)]
    pub sme_zdn2: u32,
    #[bits(3)]
    pub off3: u32,
    #[bits(5)]
    pub _op_8: u32,
    #[bits(2)]
    pub sme_rv: u32,
    #[bits(1)]
    pub sme_v: u32,
    #[bits(16)]
    pub _op_16: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOVAZ_SME_Zdnx2_SME_ZA_array_vrss_1 {
    #[bits(1)]
    pub _op_0: u32,
    #[bits(4)]
    pub sme_zdn2: u32,
    #[bits(1)]
    pub ol: u32,
    #[bits(2)]
    pub zan_2: u32,
    #[bits(5)]
    pub _op_8: u32,
    #[bits(2)]
    pub sme_rv: u32,
    #[bits(1)]
    pub sme_v: u32,
    #[bits(16)]
    pub _op_16: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOVAZ_SME_Zdnx2_SME_ZA_array_vrsh_1 {
    #[bits(1)]
    pub _op_0: u32,
    #[bits(4)]
    pub sme_zdn2: u32,
    #[bits(2)]
    pub off2: u32,
    #[bits(1)]
    pub zan_1: u32,
    #[bits(5)]
    pub _op_8: u32,
    #[bits(2)]
    pub sme_rv: u32,
    #[bits(1)]
    pub sme_v: u32,
    #[bits(16)]
    pub _op_16: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOVAZ_SME_Zdnx2_SME_ZA_array_vrsd_1 {
    #[bits(1)]
    pub _op_0: u32,
    #[bits(4)]
    pub sme_zdn2: u32,
    #[bits(3)]
    pub zan_3: u32,
    #[bits(5)]
    pub _op_8: u32,
    #[bits(2)]
    pub sme_rv: u32,
    #[bits(1)]
    pub sme_v: u32,
    #[bits(16)]
    pub _op_16: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOVAZ_SME_Zdnx4_SME_ZA_array_vrsb_2 {
    #[bits(2)]
    pub _op_0: u32,
    #[bits(3)]
    pub sme_zdn4: u32,
    #[bits(2)]
    pub off2: u32,
    #[bits(6)]
    pub _op_7: u32,
    #[bits(2)]
    pub sme_rv: u32,
    #[bits(1)]
    pub sme_v: u32,
    #[bits(16)]
    pub _op_16: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOVAZ_SME_Zdnx4_SME_ZA_array_vrss_2 {
    #[bits(2)]
    pub _op_0: u32,
    #[bits(3)]
    pub sme_zdn4: u32,
    #[bits(2)]
    pub off2: u32,
    #[bits(6)]
    pub _op_7: u32,
    #[bits(2)]
    pub sme_rv: u32,
    #[bits(1)]
    pub sme_v: u32,
    #[bits(16)]
    pub _op_16: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOVAZ_SME_Zdnx4_SME_ZA_array_vrsh_2 {
    #[bits(2)]
    pub _op_0: u32,
    #[bits(3)]
    pub sme_zdn4: u32,
    #[bits(1)]
    pub ol: u32,
    #[bits(1)]
    pub zan: u32,
    #[bits(6)]
    pub _op_7: u32,
    #[bits(2)]
    pub sme_rv: u32,
    #[bits(1)]
    pub sme_v: u32,
    #[bits(16)]
    pub _op_16: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOVAZ_SME_Zdnx4_SME_ZA_array_vrsd_2 {
    #[bits(2)]
    pub _op_0: u32,
    #[bits(3)]
    pub sme_zdn4: u32,
    #[bits(3)]
    pub zan_3: u32,
    #[bits(5)]
    pub _op_8: u32,
    #[bits(2)]
    pub sme_rv: u32,
    #[bits(1)]
    pub sme_v: u32,
    #[bits(16)]
    pub _op_16: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOVI_Vd_SIMD_IMM_SFT {
    #[bits(5)]
    pub rd: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOVI_Vd_V_4H_SIMD_IMM_SFT_LSL {
    #[bits(5)]
    pub rd: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOVI_Vd_V_2S_SIMD_IMM_SFT_MSL {
    #[bits(5)]
    pub rd: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOVI_Vd_V_8B_SIMD_IMM_SFT_LSL {
    #[bits(5)]
    pub rd: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOVI_Sd_SIMD_IMM {
    #[bits(5)]
    pub rd: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOVI_Vd_SIMD_IMM {
    #[bits(5)]
    pub rd: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOVK_Rd_HALF {
    #[bits(5)]
    pub rd: u32,
    #[bits(16)]
    pub imm16_5: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOVN_Rd_HALF {
    #[bits(5)]
    pub rd: u32,
    #[bits(16)]
    pub imm16_5: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOVPRFX_SVE_Zd_SVE_Pg3_SVE_Zn {
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
pub struct MOVPRFX_SVE_Zd_SVE_Zn {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOVT_Rt_SME_ZT0_INDEX {
    #[bits(5)]
    pub rt: u32,
    #[bits(7)]
    pub _op_5: u32,
    #[bits(3)]
    pub imm3_12: u32,
    #[bits(17)]
    pub _op_15: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOVT_SME_ZT0_INDEX_Rt {
    #[bits(5)]
    pub rt: u32,
    #[bits(7)]
    pub _op_5: u32,
    #[bits(3)]
    pub imm3_12: u32,
    #[bits(17)]
    pub _op_15: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MOVZ_Rd_HALF {
    #[bits(5)]
    pub rd: u32,
    #[bits(16)]
    pub imm16_5: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MSUB_Rd_Rn_Rm_Ra {
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
pub struct MUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
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
pub struct MUL_SVE_Zd_SVE_Zn_SVE_Zm_16 {
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
pub struct MUL_SVE_Zd_SVE_Zn_SVE_Zm3_22_INDEX {
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
pub struct MUL_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX {
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
pub struct MUL_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX {
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
pub struct MUL_SVE_Zd_SVE_Zd_SVE_SIMM8 {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(8)]
    pub sve_imm8: u32,
    #[bits(19)]
    pub _op_13: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MUL_Vd_Vn_Vm {
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
pub struct MUL_Vd_Vn_Em16 {
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
pub struct MVNI_Vd_SIMD_IMM_SFT {
    #[bits(5)]
    pub rd: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MVNI_Vd_V_4H_SIMD_IMM_SFT_LSL {
    #[bits(5)]
    pub rd: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MVNI_Vd_V_2S_SIMD_IMM_SFT_MSL {
    #[bits(5)]
    pub rd: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct NEG_SVE_Zd_SVE_Pg3_SVE_Zn {
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
pub struct NEG_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct NEG_Sd_Sn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ORN_Rd_Rn_Rm_SFT {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ORN_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm {
    #[bits(4)]
    pub sve_pd: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(4)]
    pub sve_pn: u32,
    #[bits(1)]
    pub _op_9: u32,
    #[bits(4)]
    pub sve_pg4_10: u32,
    #[bits(2)]
    pub _op_14: u32,
    #[bits(4)]
    pub sve_pm: u32,
    #[bits(12)]
    pub _op_20: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ORN_Vd_Vn_Vm {
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
pub struct ORNS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm {
    #[bits(4)]
    pub sve_pd: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(4)]
    pub sve_pn: u32,
    #[bits(1)]
    pub _op_9: u32,
    #[bits(4)]
    pub sve_pg4_10: u32,
    #[bits(2)]
    pub _op_14: u32,
    #[bits(4)]
    pub sve_pm: u32,
    #[bits(12)]
    pub _op_20: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ORR_Rd_SP_Rn_LIMM {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(6)]
    pub imms: u32,
    #[bits(6)]
    pub immr: u32,
    #[bits(1)]
    pub n: u32,
    #[bits(9)]
    pub _op_23: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ORR_Rd_Rn_Rm_SFT {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ORR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
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
pub struct ORR_SVE_Zd_SVE_Zn_SVE_Zm_16 {
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
pub struct ORR_SVE_Zd_SVE_Zd_SVE_LIMM {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(6)]
    pub sve_imms: u32,
    #[bits(6)]
    pub sve_immr: u32,
    #[bits(1)]
    pub sve_n: u32,
    #[bits(14)]
    pub _op_18: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ORR_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm {
    #[bits(4)]
    pub sve_pd: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(4)]
    pub sve_pn: u32,
    #[bits(1)]
    pub _op_9: u32,
    #[bits(4)]
    pub sve_pg4_10: u32,
    #[bits(2)]
    pub _op_14: u32,
    #[bits(4)]
    pub sve_pm: u32,
    #[bits(12)]
    pub _op_20: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ORR_Vd_Vn_Vm {
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
pub struct ORR_Vd_SIMD_IMM_SFT {
    #[bits(5)]
    pub rd: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ORR_Vd_V_4H_SIMD_IMM_SFT_LSL {
    #[bits(5)]
    pub rd: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ORRS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm {
    #[bits(4)]
    pub sve_pd: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(4)]
    pub sve_pn: u32,
    #[bits(1)]
    pub _op_9: u32,
    #[bits(4)]
    pub sve_pg4_10: u32,
    #[bits(2)]
    pub _op_14: u32,
    #[bits(4)]
    pub sve_pm: u32,
    #[bits(12)]
    pub _op_20: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ORV_SVE_Vd_SVE_Pg3_SVE_Zn {
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
pub struct RET_Rn {
    #[bits(5)]
    pub _op_0: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct RETAA {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct RETAB {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SB {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SBC_Rd_Rn_Rm {
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
pub struct SBCLB_SVE_Zd_SVE_Zn_SVE_Zm_16 {
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
pub struct SBCLT_SVE_Zd_SVE_Zn_SVE_Zm_16 {
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
pub struct SBCS_Rd_Rn_Rm {
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
pub struct SBFM_Rd_Rn_IMMR_IMMS {
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
pub struct SCVTF_SME_Zdnx2_SME_Znx2 {
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
pub struct SCVTF_SME_Zdnx4_SME_Znx4 {
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
pub struct SCVTF_SVE_Zd_SVE_Pg3_SVE_Zn {
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
pub struct SCVTF_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_S {
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
pub struct SCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_S {
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
pub struct SCVTF_SVE_Zd_S_S_SVE_Pg3_P_M_SVE_Zn_S_D {
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
pub struct SCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_H {
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
pub struct SCVTF_SVE_Zd_S_H_SVE_Pg3_P_M_SVE_Zn_S_D {
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
pub struct SCVTF_SVE_Zd_S_D_SVE_Pg3_P_M_SVE_Zn_S_D {
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
pub struct SCVTF_Fd_Rn_FBITS {
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
pub struct SCVTF_Fd_S_D_Rn_W_FBITS_imm_1_32 {
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
pub struct SCVTF_Fd_Rn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SCVTF_Fd_S_D_Rn_W {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SCVTF_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SCVTF_Sd_Sn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SCVTF_Vd_V_4H_Vn_V_4H {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SCVTF_Sd_S_H_Sn_S_H {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SCVTF_Vd_Vn_IMM_VLSR {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SCVTF_Vd_V_2S_Vn_V_2S_IMM_VLSR_V_2S {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SCVTF_Sd_Sn_IMM_VLSR {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SCVTF_Sd_S_S_Sn_S_S_IMM_VLSR_S_S {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SDIV_Rd_Rn_Rm {
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
pub struct SDIV_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
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
pub struct SDIVR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
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
pub struct SDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2 {
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
pub struct SDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX2 {
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
pub struct SDOT_SME_ZA_array_off3_0_S_S_SME_Znx2_S_B_SME_Zm_INDEX2_S_B {
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
pub struct SDOT_SME_ZA_array_off3_0_S_S_SME_Znx4_S_B_SME_Zm_INDEX2_S_B {
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
pub struct SDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX1 {
    #[bits(3)]
    pub imm3_0: u32,
    #[bits(3)]
    pub _op_3: u32,
    #[bits(4)]
    pub sme_zn2: u32,
    #[bits(1)]
    pub imm1_10: u32,
    #[bits(2)]
    pub _op_11: u32,
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
pub struct SDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX1 {
    #[bits(3)]
    pub imm3_0: u32,
    #[bits(4)]
    pub _op_3: u32,
    #[bits(3)]
    pub sme_zn4: u32,
    #[bits(1)]
    pub imm1_10: u32,
    #[bits(2)]
    pub _op_11: u32,
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
pub struct SDOT_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm {
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
pub struct SDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_B_SME_Zm_S_B {
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
pub struct SDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2 {
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
pub struct SDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4 {
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
pub struct SDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H {
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
pub struct SDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H_c1701408 {
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
