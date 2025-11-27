//! Arithmetic instruction translation

use super::{context::TranslationContext, TranslateResult};
use crate::ir::{Type, Value};
use zkir_spec::{Instruction, Register};

/// Translate add instruction
pub fn translate_add(
    ctx: &mut TranslationContext,
    result: &str,
    ty: &Type,
    lhs: &Value,
    rhs: &Value,
) -> TranslateResult<()> {
    let bits = ty.bit_width();

    if bits <= 32 {
        translate_add_32(ctx, result, lhs, rhs)
    } else if bits == 64 {
        translate_add_64(ctx, result, lhs, rhs)
    } else {
        Err(super::TranslateError::UnsupportedWidth(bits))
    }
}

fn translate_add_32(
    ctx: &mut TranslationContext,
    result: &str,
    lhs: &Value,
    rhs: &Value,
) -> TranslateResult<()> {
    let rs1 = ctx.load_value(lhs)?;
    let rs2 = ctx.load_value(rhs)?;
    let rd = ctx.alloc_temp()?;

    ctx.emit(Instruction::Add { rd, rs1, rs2 });
    ctx.bind(result, super::context::Location::Reg(rd));

    Ok(())
}

fn translate_add_64(
    ctx: &mut TranslationContext,
    result: &str,
    lhs: &Value,
    rhs: &Value,
) -> TranslateResult<()> {
    let (a_lo, a_hi) = ctx.load_value_pair(lhs)?;
    let (b_lo, b_hi) = ctx.load_value_pair(rhs)?;

    let sum_lo = ctx.alloc_temp()?;
    let carry = ctx.alloc_temp()?;
    let sum_hi = ctx.alloc_temp()?;

    // sum_lo = a_lo + b_lo
    ctx.emit(Instruction::Add {
        rd: sum_lo,
        rs1: a_lo,
        rs2: b_lo,
    });

    // carry = (sum_lo < a_lo) ? 1 : 0
    ctx.emit(Instruction::Sltu {
        rd: carry,
        rs1: sum_lo,
        rs2: a_lo,
    });

    // sum_hi = a_hi + b_hi + carry
    let tmp = ctx.alloc_temp()?;
    ctx.emit(Instruction::Add {
        rd: tmp,
        rs1: a_hi,
        rs2: b_hi,
    });
    ctx.emit(Instruction::Add {
        rd: sum_hi,
        rs1: tmp,
        rs2: carry,
    });

    ctx.bind(
        result,
        super::context::Location::RegPair {
            lo: sum_lo,
            hi: sum_hi,
        },
    );

    Ok(())
}

/// Translate sub instruction
pub fn translate_sub(
    ctx: &mut TranslationContext,
    result: &str,
    ty: &Type,
    lhs: &Value,
    rhs: &Value,
) -> TranslateResult<()> {
    let bits = ty.bit_width();

    if bits <= 32 {
        let rs1 = ctx.load_value(lhs)?;
        let rs2 = ctx.load_value(rhs)?;
        let rd = ctx.alloc_temp()?;

        ctx.emit(Instruction::Sub { rd, rs1, rs2 });
        ctx.bind(result, super::context::Location::Reg(rd));
        Ok(())
    } else {
        Err(super::TranslateError::UnsupportedWidth(bits))
    }
}

/// Translate mul instruction
pub fn translate_mul(
    ctx: &mut TranslationContext,
    result: &str,
    ty: &Type,
    lhs: &Value,
    rhs: &Value,
) -> TranslateResult<()> {
    let bits = ty.bit_width();

    if bits <= 32 {
        let rs1 = ctx.load_value(lhs)?;
        let rs2 = ctx.load_value(rhs)?;
        let rd = ctx.alloc_temp()?;

        ctx.emit(Instruction::Mul { rd, rs1, rs2 });
        ctx.bind(result, super::context::Location::Reg(rd));
        Ok(())
    } else {
        Err(super::TranslateError::UnsupportedWidth(bits))
    }
}

/// Translate unsigned div
pub fn translate_udiv(
    ctx: &mut TranslationContext,
    result: &str,
    ty: &Type,
    lhs: &Value,
    rhs: &Value,
) -> TranslateResult<()> {
    let rs1 = ctx.load_value(lhs)?;
    let rs2 = ctx.load_value(rhs)?;
    let rd = ctx.alloc_temp()?;

    ctx.emit(Instruction::Divu { rd, rs1, rs2 });
    ctx.bind(result, super::context::Location::Reg(rd));
    Ok(())
}

