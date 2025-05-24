#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Mnemonic {
    r#add,
    r#addg,
    r#addha,
    r#addhn,
    r#addhn2,
    r#addhnb,
    r#addhnt,
    r#addp,
    r#addpl,
    r#addqv,
    r#adds,
    r#addspl,
    r#addsvl,
    r#addv,
    r#addva,
    r#addvl,
    r#adr,
    r#adrp,
    r#and,
    r#andqv,
    r#ands,
    r#andv,
    r#asr,
    r#asrd,
    r#asrr,
    r#asrv,
    r#b,
    r#bfadd,
    r#bfclamp,
    r#bfcvt,
    r#bfcvtn,
    r#bfcvtn2,
    r#bfcvtnt,
    r#bfdot,
    r#bfm,
    r#bfmax,
    r#bfmaxnm,
    r#bfmin,
    r#bfminnm,
    r#bfmla,
    r#bfmlal,
    r#bfmlalb,
    r#bfmlalt,
    r#bfmls,
    r#bfmlsl,
    r#bfmlslb,
    r#bfmlslt,
    r#bfmmla,
    r#bfmopa,
    r#bfmops,
    r#bfmul,
    r#bfsub,
    r#bfvdot,
    r#bic,
    r#bics,
    r#bif,
    r#bit,
    r#bl,
    r#blr,
    r#blraa,
    r#blraaz,
    r#blrab,
    r#blrabz,
    r#br,
    r#braa,
    r#braaz,
    r#brab,
    r#brabz,
    r#brk,
    r#brka,
    r#brkas,
    r#brkb,
    r#brkbs,
    r#brkn,
    r#brkns,
    r#brkpa,
    r#brkpas,
    r#brkpb,
    r#brkpbs,
    r#cbnz,
    r#cbz,
    r#ccmn,
    r#ccmp,
    r#cdot,
    r#cfinv,
    r#chkfeat,
    r#clasta,
    r#clastb,
    r#clrex,
    r#cls,
    r#clz,
    r#cmeq,
    r#cmge,
    r#cmgt,
    r#cmhi,
    r#cmhs,
    r#cmla,
    r#cmle,
    r#cmlt,
    r#cmpeq,
    r#cmpge,
    r#cmpgt,
    r#cmphi,
    r#cmphs,
    r#cmple,
    r#cmplo,
    r#cmpls,
    r#cmplt,
    r#cmpne,
    r#cmtst,
    r#cnot,
    r#cnt,
    r#cntb,
    r#cntd,
    r#cnth,
    r#cntp,
    r#cntw,
    r#csel,
    r#csinc,
    r#csinv,
    r#csneg,
    r#ctermeq,
    r#ctermne,
    r#ctz,
    r#eor,
    r#eor3,
    r#eorbt,
    r#eorqv,
    r#eors,
    r#eortb,
    r#eorv,
    r#fadd,
    r#fadda,
    r#faddp,
    r#faddqv,
    r#faddv,
    r#fcadd,
    r#fccmp,
    r#fccmpe,
    r#fclamp,
    r#fcmeq,
    r#fcmge,
    r#fcmgt,
    r#fcmla,
    r#fcmle,
    r#fcmlt,
    r#fcmne,
    r#fcmp,
    r#fcmpe,
    r#fcmuo,
    r#fcpy,
    r#fcsel,
    r#fcvt,
    r#fcvtas,
    r#fcvtau,
    r#fcvtl,
    r#fcvtl2,
    r#fcvtlt,
    r#fcvtms,
    r#fcvtmu,
    r#fcvtn,
    r#fcvtn2,
    r#fcvtns,
    r#fcvtnt,
    r#fcvtnu,
    r#fcvtps,
    r#fcvtpu,
    r#fcvtx,
    r#fcvtxn,
    r#fcvtxn2,
    r#fcvtxnt,
    r#fcvtzs,
    r#fcvtzu,
    r#fdiv,
    r#fdivr,
    r#fdot,
    r#fdup,
    r#fexpa,
    r#fjcvtzs,
    r#fmopa,
    r#fmops,
    r#fmov,
    r#fmsb,
    r#fmsub,
    r#fmul,
    r#fmulx,
    r#fsub,
    r#fsubr,
    r#ldp,
    r#ldpsw,
    r#ldr,
    r#ldraa,
    r#ldrab,
    r#ldrb,
    r#ldrh,
    r#ldrsb,
    r#ldrsh,
    r#ldrsw,
    r#ldur,
    r#ldurb,
    r#ldurh,
    r#ldursb,
    r#ldursh,
    r#ldursw,
    r#lsl,
    r#lslr,
    r#lslv,
    r#lsr,
    r#lsrr,
    r#lsrv,
    r#madd,
    r#mov,
    r#mova,
    r#movaz,
    r#movi,
    r#movk,
    r#movn,
    r#movprfx,
    r#movt,
    r#movz,
    r#msub,
    r#mul,
    r#mvni,
    r#neg,
    r#orn,
    r#orns,
    r#orr,
    r#orrs,
    r#orv,
    r#ret,
    r#retaa,
    r#retab,
    r#sb,
    r#sbc,
    r#sbclb,
    r#sbclt,
    r#sbcs,
    r#sbfm,
    r#scvtf,
    r#sdiv,
    r#sdivr,
    r#sdot,
    r#smaddl,
    r#smulh,
    r#smull,
    r#smull2,
    r#smullb,
    r#smullt,
    r#stp,
    r#str,
    r#strb,
    r#strh,
    r#sttr,
    r#sttrb,
    r#sttrh,
    r#stur,
    r#sturb,
    r#sturh,
    r#stxp,
    r#stxr,
    r#stxrb,
    r#stxrh,
    r#stz2g,
    r#stzg,
    r#stzgm,
    r#sub,
    r#subg,
    r#subhn,
    r#subhn2,
    r#subhnb,
    r#subhnt,
    r#subp,
    r#subps,
    r#subr,
    r#subs,
    r#sxtb,
    r#sxth,
    r#sxtw,
    r#tbl,
    r#tbnz,
    r#tbx,
    r#tbz,
    r#trn1,
    r#trn2,
    r#tstart,
    r#ttest,
    r#uaddl,
    r#uaddl2,
    r#uaddlb,
    r#uaddlp,
    r#uaddlt,
    r#uaddlv,
    r#uaddv,
    r#uaddw,
    r#uaddw2,
    r#uaddwb,
    r#uaddwt,
    r#ubfm,
    r#uclamp,
    r#ucvtf,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ADD_SME_Zdnx2_SME_Zdnx2_SME_Zm {
    #[bits(1)]
    pub _op_0: u32,
    #[bits(4)]
    pub sme_zdn2: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(4)]
    pub sme_zm: u32,
    #[bits(12)]
    pub _op_20: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ADD_SME_Zdnx4_SME_Zdnx4_SME_Zm {
    #[bits(2)]
    pub _op_0: u32,
    #[bits(3)]
    pub sme_zdn4: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(4)]
    pub sme_zm: u32,
    #[bits(12)]
    pub _op_20: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ADD_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm {
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
pub struct ADD_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_S_SME_Zm_S_S {
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
pub struct ADD_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2 {
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
pub struct ADD_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4 {
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
pub struct ADD_SME_ZA_array_off3_0_SME_Znx2 {
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
pub struct ADD_SME_ZA_array_off3_0_SME_Znx4 {
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
pub struct ADD_Rd_SP_Rn_SP_AIMM {
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
pub struct ADD_Rd_Rn_Rm_SFT {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ADD_Rd_SP_Rn_SP_Rm_EXT {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
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
pub struct ADD_SVE_Zd_SVE_Zn_SVE_Zm_16 {
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
pub struct ADD_SVE_Zd_SVE_Zd_SVE_AIMM {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(9)]
    pub sve_imm9: u32,
    #[bits(18)]
    pub _op_14: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ADD_Vd_Vn_Vm {
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
pub struct ADD_Sd_Sn_Sm {
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
pub struct ADDG_Rd_SP_Rn_SP_UIMM10_UIMM4_ADDG {
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
pub struct ADDHA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn {
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
    #[bits(16)]
    pub _op_16: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ADDHA_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn {
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
    #[bits(16)]
    pub _op_16: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ADDHN_Vd_Vn_Vm {
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
pub struct ADDHN2_Vd_Vn_Vm {
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
pub struct ADDHNB_SVE_Zd_SVE_Zn_SVE_Zm_16 {
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
pub struct ADDHNT_SVE_Zd_SVE_Zn_SVE_Zm_16 {
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
pub struct ADDP_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zn {
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
pub struct ADDP_Sd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ADDP_Vd_Vn_Vm {
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
pub struct ADDPL_Rd_SP_SVE_Rn_SP_SVE_SIMM6 {
    #[bits(5)]
    pub rd: u32,
    #[bits(6)]
    pub sve_imms: u32,
    #[bits(5)]
    pub _op_11: u32,
    #[bits(5)]
    pub sve_rn: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ADDQV_Vd_SVE_Pg3_SVE_Zn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(19)]
    pub _op_13: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ADDS_Rd_Rn_SP_AIMM {
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
pub struct ADDS_Rd_Rn_Rm_SFT {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ADDS_Rd_Rn_SP_Rm_EXT {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ADDSPL_Rd_SP_SVE_Rn_SP_SVE_SIMM6 {
    #[bits(5)]
    pub rd: u32,
    #[bits(6)]
    pub sve_imms: u32,
    #[bits(5)]
    pub _op_11: u32,
    #[bits(5)]
    pub sve_rn: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ADDSVL_Rd_SP_SVE_Rn_SP_SVE_SIMM6 {
    #[bits(5)]
    pub rd: u32,
    #[bits(6)]
    pub sve_imms: u32,
    #[bits(5)]
    pub _op_11: u32,
    #[bits(5)]
    pub sve_rn: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ADDV_Fd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ADDVA_SME_ZAda_2b_SVE_Pg3_SME_Pm_SVE_Zn {
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
    #[bits(16)]
    pub _op_16: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ADDVA_SME_ZAda_3b_SVE_Pg3_SME_Pm_SVE_Zn {
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
    #[bits(16)]
    pub _op_16: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ADDVL_Rd_SP_SVE_Rn_SP_SVE_SIMM6 {
    #[bits(5)]
    pub rd: u32,
    #[bits(6)]
    pub sve_imms: u32,
    #[bits(5)]
    pub _op_11: u32,
    #[bits(5)]
    pub sve_rn: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ADR_Rd_ADDR_PCREL21 {
    #[bits(5)]
    pub rd: u32,
    #[bits(19)]
    pub immhi: u32,
    #[bits(5)]
    pub _op_24: u32,
    #[bits(2)]
    pub immlo: u32,
    #[bits(1)]
    pub _op_31: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ADR_SVE_Zd_SVE_ADDR_ZZ_SXTW {
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
pub struct ADR_SVE_Zd_SVE_ADDR_ZZ_UXTW {
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
pub struct ADR_SVE_Zd_SVE_ADDR_ZZ_LSL {
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
pub struct ADRP_Rd_ADDR_ADRP {
    #[bits(5)]
    pub rd: u32,
    #[bits(19)]
    pub immhi: u32,
    #[bits(5)]
    pub _op_24: u32,
    #[bits(2)]
    pub immlo: u32,
    #[bits(1)]
    pub _op_31: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct AND_Rd_SP_Rn_LIMM {
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
pub struct AND_Rd_Rn_Rm_SFT {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct AND_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
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
pub struct AND_SVE_Zd_SVE_Zn_SVE_Zm_16 {
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
pub struct AND_SVE_Zd_SVE_Zd_SVE_LIMM {
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
pub struct AND_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm {
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
pub struct AND_Vd_Vn_Vm {
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
pub struct ANDQV_Vd_SVE_Pg3_SVE_Zn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(3)]
    pub sve_pg3: u32,
    #[bits(19)]
    pub _op_13: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ANDS_Rd_Rn_LIMM {
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
pub struct ANDS_Rd_Rn_Rm_SFT {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ANDS_SVE_Pd_SVE_Pg4_10_SVE_Pn_SVE_Pm {
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
pub struct ANDV_SVE_Vd_SVE_Pg3_SVE_Zn {
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
pub struct ASR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHRIMM_PRED {
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
pub struct ASR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
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
pub struct ASR_SVE_Zd_S_B_SVE_Pg3_P_M_SVE_Zd_S_B_SVE_Zm_5_S_D {
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
pub struct ASR_SVE_Zd_SVE_Zn_SVE_Zm_16 {
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
pub struct ASR_SVE_Zd_SVE_Zn_SVE_SHRIMM_UNPRED {
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
pub struct ASRD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_SHRIMM_PRED {
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
pub struct ASRR_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
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
pub struct ASRV_Rd_Rn_Rm {
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
pub struct B_ADDR_PCREL26 {
    #[bits(26)]
    pub imm26: u32,
    #[bits(6)]
    pub _op_26: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BFADD_SVE_Zd_SVE_Zn_SVE_Zm_16 {
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
pub struct BFADD_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
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
pub struct BFCLAMP_SVE_Zd_SVE_Zn_SVE_Zm_16 {
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
pub struct BFCVT_SVE_Zd_SME_Znx2 {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(1)]
    pub _op_5: u32,
    #[bits(4)]
    pub sme_zn2: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BFCVT_SVE_Zd_SVE_Pg3_SVE_Zn {
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
pub struct BFCVT_Fd_Fn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BFCVTN_SVE_Zd_SME_Znx2 {
    #[bits(5)]
    pub sve_zd: u32,
    #[bits(1)]
    pub _op_5: u32,
    #[bits(4)]
    pub sme_zn2: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BFCVTN_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BFCVTN2_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BFCVTNT_SVE_Zd_SVE_Pg3_SVE_Zn {
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
pub struct BFDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zm_INDEX2 {
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
pub struct BFDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zm_INDEX2 {
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
pub struct BFDOT_SME_ZA_array_off3_0_SVE_ZnxN_SME_Zm {
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
pub struct BFDOT_SME_ZA_array_off3_0_S_S_SVE_ZnxN_S_H_SME_Zm_S_H {
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
pub struct BFDOT_SME_ZA_array_off3_0_SME_Znx2_SME_Zmx2 {
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
pub struct BFDOT_SME_ZA_array_off3_0_SME_Znx4_SME_Zmx4 {
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
pub struct BFDOT_SVE_Zd_SVE_Zn_SVE_Zm_16 {
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
pub struct BFDOT_SVE_Zd_SVE_Zn_SVE_Zm3_INDEX {
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
pub struct BFDOT_Vd_Vn_Vm {
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
pub struct BFDOT_Vd_Vn_Em {
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
pub struct BFM_Rd_Rn_IMMR_IMMS {
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
pub struct BFMAX_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
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
pub struct BFMAXNM_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
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
pub struct BFMIN_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
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
pub struct BFMINNM_SVE_Zd_SVE_Pg3_SVE_Zd_SVE_Zm_5 {
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
pub struct BFMLA_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX {
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
pub struct BFMLA_SVE_Zd_SVE_Pg3_SVE_Zn_SVE_Zm_16 {
    #[bits(5)]
    pub sve_zd: u32,
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
pub struct BFMLAL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm_INDEX3_10 {
    #[bits(3)]
    pub imm3_0: u32,
    #[bits(2)]
    pub _op_3: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(2)]
    pub imm2_10: u32,
    #[bits(1)]
    pub _op_12: u32,
    #[bits(2)]
    pub sme_rv: u32,
    #[bits(1)]
    pub imm1_15: u32,
    #[bits(4)]
    pub sme_zm: u32,
    #[bits(12)]
    pub _op_20: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BFMLAL_SME_ZA_array_off2x2_SME_Znx2_SME_Zm_INDEX3_2 {
    #[bits(2)]
    pub imm2_0: u32,
    #[bits(1)]
    pub imm1_2: u32,
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
pub struct BFMLAL_SME_ZA_array_off2x2_SME_Znx4_SME_Zm_INDEX3_2 {
    #[bits(2)]
    pub imm2_0: u32,
    #[bits(1)]
    pub imm1_2: u32,
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
pub struct BFMLAL_SME_ZA_array_off2x2_SVE_ZnxN_SME_Zm {
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
pub struct BFMLAL_SME_ZA_array_off2x2_S_S_SVE_ZnxN_S_H_SME_Zm_S_H {
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
pub struct BFMLAL_SME_ZA_array_off2x2_SME_Znx2_SME_Zmx2 {
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
pub struct BFMLAL_SME_ZA_array_off2x2_SME_Znx4_SME_Zmx4 {
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
pub struct BFMLAL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm {
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
pub struct BFMLALB_SVE_Zd_SVE_Zn_SVE_Zm_16 {
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
pub struct BFMLALB_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX {
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
pub struct BFMLALB_Vd_Vn_Vm {
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
pub struct BFMLALB_Vd_Vn_Em16 {
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
pub struct BFMLALT_SVE_Zd_SVE_Zn_SVE_Zm_16 {
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
pub struct BFMLALT_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX {
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
pub struct BFMLALT_Vd_Vn_Vm {
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
pub struct BFMLALT_Vd_Vn_Em16 {
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
pub struct BFMLS_SVE_Zd_SVE_Zn_SVE_Zm3_11_INDEX {
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
pub struct BFMLS_SVE_Zd_SVE_Pg3_SVE_Zn_SVE_Zm_16 {
    #[bits(5)]
    pub sve_zd: u32,
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
pub struct BFMLSL_SME_ZA_array_off3x2_SVE_Zn_SME_Zm_INDEX3_10 {
    #[bits(3)]
    pub imm3_0: u32,
    #[bits(2)]
    pub _op_3: u32,
    #[bits(5)]
    pub sve_zn: u32,
    #[bits(2)]
    pub imm2_10: u32,
    #[bits(1)]
    pub _op_12: u32,
    #[bits(2)]
    pub sme_rv: u32,
    #[bits(1)]
    pub imm1_15: u32,
    #[bits(4)]
    pub sme_zm: u32,
    #[bits(12)]
    pub _op_20: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BFMLSL_SME_ZA_array_off2x2_SME_Znx2_SME_Zm_INDEX3_2 {
    #[bits(2)]
    pub imm2_0: u32,
    #[bits(1)]
    pub imm1_2: u32,
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
pub struct BFMLSL_SME_ZA_array_off2x2_SME_Znx4_SME_Zm_INDEX3_2 {
    #[bits(2)]
    pub imm2_0: u32,
    #[bits(1)]
    pub imm1_2: u32,
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
pub struct BFMLSL_SME_ZA_array_off2x2_SVE_ZnxN_SME_Zm {
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
