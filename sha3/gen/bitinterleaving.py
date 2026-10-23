#!/usr/bin/env python3
# SPDX-FileCopyrightText: 2026 Krzysztof Kwiatkowski
# SPDX-License-Identifier: CC0-1.0

# Bit interleaving routines for 32-bit version of Keccak-p1600 function.
# Credit: Henry S. Warren, Hacker's Delight, Addison-Wesley, 2002

import argparse
from nervapy import *
from nervapy.arm import *
from nervapy.arm.abi import arm_gnueabihf
from nervapy.arm.microarchitecture import Microarchitecture
from nervapy.arm.formats import AssemblyFormat
from nervapy.arm.registers import r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, lr, sp
from pathlib import Path

def toBitInterleaving(x0, x1, s0, s1, t, s0_ptr, s1_ptr, over):
    """
    Convert two 32-bit words (x0, x1) from natural to bit-interleaved representation.    
    Even bits -> s0, odd bits -> s1.

    over=True: overwrite s0/s1.  over=False: XOR into s0/s1.
    Clobbers: t.
    """
    # Pack even-position bits down toward the low halfword.
    AND(t, x0, 0x55555555)

    ORR(t, t, t.LSR(1))

    AND(t, t, 0x33333333)
    ORR(t, t, t.LSR(2))

    AND(t, t, 0x0F0F0F0F)
    ORR(t, t, t.LSR(4))

    AND(t, t, 0x00FF00FF)
    # BFI t, t, #8, #8: t[15:8] = t[7:0] (bits[15:8] are 0 after AND above)
    ORR(t, t, t.LSL(8))
    if over:
        LSR(s0, t, 8)
    else:
        EOR(s0, s0, t.LSR(8))
    UXTH(s0, s0)  # Clear upper halfword of s0

    # Fold the second input word into the upper half of s0.
    AND(t, x1, 0x55555555)
    ORR(t, t, t.LSR(1))
    AND(t, t, 0x33333333)
    ORR(t, t, t.LSR(2))
    AND(t, t, 0x0F0F0F0F)
    ORR(t, t, t.LSR(4))
    AND(t, t, 0x00FF00FF)
    ORR(t, t, t.LSR(8))
    EOR(s0, s0, t.LSL(16))
    STR(s0, [s0_ptr])

    # Repeat the same butterfly for odd-position bits.
    AND(t, x0, 0xAAAAAAAA)
    ORR(t, t, t.LSL(1))
    AND(t, t, 0xCCCCCCCC)
    ORR(t, t, t.LSL(2))
    AND(t, t, 0xF0F0F0F0)
    ORR(t, t, t.LSL(4))
    AND(t, t, 0xFF00FF00)
    ORR(t, t, t.LSL(8))
    if over:
        LSR(s1, t, 16)
    else:
        EOR(s1, s1, t.LSR(16))

    # Fold the second input word into the upper half of s1.
    AND(t, x1, 0xAAAAAAAA)
    ORR(t, t, t.LSL(1))
    AND(t, t, 0xCCCCCCCC)
    ORR(t, t, t.LSL(2))
    AND(t, t, 0xF0F0F0F0)
    ORR(t, t, t.LSL(4))
    AND(t, t, 0xFF00FF00)
    ORR(t, t, t.LSL(8))
    # BFC t, #0, #16; EOR s1, s1, t  →  s1 ^= (t & 0xFFFF0000)
    LSR(t, t, 16)
    EOR(s1, s1, t.LSL(16))
    STR(s1, [s1_ptr])

def fromBitInterleaving(x0, x1, t):
    """
    Convert two 32-bit words from bit-interleaved back to natural representation.
    Even bits in x0, odd bits in x1. Result written in-place.

    Clobbers: t.
    """
    # Merge interleaved halves:
    #   new_x0 = (x0 & 0x0000FFFF) | (x1[15:0] << 16)
    #   new_x1 = (x1 & 0xFFFF0000) | (x0[31:16])
    MOV(t, x0)
    LSL(x0, x0, 16)
    LSR(x0, x0, 16)            # x0 &= 0x0000FFFF
    ORR(x0, x0, x1.LSL(16))    # x0[31:16] = x1[15:0]
    LSR(x1, x1, 16)
    LSL(x1, x1, 16)            # x1 &= 0xFFFF0000
    ORR(x1, x1, t.LSR(16))     # x1[15:0] = old_x0[31:16]

    # Undo the interleaving with the inverse butterfly stages.
    EOR(t, x0, x0.LSR(8))
    AND(t, t, 0x0000FF00)
    EOR(x0, x0, t)
    EOR(x0, x0, t.LSL(8))

    EOR(t, x0, x0.LSR(4))
    AND(t, t, 0x00F000F0)
    EOR(x0, x0, t)
    EOR(x0, x0, t.LSL(4))

    EOR(t, x0, x0.LSR(2))
    AND(t, t, 0x0C0C0C0C)
    EOR(x0, x0, t)
    EOR(x0, x0, t.LSL(2))

    EOR(t, x0, x0.LSR(1))
    AND(t, t, 0x22222222)
    EOR(x0, x0, t)
    EOR(x0, x0, t.LSL(1))

    # Apply the same inverse butterfly to the odd-bit word.
    EOR(t, x1, x1.LSR(8))
    AND(t, t, 0x0000FF00)
    EOR(x1, x1, t)
    EOR(x1, x1, t.LSL(8))

    EOR(t, x1, x1.LSR(4))
    AND(t, t, 0x00F000F0)
    EOR(x1, x1, t)
    EOR(x1, x1, t.LSL(4))

    EOR(t, x1, x1.LSR(2))
    AND(t, t, 0x0C0C0C0C)
    EOR(x1, x1, t)
    EOR(x1, x1, t.LSL(2))

    EOR(t, x1, x1.LSR(1))
    AND(t, t, 0x22222222)
    EOR(x1, x1, t)
    EOR(x1, x1, t.LSL(1))


