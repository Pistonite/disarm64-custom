#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
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
pub struct SUQADD_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SUQADD_Sd_Sn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
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
pub struct UABA_Vd_Vn_Vm {
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
pub struct UABAL_Vd_Vn_Vm {
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
pub struct UABAL2_Vd_Vn_Vm {
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
pub struct UABD_Vd_Vn_Vm {
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
pub struct UABDL_Vd_Vn_Vm {
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
pub struct UABDL2_Vd_Vn_Vm {
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
pub struct UADALP_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
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
