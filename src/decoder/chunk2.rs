#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FMLAL_Vd_Vn_Em16 {
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
pub struct FMLAL_Vd_V_4S_Vn_V_4H_Em16_S_H {
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
pub struct FMLAL2_Vd_Vn_Vm {
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
pub struct FMLAL2_Vd_V_4S_Vn_V_4H_Vm_V_4H {
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
pub struct FMLAL2_Vd_Vn_Em16 {
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
pub struct FMLAL2_Vd_V_4S_Vn_V_4H_Em16_S_H {
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
pub struct FMLS_Vd_Vn_Vm {
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
pub struct FMLS_Vd_V_2S_Vn_V_2S_Vm_V_2S {
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
pub struct FMLS_Vd_Vn_Em16 {
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
pub struct FMLS_Vd_Vn_Em {
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
pub struct FMLS_Sd_Sn_Em16 {
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
pub struct FMLS_Sd_Sn_Em {
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
pub struct FMLSL_Vd_Vn_Vm {
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
pub struct FMLSL_Vd_V_4S_Vn_V_4H_Vm_V_4H {
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
pub struct FMLSL_Vd_Vn_Em16 {
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
pub struct FMLSL_Vd_V_4S_Vn_V_4H_Em16_S_H {
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
pub struct FMLSL2_Vd_Vn_Vm {
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
pub struct FMLSL2_Vd_V_4S_Vn_V_4H_Vm_V_4H {
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
pub struct FMLSL2_Vd_Vn_Em16 {
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
pub struct FMLSL2_Vd_V_4S_Vn_V_4H_Em16_S_H {
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
pub struct FNEG_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FNEG_Vd_V_4H_Vn_V_4H {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FRECPE_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FRECPE_Sd_Sn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FRECPE_Vd_V_4H_Vn_V_4H {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FRECPE_Sd_S_H_Sn_S_H {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FRECPS_Vd_Vn_Vm {
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
pub struct FRECPS_Sd_Sn_Sm {
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
pub struct FRECPS_Vd_V_2S_Vn_V_2S_Vm_V_2S {
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
pub struct FRECPS_Sd_S_S_Sn_S_S_Sm_S_S {
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
pub struct FRECPX_Sd_Sn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FRECPX_Sd_S_H_Sn_S_H {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FRINT32X_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FRINT32Z_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FRINT64X_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FRINT64Z_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FRINTA_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FRINTA_Vd_V_4H_Vn_V_4H {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FRINTI_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FRINTI_Vd_V_4H_Vn_V_4H {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FRINTM_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FRINTM_Vd_V_4H_Vn_V_4H {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FRINTN_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FRINTN_Vd_V_4H_Vn_V_4H {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FRINTP_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FRINTP_Vd_V_4H_Vn_V_4H {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FRINTX_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FRINTX_Vd_V_4H_Vn_V_4H {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FRINTZ_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FRINTZ_Vd_V_4H_Vn_V_4H {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FRSQRTE_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FRSQRTE_Sd_Sn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FRSQRTE_Vd_V_4H_Vn_V_4H {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FRSQRTE_Sd_S_H_Sn_S_H {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FRSQRTS_Vd_Vn_Vm {
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
pub struct FRSQRTS_Sd_Sn_Sm {
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
pub struct FRSQRTS_Vd_V_2S_Vn_V_2S_Vm_V_2S {
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
pub struct FRSQRTS_Sd_S_S_Sn_S_S_Sm_S_S {
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
pub struct FSQRT_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FSQRT_Vd_V_4H_Vn_V_4H {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
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
pub struct GMI_Rd_Rn_SP_Rm {
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
pub struct INS_Ed_Rn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct INS_Ed_En {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct IRG_Rd_SP_Rn_SP_Rm {
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
pub struct LD1_LVt_SIMD_ADDR_SIMPLE {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LD1_LEt_SIMD_ADDR_SIMPLE {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LD1_LVt_SIMD_ADDR_POST {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LD1_LEt_SIMD_ADDR_POST {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LD1R_LVt_AL_SIMD_ADDR_SIMPLE {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LD1R_LVt_AL_SIMD_ADDR_POST {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LD2_LVt_SIMD_ADDR_SIMPLE {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LD2_LEt_SIMD_ADDR_SIMPLE {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LD2_LVt_SIMD_ADDR_POST {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LD2_LEt_SIMD_ADDR_POST {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LD2R_LVt_AL_SIMD_ADDR_SIMPLE {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LD2R_LVt_AL_SIMD_ADDR_POST {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LD3_LVt_SIMD_ADDR_SIMPLE {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LD3_LEt_SIMD_ADDR_SIMPLE {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LD3_LVt_SIMD_ADDR_POST {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LD3_LEt_SIMD_ADDR_POST {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LD3R_LVt_AL_SIMD_ADDR_SIMPLE {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LD3R_LVt_AL_SIMD_ADDR_POST {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LD4_LVt_SIMD_ADDR_SIMPLE {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LD4_LEt_SIMD_ADDR_SIMPLE {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LD4_LVt_SIMD_ADDR_POST {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LD4_LEt_SIMD_ADDR_POST {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LD4R_LVt_AL_SIMD_ADDR_SIMPLE {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LD4R_LVt_AL_SIMD_ADDR_POST {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAPR_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAPRB_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAPRH_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAPUR_Rt_ADDR_OFFSET {
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
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAPUR_Rt_X_ADDR_OFFSET {
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
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAPURB_Rt_ADDR_OFFSET {
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
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAPURH_Rt_ADDR_OFFSET {
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
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAPURSB_Rt_ADDR_OFFSET {
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
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAPURSB_Rt_W_ADDR_OFFSET {
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
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAPURSH_Rt_ADDR_OFFSET {
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
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAPURSH_Rt_W_ADDR_OFFSET {
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
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAPURSW_Rt_ADDR_OFFSET {
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
    #[bits(11)]
    pub _op_21: u32,
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
pub struct LDAXP_Rt_Rt2_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub _op_5: u32,
    #[bits(5)]
    pub rt2: u32,
    #[bits(17)]
    pub _op_15: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAXR_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAXRB_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAXRH_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDG_Rt_ADDR_SIMM13 {
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
pub struct LDGM_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDLAR_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDLARB_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDLARH_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDNP_Rt_Rt2_ADDR_SIMM7 {
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
pub struct LDNP_Ft_Ft2_ADDR_SIMM7 {
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
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDRSW_Rt_ADDR_REGOFF {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
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
pub struct LDTR_Rt_ADDR_SIMM9 {
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
pub struct LDTRB_Rt_ADDR_SIMM9 {
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
pub struct LDTRH_Rt_ADDR_SIMM9 {
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
pub struct LDTRSB_Rt_ADDR_SIMM9 {
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
pub struct LDTRSH_Rt_ADDR_SIMM9 {
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
