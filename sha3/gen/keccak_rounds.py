#!/usr/bin/env python3
# SPDX-FileCopyrightText: 2026 Krzysztof Kwiatkowski
# SPDX-License-Identifier: CC0-1.0

from nervapy.arm import *
from nervapy.arm.registers import r2, r3, r4, r6, r7, r8, r9, r10, r11, r12, lr, sp

from keccak_permute import KeccakThetaRhoPiChi, KeccakThetaRhoPiChiIota, xor5, xor5str, xorrol


# These offsets mirror the layout in the handwritten assembly: 25 lanes, each split
# into two 32-bit words in the interleaved representation.
Aba0 = 0 * 4
Aba1 = 1 * 4
Abe0 = 2 * 4
Abe1 = 3 * 4
Abi0 = 4 * 4
Abi1 = 5 * 4
Abo0 = 6 * 4
Abo1 = 7 * 4
Abu0 = 8 * 4
Abu1 = 9 * 4
Aga0 = 10 * 4
Aga1 = 11 * 4
Age0 = 12 * 4
Age1 = 13 * 4
Agi0 = 14 * 4
Agi1 = 15 * 4
Ago0 = 16 * 4
Ago1 = 17 * 4
Agu0 = 18 * 4
Agu1 = 19 * 4
Aka0 = 20 * 4
Aka1 = 21 * 4
Ake0 = 22 * 4
Ake1 = 23 * 4
Aki0 = 24 * 4
Aki1 = 25 * 4
Ako0 = 26 * 4
Ako1 = 27 * 4
Aku0 = 28 * 4
Aku1 = 29 * 4
Ama0 = 30 * 4
Ama1 = 31 * 4
Ame0 = 32 * 4
Ame1 = 33 * 4
Ami0 = 34 * 4
Ami1 = 35 * 4
Amo0 = 36 * 4
Amo1 = 37 * 4
Amu0 = 38 * 4
Amu1 = 39 * 4
Asa0 = 40 * 4
Asa1 = 41 * 4
Ase0 = 42 * 4
Ase1 = 43 * 4
Asi0 = 44 * 4
Asi1 = 45 * 4
Aso0 = 46 * 4
Aso1 = 47 * 4
Asu0 = 48 * 4
Asu1 = 49 * 4

mDa0 = 0 * 4
mDa1 = 1 * 4
mDo0 = 2 * 4
mDo1 = 3 * 4
mDi0 = 4 * 4
mRC = 5 * 4
# One word padding beyond the 6 slots actually used (mDa0..mDi0, mRC): the
# prologue does PUSH {lr} (4 bytes) then SUB sp, sp, mSize. 4 + 6*4 = 28,
# which is not a multiple of 8, leaving SP 8-byte-misaligned for the whole
# function body instead of just internally-but-safely offset. 4 + 7*4 = 32
# restores AAPCS double-word alignment at every point inside the function.
mSize = 7 * 4


