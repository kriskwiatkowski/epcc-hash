#!/usr/bin/env python3
# SPDX-FileCopyrightText: 2026 Krzysztof Kwiatkowski
# SPDX-License-Identifier: CC0-1.0

"""Generate the RV32 Keccak-f[1600] permutation.

The permutation runs entirely in the bit-interleaved domain, so a 64-bit
lane rotation becomes one or two 32-bit `rori` instructions (Zbb), and
the chi step's `NOT b AND c` becomes a single `andn`. Following
Zhang et al., "Optimized Software Implementation of Keccak, Kyber, and
Dilithium on RV{32,64}IM{B}{V}" (eprint 2024/1515, Sec 4.2), bit
interleaving is only worthwhile on RV32 once the B extension provides
`rori`; on plain RV32I the encode/decode overhead outweighs it, which is
why this generator targets Zbb only.

Unlike that work we do not use the in-place lane relabelling: keeping the
round body uniform lets it stay a loop rather than 24 unrolled bodies,
which matters more than the saved moves on a code-size-constrained
target. Instead the round writes into a stack ping-pong buffer; 24 rounds
is even, so the result lands back in the caller's buffer with no copy.

The script has no third-party dependencies.
"""

import argparse
from pathlib import Path

# ---------------------------------------------------------------------
# Keccak constants
# ---------------------------------------------------------------------

