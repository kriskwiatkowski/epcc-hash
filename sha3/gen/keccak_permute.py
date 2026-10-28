#!/usr/bin/env python3
# SPDX-FileCopyrightText: 2026 Krzysztof Kwiatkowski
# SPDX-License-Identifier: CC0-1.0

import argparse

from nervapy import *
from nervapy.arm import *
from nervapy.arm.abi import arm_gnueabihf
from nervapy.arm.formats import AssemblyFormat
from nervapy.arm.microarchitecture import Microarchitecture
from nervapy.arm.pseudo import Label
from nervapy.arm.registers import r0, r1, r3, r4, r5, r6, r7, r11, r12, lr, sp
from pathlib import Path

_ROUND_CONSTANTS_WITH_TERMINATOR = [
    (0x00000001, 0x00000000),
    (0x00000000, 0x00000089),
    (0x00000000, 0x8000008B),
    (0x00000000, 0x80008080),
    (0x00000001, 0x0000008B),
    (0x00000001, 0x00008000),
    (0x00000001, 0x80008088),
    (0x00000001, 0x80000082),
    (0x00000000, 0x0000000B),
    (0x00000000, 0x0000000A),
    (0x00000001, 0x00008082),
    (0x00000000, 0x00008003),
    (0x00000001, 0x0000808B),
    (0x00000001, 0x8000000B),
    (0x00000001, 0x8000008A),
    (0x00000001, 0x80000081),
    (0x00000000, 0x80000081),
    (0x00000000, 0x80000008),
    (0x00000000, 0x00000083),
    (0x00000000, 0x80008003),
    (0x00000001, 0x80008088),
    (0x00000000, 0x80000088),
    (0x00000001, 0x00008000),
    (0x00000000, 0x80008082),
]

_ROUND_CONSTANTS_TERMINATOR = 0x000000FF
_ROUND_CONSTANTS_LABEL = "KeccakF1600_StatePermute_RoundConstantsWithTerminator"
_STATE_PERMUTE = "epcc_keccak_f1600_permute"


def _ror_delta(rot1, rot2):
    # The handwritten macros express rotations relative to each operand's alignment.
    return (rot1 - rot2) % 32


def eorror(dst, src1, src2, rot1, rot2):
    # XOR two possibly misaligned operands, normalizing src2 with a rotate.
    rotation = _ror_delta(rot1, rot2)
    if rotation == 0:
        EOR(dst, src1, src2)
    else:
        EOR(dst, src1, src2.ROR(rotation))


def bicror(dst, src1, src2, rot1, rot2):
    # Same alignment fix-up as eorror, but for the BIC path used by Chi.
    rotation = _ror_delta(rot1, rot2)
    if rotation == 0:
        BIC(dst, src1, src2)
    else:
        BIC(dst, src1, src2.ROR(rotation))


def xorrol(dst, src1, src2, rot):
    # The macro is "xor with src2 rotated left by 1", which maps to ROR(rot-1).
    rotation = (rot - 1) % 32
    if rotation == 0:
        EOR(dst, src1, src2)
    else:
        EOR(dst, src1, src2.ROR(rotation))


def xor5(dst, src1, src2, src3, src4, src5, rot1, rot2, rot3, rot4, rot5):
    # Compute the column parity from five lanes with their logical alignments preserved.
    LDR_W(dst, [r0, src1])
    LDR_W(r1, [r0, src2])
    LDR_W(r5, [r0, src3])
    LDR(r11, [r0, src4])
    LDR(r12, [r0, src5])
    eorror(dst, dst, r1, rot1, rot2)
    eorror(dst, dst, r5, rot1, rot3)
    eorror(dst, dst, r11, rot1, rot4)
    eorror(dst, dst, r12, rot1, rot5)


def xor5str(
    dst,
    src1,
    src2,
    src3,
    src4,
    src5,
    rot1,
    rot2,
    rot3,
    rot4,
    rot5,
    strreg,
    stradr,
    strofs,
):
    # Interleave the previous round's store with the next round's load bundle.
    LDR_W(dst, [r0, src1])
    LDR_W(r1, [r0, src2])
    LDR_W(r5, [r0, src3])
    LDR(r11, [r0, src4])
    LDR(r12, [r0, src5])
    STR_W(strreg, [stradr, strofs])
    eorror(dst, dst, r1, rot1, rot2)
    eorror(dst, dst, r5, rot1, rot3)
    eorror(dst, dst, r11, rot1, rot4)
    eorror(dst, dst, r12, rot1, rot5)


def xandnotlazystr(resofs, src1, src2, src3, rot1, rot2, rot3):
    # Lazy variants defer the explicit Rho rotate to the next round.
    bicror(r1, src3, src2, rot3, rot2)
    eorror(r1, r1, src1, rot3, rot1)
    STR_W(r1, [r0, resofs])


def xandnotlazy(src1, src2, src3, rot1, rot2, rot3):
    bicror(r1, src3, src2, rot3, rot2)
    eorror(r1, r1, src1, rot3, rot1)