def KeccakRound0():
    # Round 0 starts from the non-lazy, classically aligned state representation.
    xor5(r3, Abu0, Agu0, Aku0, Amu0, Asu0, 0, 0, 0, 0, 0)
    xor5(r7, Abe1, Age1, Ake1, Ame1, Ase1, 0, 0, 0, 0, 0)
    xorrol(r6, r3, r7, 32)
    xor5str(r4, Abi1, Agi1, Aki1, Ami1, Asi1, 0, 0, 0, 0, 0, r6, sp, mDa0)
    EOR(r6, r3, r4)
    xor5str(r3, Abo0, Ago0, Ako0, Amo0, Aso0, 0, 0, 0, 0, 0, r6, sp, mDo1)
    EOR(r2, r7, r3)
    xor5(r7, Aba0, Aga0, Aka0, Ama0, Asa0, 0, 0, 0, 0, 0)
    xorrol(r10, r7, r4, 32)
    xor5(r4, Abo1, Ago1, Ako1, Amo1, Aso1, 0, 0, 0, 0, 0)
    EOR(lr, r4, r7)
    xor5(r7, Abe0, Age0, Ake0, Ame0, Ase0, 0, 0, 0, 0, 0)
    xorrol(r6, r7, r4, 32)
    xor5str(r4, Abu1, Agu1, Aku1, Amu1, Asu1, 0, 0, 0, 0, 0, r6, sp, mDi0)
    EOR(r8, r4, r7)
    xor5str(r7, Abi0, Agi0, Aki0, Ami0, Asi0, 0, 0, 0, 0, 0, r8, sp, mDa1)
    xorrol(r9, r7, r4, 32)
    xor5str(r4, Aba1, Aga1, Aka1, Ama1, Asa1, 0, 0, 0, 0, 0, r9, sp, mDo0)
    EOR(r11, r4, r7)
    xorrol(r12, r3, r4, 32)
    KeccakThetaRhoPiChi(Abo0, Aka1, r9, 14, 0, Agu0, Ame1, r12, 10, 0, Aka1, Asi1, r8, 2, 0, Ame1, Abo0, r11, 23, 0, Asi1, Agu0, r2, 31, 0, 1, Aka1)
    KeccakThetaRhoPiChi(Abe0, Asa1, r10, 0, 0, Agi1, Abe0, r2, 3, 0, Ako0, Agi1, r9, 12, 0, Amu1, Ako0, lr, 4, 0, Asa1, Amu1, r8, 9, 0, 1, Agu0)
    LDR(r8, [sp, mDa0])
    KeccakThetaRhoPiChi(Abu1, Aga0, lr, 14, 0, Aga0, Ake0, r8, 18, 0, Ake0, Ami1, r10, 5, 0, Ami1, Aso0, r2, 8, 0, Aso0, Abu1, r9, 28, 0, 1, Amu1)
    KeccakThetaRhoPiChi(Abi1, Ama0, r2, 31, 0, Ago0, Ase1, r9, 27, 0, Aku0, Abi1, r12, 19, 0, Ama0, Ago0, r8, 20, 0, Ase1, Aku0, r11, 1, 0, 1, Abu1)
    LDR(r9, [sp, mDo1])
    KeccakThetaRhoPiChiIota(Aba0, r8, 0, Age0, r10, 22, 0, Aki1, r2, 22, 0, Amo1, r9, 11, 0, Asu0, r12, 7, 0, 0, 0, 1, Aku0, r1, mRC)
    LDR_W(r2, [sp, mDi0])
    KeccakThetaRhoPiChi(Abo1, Aka0, r9, 14, 0, Agu1, Ame0, lr, 10, 0, Aka0, Asi0, r8, 1, 0, Ame0, Abo1, r10, 22, 0, Asi0, Agu1, r2, 30, 0, 1, Aba0)
    KeccakThetaRhoPiChi(Abe1, Asa0, r11, 1, 0, Agi0, Abe1, r2, 3, 0, Ako1, Agi0, r9, 13, 0, Amu0, Ako1, r12, 4, 0, Asa0, Amu0, r8, 9, 0, 1, Agu1)
    LDR(r8, [sp, mDa1])
    KeccakThetaRhoPiChi(Abu0, Aga1, r12, 13, 0, Aga1, Ake1, r8, 18, 0, Ake1, Ami0, r11, 5, 0, Ami0, Aso1, r2, 7, 0, Aso1, Abu0, r9, 28, 0, 1, Amu0)
    KeccakThetaRhoPiChi(Abi0, Ama1, r2, 31, 0, Ago1, Ase0, r9, 28, 0, Aku1, Abi0, lr, 20, 0, Ama1, Ago1, r8, 21, 0, Ase0, Aku1, r10, 1, 0, 1, Abu0)
    LDR(r9, [sp, mDo0])
    KeccakThetaRhoPiChiIota(Aba1, r8, 0, Age1, r11, 22, 0, Aki0, r2, 21, 0, Amo0, r9, 10, 0, Asu1, lr, 7, 0, 4, 0, 1, Aku1, lr, mRC)