/// Translate signed div
pub fn translate_sdiv(
    ctx: &mut TranslationContext,
    result: &str,
    ty: &Type,
    lhs: &Value,
    rhs: &Value,
) -> TranslateResult<()> {
    let rs1 = ctx.load_value(lhs)?;
    let rs2 = ctx.load_value(rhs)?;
    let rd = ctx.alloc_temp()?;

    ctx.emit(Instruction::Div { rd, rs1, rs2 });
    ctx.bind(result, super::context::Location::Reg(rd));
    Ok(())
}

/// Translate unsigned rem
pub fn translate_urem(
    ctx: &mut TranslationContext,
    result: &str,
    ty: &Type,
    lhs: &Value,
    rhs: &Value,
) -> TranslateResult<()> {
    let rs1 = ctx.load_value(lhs)?;
    let rs2 = ctx.load_value(rhs)?;
    let rd = ctx.alloc_temp()?;

    ctx.emit(Instruction::Remu { rd, rs1, rs2 });
    ctx.bind(result, super::context::Location::Reg(rd));
    Ok(())
}

/// Translate signed rem
pub fn translate_srem(
    ctx: &mut TranslationContext,
    result: &str,
    ty: &Type,
    lhs: &Value,
    rhs: &Value,
) -> TranslateResult<()> {
    let rs1 = ctx.load_value(lhs)?;
    let rs2 = ctx.load_value(rhs)?;
    let rd = ctx.alloc_temp()?;

    ctx.emit(Instruction::Rem { rd, rs1, rs2 });
    ctx.bind(result, super::context::Location::Reg(rd));
    Ok(())
}

// Bitwise operations

pub fn translate_and(
    ctx: &mut TranslationContext,
    result: &str,
    ty: &Type,
    lhs: &Value,
    rhs: &Value,
) -> TranslateResult<()> {
    let rs1 = ctx.load_value(lhs)?;
    let rs2 = ctx.load_value(rhs)?;
    let rd = ctx.alloc_temp()?;

    ctx.emit(Instruction::And { rd, rs1, rs2 });
    ctx.bind(result, super::context::Location::Reg(rd));
    Ok(())
}

pub fn translate_or(
    ctx: &mut TranslationContext,
    result: &str,
    ty: &Type,
    lhs: &Value,
    rhs: &Value,
) -> TranslateResult<()> {
    let rs1 = ctx.load_value(lhs)?;
    let rs2 = ctx.load_value(rhs)?;
    let rd = ctx.alloc_temp()?;

    ctx.emit(Instruction::Or { rd, rs1, rs2 });
    ctx.bind(result, super::context::Location::Reg(rd));
    Ok(())
}

pub fn translate_xor(
    ctx: &mut TranslationContext,
    result: &str,
    ty: &Type,
    lhs: &Value,
    rhs: &Value,
) -> TranslateResult<()> {
    let rs1 = ctx.load_value(lhs)?;
    let rs2 = ctx.load_value(rhs)?;
    let rd = ctx.alloc_temp()?;

    ctx.emit(Instruction::Xor { rd, rs1, rs2 });
    ctx.bind(result, super::context::Location::Reg(rd));
    Ok(())
}

pub fn translate_shl(
    ctx: &mut TranslationContext,
    result: &str,
    ty: &Type,
    lhs: &Value,
    rhs: &Value,
) -> TranslateResult<()> {
    let rs1 = ctx.load_value(lhs)?;
    let rs2 = ctx.load_value(rhs)?;
    let rd = ctx.alloc_temp()?;

    ctx.emit(Instruction::Sll { rd, rs1, rs2 });
    ctx.bind(result, super::context::Location::Reg(rd));
    Ok(())
}

pub fn translate_lshr(
    ctx: &mut TranslationContext,
    result: &str,
    ty: &Type,
    lhs: &Value,
    rhs: &Value,
) -> TranslateResult<()> {
    let rs1 = ctx.load_value(lhs)?;
    let rs2 = ctx.load_value(rhs)?;
    let rd = ctx.alloc_temp()?;

    ctx.emit(Instruction::Srl { rd, rs1, rs2 });
    ctx.bind(result, super::context::Location::Reg(rd));
    Ok(())
}

pub fn translate_ashr(
    ctx: &mut TranslationContext,
    result: &str,
    ty: &Type,
    lhs: &Value,
    rhs: &Value,
) -> TranslateResult<()> {
    let rs1 = ctx.load_value(lhs)?;
    let rs2 = ctx.load_value(rhs)?;
    let rd = ctx.alloc_temp()?;

    ctx.emit(Instruction::Sra { rd, rs1, rs2 });
    ctx.bind(result, super::context::Location::Reg(rd));
    Ok(())
}