def xandnotstr(resofs, src1, src2, src3, rot1, rot2, rot3):
    bicror(r1, src3, src2, rot3, rot2)
    eorror(r1, r1, src1, rot3, rot1)
    if rot3 > 0:
        # Non-lazy rounds materialize the lane's final rotation before storing.
        ROR(r1, r1, 32 - rot3)
    STR_W(r1, [r0, resofs])


def xandnot(src1, src2, src3, rot1, rot2, rot3):
    bicror(r1, src3, src2, rot3, rot2)
    eorror(r1, r1, src1, rot3, rot1)
    if rot3 > 0:
        ROR(r1, r1, 32 - rot3)


def xandnotiota(out, rot3, rot2, rcofs, last, mrc_ofs):
    # Iota shares the Chi temporary in r5 and advances the round-constant cursor
    # only on the final call in a 4-round block.
    bicror(r5, r5, r4, rot3, rot2)
    LDR(r1, [sp, mrc_ofs])
    LDR(r4, [r1, rcofs])
    if last == 1:
        LDR(r7, [r1.wb(), 32])
        STR(r1, [sp, mrc_ofs])
        CMP(r7, 0xFF)
    if rot3 > 0:
        EOR(r3, r3, r5.ROR(32 - rot3))
    else:
        EOR(r3, r3, r5)
    EOR(out, r4, r3)


def addparity(par1, dly1, par2, dly2, par3, dly3, par4, dly4, par5, dly5):
    # r3-r7 hold the active plane. "dly*" accounts for any lazy Rho rotation still pending.
    if dly1 > 0:
        EOR(r3, par1, r3.ROR(32 - dly1))
    else:
        EOR(r3, par1, r3)
    if dly2 > 0:
        EOR(r4, par2, r4.ROR(32 - dly2))
    else:
        EOR(r4, par2, r4)
    if dly3 > 0:
        EOR(r5, par3, r5.ROR(32 - dly3))
    else:
        EOR(r5, par3, r5)
    if dly4 > 0:
        EOR(r6, par4, r6.ROR(32 - dly4))
    else:
        EOR(r6, par4, r6)
    if dly5 > 0:
        EOR(r7, par5, r7.ROR(32 - dly5))
    else:
        EOR(r7, par5, r7)


def KeccakThetaRhoPiChiIota(
    src1,
    par1,
    dly1,
    src2,
    par2,
    rot2,
    dly2,
    src3,
    par3,
    rot3,
    dly3,
    src4,
    par4,
    rot4,
    dly4,
    src5,
    par5,
    rot5,
    dly5,
    ofs,
    last,
    lazy,
    strofs,
    reg,
    mrc_ofs,
):
    # This mirrors the original macro structure: load a half-plane, inject parity,
    # apply Chi (lazy or eager), then finish with Iota.
    LDR_W(r3, [r0, src1])
    LDR(r4, [r0, src2])
    LDR(r5, [r0, src3])
    LDR(r6, [r0, src4])
    LDR(r7, [r0, src5])
    STR_W(r1, [r0, strofs])
    addparity(par1, dly1, par2, dly2, par3, dly3, par4, dly4, par5, dly5)
    if lazy == 1:
        xandnotlazystr(src2, r4, r5, r6, rot2, rot3, rot4)
        xandnotlazystr(src3, r5, r6, r7, rot3, rot4, rot5)
        xandnotlazystr(src4, r6, r7, r3, rot4, rot5, 0)
        xandnotlazystr(src5, r7, r3, r4, rot5, 0, rot2)
    else:
        xandnotstr(src2, r4, r5, r6, rot2, rot3, rot4)
        xandnotstr(src3, r5, r6, r7, rot3, rot4, rot5)
        xandnotstr(src4, r6, r7, r3, rot4, rot5, 0)
        xandnotstr(src5, r7, r3, r4, rot5, 0, rot2)
    xandnotiota(reg, rot3, rot2, ofs, last, mrc_ofs)


def KeccakThetaRhoPiChi(
    src1,
    dst1,
    par1,
    rot1,
    dly1,
    src2,
    dst2,
    par2,
    rot2,
    dly2,
    src3,
    dst3,
    par3,
    rot3,
    dly3,
    src4,
    dst4,
    par4,
    rot4,
    dly4,
    src5,
    dst5,
    par5,
    rot5,
    dly5,
    lazy,
    strofs,
):
    # Same as the Iota form, but leaves the final lane in r1 for the caller to consume.
    LDR_W(r3, [r0, src1])
    LDR_W(r4, [r0, src2])
    LDR_W(r5, [r0, src3])
    LDR_W(r6, [r0, src4])
    LDR_W(r7, [r0, src5])
    STR_W(r1, [r0, strofs])
    addparity(par1, dly1, par2, dly2, par3, dly3, par4, dly4, par5, dly5)
    if lazy == 1:
        xandnotlazystr(dst1, r3, r4, r5, rot1, rot2, rot3)
        xandnotlazystr(dst2, r4, r5, r6, rot2, rot3, rot4)
        xandnotlazystr(dst3, r5, r6, r7, rot3, rot4, rot5)
        xandnotlazystr(dst4, r6, r7, r3, rot4, rot5, rot1)
        xandnotlazy(r7, r3, r4, rot5, rot1, rot2)
    else:
        xandnotstr(dst1, r3, r4, r5, rot1, rot2, rot3)
        xandnotstr(dst2, r4, r5, r6, rot2, rot3, rot4)
        xandnotstr(dst3, r5, r6, r7, rot3, rot4, rot5)
        xandnotstr(dst4, r6, r7, r3, rot4, rot5, rot1)
        xandnot(r7, r3, r4, rot5, rot1, rot2)