def KeccakRound1():
    # Rounds 1 and 2 operate on the lazily rotated state produced by the previous round.
    xor5str(r3, Asu0, Agu0, Amu0, Abu1, Aku1, 22, 10, 3, 18, 28, lr, r0, Aba1)
    xor5(r7, Age1, Ame0, Abe0, Ake1, Ase1, 10, 22, 4, 7, 20)
    ROR(r3, r3, 32 - 22)
    xorrol(r6, r3, r7, 32 - 10)
    xor5str(r4, Aki0, Asi0, Agi1, Ami0, Abi1, 7, 30, 9, 28, 1, r6, sp, mDa0)
    EOR(r6, r3, r4.ROR(32 - 7))
    xor5str(r3, Amo1, Abo0, Ako1, Aso0, Ago1, 0, 14, 1, 14, 31, r6, sp, mDo1)
    EOR(r2, r3, r7.ROR(32 - 10))
    xor5(r7, Aba0, Aka1, Asa0, Aga0, Ama1, 0, 2, 13, 5, 20)
    xorrol(r10, r7, r4, 32 - 7)
    xor5(r4, Amo0, Abo1, Ako0, Aso1, Ago0, 0, 14, 0, 13, 31)
    EOR(lr, r4, r7)
    xor5(r7, Age0, Ame1, Abe1, Ake0, Ase0, 11, 23, 4, 8, 21)
    ROR(r7, r7, 32 - 11)
    xorrol(r6, r7, r4, 32)
    xor5str(r4, Asu1, Agu1, Amu1, Abu0, Aku0, 22, 10, 3, 18, 27, r6, sp, mDi0)
    EOR(r8, r7, r4.ROR(32 - 22))
    xor5str(r7, Aki1, Asi1, Agi0, Ami1, Abi0, 7, 31, 9, 28, 1, r8, sp, mDa1)
    ROR(r7, r7, 32 - 7)
    xorrol(r9, r7, r4, 32 - 22)
    xor5str(r4, Aba1, Aka0, Asa1, Aga1, Ama0, 0, 1, 12, 5, 19, r9, sp, mDo0)
    EOR(r11, r4, r7)
    xorrol(r12, r3, r4, 32)
    KeccakThetaRhoPiChi(Amo1, Asa1, r9, 14, 0, Agu0, Ake1, r12, 10, 10, Asa1, Abi1, r8, 2, 12, Ake1, Amo1, r11, 23, 7, Abi1, Agu0, r2, 31, 1, 1, Asa1)
    KeccakThetaRhoPiChi(Age0, Ama0, r10, 0, 11, Asi0, Age0, r2, 3, 30, Ako1, Asi0, r9, 12, 1, Abu0, Ako1, lr, 4, 18, Ama0, Abu0, r8, 9, 19, 1, Agu0)
    LDR(r8, [sp, mDa0])
    KeccakThetaRhoPiChi(Asu1, Aka1, lr, 14, 22, Aka1, Abe1, r8, 18, 2, Abe1, Ami0, r10, 5, 4, Ami0, Ago1, r2, 8, 28, Ago1, Asu1, r9, 28, 31, 1, Abu0)
    KeccakThetaRhoPiChi(Aki0, Aga0, r2, 31, 7, Abo0, Ase1, r9, 27, 14, Amu0, Aki0, r12, 19, 3, Aga0, Abo0, r8, 20, 5, Ase1, Amu0, r11, 1, 20, 1, Asu1)
    LDR(r9, [sp, mDo1])
    KeccakThetaRhoPiChiIota(Aba0, r8, 0, Ame1, r10, 22, 23, Agi1, r2, 22, 9, Aso1, r9, 11, 13, Aku1, r12, 7, 28, 8, 0, 1, Amu0, r1, mRC)
    LDR_W(r2, [sp, mDi0])
    KeccakThetaRhoPiChi(Amo0, Asa0, r9, 14, 0, Agu1, Ake0, lr, 10, 10, Asa0, Abi0, r8, 1, 13, Ake0, Amo0, r10, 22, 8, Abi0, Agu1, r2, 30, 1, 1, Aba0)
    KeccakThetaRhoPiChi(Age1, Ama1, r11, 1, 10, Asi1, Age1, r2, 3, 31, Ako0, Asi1, r9, 13, 0, Abu1, Ako0, r12, 4, 18, Ama1, Abu1, r8, 9, 20, 1, Agu1)
    LDR(r8, [sp, mDa1])
    KeccakThetaRhoPiChi(Asu0, Aka0, r12, 13, 22, Aka0, Abe0, r8, 18, 1, Abe0, Ami1, r11, 5, 4, Ami1, Ago0, r2, 7, 28, Ago0, Asu0, r9, 28, 31, 1, Abu1)
    KeccakThetaRhoPiChi(Aki1, Aga1, r2, 31, 7, Abo1, Ase0, r9, 28, 14, Amu1, Aki1, lr, 20, 3, Aga1, Abo1, r8, 21, 5, Ase0, Amu1, r10, 1, 21, 1, Asu0)
    LDR(r9, [sp, mDo0])
    KeccakThetaRhoPiChiIota(Aba1, r8, 0, Ame0, r11, 22, 22, Agi0, r2, 21, 9, Aso0, r9, 10, 14, Aku0, lr, 7, 27, 12, 0, 1, Amu1, lr, mRC)


