#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BFMLSL_SME_ZA_array_off2x2_S_S_SVE_ZnxN_S_H_SME_Zm_S_H {
    #[bits(2)]
    pub imm2_0: u32,
    #[bits(3)]
    pub _op_2: u32,
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
pub struct BFMLSL_SME_ZA_array_off2x2_SME_Znx2_SME_Zmx2 {
    #[bits(2)]
    pub imm2_0: u32,
    #[bits(4)]
    pub _op_2: u32,
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
pub struct BFMLSL_SME_ZA_array_off2x2_SME_Znx4_SME_Zmx4 {
    #[bits(2)]
    pub imm2_0: u32,
    #[bits(5)]
    pub _op_2: u32,
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
pub struct BFMLSL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm {
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
pub struct BFMLSLB_SVE_Zd_SVE_Zn_SVE_Zm_16 {
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
pub struct BFMLSLB_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX {
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
pub struct BFMLSLT_SVE_Zd_SVE_Zn_SVE_Zm_16 {
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
pub struct BFMLSLT_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX {
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
pub struct BFMMLA_SVE_Zd_SVE_Zn_SVE_Zm_16 {
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
pub struct BFMMLA_Vd_Vn_Vm {
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
pub struct BFMOPA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16 {
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
pub struct BFMOPS_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn_SVE_Zm_16 {
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
pub struct BFMUL_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX {
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
pub struct BFMUL_SVE_Zd_SVE_Zn_SVE_Zm_16 {
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
pub struct BFMUL_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
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
pub struct BFSUB_SVE_Zd_SVE_Zn_SVE_Zm_16 {
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
pub struct BFSUB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
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
pub struct BFVDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2 {
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
pub struct BIC_Rd_Rn_Rm_SFT {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BIC_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
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
pub struct BIC_SVE_Zd_SVE_Zn_SVE_Zm_16 {
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
pub struct BIC_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm {
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
pub struct BIC_Vd_Vn_Vm {
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
pub struct BIC_Vd_SIMD_IMM_SFT {
    #[bits(5)]
    pub rd: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BIC_Vd_V_4H_SIMD_IMM_SFT_LSL {
    #[bits(5)]
    pub rd: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BICS_Rd_Rn_Rm_SFT {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BICS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm {
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
pub struct BIF_Vd_Vn_Vm {
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
pub struct BIT_Vd_Vn_Vm {
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
pub struct BL_ADDR_PCREL26 {
    #[bits(26)]
    pub imm26: u32,
    #[bits(6)]
    pub _op_26: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BLR_Rn {
    #[bits(5)]
    pub _op_0: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BLRAA_Rn_Rd_SP {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BLRAAZ_Rn {
    #[bits(5)]
    pub _op_0: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BLRAB_Rn_Rd_SP {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BLRABZ_Rn {
    #[bits(5)]
    pub _op_0: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BR_Rn {
    #[bits(5)]
    pub _op_0: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BRAA_Rn_Rd_SP {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BRAAZ_Rn {
    #[bits(5)]
    pub _op_0: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BRAB_Rn_Rd_SP {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BRABZ_Rn {
    #[bits(5)]
    pub _op_0: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BRK_EXCEPTION {
    #[bits(5)]
    pub _op_0: u32,
    #[bits(16)]
    pub imm16_5: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BRKA_SVE_Pd_SVE_Pg4_10_SVE_Pn {
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
    #[bits(18)]
    pub _op_14: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BRKAS_SVE_Pd_SVE_Pg4_10_SVE_Pn {
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
    #[bits(18)]
    pub _op_14: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BRKB_SVE_Pd_SVE_Pg4_10_SVE_Pn {
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
    #[bits(18)]
    pub _op_14: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BRKBS_SVE_Pd_SVE_Pg4_10_SVE_Pn {
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
    #[bits(18)]
    pub _op_14: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BRKN_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pd {
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
    #[bits(18)]
    pub _op_14: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BRKNS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pd {
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
    #[bits(18)]
    pub _op_14: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BRKPA_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm {
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
pub struct BRKPAS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm {
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
pub struct BRKPB_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm {
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
pub struct BRKPBS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm {
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
pub struct CBNZ_Rt_ADDR_PCREL19 {
    #[bits(5)]
    pub rt: u32,
    #[bits(19)]
    pub imm19: u32,
    #[bits(8)]
    pub _op_24: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CBZ_Rt_ADDR_PCREL19 {
    #[bits(5)]
    pub rt: u32,
    #[bits(19)]
    pub imm19: u32,
    #[bits(8)]
    pub _op_24: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CCMN_Rn_Rm_NZCV_COND {
    #[bits(4)]
    pub nzcv: u32,
    #[bits(1)]
    pub _op_4: u32,
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
pub struct CCMN_Rn_CCMP_IMM_NZCV_COND {
    #[bits(4)]
    pub nzcv: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub imm5: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CCMP_Rn_Rm_NZCV_COND {
    #[bits(4)]
    pub nzcv: u32,
    #[bits(1)]
    pub _op_4: u32,
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
pub struct CCMP_Rn_CCMP_IMM_NZCV_COND {
    #[bits(4)]
    pub nzcv: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub imm5: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CDOT_SVE_Zd_SVE_Zn_SVE_Zm_16_SVE_IMM_ROT2 {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(2)]
    pub sve_rot2: u32,
    #[bits(4)]
    pub _op_12: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CDOT_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX_SVE_IMM_ROT2 {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(2)]
    pub sve_rot2: u32,
    #[bits(4)]
    pub _op_12: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CDOT_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX_SVE_IMM_ROT2 {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(2)]
    pub sve_rot2: u32,
    #[bits(4)]
    pub _op_12: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CFINV {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CHKFEAT_X16 {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CLASTA_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
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
pub struct CLASTA_SVE_Vd_SVE_Pg3_SVE_Vd_SVE_Zm_5 {
    #[bits(5)]
    pub sve_vd: u32,
    #[bits(5)]
    pub sve_zm_5: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(19)]
    pub _op_13: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CLASTA_Rd_SVE_Pg3_Rd_SVE_Zm_5 {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub sve_zm_5: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(19)]
    pub _op_13: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CLASTB_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
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
pub struct CLASTB_SVE_Vd_SVE_Pg3_SVE_Vd_SVE_Zm_5 {
    #[bits(5)]
    pub sve_vd: u32,
    #[bits(5)]
    pub sve_zm_5: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(19)]
    pub _op_13: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CLASTB_Rd_SVE_Pg3_Rd_SVE_Zm_5 {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub sve_zm_5: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(19)]
    pub _op_13: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CLREX_UIMM4 {
    #[bits(8)]
    pub _op_0: u32,
    #[bits(4)]
    pub crm: u32,
    #[bits(20)]
    pub _op_12: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CLS_Rd_Rn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CLS_SVE_Zd_SVE_Pg3_SVE_Zn {
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
pub struct CLS_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CLZ_Rd_Rn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CLZ_SVE_Zd_SVE_Pg3_SVE_Zn {
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
pub struct CLZ_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMEQ_Vd_Vn_IMM0 {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMEQ_Sd_Sn_IMM0 {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMEQ_Vd_Vn_Vm {
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
pub struct CMEQ_Sd_Sn_Sm {
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
pub struct CMGE_Vd_Vn_IMM0 {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMGE_Sd_Sn_IMM0 {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMGE_Vd_Vn_Vm {
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
pub struct CMGE_Sd_Sn_Sm {
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
pub struct CMGT_Vd_Vn_IMM0 {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMGT_Sd_Sn_IMM0 {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMGT_Vd_Vn_Vm {
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
pub struct CMGT_Sd_Sn_Sm {
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
pub struct CMHI_Vd_Vn_Vm {
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
pub struct CMHI_Sd_Sn_Sm {
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
pub struct CMHS_Vd_Vn_Vm {
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
pub struct CMHS_Sd_Sn_Sm {
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
pub struct CMLA_SVE_Zd_SVE_Zn_SVE_Zm_16_SVE_IMM_ROT2 {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(2)]
    pub sve_rot2: u32,
    #[bits(4)]
    pub _op_12: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMLA_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX_SVE_IMM_ROT2 {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(2)]
    pub sve_rot2: u32,
    #[bits(4)]
    pub _op_12: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMLA_SVE_Zd_SVE_Zn_SVE_Zm4_INDEX_SVE_IMM_ROT2 {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(2)]
    pub sve_rot2: u32,
    #[bits(4)]
    pub _op_12: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMLE_Vd_Vn_IMM0 {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMLE_Sd_Sn_IMM0 {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMLT_Vd_Vn_IMM0 {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMLT_Sd_Sn_IMM0 {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMPEQ_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16 {
    #[bits(4)]
    pub sve_pd: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(3)]
    pub _op_13: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMPEQ_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_B {
    #[bits(4)]
    pub sve_pd: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(3)]
    pub _op_13: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMPEQ_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5 {
    #[bits(4)]
    pub sve_pd: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(3)]
    pub _op_13: u32,
    #[bits(5)]
    pub imm5: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMPGE_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16 {
    #[bits(4)]
    pub sve_pd: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(3)]
    pub _op_13: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMPGE_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_B {
    #[bits(4)]
    pub sve_pd: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(3)]
    pub _op_13: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMPGE_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5 {
    #[bits(4)]
    pub sve_pd: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(3)]
    pub _op_13: u32,
    #[bits(5)]
    pub imm5: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMPGT_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16 {
    #[bits(4)]
    pub sve_pd: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(3)]
    pub _op_13: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMPGT_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_B {
    #[bits(4)]
    pub sve_pd: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(3)]
    pub _op_13: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMPGT_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5 {
    #[bits(4)]
    pub sve_pd: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(3)]
    pub _op_13: u32,
    #[bits(5)]
    pub imm5: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMPHI_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16 {
    #[bits(4)]
    pub sve_pd: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(3)]
    pub _op_13: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMPHI_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_D {
    #[bits(4)]
    pub sve_pd: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(3)]
    pub _op_13: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMPHI_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_UIMM7 {
    #[bits(4)]
    pub sve_pd: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(1)]
    pub _op_13: u32,
    #[bits(7)]
    pub sve_imm7: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMPHS_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16 {
    #[bits(4)]
    pub sve_pd: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(3)]
    pub _op_13: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMPHS_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_D {
    #[bits(4)]
    pub sve_pd: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(3)]
    pub _op_13: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMPHS_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_UIMM7 {
    #[bits(4)]
    pub sve_pd: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(1)]
    pub _op_13: u32,
    #[bits(7)]
    pub sve_imm7: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMPLE_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16 {
    #[bits(4)]
    pub sve_pd: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(3)]
    pub _op_13: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMPLE_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5 {
    #[bits(4)]
    pub sve_pd: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(3)]
    pub _op_13: u32,
    #[bits(5)]
    pub imm5: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMPLO_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16 {
    #[bits(4)]
    pub sve_pd: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(3)]
    pub _op_13: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMPLO_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_UIMM7 {
    #[bits(4)]
    pub sve_pd: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(1)]
    pub _op_13: u32,
    #[bits(7)]
    pub sve_imm7: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMPLS_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16 {
    #[bits(4)]
    pub sve_pd: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(3)]
    pub _op_13: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMPLS_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_UIMM7 {
    #[bits(4)]
    pub sve_pd: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(1)]
    pub _op_13: u32,
    #[bits(7)]
    pub sve_imm7: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMPLT_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16 {
    #[bits(4)]
    pub sve_pd: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(3)]
    pub _op_13: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMPLT_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5 {
    #[bits(4)]
    pub sve_pd: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(3)]
    pub _op_13: u32,
    #[bits(5)]
    pub imm5: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMPNE_SVE_Pd_SVE_Pg3_SVE_Zn_SVE_Zm_16 {
    #[bits(4)]
    pub sve_pd: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(3)]
    pub _op_13: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMPNE_SVE_Pd_S_B_SVE_Pg3_P_Z_SVE_Zn_S_B_SVE_Zm_16_S_B {
    #[bits(4)]
    pub sve_pd: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(3)]
    pub _op_13: u32,
    #[bits(5)]
    pub sve_zm_16: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMPNE_SVE_Pd_SVE_Pg3_SVE_Zn_SIMM5 {
    #[bits(4)]
    pub sve_pd: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(3)]
    pub _op_13: u32,
    #[bits(5)]
    pub imm5: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CMTST_Vd_Vn_Vm {
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
pub struct CMTST_Sd_Sn_Sm {
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
pub struct CNOT_SVE_Zd_SVE_Pg3_SVE_Zn {
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
pub struct CNT_Rd_Rn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CNT_SVE_Zd_SVE_Pg3_SVE_Zn {
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
pub struct CNT_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CNTB_Rd_SVE_PATTERN_SCALED {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub sve_pattern: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CNTD_Rd_SVE_PATTERN_SCALED {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub sve_pattern: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CNTH_Rd_SVE_PATTERN_SCALED {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub sve_pattern: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CNTP_Rd_SVE_Pg4_10_SVE_Pn {
    #[bits(5)]
    pub rd: u32,
    #[bits(4)]
    pub sve_pn: u32,
    #[bits(1)]
    pub _op_9: u32,
    #[bits(4)]
    pub sve_pg4_10: u32,
    #[bits(18)]
    pub _op_14: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CNTP_Rd_SME_PNn_SME_VLxN_10 {
    #[bits(5)]
    pub rd: u32,
    #[bits(4)]
    pub sve_pn: u32,
    #[bits(1)]
    pub _op_9: u32,
    #[bits(1)]
    pub sme_vl_10: u32,
    #[bits(21)]
    pub _op_11: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CNTW_Rd_SVE_PATTERN_SCALED {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub sve_pattern: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CSEL_Rd_Rn_Rm_COND {
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
pub struct CSINC_Rd_Rn_Rm_COND {
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
pub struct CSINV_Rd_Rn_Rm_COND {
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
pub struct CSNEG_Rd_Rn_Rm_COND {
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
pub struct CTERMEQ_Rn_Rm {
    #[bits(5)]
    pub _op_0: u32,
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
pub struct CTERMNE_Rn_Rm {
    #[bits(5)]
    pub _op_0: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(6)]
    pub _op_10: u32,
    #[bits(5)]
    pub rm: u32,
    #[bits(11)]
    pub _op_21: u32,
}
