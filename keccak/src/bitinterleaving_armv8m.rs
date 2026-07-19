core::arch::global_asm!(r#"
	.syntax unified
	.thumb
	.arch armv8-m.main

	.text

.global conv_into_bi
.type conv_into_bi, %function
.align 2
conv_into_bi:
Lconv_into_bi.ENTRY:
	PUSH {{r4}}
	SUB sp, sp, #4
	AND r12, r0, #1431655765
	ORR r12, r12, r12, LSR #1
	AND r12, r12, #858993459
	ORR r12, r12, r12, LSR #2
	AND r12, r12, #252645135
	ORR r12, r12, r12, LSR #4
	AND r12, r12, #16711935
	ORR r12, r12, r12, LSL #8
	LSR r4, r12, #8
	UXTH r4, r4
	AND r12, r1, #1431655765
	ORR r12, r12, r12, LSR #1
	AND r12, r12, #858993459
	ORR r12, r12, r12, LSR #2
	AND r12, r12, #252645135
	ORR r12, r12, r12, LSR #4
	AND r12, r12, #16711935
	ORR r12, r12, r12, LSR #8
	EOR r4, r4, r12, LSL #16
	STR r4, [r2]
	AND r12, r0, #2863311530
	ORR r12, r12, r12, LSL #1
	AND r12, r12, #3435973836
	ORR r12, r12, r12, LSL #2
	AND r12, r12, #4042322160
	ORR r12, r12, r12, LSL #4
	AND r12, r12, #4278255360
	ORR r12, r12, r12, LSL #8
	LSR r2, r12, #16
	AND r12, r1, #2863311530
	ORR r12, r12, r12, LSL #1
	AND r12, r12, #3435973836
	ORR r12, r12, r12, LSL #2
	AND r12, r12, #4042322160
	ORR r12, r12, r12, LSL #4
	AND r12, r12, #4278255360
	ORR r12, r12, r12, LSL #8
	LSR r12, r12, #16
	EOR r2, r2, r12, LSL #16
	STR r2, [r3]
	ADD sp, sp, #4
	POP {{r4}}
	BX lr

"#);

unsafe extern "C" {
    pub fn conv_into_bi(r1: u32, r2: u32, s1: *mut u32, s2: *mut u32, t: *mut u32);
}
core::arch::global_asm!(r#"
	.syntax unified
	.thumb
	.arch armv8-m.main

	.text

.global conv_from_bi
.type conv_from_bi, %function
.align 2
conv_from_bi:
Lconv_from_bi.ENTRY:
	LDR r12, [r0]
	LDR r3, [r1]
	MOV r2, r12
	LSL r12, r12, #16
	LSR r12, r12, #16
	ORR r12, r12, r3, LSL #16
	LSR r3, r3, #16
	LSL r3, r3, #16
	ORR r3, r3, r2, LSR #16
	EOR r2, r12, r12, LSR #8
	AND r2, r2, #65280
	EOR r12, r12, r2
	EOR r12, r12, r2, LSL #8
	EOR r2, r12, r12, LSR #4
	AND r2, r2, #15728880
	EOR r12, r12, r2
	EOR r12, r12, r2, LSL #4
	EOR r2, r12, r12, LSR #2
	AND r2, r2, #202116108
	EOR r12, r12, r2
	EOR r12, r12, r2, LSL #2
	EOR r2, r12, r12, LSR #1
	AND r2, r2, #572662306
	EOR r12, r12, r2
	EOR r12, r12, r2, LSL #1
	EOR r2, r3, r3, LSR #8
	AND r2, r2, #65280
	EOR r3, r3, r2
	EOR r3, r3, r2, LSL #8
	EOR r2, r3, r3, LSR #4
	AND r2, r2, #15728880
	EOR r3, r3, r2
	EOR r3, r3, r2, LSL #4
	EOR r2, r3, r3, LSR #2
	AND r2, r2, #202116108
	EOR r3, r3, r2
	EOR r3, r3, r2, LSL #2
	EOR r2, r3, r3, LSR #1
	AND r2, r2, #572662306
	EOR r3, r3, r2
	EOR r3, r3, r2, LSL #1
	STR r12, [r0]
	STR r3, [r1]
	BX lr

"#);

unsafe extern "C" {
    pub fn conv_from_bi(s1: *mut u32, s2: *mut u32, t: *mut u32);
}