def KeccakRound2():
    # The instruction mix stays the same, but the lane ordering advances with the
    # unrolled Keccak schedule.
    xor5str(r3, Aku1, Agu0, Abu1, Asu1, Amu1, 22, 10, 3, 18, 28, lr, r0, Aba1)
    xor5(r7, Ame0, Ake0, Age0, Abe0, Ase1, 10, 22, 4, 7, 20)
    ROR(r3, r3, 32 - 22)
    xorrol(r6, r3, r7, 32 - 10)
    xor5str(r4, Agi0, Abi0, Asi0, Ami1, Aki0, 7, 30, 9, 28, 1, r6, sp, mDa0)
    EOR(r6, r3, r4.ROR(32 - 7))
    xor5str(r3, Aso1, Amo1, Ako0, Ago1, Abo1, 0, 14, 1, 14, 31, r6, sp, mDo1)
    EOR(r2, r3, r7.ROR(32 - 10))
    xor5(r7, Aba0, Asa1, Ama1, Aka1, Aga1, 0, 2, 13, 5, 20)
    xorrol(r10, r7, r4, 32 - 7)
    xor5(r4, Aso0, Amo0, Ako1, Ago0, Abo0, 0, 14, 0, 13, 31)
    EOR(lr, r4, r7)
    xor5(r7, Ame1, Ake1, Age1, Abe1, Ase0, 11, 23, 4, 8, 21)
    ROR(r7, r7, 32 - 11)
    xorrol(r6, r7, r4, 32)
    xor5str(r4, Aku0, Agu1, Abu0, Asu0, Amu0, 22, 10, 3, 18, 27, r6, sp, mDi0)
    EOR(r8, r7, r4.ROR(32 - 22))
    xor5str(r7, Agi1, Abi1, Asi1, Ami0, Aki1, 7, 31, 9, 28, 1, r8, sp, mDa1)
    ROR(r7, r7, 32 - 7)
    xorrol(r9, r7, r4, 32 - 22)
    xor5str(r4, Aba1, Asa0, Ama0, Aka0, Aga0, 0, 1, 12, 5, 19, r9, sp, mDo0)
    EOR(r11, r4, r7)
    xorrol(r12, r3, r4, 32)
    KeccakThetaRhoPiChi(Aso1, Ama0, r9, 14, 0, Agu0, Abe0, r12, 10, 10, Ama0, Aki0, r8, 2, 12, Abe0, Aso1, r11, 23, 7, Aki0, Agu0, r2, 31, 1, 1, Ama0)
    KeccakThetaRhoPiChi(Ame1, Aga0, r10, 0, 11, Abi0, Ame1, r2, 3, 30, Ako0, Abi0, r9, 12, 1, Asu0, Ako0, lr, 4, 18, Aga0, Asu0, r8, 9, 19, 1, Agu0)
    LDR(r8, [sp, mDa0])
    KeccakThetaRhoPiChi(Aku0, Asa1, lr, 14, 22, Asa1, Age1, r8, 18, 2, Age1, Ami1, r10, 5, 4, Ami1, Abo1, r2, 8, 28, Abo1, Aku0, r9, 28, 31, 1, Asu0)
    KeccakThetaRhoPiChi(Agi0, Aka1, r2, 31, 7, Amo1, Ase1, r9, 27, 14, Abu1, Agi0, r12, 19, 3, Aka1, Amo1, r8, 20, 5, Ase1, Abu1, r11, 1, 20, 1, Aku0)
    LDR(r9, [sp, mDo1])
    KeccakThetaRhoPiChiIota(Aba0, r8, 0, Ake1, r10, 22, 23, Asi0, r2, 22, 9, Ago0, r9, 11, 13, Amu1, r12, 7, 28, 16, 0, 1, Abu1, r1, mRC)
    LDR_W(r2, [sp, mDi0])
    KeccakThetaRhoPiChi(Aso0, Ama1, r9, 14, 0, Agu1, Abe1, lr, 10, 10, Ama1, Aki1, r8, 1, 13, Abe1, Aso0, r10, 22, 8, Aki1, Agu1, r2, 30, 1, 1, Aba0)
    KeccakThetaRhoPiChi(Ame0, Aga1, r11, 1, 10, Abi1, Ame0, r2, 3, 31, Ako1, Abi1, r9, 13, 0, Asu1, Ako1, r12, 4, 18, Aga1, Asu1, r8, 9, 20, 1, Agu1)
    LDR(r8, [sp, mDa1])
    KeccakThetaRhoPiChi(Aku1, Asa0, r12, 13, 22, Asa0, Age0, r8, 18, 1, Age0, Ami0, r11, 5, 4, Ami0, Abo0, r2, 7, 28, Abo0, Aku1, r9, 28, 31, 1, Asu1)
    KeccakThetaRhoPiChi(Agi1, Aka0, r2, 31, 7, Amo0, Ase0, r9, 28, 14, Abu0, Agi1, lr, 20, 3, Aka0, Amo0, r8, 21, 5, Ase0, Abu0, r10, 1, 21, 1, Aku1)
    LDR(r9, [sp, mDo0])
    KeccakThetaRhoPiChiIota(Aba1, r8, 0, Ake0, r11, 22, 22, Asi1, r2, 21, 9, Ago1, r9, 10, 14, Amu0, lr, 7, 27, 20, 0, 1, Abu0, lr, mRC)