def _helper_function(name):
    return Function(
        name,
        (),
        target=Microarchitecture.CortexM33,
        abi=arm_gnueabihf,
        assembly_format=AssemblyFormat.GAS,
        is_thumb=True,
        report_generation=False,
    )


def make_state_permute(func_name=_STATE_PERMUTE, asm_format=AssemblyFormat.GAS):
    from keccak_rounds import KeccakRound0, KeccakRound1, KeccakRound2, KeccakRound3, mRC, mSize

    round_loop = Label(func_name + "_RoundLoop")

    f = Function(
        func_name,
        (),
        target=Microarchitecture.CortexM33,
        abi=arm_gnueabihf,
        assembly_format=asm_format,
        is_thumb=True,
        report_generation=False,
    )
    f.high_register_strategy = HighRegisterStrategy.STMDB
    f.alignment = 3

    with f:
        # Keep the round-constant cursor in memory so the final Iota of each 4-round
        # block can update it with pre-indexed addressing.
        ADR(r1, _ROUND_CONSTANTS_LABEL)
        PUSH((lr,))
        SUB(sp, sp, mSize)
        STR(r1, [sp, mRC])
        LABEL(round_loop)
        # The original implementation unrolls four rounds at a time and loops on the
        # comparison emitted by the last Iota step.
        KeccakRound0()
        KeccakRound1()
        KeccakRound2()
        KeccakRound3()
        BNE(round_loop)
        ADD(sp, sp, mSize)
        POP((lr,))
        RETURN()

    return f


def gen_state_permute(func_name=_STATE_PERMUTE, asm_format=AssemblyFormat.GAS):
    return make_state_permute(func_name, asm_format).assembly


def gen_round_constants_with_terminator(asm_format=AssemblyFormat.GAS):
    if asm_format == AssemblyFormat.GAS:
        # Emit both the public symbol and a local "L..." alias so ADR can target it.
        lines = [
            "\t.syntax unified",
            "\t.thumb",
            "\t.text",
            "",
            "\t.align 3",
            _ROUND_CONSTANTS_LABEL + ":",
            "L" + _ROUND_CONSTANTS_LABEL + ":",
        ]
        for low, high in _ROUND_CONSTANTS_WITH_TERMINATOR:
            lines.append("\t.long 0x{0:08X}, 0x{1:08X}".format(low, high))
        lines.append("\t.long 0x{0:08X}".format(_ROUND_CONSTANTS_TERMINATOR))
        return "\n".join(lines) + "\n"

    if asm_format == AssemblyFormat.ARMCC:
        lines = [
            "        THUMB",
            "        AREA    ||.text||, CODE, READONLY",
            "        ALIGN    8",
            _ROUND_CONSTANTS_LABEL,
            "L" + _ROUND_CONSTANTS_LABEL,
        ]
        for low, high in _ROUND_CONSTANTS_WITH_TERMINATOR:
            lines.append("        DCD     0x{0:08X}, 0x{1:08X}".format(low, high))
        lines.append("        DCD     0x{0:08X}".format(_ROUND_CONSTANTS_TERMINATOR))
        return "\n".join(lines) + "\n"

    raise ValueError("Unsupported assembly format: {0}".format(asm_format))


def run_generator(asm_format=AssemblyFormat.GAS):
    return gen_round_constants_with_terminator(asm_format) + "\n" + gen_state_permute(
        asm_format=asm_format
    )


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Generate KeccakF1600 permutation assembly")
    parser.add_argument("--output-dir", type=Path, default=Path("."),
                    help="Directory to write output files")
    parser.add_argument("--lang", choices=["rust", "generic"], default="generic",
                    help="Output language (rust or generic)")
    args = parser.parse_args()

    #gen = Generator(
    #    target=Microarchitecture.CortexM33,
    #    output_dir=args.output_dir,
    #    lang=args.lang,
    #    asm_format=AssemblyFormat.GAS)
    asm =run_generator(AssemblyFormat.GAS)
    output_file = args.output_dir / "keccak_permute.s"
    with open(output_file, "w") as f:
        f.write(asm)
    print("Written to", output_file)
