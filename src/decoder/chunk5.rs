#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
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
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UDIV_Rd_Rn_Rm {
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
pub struct UHADD_Vd_Vn_Vm {
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
pub struct UHSUB_Vd_Vn_Vm {
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
pub struct UMADDL_Rd_Rn_Rm_Ra {
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
pub struct UMAX_Vd_Vn_Vm {
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
pub struct UMAXP_Vd_Vn_Vm {
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
pub struct UMAXV_Fd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UMIN_Vd_Vn_Vm {
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
pub struct UMINP_Vd_Vn_Vm {
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
pub struct UMINV_Fd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UMLAL_Vd_Vn_Vm {
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
pub struct UMLAL_Vd_Vn_Em16 {
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
pub struct UMLAL2_Vd_Vn_Vm {
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
pub struct UMLAL2_Vd_Vn_Em16 {
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
pub struct UMLSL_Vd_Vn_Vm {
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
pub struct UMLSL_Vd_Vn_Em16 {
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
pub struct UMLSL2_Vd_Vn_Vm {
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
pub struct UMLSL2_Vd_Vn_Em16 {
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
pub struct UMOV_Rd_En {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UMSUBL_Rd_Rn_Rm_Ra {
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
pub struct UMULH_Rd_Rn_Rm {
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
pub struct UMULL_Vd_Vn_Vm {
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
pub struct UMULL_Vd_Vn_Em16 {
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
pub struct UMULL2_Vd_Vn_Vm {
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
pub struct UMULL2_Vd_Vn_Em16 {
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
pub struct UQADD_Vd_Vn_Vm {
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
pub struct UQADD_Sd_Sn_Sm {
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
pub struct UQRSHL_Vd_Vn_Vm {
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
pub struct UQRSHL_Sd_Sn_Sm {
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
pub struct UQRSHRN_Vd_Vn_IMM_VLSR {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UQRSHRN_Sd_Sn_IMM_VLSR {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UQRSHRN2_Vd_Vn_IMM_VLSR {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UQSHL_Vd_Vn_Vm {
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
pub struct UQSHL_Sd_Sn_Sm {
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
pub struct UQSHL_Vd_Vn_IMM_VLSL {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UQSHL_Sd_Sn_IMM_VLSL {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UQSHRN_Vd_Vn_IMM_VLSR {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UQSHRN_Sd_Sn_IMM_VLSR {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UQSHRN2_Vd_Vn_IMM_VLSR {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UQSUB_Vd_Vn_Vm {
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
pub struct UQSUB_Sd_Sn_Sm {
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
pub struct UQXTN_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UQXTN_Sd_Sn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UQXTN2_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct URECPE_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct URHADD_Vd_Vn_Vm {
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
pub struct URSHL_Vd_Vn_Vm {
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
pub struct URSHL_Sd_Sn_Sm {
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
pub struct URSHR_Vd_Vn_IMM_VLSR {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct URSHR_Sd_Sn_IMM_VLSR {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct URSQRTE_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct URSRA_Vd_Vn_IMM_VLSR {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct URSRA_Sd_Sn_IMM_VLSR {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct USHL_Vd_Vn_Vm {
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
pub struct USHL_Sd_Sn_Sm {
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
pub struct USHLL_Vd_Vn_IMM_VLSL {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct USHLL2_Vd_Vn_IMM_VLSL {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct USHR_Vd_Vn_IMM_VLSR {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct USHR_Sd_Sn_IMM_VLSR {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct USQADD_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct USQADD_Sd_Sn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct USRA_Vd_Vn_IMM_VLSR {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct USRA_Sd_Sn_IMM_VLSR {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct USUBL_Vd_Vn_Vm {
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
pub struct USUBL2_Vd_Vn_Vm {
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
pub struct USUBW_Vd_Vn_Vm {
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
pub struct USUBW2_Vd_Vn_Vm {
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
pub struct UZP1_Vd_Vn_Vm {
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
pub struct UZP2_Vd_Vn_Vm {
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
pub struct XPACD_Rd {
    #[bits(5)]
    pub rd: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct XPACI_Rd {
    #[bits(5)]
    pub rd: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct XTN_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct XTN2_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ZIP1_Vd_Vn_Vm {
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
pub struct ZIP2_Vd_Vn_Vm {
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
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ADDSUB_CARRY {
    ADCS_Rd_Rn_Rm(ADCS_Rd_Rn_Rm),
    ADC_Rd_Rn_Rm(ADC_Rd_Rn_Rm),
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
    FMAXNMV_Fd_S_S_Vn_V_4S(FMAXNMV_Fd_S_S_Vn_V_4S),
    FMAXNMV_Fd_Vn(FMAXNMV_Fd_Vn),
    FMAXV_Fd_S_S_Vn_V_4S(FMAXV_Fd_S_S_Vn_V_4S),
    FMAXV_Fd_Vn(FMAXV_Fd_Vn),
    FMINNMV_Fd_S_S_Vn_V_4S(FMINNMV_Fd_S_S_Vn_V_4S),
    FMINNMV_Fd_Vn(FMINNMV_Fd_Vn),
    FMINV_Fd_S_S_Vn_V_4S(FMINV_Fd_S_S_Vn_V_4S),
    FMINV_Fd_Vn(FMINV_Fd_Vn),
    SADDLV_Fd_Vn(SADDLV_Fd_Vn),
    SMAXV_Fd_Vn(SMAXV_Fd_Vn),
    SMINV_Fd_Vn(SMINV_Fd_Vn),
    UADDLV_Fd_Vn(UADDLV_Fd_Vn),
    UMAXV_Fd_Vn(UMAXV_Fd_Vn),
    UMINV_Fd_Vn(UMINV_Fd_Vn),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ASIMDDIFF {
    ADDHN2_Vd_Vn_Vm(ADDHN2_Vd_Vn_Vm),
    ADDHN_Vd_Vn_Vm(ADDHN_Vd_Vn_Vm),
    PMULL2_Vd_V_1Q_Vn_V_2D_Vm_V_2D(PMULL2_Vd_V_1Q_Vn_V_2D_Vm_V_2D),
    PMULL2_Vd_Vn_Vm(PMULL2_Vd_Vn_Vm),
    PMULL_Vd_V_1Q_Vn_V_1D_Vm_V_1D(PMULL_Vd_V_1Q_Vn_V_1D_Vm_V_1D),
    PMULL_Vd_Vn_Vm(PMULL_Vd_Vn_Vm),
    RADDHN2_Vd_Vn_Vm(RADDHN2_Vd_Vn_Vm),
    RADDHN_Vd_Vn_Vm(RADDHN_Vd_Vn_Vm),
    RSUBHN2_Vd_Vn_Vm(RSUBHN2_Vd_Vn_Vm),
    RSUBHN_Vd_Vn_Vm(RSUBHN_Vd_Vn_Vm),
    SABAL2_Vd_Vn_Vm(SABAL2_Vd_Vn_Vm),
    SABAL_Vd_Vn_Vm(SABAL_Vd_Vn_Vm),
    SABDL2_Vd_Vn_Vm(SABDL2_Vd_Vn_Vm),
    SABDL_Vd_Vn_Vm(SABDL_Vd_Vn_Vm),
    SADDL2_Vd_Vn_Vm(SADDL2_Vd_Vn_Vm),
    SADDL_Vd_Vn_Vm(SADDL_Vd_Vn_Vm),
    SADDW2_Vd_Vn_Vm(SADDW2_Vd_Vn_Vm),
    SADDW_Vd_Vn_Vm(SADDW_Vd_Vn_Vm),
    SMLAL2_Vd_Vn_Vm(SMLAL2_Vd_Vn_Vm),
    SMLAL_Vd_Vn_Vm(SMLAL_Vd_Vn_Vm),
    SMLSL2_Vd_Vn_Vm(SMLSL2_Vd_Vn_Vm),
    SMLSL_Vd_Vn_Vm(SMLSL_Vd_Vn_Vm),
    SMULL2_Vd_Vn_Vm(SMULL2_Vd_Vn_Vm),
    SMULL_Vd_Vn_Vm(SMULL_Vd_Vn_Vm),
    SQDMLAL2_Vd_Vn_Vm(SQDMLAL2_Vd_Vn_Vm),
    SQDMLAL_Vd_Vn_Vm(SQDMLAL_Vd_Vn_Vm),
    SQDMLSL2_Vd_Vn_Vm(SQDMLSL2_Vd_Vn_Vm),
    SQDMLSL_Vd_Vn_Vm(SQDMLSL_Vd_Vn_Vm),
    SQDMULL2_Vd_Vn_Vm(SQDMULL2_Vd_Vn_Vm),
    SQDMULL_Vd_Vn_Vm(SQDMULL_Vd_Vn_Vm),
    SSUBL2_Vd_Vn_Vm(SSUBL2_Vd_Vn_Vm),
    SSUBL_Vd_Vn_Vm(SSUBL_Vd_Vn_Vm),
    SSUBW2_Vd_Vn_Vm(SSUBW2_Vd_Vn_Vm),
    SSUBW_Vd_Vn_Vm(SSUBW_Vd_Vn_Vm),
    SUBHN2_Vd_Vn_Vm(SUBHN2_Vd_Vn_Vm),
    SUBHN_Vd_Vn_Vm(SUBHN_Vd_Vn_Vm),
    UABAL2_Vd_Vn_Vm(UABAL2_Vd_Vn_Vm),
    UABAL_Vd_Vn_Vm(UABAL_Vd_Vn_Vm),
    UABDL2_Vd_Vn_Vm(UABDL2_Vd_Vn_Vm),
    UABDL_Vd_Vn_Vm(UABDL_Vd_Vn_Vm),
    UADDL2_Vd_Vn_Vm(UADDL2_Vd_Vn_Vm),
    UADDL_Vd_Vn_Vm(UADDL_Vd_Vn_Vm),
    UADDW2_Vd_Vn_Vm(UADDW2_Vd_Vn_Vm),
    UADDW_Vd_Vn_Vm(UADDW_Vd_Vn_Vm),
    UMLAL2_Vd_Vn_Vm(UMLAL2_Vd_Vn_Vm),
    UMLAL_Vd_Vn_Vm(UMLAL_Vd_Vn_Vm),
    UMLSL2_Vd_Vn_Vm(UMLSL2_Vd_Vn_Vm),
    UMLSL_Vd_Vn_Vm(UMLSL_Vd_Vn_Vm),
    UMULL2_Vd_Vn_Vm(UMULL2_Vd_Vn_Vm),
    UMULL_Vd_Vn_Vm(UMULL_Vd_Vn_Vm),
    USUBL2_Vd_Vn_Vm(USUBL2_Vd_Vn_Vm),
    USUBL_Vd_Vn_Vm(USUBL_Vd_Vn_Vm),
    USUBW2_Vd_Vn_Vm(USUBW2_Vd_Vn_Vm),
    USUBW_Vd_Vn_Vm(USUBW_Vd_Vn_Vm),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ASIMDELEM {
    FCMLA_Vd_Vn_Em_IMM_ROT2(FCMLA_Vd_Vn_Em_IMM_ROT2),
    FMLAL2_Vd_V_4S_Vn_V_4H_Em16_S_H(FMLAL2_Vd_V_4S_Vn_V_4H_Em16_S_H),
    FMLAL2_Vd_Vn_Em16(FMLAL2_Vd_Vn_Em16),
    FMLAL_Vd_V_4S_Vn_V_4H_Em16_S_H(FMLAL_Vd_V_4S_Vn_V_4H_Em16_S_H),
    FMLAL_Vd_Vn_Em16(FMLAL_Vd_Vn_Em16),
    FMLA_Vd_Vn_Em(FMLA_Vd_Vn_Em),
    FMLA_Vd_Vn_Em16(FMLA_Vd_Vn_Em16),
    FMLSL2_Vd_V_4S_Vn_V_4H_Em16_S_H(FMLSL2_Vd_V_4S_Vn_V_4H_Em16_S_H),
    FMLSL2_Vd_Vn_Em16(FMLSL2_Vd_Vn_Em16),
    FMLSL_Vd_V_4S_Vn_V_4H_Em16_S_H(FMLSL_Vd_V_4S_Vn_V_4H_Em16_S_H),
    FMLSL_Vd_Vn_Em16(FMLSL_Vd_Vn_Em16),
    FMLS_Vd_Vn_Em(FMLS_Vd_Vn_Em),
    FMLS_Vd_Vn_Em16(FMLS_Vd_Vn_Em16),
    FMULX_Vd_Vn_Em(FMULX_Vd_Vn_Em),
    FMULX_Vd_Vn_Em16(FMULX_Vd_Vn_Em16),
    FMUL_Vd_Vn_Em(FMUL_Vd_Vn_Em),
    FMUL_Vd_Vn_Em16(FMUL_Vd_Vn_Em16),
    MLA_Vd_Vn_Em16(MLA_Vd_Vn_Em16),
    MLS_Vd_Vn_Em16(MLS_Vd_Vn_Em16),
    MUL_Vd_Vn_Em16(MUL_Vd_Vn_Em16),
    SMLAL2_Vd_Vn_Em16(SMLAL2_Vd_Vn_Em16),
    SMLAL_Vd_Vn_Em16(SMLAL_Vd_Vn_Em16),
    SMLSL2_Vd_Vn_Em16(SMLSL2_Vd_Vn_Em16),
    SMLSL_Vd_Vn_Em16(SMLSL_Vd_Vn_Em16),
    SMULL2_Vd_Vn_Em16(SMULL2_Vd_Vn_Em16),
    SMULL_Vd_Vn_Em16(SMULL_Vd_Vn_Em16),
    SQDMLAL2_Vd_Vn_Em16(SQDMLAL2_Vd_Vn_Em16),
    SQDMLAL_Vd_Vn_Em16(SQDMLAL_Vd_Vn_Em16),
    SQDMLSL2_Vd_Vn_Em16(SQDMLSL2_Vd_Vn_Em16),
    SQDMLSL_Vd_Vn_Em16(SQDMLSL_Vd_Vn_Em16),
    SQDMULH_Vd_Vn_Em16(SQDMULH_Vd_Vn_Em16),
    SQDMULL2_Vd_Vn_Em16(SQDMULL2_Vd_Vn_Em16),
    SQDMULL_Vd_Vn_Em16(SQDMULL_Vd_Vn_Em16),
    SQRDMLAH_Vd_Vn_Em16(SQRDMLAH_Vd_Vn_Em16),
    SQRDMLSH_Vd_Vn_Em16(SQRDMLSH_Vd_Vn_Em16),
    SQRDMULH_Vd_Vn_Em16(SQRDMULH_Vd_Vn_Em16),
    UMLAL2_Vd_Vn_Em16(UMLAL2_Vd_Vn_Em16),
    UMLAL_Vd_Vn_Em16(UMLAL_Vd_Vn_Em16),
    UMLSL2_Vd_Vn_Em16(UMLSL2_Vd_Vn_Em16),
    UMLSL_Vd_Vn_Em16(UMLSL_Vd_Vn_Em16),
    UMULL2_Vd_Vn_Em16(UMULL2_Vd_Vn_Em16),
    UMULL_Vd_Vn_Em16(UMULL_Vd_Vn_Em16),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ASIMDEXT {
    EXT_Vd_Vn_Vm_IDX(EXT_Vd_Vn_Vm_IDX),
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
pub enum ASIMDINS {
    DUP_Vd_En(DUP_Vd_En),
    DUP_Vd_Rn(DUP_Vd_Rn),
    INS_Ed_En(INS_Ed_En),
    INS_Ed_Rn(INS_Ed_Rn),
    SMOV_Rd_En(SMOV_Rd_En),
    UMOV_Rd_En(UMOV_Rd_En),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ASIMDMISC {
    ABS_Vd_Vn(ABS_Vd_Vn),
    CLS_Vd_Vn(CLS_Vd_Vn),
    CLZ_Vd_Vn(CLZ_Vd_Vn),
    CMEQ_Vd_Vn_IMM0(CMEQ_Vd_Vn_IMM0),
    CMGE_Vd_Vn_IMM0(CMGE_Vd_Vn_IMM0),
    CMGT_Vd_Vn_IMM0(CMGT_Vd_Vn_IMM0),
    CMLE_Vd_Vn_IMM0(CMLE_Vd_Vn_IMM0),
    CMLT_Vd_Vn_IMM0(CMLT_Vd_Vn_IMM0),
    CNT_Vd_Vn(CNT_Vd_Vn),
    FABS_Vd_V_4H_Vn_V_4H(FABS_Vd_V_4H_Vn_V_4H),
    FABS_Vd_Vn(FABS_Vd_Vn),
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
    FNEG_Vd_V_4H_Vn_V_4H(FNEG_Vd_V_4H_Vn_V_4H),
    FNEG_Vd_Vn(FNEG_Vd_Vn),
    FRECPE_Vd_V_4H_Vn_V_4H(FRECPE_Vd_V_4H_Vn_V_4H),
    FRECPE_Vd_Vn(FRECPE_Vd_Vn),
    FRINT32X_Vd_Vn(FRINT32X_Vd_Vn),
    FRINT32Z_Vd_Vn(FRINT32Z_Vd_Vn),
    FRINT64X_Vd_Vn(FRINT64X_Vd_Vn),
    FRINT64Z_Vd_Vn(FRINT64Z_Vd_Vn),
    FRINTA_Vd_V_4H_Vn_V_4H(FRINTA_Vd_V_4H_Vn_V_4H),
    FRINTA_Vd_Vn(FRINTA_Vd_Vn),
    FRINTI_Vd_V_4H_Vn_V_4H(FRINTI_Vd_V_4H_Vn_V_4H),
    FRINTI_Vd_Vn(FRINTI_Vd_Vn),
    FRINTM_Vd_V_4H_Vn_V_4H(FRINTM_Vd_V_4H_Vn_V_4H),
    FRINTM_Vd_Vn(FRINTM_Vd_Vn),
    FRINTN_Vd_V_4H_Vn_V_4H(FRINTN_Vd_V_4H_Vn_V_4H),
    FRINTN_Vd_Vn(FRINTN_Vd_Vn),
    FRINTP_Vd_V_4H_Vn_V_4H(FRINTP_Vd_V_4H_Vn_V_4H),
    FRINTP_Vd_Vn(FRINTP_Vd_Vn),
    FRINTX_Vd_V_4H_Vn_V_4H(FRINTX_Vd_V_4H_Vn_V_4H),
    FRINTX_Vd_Vn(FRINTX_Vd_Vn),
    FRINTZ_Vd_V_4H_Vn_V_4H(FRINTZ_Vd_V_4H_Vn_V_4H),
    FRINTZ_Vd_Vn(FRINTZ_Vd_Vn),
    FRSQRTE_Vd_V_4H_Vn_V_4H(FRSQRTE_Vd_V_4H_Vn_V_4H),
    FRSQRTE_Vd_Vn(FRSQRTE_Vd_Vn),
    FSQRT_Vd_V_4H_Vn_V_4H(FSQRT_Vd_V_4H_Vn_V_4H),
    FSQRT_Vd_Vn(FSQRT_Vd_Vn),
    NEG_Vd_Vn(NEG_Vd_Vn),
    NOT_Vd_Vn(NOT_Vd_Vn),
    RBIT_Vd_Vn(RBIT_Vd_Vn),
    REV16_Vd_Vn(REV16_Vd_Vn),
    REV32_Vd_Vn(REV32_Vd_Vn),
    REV64_Vd_Vn(REV64_Vd_Vn),
    SADALP_Vd_Vn(SADALP_Vd_Vn),
    SADDLP_Vd_Vn(SADDLP_Vd_Vn),
    SCVTF_Vd_V_4H_Vn_V_4H(SCVTF_Vd_V_4H_Vn_V_4H),
    SCVTF_Vd_Vn(SCVTF_Vd_Vn),
    SHLL2_Vd_Vn_SHLL_IMM(SHLL2_Vd_Vn_SHLL_IMM),
    SHLL_Vd_Vn_SHLL_IMM(SHLL_Vd_Vn_SHLL_IMM),
    SQABS_Vd_Vn(SQABS_Vd_Vn),
    SQNEG_Vd_Vn(SQNEG_Vd_Vn),
    SQXTN2_Vd_Vn(SQXTN2_Vd_Vn),
    SQXTN_Vd_Vn(SQXTN_Vd_Vn),
    SQXTUN2_Vd_Vn(SQXTUN2_Vd_Vn),
    SQXTUN_Vd_Vn(SQXTUN_Vd_Vn),
    SUQADD_Vd_Vn(SUQADD_Vd_Vn),
    UADALP_Vd_Vn(UADALP_Vd_Vn),
    UADDLP_Vd_Vn(UADDLP_Vd_Vn),
    UCVTF_Vd_V_4H_Vn_V_4H(UCVTF_Vd_V_4H_Vn_V_4H),
    UCVTF_Vd_Vn(UCVTF_Vd_Vn),
    UQXTN2_Vd_Vn(UQXTN2_Vd_Vn),
    UQXTN_Vd_Vn(UQXTN_Vd_Vn),
    URECPE_Vd_Vn(URECPE_Vd_Vn),
    URSQRTE_Vd_Vn(URSQRTE_Vd_Vn),
    USQADD_Vd_Vn(USQADD_Vd_Vn),
    XTN2_Vd_Vn(XTN2_Vd_Vn),
    XTN_Vd_Vn(XTN_Vd_Vn),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ASIMDPERM {
    TRN1_Vd_Vn_Vm(TRN1_Vd_Vn_Vm),
    TRN2_Vd_Vn_Vm(TRN2_Vd_Vn_Vm),
    UZP1_Vd_Vn_Vm(UZP1_Vd_Vn_Vm),
    UZP2_Vd_Vn_Vm(UZP2_Vd_Vn_Vm),
    ZIP1_Vd_Vn_Vm(ZIP1_Vd_Vn_Vm),
    ZIP2_Vd_Vn_Vm(ZIP2_Vd_Vn_Vm),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ASIMDSAME {
    ADDP_Vd_Vn_Vm(ADDP_Vd_Vn_Vm),
    ADD_Vd_Vn_Vm(ADD_Vd_Vn_Vm),
    AND_Vd_Vn_Vm(AND_Vd_Vn_Vm),
    BIC_Vd_Vn_Vm(BIC_Vd_Vn_Vm),
    BIF_Vd_Vn_Vm(BIF_Vd_Vn_Vm),
    BIT_Vd_Vn_Vm(BIT_Vd_Vn_Vm),
    BSL_Vd_Vn_Vm(BSL_Vd_Vn_Vm),
    CMEQ_Vd_Vn_Vm(CMEQ_Vd_Vn_Vm),
    CMGE_Vd_Vn_Vm(CMGE_Vd_Vn_Vm),
    CMGT_Vd_Vn_Vm(CMGT_Vd_Vn_Vm),
    CMHI_Vd_Vn_Vm(CMHI_Vd_Vn_Vm),
    CMHS_Vd_Vn_Vm(CMHS_Vd_Vn_Vm),
    CMTST_Vd_Vn_Vm(CMTST_Vd_Vn_Vm),
    EOR_Vd_Vn_Vm(EOR_Vd_Vn_Vm),
    FABD_Vd_V_2S_Vn_V_2S_Vm_V_2S(FABD_Vd_V_2S_Vn_V_2S_Vm_V_2S),
    FABD_Vd_Vn_Vm(FABD_Vd_Vn_Vm),
    FACGE_Vd_V_2S_Vn_V_2S_Vm_V_2S(FACGE_Vd_V_2S_Vn_V_2S_Vm_V_2S),
    FACGE_Vd_Vn_Vm(FACGE_Vd_Vn_Vm),
    FACGT_Vd_V_2S_Vn_V_2S_Vm_V_2S(FACGT_Vd_V_2S_Vn_V_2S_Vm_V_2S),
    FACGT_Vd_Vn_Vm(FACGT_Vd_Vn_Vm),
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
    FMAXNMP_Vd_V_2S_Vn_V_2S_Vm_V_2S(FMAXNMP_Vd_V_2S_Vn_V_2S_Vm_V_2S),
    FMAXNMP_Vd_Vn_Vm(FMAXNMP_Vd_Vn_Vm),
    FMAXNM_Vd_V_2S_Vn_V_2S_Vm_V_2S(FMAXNM_Vd_V_2S_Vn_V_2S_Vm_V_2S),
    FMAXNM_Vd_Vn_Vm(FMAXNM_Vd_Vn_Vm),
    FMAXP_Vd_V_2S_Vn_V_2S_Vm_V_2S(FMAXP_Vd_V_2S_Vn_V_2S_Vm_V_2S),
    FMAXP_Vd_Vn_Vm(FMAXP_Vd_Vn_Vm),
    FMAX_Vd_V_2S_Vn_V_2S_Vm_V_2S(FMAX_Vd_V_2S_Vn_V_2S_Vm_V_2S),
    FMAX_Vd_Vn_Vm(FMAX_Vd_Vn_Vm),
    FMINNMP_Vd_V_2S_Vn_V_2S_Vm_V_2S(FMINNMP_Vd_V_2S_Vn_V_2S_Vm_V_2S),
    FMINNMP_Vd_Vn_Vm(FMINNMP_Vd_Vn_Vm),
    FMINNM_Vd_V_2S_Vn_V_2S_Vm_V_2S(FMINNM_Vd_V_2S_Vn_V_2S_Vm_V_2S),
    FMINNM_Vd_Vn_Vm(FMINNM_Vd_Vn_Vm),
    FMINP_Vd_V_2S_Vn_V_2S_Vm_V_2S(FMINP_Vd_V_2S_Vn_V_2S_Vm_V_2S),
    FMINP_Vd_Vn_Vm(FMINP_Vd_Vn_Vm),
    FMIN_Vd_V_2S_Vn_V_2S_Vm_V_2S(FMIN_Vd_V_2S_Vn_V_2S_Vm_V_2S),
    FMIN_Vd_Vn_Vm(FMIN_Vd_Vn_Vm),
    FMLAL2_Vd_V_4S_Vn_V_4H_Vm_V_4H(FMLAL2_Vd_V_4S_Vn_V_4H_Vm_V_4H),
    FMLAL2_Vd_Vn_Vm(FMLAL2_Vd_Vn_Vm),
    FMLAL_Vd_V_4S_Vn_V_4H_Vm_V_4H(FMLAL_Vd_V_4S_Vn_V_4H_Vm_V_4H),
    FMLAL_Vd_Vn_Vm(FMLAL_Vd_Vn_Vm),
    FMLA_Vd_V_2S_Vn_V_2S_Vm_V_2S(FMLA_Vd_V_2S_Vn_V_2S_Vm_V_2S),
    FMLA_Vd_Vn_Vm(FMLA_Vd_Vn_Vm),
    FMLSL2_Vd_V_4S_Vn_V_4H_Vm_V_4H(FMLSL2_Vd_V_4S_Vn_V_4H_Vm_V_4H),
    FMLSL2_Vd_Vn_Vm(FMLSL2_Vd_Vn_Vm),
    FMLSL_Vd_V_4S_Vn_V_4H_Vm_V_4H(FMLSL_Vd_V_4S_Vn_V_4H_Vm_V_4H),
    FMLSL_Vd_Vn_Vm(FMLSL_Vd_Vn_Vm),
    FMLS_Vd_V_2S_Vn_V_2S_Vm_V_2S(FMLS_Vd_V_2S_Vn_V_2S_Vm_V_2S),
    FMLS_Vd_Vn_Vm(FMLS_Vd_Vn_Vm),
    FMULX_Vd_V_2S_Vn_V_2S_Vm_V_2S(FMULX_Vd_V_2S_Vn_V_2S_Vm_V_2S),
    FMULX_Vd_Vn_Vm(FMULX_Vd_Vn_Vm),
    FMUL_Vd_V_2S_Vn_V_2S_Vm_V_2S(FMUL_Vd_V_2S_Vn_V_2S_Vm_V_2S),
    FMUL_Vd_Vn_Vm(FMUL_Vd_Vn_Vm),
    FRECPS_Vd_V_2S_Vn_V_2S_Vm_V_2S(FRECPS_Vd_V_2S_Vn_V_2S_Vm_V_2S),
    FRECPS_Vd_Vn_Vm(FRECPS_Vd_Vn_Vm),
    FRSQRTS_Vd_V_2S_Vn_V_2S_Vm_V_2S(FRSQRTS_Vd_V_2S_Vn_V_2S_Vm_V_2S),
    FRSQRTS_Vd_Vn_Vm(FRSQRTS_Vd_Vn_Vm),
    FSUB_Vd_V_2S_Vn_V_2S_Vm_V_2S(FSUB_Vd_V_2S_Vn_V_2S_Vm_V_2S),
    FSUB_Vd_Vn_Vm(FSUB_Vd_Vn_Vm),
    MLA_Vd_Vn_Vm(MLA_Vd_Vn_Vm),
    MLS_Vd_Vn_Vm(MLS_Vd_Vn_Vm),
    MUL_Vd_Vn_Vm(MUL_Vd_Vn_Vm),
    ORN_Vd_Vn_Vm(ORN_Vd_Vn_Vm),
    ORR_Vd_Vn_Vm(ORR_Vd_Vn_Vm),
    PMUL_Vd_Vn_Vm(PMUL_Vd_Vn_Vm),
    SABA_Vd_Vn_Vm(SABA_Vd_Vn_Vm),
    SABD_Vd_Vn_Vm(SABD_Vd_Vn_Vm),
    SHADD_Vd_Vn_Vm(SHADD_Vd_Vn_Vm),
    SHSUB_Vd_Vn_Vm(SHSUB_Vd_Vn_Vm),
    SMAXP_Vd_Vn_Vm(SMAXP_Vd_Vn_Vm),
    SMAX_Vd_Vn_Vm(SMAX_Vd_Vn_Vm),
    SMINP_Vd_Vn_Vm(SMINP_Vd_Vn_Vm),
    SMIN_Vd_Vn_Vm(SMIN_Vd_Vn_Vm),
    SQADD_Vd_Vn_Vm(SQADD_Vd_Vn_Vm),
    SQDMULH_Vd_Vn_Vm(SQDMULH_Vd_Vn_Vm),
    SQRDMLAH_Sd_Sn_Sm(SQRDMLAH_Sd_Sn_Sm),
    SQRDMLAH_Vd_Vn_Vm(SQRDMLAH_Vd_Vn_Vm),
    SQRDMLSH_Sd_Sn_Sm(SQRDMLSH_Sd_Sn_Sm),
    SQRDMLSH_Vd_Vn_Vm(SQRDMLSH_Vd_Vn_Vm),
    SQRDMULH_Vd_Vn_Vm(SQRDMULH_Vd_Vn_Vm),
    SQRSHL_Vd_Vn_Vm(SQRSHL_Vd_Vn_Vm),
    SQSHL_Vd_Vn_Vm(SQSHL_Vd_Vn_Vm),
    SQSUB_Vd_Vn_Vm(SQSUB_Vd_Vn_Vm),
    SRHADD_Vd_Vn_Vm(SRHADD_Vd_Vn_Vm),
    SRSHL_Vd_Vn_Vm(SRSHL_Vd_Vn_Vm),
    SSHL_Vd_Vn_Vm(SSHL_Vd_Vn_Vm),
    SUB_Vd_Vn_Vm(SUB_Vd_Vn_Vm),
    UABA_Vd_Vn_Vm(UABA_Vd_Vn_Vm),
    UABD_Vd_Vn_Vm(UABD_Vd_Vn_Vm),
    UHADD_Vd_Vn_Vm(UHADD_Vd_Vn_Vm),
    UHSUB_Vd_Vn_Vm(UHSUB_Vd_Vn_Vm),
    UMAXP_Vd_Vn_Vm(UMAXP_Vd_Vn_Vm),
    UMAX_Vd_Vn_Vm(UMAX_Vd_Vn_Vm),
    UMINP_Vd_Vn_Vm(UMINP_Vd_Vn_Vm),
    UMIN_Vd_Vn_Vm(UMIN_Vd_Vn_Vm),
    UQADD_Vd_Vn_Vm(UQADD_Vd_Vn_Vm),
    UQRSHL_Vd_Vn_Vm(UQRSHL_Vd_Vn_Vm),
    UQSHL_Vd_Vn_Vm(UQSHL_Vd_Vn_Vm),
    UQSUB_Vd_Vn_Vm(UQSUB_Vd_Vn_Vm),
    URHADD_Vd_Vn_Vm(URHADD_Vd_Vn_Vm),
    URSHL_Vd_Vn_Vm(URSHL_Vd_Vn_Vm),
    USHL_Vd_Vn_Vm(USHL_Vd_Vn_Vm),
}
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