def KeccakRound3():
    # Round 3 resolves the lazy rotations so the next 4-round block restarts from
    # the classical representation.
    xor5str(r3, Amu1, Agu0, Asu1, Aku0, Abu0, 22, 10, 3, 18, 28, lr, r0, Aba1)
    xor5(r7, Ake0, Abe1, Ame1, Age0, Ase1, 10, 22, 4, 7, 20)
    ROR(r3, r3, 32 - 22)
    xorrol(r6, r3, r7, 32 - 10)
    xor5str(r4, Asi1, Aki1, Abi0, Ami0, Agi0, 7, 30, 9, 28, 1, r6, sp, mDa0)
    EOR(r6, r3, r4.ROR(32 - 7))
    xor5str(r3, Ago0, Aso1, Ako1, Abo1, Amo0, 0, 14, 1, 14, 31, r6, sp, mDo1)
    EOR(r2, r3, r7.ROR(32 - 10))
    xor5(r7, Aba0, Ama0, Aga1, Asa1, Aka0, 0, 2, 13, 5, 20)
    xorrol(r10, r7, r4, 32 - 7)
    xor5(r4, Ago1, Aso0, Ako0, Abo0, Amo1, 0, 14, 0, 13, 31)
    EOR(lr, r4, r7)
    xor5(r7, Ake1, Abe0, Ame0, Age1, Ase0, 11, 23, 4, 8, 21)
    ROR(r7, r7, 32 - 11)
    xorrol(r6, r7, r4, 32)
    xor5str(r4, Amu0, Agu1, Asu0, Aku1, Abu1, 22, 10, 3, 18, 27, r6, sp, mDi0)
    EOR(r8, r7, r4.ROR(32 - 22))
    xor5str(r7, Asi0, Aki0, Abi1, Ami1, Agi1, 7, 31, 9, 28, 1, r8, sp, mDa1)
    ROR(r7, r7, 32 - 7)
    xorrol(r9, r7, r4, 32 - 22)
    xor5str(r4, Aba1, Ama1, Aga0, Asa0, Aka1, 0, 1, 12, 5, 19, r9, sp, mDo0)
    EOR(r11, r4, r7)
    xorrol(r12, r3, r4, 32)
    KeccakThetaRhoPiChi(Ago0, Aga0, r9, 14, 0, Agu0, Age0, r12, 10, 10, Aga0, Agi0, r8, 2, 12, Age0, Ago0, r11, 23, 7, Agi0, Agu0, r2, 31, 1, 0, Aga0)
    KeccakThetaRhoPiChi(Ake1, Aka1, r10, 0, 11, Aki1, Ake1, r2, 3, 30, Ako1, Aki1, r9, 12, 1, Aku1, Ako1, lr, 4, 18, Aka1, Aku1, r8, 9, 19, 0, Agu0)
    LDR(r8, [sp, mDa0])
    KeccakThetaRhoPiChi(Amu0, Ama0, lr, 14, 22, Ama0, Ame0, r8, 18, 2, Ame0, Ami0, r10, 5, 4, Ami0, Amo0, r2, 8, 28, Amo0, Amu0, r9, 28, 31, 0, Aku1)
    KeccakThetaRhoPiChi(Asi1, Asa1, r2, 31, 7, Aso1, Ase1, r9, 27, 14, Asu1, Asi1, r12, 19, 3, Asa1, Aso1, r8, 20, 5, Ase1, Asu1, r11, 1, 20, 0, Amu0)
    LDR(r9, [sp, mDo1])
    KeccakThetaRhoPiChiIota(Aba0, r8, 0, Abe0, r10, 22, 23, Abi0, r2, 22, 9, Abo0, r9, 11, 13, Abu0, r12, 7, 28, 24, 0, 0, Asu1, r1, mRC)
    LDR_W(r2, [sp, mDi0])
    KeccakThetaRhoPiChi(Ago1, Aga1, r9, 14, 0, Agu1, Age1, lr, 10, 10, Aga1, Agi1, r8, 1, 13, Age1, Ago1, r10, 22, 8, Agi1, Agu1, r2, 30, 1, 0, Aba0)
    KeccakThetaRhoPiChi(Ake0, Aka0, r11, 1, 10, Aki0, Ake0, r2, 3, 31, Ako0, Aki0, r9, 13, 0, Aku0, Ako0, r12, 4, 18, Aka0, Aku0, r8, 9, 20, 0, Agu1)
    LDR(r8, [sp, mDa1])
    KeccakThetaRhoPiChi(Amu1, Ama1, r12, 13, 22, Ama1, Ame1, r8, 18, 1, Ame1, Ami1, r11, 5, 4, Ami1, Amo1, r2, 7, 28, Amo1, Amu1, r9, 28, 31, 0, Aku0)
    KeccakThetaRhoPiChi(Asi0, Asa0, r2, 31, 7, Aso0, Ase0, r9, 28, 14, Asu0, Asi0, lr, 20, 3, Asa0, Aso0, r8, 21, 5, Ase0, Asu0, r10, 1, 21, 0, Amu1)
    LDR(r9, [sp, mDo0])
    KeccakThetaRhoPiChiIota(Aba1, r8, 0, Abe1, r11, 22, 22, Abi1, r2, 21, 9, Abo1, r9, 10, 14, Abu1, lr, 7, 27, 28, 1, 0, Asu0, r1, mRC)
    STR_W(r1, [r0, Aba1])
