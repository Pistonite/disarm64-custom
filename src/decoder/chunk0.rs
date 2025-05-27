#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Mnemonic {
    r#abs,
    r#adc,
    r#adcs,
    r#add,
    r#addg,
    r#addhn,
    r#addhn2,
    r#addp,
    r#adds,
    r#addv,
    r#adr,
    r#adrp,
    r#and,
    r#ands,
    r#asrv,
    r#autda,
    r#autdb,
    r#autdza,
    r#autdzb,
    r#autia,
    r#autib,
    r#autiza,
    r#autizb,
    r#b,
    r#b_,
    r#bc_,
    r#bfm,
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
    r#bsl,
    r#cbnz,
    r#cbz,
    r#ccmn,
    r#ccmp,
    r#cls,
    r#clz,
    r#cmeq,
    r#cmge,
    r#cmgt,
    r#cmhi,
    r#cmhs,
    r#cmle,
    r#cmlt,
    r#cmtst,
    r#cnt,
    r#crc32b,
    r#crc32cb,
    r#crc32ch,
    r#crc32cw,
    r#crc32cx,
    r#crc32h,
    r#crc32w,
    r#crc32x,
    r#csel,
    r#csinc,
    r#csinv,
    r#csneg,
    r#drps,
    r#dup,
    r#eon,
    r#eor,
    r#eret,
    r#eretaa,
    r#eretab,
    r#ext,
    r#fabd,
    r#fabs,
    r#facge,
    r#facgt,
    r#fadd,
    r#faddp,
    r#fcadd,
    r#fccmp,
    r#fccmpe,
    r#fcmeq,
    r#fcmge,
    r#fcmgt,
    r#fcmla,
    r#fcmle,
    r#fcmlt,
    r#fcmp,
    r#fcmpe,
    r#fcsel,
    r#fcvt,
    r#fcvtas,
    r#fcvtau,
    r#fcvtl,
    r#fcvtl2,
    r#fcvtms,
    r#fcvtmu,
    r#fcvtn,
    r#fcvtn2,
    r#fcvtns,
    r#fcvtnu,
    r#fcvtps,
    r#fcvtpu,
    r#fcvtxn,
    r#fcvtxn2,
    r#fcvtzs,
    r#fcvtzu,
    r#fdiv,
    r#fjcvtzs,
    r#fmadd,
    r#fmax,
    r#fmaxnm,
    r#fmaxnmp,
    r#fmaxnmv,
    r#fmaxp,
    r#fmaxv,
    r#fmin,
    r#fminnm,
    r#fminnmp,
    r#fminnmv,
    r#fminp,
    r#fminv,
    r#fmla,
    r#fmlal,
    r#fmlal2,
    r#fmls,
    r#fmlsl,
    r#fmlsl2,
    r#fmov,
    r#fmsub,
    r#fmul,
    r#fmulx,
    r#fneg,
    r#fnmadd,
    r#fnmsub,
    r#fnmul,
    r#frecpe,
    r#frecps,
    r#frecpx,
    r#frint32x,
    r#frint32z,
    r#frint64x,
    r#frint64z,
    r#frinta,
    r#frinti,
    r#frintm,
    r#frintn,
    r#frintp,
    r#frintx,
    r#frintz,
    r#frsqrte,
    r#frsqrts,
    r#fsqrt,
    r#fsub,
    r#gmi,
    r#ins,
    r#irg,
    r#ld1,
    r#ld1r,
    r#ld2,
    r#ld2r,
    r#ld3,
    r#ld3r,
    r#ld4,
    r#ld4r,
    r#ldapr,
    r#ldaprb,
    r#ldaprh,
    r#ldapur,
    r#ldapurb,
    r#ldapurh,
    r#ldapursb,
    r#ldapursh,
    r#ldapursw,
    r#ldar,
    r#ldarb,
    r#ldarh,
    r#ldaxp,
    r#ldaxr,
    r#ldaxrb,
    r#ldaxrh,
    r#ldg,
    r#ldgm,
    r#ldlar,
    r#ldlarb,
    r#ldlarh,
    r#ldnp,
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
    r#ldtr,
    r#ldtrb,
    r#ldtrh,
    r#ldtrsb,
    r#ldtrsh,
    r#ldtrsw,
    r#ldur,
    r#ldurb,
    r#ldurh,
    r#ldursb,
    r#ldursh,
    r#ldursw,
    r#ldxp,
    r#ldxr,
    r#ldxrb,
    r#ldxrh,
    r#lslv,
    r#lsrv,
    r#madd,
    r#mla,
    r#mls,
    r#movi,
    r#movk,
    r#movn,
    r#movz,
    r#msub,
    r#mul,
    r#mvni,
    r#neg,
    r#not,
    r#orn,
    r#orr,
    r#pacda,
    r#pacdb,
    r#pacdza,
    r#pacdzb,
    r#pacga,
    r#pacia,
    r#pacib,
    r#paciza,
    r#pacizb,
    r#pmul,
    r#pmull,
    r#pmull2,
    r#prfm,
    r#prfum,
    r#raddhn,
    r#raddhn2,
    r#rbit,
    r#ret,
    r#retaa,
    r#retab,
    r#rev,
    r#rev16,
    r#rev32,
    r#rev64,
    r#rorv,
    r#rshrn,
    r#rshrn2,
    r#rsubhn,
    r#rsubhn2,
    r#saba,
    r#sabal,
    r#sabal2,
    r#sabd,
    r#sabdl,
    r#sabdl2,
    r#sadalp,
    r#saddl,
    r#saddl2,
    r#saddlp,
    r#saddlv,
    r#saddw,
    r#saddw2,
    r#sbc,
    r#sbcs,
    r#sbfm,
    r#scvtf,
    r#sdiv,
    r#shadd,
    r#shl,
    r#shll,
    r#shll2,
    r#shrn,
    r#shrn2,
    r#shsub,
    r#sli,
    r#smaddl,
    r#smax,
    r#smaxp,
    r#smaxv,
    r#smin,
    r#sminp,
    r#sminv,
    r#smlal,
    r#smlal2,
    r#smlsl,
    r#smlsl2,
    r#smov,
    r#smsubl,
    r#smulh,
    r#smull,
    r#smull2,
    r#sqabs,
    r#sqadd,
    r#sqdmlal,
    r#sqdmlal2,
    r#sqdmlsl,
    r#sqdmlsl2,
    r#sqdmulh,
    r#sqdmull,
    r#sqdmull2,
    r#sqneg,
    r#sqrdmlah,
    r#sqrdmlsh,
    r#sqrdmulh,
    r#sqrshl,
    r#sqrshrn,
    r#sqrshrn2,
    r#sqrshrun,
    r#sqrshrun2,
    r#sqshl,
    r#sqshlu,
    r#sqshrn,
    r#sqshrn2,
    r#sqshrun,
    r#sqshrun2,
    r#sqsub,
    r#sqxtn,
    r#sqxtn2,
    r#sqxtun,
    r#sqxtun2,
    r#srhadd,
    r#sri,
    r#srshl,
    r#srshr,
    r#srsra,
    r#sshl,
    r#sshll,
    r#sshll2,
    r#sshr,
    r#ssra,
    r#ssubl,
    r#ssubl2,
    r#ssubw,
    r#ssubw2,
    r#st1,
    r#st2,
    r#st2g,
    r#st3,
    r#st4,
    r#stg,
    r#stgm,
    r#stgp,
    r#stllr,
    r#stllrb,
    r#stllrh,
    r#stlr,
    r#stlrb,
    r#stlrh,
    r#stlur,
    r#stlurb,
    r#stlurh,
    r#stlxp,
    r#stlxr,
    r#stlxrb,
    r#stlxrh,
    r#stnp,
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
    r#subp,
    r#subps,
    r#subs,
    r#suqadd,
    r#tbl,
    r#tbnz,
    r#tbx,
    r#tbz,
    r#trn1,
    r#trn2,
    r#uaba,
    r#uabal,
    r#uabal2,
    r#uabd,
    r#uabdl,
    r#uabdl2,
    r#uadalp,
    r#uaddl,
    r#uaddl2,
    r#uaddlp,
    r#uaddlv,
    r#uaddw,
    r#uaddw2,
    r#ubfm,
    r#ucvtf,
    r#udiv,
    r#uhadd,
    r#uhsub,
    r#umaddl,
    r#umax,
    r#umaxp,
    r#umaxv,
    r#umin,
    r#uminp,
    r#uminv,
    r#umlal,
    r#umlal2,
    r#umlsl,
    r#umlsl2,
    r#umov,
    r#umsubl,
    r#umulh,
    r#umull,
    r#umull2,
    r#uqadd,
    r#uqrshl,
    r#uqrshrn,
    r#uqrshrn2,
    r#uqshl,
    r#uqshrn,
    r#uqshrn2,
    r#uqsub,
    r#uqxtn,
    r#uqxtn2,
    r#urecpe,
    r#urhadd,
    r#urshl,
    r#urshr,
    r#ursqrte,
    r#ursra,
    r#ushl,
    r#ushll,
    r#ushll2,
    r#ushr,
    r#usqadd,
    r#usra,
    r#usubl,
    r#usubl2,
    r#usubw,
    r#usubw2,
    r#uzp1,
    r#uzp2,
    r#xpacd,
    r#xpaci,
    r#xtn,
    r#xtn2,
    r#zip1,
    r#zip2,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ABS_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ABS_Sd_Sn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ADC_Rd_Rn_Rm {
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
pub struct ADCS_Rd_Rn_Rm {
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
pub struct AUTDA_Rd_Rn_SP {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct AUTDB_Rd_Rn_SP {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct AUTDZA_Rd {
    #[bits(5)]
    pub rd: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct AUTDZB_Rd {
    #[bits(5)]
    pub rd: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct AUTIA_Rd_Rn_SP {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct AUTIB_Rd_Rn_SP {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct AUTIZA_Rd {
    #[bits(5)]
    pub rd: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct AUTIZB_Rd {
    #[bits(5)]
    pub rd: u32,
    #[bits(27)]
    pub _op_5: u32,
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
pub struct B__ADDR_PCREL19 {
    #[bits(4)]
    pub cond: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(19)]
    pub imm19: u32,
    #[bits(8)]
    pub _op_24: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BC__ADDR_PCREL19 {
    #[bits(4)]
    pub cond: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(19)]
    pub imm19: u32,
    #[bits(8)]
    pub _op_24: u32,
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
pub struct BSL_Vd_Vn_Vm {
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
pub struct CRC32B_Rd_Rn_Rm {
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
pub struct CRC32CB_Rd_Rn_Rm {
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
pub struct CRC32CH_Rd_Rn_Rm {
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
pub struct CRC32CW_Rd_Rn_Rm {
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
pub struct CRC32CX_Rd_Rn_Rm {
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
pub struct CRC32H_Rd_Rn_Rm {
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
pub struct CRC32W_Rd_Rn_Rm {
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
pub struct CRC32X_Rd_Rn_Rm {
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
pub struct DRPS {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct DUP_Vd_En {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct DUP_Vd_Rn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct DUP_Sd_En {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct EON_Rd_Rn_Rm_SFT {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct EOR_Rd_SP_Rn_LIMM {
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
pub struct EOR_Rd_Rn_Rm_SFT {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct EOR_Vd_Vn_Vm {
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
pub struct ERET {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ERETAA {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ERETAB {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct EXT_Vd_Vn_Vm_IDX {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(1)]
    pub _op_10: u32,
    #[bits(4)]
    pub imm4_11: u32,
    #[bits(1)]
    pub _op_15: u32,
    #[bits(5)]
    pub rm: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FABD_Vd_Vn_Vm {
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
pub struct FABD_Sd_Sn_Sm {
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
pub struct FABD_Vd_V_2S_Vn_V_2S_Vm_V_2S {
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
pub struct FABD_Sd_S_S_Sn_S_S_Sm_S_S {
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
pub struct FABS_Fd_Fn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FABS_Fd_S_S_Fn_S_S {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FABS_Vd_Vn {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FABS_Vd_V_4H_Vn_V_4H {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FACGE_Vd_Vn_Vm {
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
pub struct FACGE_Sd_Sn_Sm {
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
pub struct FACGE_Vd_V_2S_Vn_V_2S_Vm_V_2S {
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
pub struct FACGE_Sd_S_S_Sn_S_S_Sm_S_S {
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
pub struct FACGT_Vd_Vn_Vm {
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
pub struct FACGT_Sd_Sn_Sm {
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
pub struct FACGT_Vd_V_2S_Vn_V_2S_Vm_V_2S {
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