# Rotation offsets r[x][y], from the standard (x, y) walk.
ROT = [[0] * 5 for _ in range(5)]
_x, _y = 1, 0
for _t in range(24):
    ROT[_x][_y] = ((_t + 1) * (_t + 2) // 2) % 64
    _x, _y = _y, (2 * _x + 3 * _y) % 5


def _round_constants():
    rcs, lfsr = [], 1
    for _ in range(24):
        rc = 0
        for j in range(7):
            bit = lfsr & 1
            lfsr = ((lfsr << 1) ^ (0x71 if lfsr & 0x80 else 0)) & 0xFF
            if bit:
                rc ^= 1 << ((1 << j) - 1)
        rcs.append(rc)
    return rcs


def _into_bi(v):
    """Split a 64-bit value into its even-indexed and odd-indexed bits."""
    e = o = 0
    for i in range(32):
        e |= ((v >> (2 * i)) & 1) << i
        o |= ((v >> (2 * i + 1)) & 1) << i
    return e, o


RC_BI = [_into_bi(v) for v in _round_constants()]

# ---------------------------------------------------------------------
# Register allocation
#
# RV32 leaves 28 usable registers once x0/sp/gp/tp are excluded, and this
# uses all of them: 10 hold D[], 10 hold one row of B[], four are
# temporaries, two are the src/dst state pointers and two drive the loop.
# ---------------------------------------------------------------------

SRC = "a0"
DST = "t6"
D = [("s2", "s3"), ("s4", "s5"), ("s6", "s7"), ("s8", "s9"), ("s10", "s11")]
B = [("a1", "a2"), ("a3", "a4"), ("a5", "a6"), ("a7", "s0"), ("s1", "ra")]
T = [("t0", "t1"), ("t2", "t3")]
RCP, RCEND = "t4", "t5"

SAVED = ["ra", "s0", "s1", "s2", "s3", "s4", "s5", "s6",
         "s7", "s8", "s9", "s10", "s11"]
SCRATCH = 200                                    # ping-pong copy of the state
FRAME = (SCRATCH + 4 * len(SAVED) + 15) & ~15    # 16-byte aligned


class Asm:
    def __init__(self):
        self.lines = []

    def __call__(self, text, comment=None):
        self.lines.append(f"\t{text:<26} # {comment}" if comment else f"\t{text}")

    def label(self, name):
        self.lines.append(f"{name}:")

    def note(self, comment=""):
        self.lines.append(f"\t# {comment}" if comment else "")

    def text(self):
        return "\n".join(self.lines)


def rotl(a, dst, src, k, comment=None):
    """32-bit rotate-left by k. Zbb has no rotate-left immediate, so this
    is a rori by the complement; k == 0 degenerates to a rename."""
    k %= 32
    if k == 0:
        if dst != src:
            a(f"mv {dst}, {src}", comment)
    else:
        a(f"rori {dst}, {src}, {32 - k}", comment)


def emit_rot_lane(a, dst_pair, src_pair, r):
    """Rotate one interleaved lane left by r bits.

    An even offset rotates each half by r/2. An odd offset also swaps the
    halves, so r == 1 costs a single rori plus a rename.
    """
    be, bo = dst_pair
    ae, ao = src_pair
    if r % 2 == 0:
        rotl(a, be, ae, r // 2)
        rotl(a, bo, ao, r // 2)
    else:
        rotl(a, be, ao, (r + 1) // 2)
        rotl(a, bo, ae, r // 2)


def emit_theta_c(a):
    """C[x] = A[x,0] ^ ... ^ A[x,4], accumulated in the B[] registers.

    Each load is issued at least two instructions before the xor that
    consumes it, so the sequence does not stall on an in-order core
    (eprint 2024/1515, Listing 2).
    """
    a.note()
    a.note("theta: column parities C[x], parked in the B[] registers")
    for x in range(5):
        ce, co = B[x]
        off = lambda y: 8 * (x + 5 * y)
        a(f"lw {ce}, {off(0)}({SRC})", f"C[{x}]  = A[{x},0]")
        a(f"lw {co}, {off(0) + 4}({SRC})")
        a(f"lw {T[0][0]}, {off(1)}({SRC})")
        a(f"lw {T[0][1]}, {off(1) + 4}({SRC})")
        a(f"lw {T[1][0]}, {off(2)}({SRC})")
        a(f"lw {T[1][1]}, {off(2) + 4}({SRC})")
        a(f"xor {ce}, {ce}, {T[0][0]}", f"      ^= A[{x},1]")
        a(f"xor {co}, {co}, {T[0][1]}")
        a(f"lw {T[0][0]}, {off(3)}({SRC})")
        a(f"lw {T[0][1]}, {off(3) + 4}({SRC})")
        a(f"xor {ce}, {ce}, {T[1][0]}", f"      ^= A[{x},2]")
        a(f"xor {co}, {co}, {T[1][1]}")
        a(f"lw {T[1][0]}, {off(4)}({SRC})")
        a(f"lw {T[1][1]}, {off(4) + 4}({SRC})")
        a(f"xor {ce}, {ce}, {T[0][0]}", f"      ^= A[{x},3]")
        a(f"xor {co}, {co}, {T[0][1]}")
        a(f"xor {ce}, {ce}, {T[1][0]}", f"      ^= A[{x},4]")
        a(f"xor {co}, {co}, {T[1][1]}")


def emit_theta_d(a):
    """D[x] = C[x-1] ^ ROT(C[x+1], 1).

    Rotating by 1 in the interleaved domain touches only the odd half, so
    each D[x] is one rori and two xors. The independent odd-half xor sits
    between the rori and its use to cover the latency.
    """
    a.note()
    a.note("theta: D[x] = C[x-1] ^ ROT(C[x+1], 1)")
    for x in range(5):
        de, do = D[x]
        ce_lo, co_lo = B[(x + 4) % 5]
        ce_hi, co_hi = B[(x + 1) % 5]
        a(f"rori {T[0][0]}, {co_hi}, 31", f"D[{x}]")
        a(f"xor {do}, {co_lo}, {ce_hi}")
        a(f"xor {de}, {ce_lo}, {T[0][0]}")


def emit_row(a, y_out):
    """One output row of the state.

    rho+pi is fused with theta's D[] fold and with the read of A, so each
    lane is loaded exactly once per round and theta never needs a
    separate write-back pass. chi then consumes the five B[] lanes and
    stores the row.
    """
    a.note()
    a.note(f"row y={y_out}: rho+pi (with theta folded into the read)")

    def source(bx):
        """Lane feeding B[bx, y_out], from inverting pi: B[y, 2x+3y] = A[x, y]."""
        x = (3 * (y_out - 3 * bx)) % 5
        return x, bx

    cur = 0
    x0, y0 = source(0)
    lane0 = x0 + 5 * y0
    a(f"lw {T[0][0]}, {8 * lane0}({SRC})", f"A[{x0},{y0}]")
    a(f"lw {T[0][1]}, {8 * lane0 + 4}({SRC})")
    for bx in range(5):
        x, y = source(bx)
        de, do = D[x]
        p, nxt = T[cur], T[1 - cur]
        if bx < 4:
            nx, ny = source(bx + 1)
            nlane = nx + 5 * ny
            a(f"lw {nxt[0]}, {8 * nlane}({SRC})", f"A[{nx},{ny}]")
            a(f"lw {nxt[1]}, {8 * nlane + 4}({SRC})")
        a(f"xor {p[0]}, {p[0]}, {de}", f"A[{x},{y}] ^= D[{x}], rot {ROT[x][y]}")
        a(f"xor {p[1]}, {p[1]}, {do}")
        emit_rot_lane(a, B[bx], p, ROT[x][y])
        cur ^= 1

    a.note()
    a.note(f"row y={y_out}: chi, iota, store")
    for bx in range(5):
        be, bo = B[bx]
        n1e, n1o = B[(bx + 1) % 5]
        n2e, n2o = B[(bx + 2) % 5]
        lane = bx + 5 * y_out
        a(f"andn {T[0][0]}, {n2e}, {n1e}", f"A[{bx},{y_out}]")
        a(f"andn {T[0][1]}, {n2o}, {n1o}")
        a(f"xor {T[0][0]}, {T[0][0]}, {be}")
        a(f"xor {T[0][1]}, {T[0][1]}, {bo}")
        if bx == 0 and y_out == 0:
            a(f"lw {T[1][0]}, 0({RCP})", "iota")
            a(f"lw {T[1][1]}, 4({RCP})")
            a(f"xor {T[0][0]}, {T[0][0]}, {T[1][0]}")
            a(f"xor {T[0][1]}, {T[0][1]}, {T[1][1]}")
        a(f"sw {T[0][0]}, {8 * lane}({DST})")
        a(f"sw {T[0][1]}, {8 * lane + 4}({DST})")


def generate_asm():
    a = Asm()
    a(".option arch, +zbb")
    a(".text")
    a(".global keccak_f1600_bi_rv32")
    a(".type keccak_f1600_bi_rv32, @function")
    a(".align 2")
    a.label("keccak_f1600_bi_rv32")
    a.note("a0 = state: 25 interleaved lanes, even half at 8*i, odd at 8*i+4")
    a(f"addi sp, sp, -{FRAME}")
    for i, r in enumerate(SAVED):
        a(f"sw {r}, {SCRATCH + 4 * i}(sp)")
    a.note()
    a.note("Rounds alternate between the caller's buffer and a stack copy.")
    a.note("24 is even, so the final round writes the caller's buffer.")
    a(f"mv {DST}, sp")
    a(f"la {RCP}, keccak_rc_bi")
    a(f"addi {RCEND}, {RCP}, {24 * 8}")
    a.note()
    a.label(".Lkeccak_round")
    emit_theta_c(a)
    emit_theta_d(a)
    for y in range(5):
        emit_row(a, y)
    a.note()
    a.note("next round constant, then swap src/dst")
    a(f"addi {RCP}, {RCP}, 8")
    a(f"mv {T[1][0]}, {SRC}")
    a(f"mv {SRC}, {DST}")
    a(f"mv {DST}, {T[1][0]}")
    a(f"bne {RCP}, {RCEND}, .Lkeccak_round")
    a.note()
    for i, r in enumerate(SAVED):
        a(f"lw {r}, {SCRATCH + 4 * i}(sp)")
    a(f"addi sp, sp, {FRAME}")
    a("ret")
    a(".size keccak_f1600_bi_rv32, .-keccak_f1600_bi_rv32")
    a.note()
    a(".section .rodata")
    a(".align 2")
    a(".type keccak_rc_bi, @object")
    a.label("keccak_rc_bi")
    a.note("round constants, pre-split into (even bits, odd bits)")
    for e, o in RC_BI:
        a(f".word 0x{e:08x}, 0x{o:08x}")
    return a.text()


RUST_TEMPLATE = '''//! Keccak-f[1600] for RV32 with the B extension.
//!
//! GENERATED by `gen/permutation_rv32.py` -- do not edit by hand.
//!
//! The permutation stays in the bit-interleaved domain for all 24
//! rounds, which turns each 64-bit lane rotation into one or two 32-bit
//! `rori` instructions and each chi step into an `andn`. See the
//! generator for the rationale and the register allocation.

core::arch::global_asm!(
    r#"
{asm}
"#
);

unsafe extern "C" {{
    /// 25 interleaved lanes: even half of lane `i` at `8*i`, odd at `8*i+4`.
    pub fn keccak_f1600_bi_rv32(state: *mut u32);
}}
'''


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", type=Path, default=Path("."),
                        help="Directory to write output files")
    parser.add_argument("--lang", choices=["rust", "generic"], default="rust",
                        help="Output language (rust module or bare assembly)")
    args = parser.parse_args()

    asm = generate_asm()
    assert "{" not in asm and "}" not in asm, "braces would break global_asm!"
    assert '"' not in asm, "a quote would terminate the raw string"

    if args.lang == "rust":
        path = args.output_dir / "permutation_rv32.rs"
        content = RUST_TEMPLATE.format(asm=asm)
    else:
        path = args.output_dir / "permutation_rv32.s"
        content = asm + "\n"

    # Only rewrite when the content actually changes, so a build does not
    # needlessly invalidate itself.
    if not path.exists() or path.read_text() != content:
        path.write_text(content)


if __name__ == "__main__":
    main()