class Generator(object):
    def __init__(self, asm_format=AssemblyFormat.GAS, target=Microarchitecture.CortexM33, output_dir=Path("."), lang="generic"):
        self.asm_format = asm_format
        self.target = target
        self.output_dir = output_dir
        self.lang = lang

    class Writer(object):
        ''' Context manager for writing assembly output to a file in a format required by the target language. '''
        def __init__(self, output_dir, language, filename):
            ''' Initialize the writer with the output directory, target language, and filename. 
            
            Args:
                output_dir (Path): The directory where the output file will be written.
                language (str): The target language for the output file. Can be "rust" or
                    "generic" (for assembly).
                filename (str): The base name of the output file (without extension).
            '''
            self.output_dir = output_dir
            self.language = language
            if language == "rust":
                self.fname = filename + ".rs"
            else:
                self.fname = filename + ".s"

        def __enter__(self):
            self.fhandle = open(self.output_dir / self.fname, "w")
            return self

        def __exit__(self, exc_type, exc_value, traceback):
            self.fhandle.close()
            return False

        def write(self, asm):
            ''' Write (append) the assembly output to the file in the appropriate format for the target language. 
            
            Args:
                asm: The assembly output to write.
            '''
            if self.language == "rust":
                self.fhandle.write(asm.rust_module)
            else:
                self.fhandle.write(asm.assembly)

    def _func_common_setup(self, f):
        f.abi = arm_gnueabihf
        f.target = self.target
        f.assembly_format = self.asm_format
        f.is_thumb = True
        f.high_register_strategy = HighRegisterStrategy.STMDB
        f.alignment = 2
        return f

    def run_generator_and_write(self, filename, prefix=""):
        ''' Run the generator and write the output files to the specified directory. '''
        with self.Writer(self.output_dir, self.lang, filename) as w:
            # Define function arguments
            r1_arg = Argument(const_uint32_t)             # Input buffer
            r2_arg = Argument(const_uint32_t)             # Input buffer
            s1_arg = Argument(ptr(uint32_t))        # Output buffer
            s2_arg = Argument(ptr(uint32_t))        # Output buffer
            t_arg = Argument(ptr(uint32_t))         # Temporary buffer

            f_to = Function(prefix + "conv_into_bi", (r1_arg,r2_arg,s1_arg,s2_arg,t_arg), abi=arm_gnueabihf)
            self._func_common_setup(f_to)
            with f_to:
                r1 = GeneralPurposeRegister()
                r2 = GeneralPurposeRegister()
                s1_ptr = GeneralPurposeRegister()
                s2_ptr = GeneralPurposeRegister()
                s1 = GeneralPurposeRegister()
                s2 = GeneralPurposeRegister()
                t = GeneralPurposeRegister()
                LOAD.ARGUMENT(r1, r1_arg)
                LOAD.ARGUMENT(r2, r2_arg)
                LOAD.ARGUMENT(s1_ptr, s1_arg)
                LOAD.ARGUMENT(s2_ptr, s2_arg)
                toBitInterleaving(r1, r2, s1, s2, t, s1_ptr, s2_ptr, True)
                RETURN()
            w.write(f_to)

            f_from = Function(prefix + "conv_from_bi", (s1_arg,s2_arg,t_arg), abi=arm_gnueabihf)
            self._func_common_setup(f_from)
            with f_from:
                s1_ptr = GeneralPurposeRegister()
                s2_ptr = GeneralPurposeRegister()
                s1 = GeneralPurposeRegister()
                s2 = GeneralPurposeRegister()
                t = GeneralPurposeRegister()
                LOAD.ARGUMENT(s1_ptr, s1_arg)
                LOAD.ARGUMENT(s2_ptr, s2_arg)
                LDR(s1, [s1_ptr])
                LDR(s2, [s2_ptr])
                fromBitInterleaving(s1, s2, t)
                STR(s1, [s1_ptr])
                STR(s2, [s2_ptr])
                RETURN()
            w.write(f_from)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='')
    parser.add_argument("--output-dir", type=Path, default=Path("."),
                    help="Directory to write output files")
    parser.add_argument("--lang", choices=["rust", "generic"], default="generic",
                    help="Output language (rust or generic)")
    args = parser.parse_args()

    gen = Generator(
        target=Microarchitecture.CortexM33,
        output_dir=args.output_dir,
        lang=args.lang,
        asm_format=AssemblyFormat.GAS)
    gen.run_generator_and_write("bitinterleaving_armv8m")
