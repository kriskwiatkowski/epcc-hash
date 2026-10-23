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
