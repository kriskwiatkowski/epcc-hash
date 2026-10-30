core::arch::global_asm!(r#"
	.syntax unified
	.thumb
	.arch armv8-m.main

	.text

.align 4
epcc_keccak_rct:
Lepcc_keccak_rct:
	.long 0x00000001, 0x00000000
	.long 0x00000000, 0x00000089
	.long 0x00000000, 0x8000008B
	.long 0x00000000, 0x80008080
	.long 0x00000001, 0x0000008B
	.long 0x00000001, 0x00008000
	.long 0x00000001, 0x80008088
	.long 0x00000001, 0x80000082
	.long 0x00000000, 0x0000000B
	.long 0x00000000, 0x0000000A
	.long 0x00000001, 0x00008082
	.long 0x00000000, 0x00008003
	.long 0x00000001, 0x0000808B
	.long 0x00000001, 0x8000000B
	.long 0x00000001, 0x8000008A
	.long 0x00000001, 0x80000081
	.long 0x00000000, 0x80000081
	.long 0x00000000, 0x80000008
	.long 0x00000000, 0x00000083
	.long 0x00000000, 0x80008003
	.long 0x00000001, 0x80008088
	.long 0x00000000, 0x80000088
	.long 0x00000001, 0x00008000
	.long 0x00000000, 0x80008082
	.long 0x000000FF
.global epcc_keccak_permute_asm
.type epcc_keccak_permute_asm, %function
epcc_keccak_permute_asm:
Lepcc_keccak_permute_asm.ENTRY:
	PUSH.W {{r4, r5, r6, r7, r8, r9, r10, r11}}
	ADR r1, Lepcc_keccak_rct
	PUSH {{lr}}
	SUB sp, sp, #28
	STR r1, [sp, #20]
Lepcc_keccak_permute_asm.round_loop_begin:
	LDR.W r3, [r0, #32]
	LDR.W r1, [r0, #72]
	LDR.W r5, [r0, #112]
	LDR r11, [r0, #152]
	LDR r12, [r0, #192]
	EOR r3, r3, r1
	EOR r3, r3, r5
	EOR r3, r3, r11
	EOR r3, r3, r12
	LDR.W r7, [r0, #12]
	LDR.W r1, [r0, #52]
	LDR.W r5, [r0, #92]
	LDR r11, [r0, #132]
	LDR r12, [r0, #172]
	EOR r7, r7, r1
	EOR r7, r7, r5
	EOR r7, r7, r11
	EOR r7, r7, r12
	EOR r6, r3, r7, ROR #31
	LDR.W r4, [r0, #20]
	LDR.W r1, [r0, #60]
	LDR.W r5, [r0, #100]
	LDR r11, [r0, #140]
	LDR r12, [r0, #180]
	STR.W r6, [sp, #0]
	EOR r4, r4, r1
	EOR r4, r4, r5
	EOR r4, r4, r11
	EOR r4, r4, r12
	EOR r6, r3, r4
	LDR.W r3, [r0, #24]
	LDR.W r1, [r0, #64]
	LDR.W r5, [r0, #104]
	LDR r11, [r0, #144]
	LDR r12, [r0, #184]
	STR.W r6, [sp, #12]
	EOR r3, r3, r1
	EOR r3, r3, r5
	EOR r3, r3, r11
	EOR r3, r3, r12
	EOR r2, r7, r3
	LDR.W r7, [r0, #0]
	LDR.W r1, [r0, #40]
	LDR.W r5, [r0, #80]
	LDR r11, [r0, #120]
	LDR r12, [r0, #160]
	EOR r7, r7, r1
	EOR r7, r7, r5
	EOR r7, r7, r11
	EOR r7, r7, r12
	EOR r10, r7, r4, ROR #31
	LDR.W r4, [r0, #28]
	LDR.W r1, [r0, #68]
	LDR.W r5, [r0, #108]
	LDR r11, [r0, #148]
	LDR r12, [r0, #188]
	EOR r4, r4, r1
	EOR r4, r4, r5
	EOR r4, r4, r11
	EOR r4, r4, r12
	EOR lr, r4, r7
	LDR.W r7, [r0, #8]
	LDR.W r1, [r0, #48]
	LDR.W r5, [r0, #88]
	LDR r11, [r0, #128]
	LDR r12, [r0, #168]
	EOR r7, r7, r1
	EOR r7, r7, r5
	EOR r7, r7, r11
	EOR r7, r7, r12
	EOR r6, r7, r4, ROR #31
	LDR.W r4, [r0, #36]
	LDR.W r1, [r0, #76]
	LDR.W r5, [r0, #116]
	LDR r11, [r0, #156]
	LDR r12, [r0, #196]
	STR.W r6, [sp, #16]
	EOR r4, r4, r1
	EOR r4, r4, r5
	EOR r4, r4, r11
	EOR r4, r4, r12
	EOR r8, r4, r7
	LDR.W r7, [r0, #16]
	LDR.W r1, [r0, #56]
	LDR.W r5, [r0, #96]
	LDR r11, [r0, #136]
	LDR r12, [r0, #176]
	STR.W r8, [sp, #4]
	EOR r7, r7, r1
	EOR r7, r7, r5
	EOR r7, r7, r11
	EOR r7, r7, r12
	EOR r9, r7, r4, ROR #31
	LDR.W r4, [r0, #4]
	LDR.W r1, [r0, #44]
	LDR.W r5, [r0, #84]
	LDR r11, [r0, #124]
	LDR r12, [r0, #164]
	STR.W r9, [sp, #8]
	EOR r4, r4, r1
	EOR r4, r4, r5
	EOR r4, r4, r11
	EOR r4, r4, r12
	EOR r11, r4, r7
	EOR r12, r3, r4, ROR #31
	LDR.W r3, [r0, #24]
	LDR.W r4, [r0, #72]
	LDR.W r5, [r0, #84]
	LDR.W r6, [r0, #132]
	LDR.W r7, [r0, #180]
	STR.W r1, [r0, #84]
	EOR r3, r9, r3
	EOR r4, r12, r4
	EOR r5, r8, r5
	EOR r6, r11, r6
	EOR r7, r2, r7
	BIC r1, r5, r4, ROR #24
	EOR r1, r1, r3, ROR #20
	STR.W r1, [r0, #84]
	BIC r1, r6, r5, ROR #21
	EOR r1, r1, r4, ROR #13
	STR.W r1, [r0, #132]
	BIC r1, r7, r6, ROR #8
	EOR r1, r1, r5, ROR #29
	STR.W r1, [r0, #180]
	BIC r1, r3, r7, ROR #15
	EOR r1, r1, r6, ROR #23
	STR.W r1, [r0, #24]
	BIC r1, r4, r3, ROR #28
	EOR r1, r1, r7, ROR #11
	LDR.W r3, [r0, #8]
	LDR.W r4, [r0, #60]
	LDR.W r5, [r0, #104]
	LDR.W r6, [r0, #156]
	LDR.W r7, [r0, #164]
	STR.W r1, [r0, #72]
	EOR r3, r10, r3
	EOR r4, r2, r4
	EOR r5, r9, r5
	EOR r6, lr, r6
	EOR r7, r8, r7
	BIC r1, r5, r4, ROR #9
	EOR r1, r1, r3, ROR #12
	STR.W r1, [r0, #164]
	BIC r1, r6, r5, ROR #24
	EOR r1, r1, r4, ROR #1
	STR.W r1, [r0, #8]
	BIC r1, r7, r6, ROR #5
	EOR r1, r1, r5, ROR #29
	STR.W r1, [r0, #60]
	BIC r1, r3, r7, ROR #23
	EOR r1, r1, r6, ROR #28
	STR.W r1, [r0, #104]
	BIC r1, r4, r3, ROR #3
	EOR r1, r1, r7, ROR #26
	LDR r8, [sp, #0]
	LDR.W r3, [r0, #36]
	LDR.W r4, [r0, #40]
	LDR.W r5, [r0, #88]
	LDR.W r6, [r0, #140]
	LDR.W r7, [r0, #184]
	STR.W r1, [r0, #156]
	EOR r3, lr, r3
	EOR r4, r8, r4
	EOR r5, r10, r5
	EOR r6, r2, r6
	EOR r7, r9, r7
	BIC r1, r5, r4, ROR #19
	EOR r1, r1, r3, ROR #23
	STR.W r1, [r0, #40]
	BIC r1, r6, r5, ROR #3
	EOR r1, r1, r4, ROR #22
	STR.W r1, [r0, #88]
	BIC r1, r7, r6, ROR #20
	EOR r1, r1, r5, ROR #23
	STR.W r1, [r0, #140]
	BIC r1, r3, r7, ROR #18
	EOR r1, r1, r6, ROR #6
	STR.W r1, [r0, #184]
	BIC r1, r4, r3, ROR #4
	EOR r1, r1, r7, ROR #22
	LDR.W r3, [r0, #20]
	LDR.W r4, [r0, #64]
	LDR.W r5, [r0, #112]
	LDR.W r6, [r0, #120]
	LDR.W r7, [r0, #172]
	STR.W r1, [r0, #36]
	EOR r3, r2, r3
	EOR r4, r9, r4
	EOR r5, r12, r5
	EOR r6, r8, r6
	EOR r7, r11, r7
	BIC r1, r5, r4, ROR #24
	EOR r1, r1, r3, ROR #20
	STR.W r1, [r0, #120]
	BIC r1, r6, r5, ROR #1
	EOR r1, r1, r4, ROR #25
	STR.W r1, [r0, #172]
	BIC r1, r7, r6, ROR #13
	EOR r1, r1, r5, ROR #14
	STR.W r1, [r0, #20]
	BIC r1, r3, r7, ROR #30
	EOR r1, r1, r6, ROR #11
	STR.W r1, [r0, #64]
	BIC r1, r4, r3, ROR #28
	EOR r1, r1, r7, ROR #26
	LDR r9, [sp, #12]
	LDR.W r3, [r0, #0]
	LDR r4, [r0, #48]
	LDR r5, [r0, #100]
	LDR r6, [r0, #148]
	LDR r7, [r0, #192]
	STR.W r1, [r0, #112]
	EOR r3, r8, r3
	EOR r4, r10, r4
	EOR r5, r2, r5
	EOR r6, r9, r6
	EOR r7, r12, r7
	BIC r1, r6, r5, ROR #21
	EOR r1, r1, r4, ROR #21
	STR.W r1, [r0, #48]
	BIC r1, r7, r6, ROR #28
	EOR r1, r1, r5, ROR #17
	STR.W r1, [r0, #100]
	BIC r1, r3, r7, ROR #25
	EOR r1, r1, r6, ROR #21
	STR.W r1, [r0, #148]
	BIC r1, r4, r3, ROR #22
	EOR r1, r1, r7, ROR #15
	STR.W r1, [r0, #192]
	BIC r5, r5, r4
	LDR r1, [sp, #20]
	LDR r4, [r1, #0]
	EOR r3, r3, r5, ROR #10
	EOR r1, r4, r3
	LDR.W r2, [sp, #16]
	LDR.W r3, [r0, #28]
	LDR.W r4, [r0, #76]
	LDR.W r5, [r0, #80]
	LDR.W r6, [r0, #128]
	LDR.W r7, [r0, #176]
	STR.W r1, [r0, #0]
	EOR r3, r9, r3
	EOR r4, lr, r4
	EOR r5, r8, r5
	EOR r6, r10, r6
	EOR r7, r2, r7
	BIC r1, r5, r4, ROR #23
	EOR r1, r1, r3, ROR #19
	STR.W r1, [r0, #80]
	BIC r1, r6, r5, ROR #21
	EOR r1, r1, r4, ROR #12
	STR.W r1, [r0, #128]
	BIC r1, r7, r6, ROR #8
	EOR r1, r1, r5, ROR #29
	STR.W r1, [r0, #176]
	BIC r1, r3, r7, ROR #16
	EOR r1, r1, r6, ROR #24
	STR.W r1, [r0, #28]
	BIC r1, r4, r3, ROR #28
	EOR r1, r1, r7, ROR #12
	LDR.W r3, [r0, #12]
	LDR.W r4, [r0, #56]
	LDR.W r5, [r0, #108]
	LDR.W r6, [r0, #152]
	LDR.W r7, [r0, #160]
	STR.W r1, [r0, #76]
	EOR r3, r11, r3
	EOR r4, r2, r4
	EOR r5, r9, r5
	EOR r6, r12, r6
	EOR r7, r8, r7
	BIC r1, r5, r4, ROR #10
	EOR r1, r1, r3, ROR #12
	STR.W r1, [r0, #160]
	BIC r1, r6, r5, ROR #23
	EOR r1, r1, r4, ROR #1
	STR.W r1, [r0, #12]
	BIC r1, r7, r6, ROR #5
	EOR r1, r1, r5, ROR #28
	STR.W r1, [r0, #56]
	BIC r1, r3, r7, ROR #24
	EOR r1, r1, r6, ROR #29
	STR.W r1, [r0, #108]
	BIC r1, r4, r3, ROR #2
	EOR r1, r1, r7, ROR #26
	LDR r8, [sp, #4]
	LDR.W r3, [r0, #32]
	LDR.W r4, [r0, #44]
	LDR.W r5, [r0, #92]
	LDR.W r6, [r0, #136]
	LDR.W r7, [r0, #188]
	STR.W r1, [r0, #152]
	EOR r3, r12, r3
	EOR r4, r8, r4
	EOR r5, r11, r5
	EOR r6, r2, r6
	EOR r7, r9, r7
	BIC r1, r5, r4, ROR #19
	EOR r1, r1, r3, ROR #24
	STR.W r1, [r0, #44]
	BIC r1, r6, r5, ROR #2
	EOR r1, r1, r4, ROR #21
	STR.W r1, [r0, #92]
	BIC r1, r7, r6, ROR #21
	EOR r1, r1, r5, ROR #23
	STR.W r1, [r0, #136]
	BIC r1, r3, r7, ROR #17
	EOR r1, r1, r6, ROR #6
	STR.W r1, [r0, #188]
	BIC r1, r4, r3, ROR #5
	EOR r1, r1, r7, ROR #22
	LDR.W r3, [r0, #16]
	LDR.W r4, [r0, #68]
	LDR.W r5, [r0, #116]
	LDR.W r6, [r0, #124]
	LDR.W r7, [r0, #168]
	STR.W r1, [r0, #32]
	EOR r3, r2, r3
	EOR r4, r9, r4
	EOR r5, lr, r5
	EOR r6, r8, r6
	EOR r7, r10, r7
	BIC r1, r5, r4, ROR #24
	EOR r1, r1, r3, ROR #21
	STR.W r1, [r0, #124]
	BIC r1, r6, r5, ROR #1
	EOR r1, r1, r4, ROR #25
	STR.W r1, [r0, #168]
	BIC r1, r7, r6, ROR #12
	EOR r1, r1, r5, ROR #13
	STR.W r1, [r0, #16]
	BIC r1, r3, r7, ROR #30
	EOR r1, r1, r6, ROR #10
	STR.W r1, [r0, #68]
	BIC r1, r4, r3, ROR #29
	EOR r1, r1, r7, ROR #27
	LDR r9, [sp, #8]
	LDR.W r3, [r0, #4]
	LDR r4, [r0, #52]
	LDR r5, [r0, #96]
	LDR r6, [r0, #144]
	LDR r7, [r0, #196]
	STR.W r1, [r0, #116]
	EOR r3, r8, r3
	EOR r4, r11, r4
	EOR r5, r2, r5
	EOR r6, r9, r6
	EOR r7, lr, r7
	BIC r1, r6, r5, ROR #21
	EOR r1, r1, r4, ROR #20
	STR.W r1, [r0, #52]
	BIC r1, r7, r6, ROR #29
	EOR r1, r1, r5, ROR #18
	STR.W r1, [r0, #96]
	BIC r1, r3, r7, ROR #25
	EOR r1, r1, r6, ROR #22
	STR.W r1, [r0, #144]
	BIC r1, r4, r3, ROR #22
	EOR r1, r1, r7, ROR #15
	STR.W r1, [r0, #196]
	BIC r5, r5, r4, ROR #31
	LDR r1, [sp, #20]
	LDR r4, [r1, #4]
	EOR r3, r3, r5, ROR #11
	EOR lr, r4, r3
	LDR.W r3, [r0, #192]
	LDR.W r1, [r0, #72]
	LDR.W r5, [r0, #152]
	LDR r11, [r0, #36]
	LDR r12, [r0, #116]
	STR.W lr, [r0, #4]
	EOR r3, r3, r1, ROR #12
	EOR r3, r3, r5, ROR #19
	EOR r3, r3, r11, ROR #4
	EOR r3, r3, r12, ROR #26
	LDR.W r7, [r0, #52]
	LDR.W r1, [r0, #128]
	LDR.W r5, [r0, #8]
	LDR r11, [r0, #92]
	LDR r12, [r0, #172]
	EOR r7, r7, r1, ROR #20
	EOR r7, r7, r5, ROR #6
	EOR r7, r7, r11, ROR #3
	EOR r7, r7, r12, ROR #22
	ROR r3, r3, #10
	EOR r6, r3, r7, ROR #21
	LDR.W r4, [r0, #96]
	LDR.W r1, [r0, #176]
	LDR.W r5, [r0, #60]
	LDR r11, [r0, #136]
	LDR r12, [r0, #20]
	STR.W r6, [sp, #0]
	EOR r4, r4, r1, ROR #9
	EOR r4, r4, r5, ROR #30
	EOR r4, r4, r11, ROR #11
	EOR r4, r4, r12, ROR #6
	EOR r6, r3, r4, ROR #25
	LDR.W r3, [r0, #148]
	LDR.W r1, [r0, #24]
	LDR.W r5, [r0, #108]
	LDR r11, [r0, #184]
	LDR r12, [r0, #68]
	STR.W r6, [sp, #12]
	EOR r3, r3, r1, ROR #18
	EOR r3, r3, r5, ROR #31
	EOR r3, r3, r11, ROR #18
	EOR r3, r3, r12, ROR #1
	EOR r2, r3, r7, ROR #22
	LDR.W r7, [r0, #0]
	LDR.W r1, [r0, #84]
	LDR.W r5, [r0, #160]
	LDR r11, [r0, #40]
	LDR r12, [r0, #124]
	EOR r7, r7, r1, ROR #30
	EOR r7, r7, r5, ROR #19
	EOR r7, r7, r11, ROR #27
	EOR r7, r7, r12, ROR #12
	EOR r10, r7, r4, ROR #24
	LDR.W r4, [r0, #144]
	LDR.W r1, [r0, #28]
	LDR.W r5, [r0, #104]
	LDR r11, [r0, #188]
	LDR r12, [r0, #64]
	EOR r4, r4, r1, ROR #18
	EOR r4, r4, r5
	EOR r4, r4, r11, ROR #19
	EOR r4, r4, r12, ROR #1
	EOR lr, r4, r7
	LDR.W r7, [r0, #48]
	LDR.W r1, [r0, #132]
	LDR.W r5, [r0, #12]
	LDR r11, [r0, #88]
	LDR r12, [r0, #168]
	EOR r7, r7, r1, ROR #20
	EOR r7, r7, r5, ROR #7
	EOR r7, r7, r11, ROR #3
	EOR r7, r7, r12, ROR #22
	ROR r7, r7, #21
	EOR r6, r7, r4, ROR #31
	LDR.W r4, [r0, #196]
	LDR.W r1, [r0, #76]
	LDR.W r5, [r0, #156]
	LDR r11, [r0, #32]
	LDR r12, [r0, #112]
	STR.W r6, [sp, #16]
	EOR r4, r4, r1, ROR #12
	EOR r4, r4, r5, ROR #19
	EOR r4, r4, r11, ROR #4
	EOR r4, r4, r12, ROR #27
	EOR r8, r7, r4, ROR #10
	LDR.W r7, [r0, #100]
	LDR.W r1, [r0, #180]
	LDR.W r5, [r0, #56]
	LDR r11, [r0, #140]
	LDR r12, [r0, #16]
	STR.W r8, [sp, #4]
	EOR r7, r7, r1, ROR #8
	EOR r7, r7, r5, ROR #30
	EOR r7, r7, r11, ROR #11
	EOR r7, r7, r12, ROR #6
	ROR r7, r7, #25
	EOR r9, r7, r4, ROR #9
	LDR.W r4, [r0, #4]
	LDR.W r1, [r0, #80]
	LDR.W r5, [r0, #164]
	LDR r11, [r0, #44]
	LDR r12, [r0, #120]
	STR.W r9, [sp, #8]
	EOR r4, r4, r1, ROR #31
	EOR r4, r4, r5, ROR #20
	EOR r4, r4, r11, ROR #27
	EOR r4, r4, r12, ROR #13
	EOR r11, r4, r7
	EOR r12, r3, r4, ROR #31
	LDR.W r3, [r0, #148]
	LDR.W r4, [r0, #72]
	LDR.W r5, [r0, #164]
	LDR.W r6, [r0, #92]
	LDR.W r7, [r0, #20]
	STR.W r1, [r0, #164]
	EOR r3, r9, r3
	EOR r4, r12, r4, ROR #22
	EOR r5, r8, r5, ROR #20
	EOR r6, r11, r6, ROR #25
	EOR r7, r2, r7, ROR #31
	BIC r1, r5, r4, ROR #24
	EOR r1, r1, r3, ROR #20
	STR.W r1, [r0, #164]
	BIC r1, r6, r5, ROR #21
	EOR r1, r1, r4, ROR #13
	STR.W r1, [r0, #92]
	BIC r1, r7, r6, ROR #8
	EOR r1, r1, r5, ROR #29
	STR.W r1, [r0, #20]
	BIC r1, r3, r7, ROR #15
	EOR r1, r1, r6, ROR #23
	STR.W r1, [r0, #148]
	BIC r1, r4, r3, ROR #28
	EOR r1, r1, r7, ROR #11
	LDR.W r3, [r0, #48]
	LDR.W r4, [r0, #176]
	LDR.W r5, [r0, #108]
	LDR.W r6, [r0, #32]
	LDR.W r7, [r0, #120]
	STR.W r1, [r0, #72]
	EOR r3, r10, r3, ROR #21
	EOR r4, r2, r4, ROR #2
	EOR r5, r9, r5, ROR #31
	EOR r6, lr, r6, ROR #14
	EOR r7, r8, r7, ROR #13
	BIC r1, r5, r4, ROR #9
	EOR r1, r1, r3, ROR #12
	STR.W r1, [r0, #120]
	BIC r1, r6, r5, ROR #24
	EOR r1, r1, r4, ROR #1
	STR.W r1, [r0, #48]
	BIC r1, r7, r6, ROR #5
	EOR r1, r1, r5, ROR #29
	STR.W r1, [r0, #176]
	BIC r1, r3, r7, ROR #23
	EOR r1, r1, r6, ROR #28
	STR.W r1, [r0, #108]
	BIC r1, r4, r3, ROR #3
	EOR r1, r1, r7, ROR #26
	LDR r8, [sp, #0]
	LDR.W r3, [r0, #196]
	LDR.W r4, [r0, #84]
	LDR.W r5, [r0, #12]
	LDR.W r6, [r0, #136]
	LDR.W r7, [r0, #68]
	STR.W r1, [r0, #32]
	EOR r3, lr, r3, ROR #10
	EOR r4, r8, r4, ROR #30
	EOR r5, r10, r5, ROR #28
	EOR r6, r2, r6, ROR #4
	EOR r7, r9, r7, ROR #1
	BIC r1, r5, r4, ROR #19
	EOR r1, r1, r3, ROR #23
	STR.W r1, [r0, #84]
	BIC r1, r6, r5, ROR #3
	EOR r1, r1, r4, ROR #22
	STR.W r1, [r0, #12]
	BIC r1, r7, r6, ROR #20
	EOR r1, r1, r5, ROR #23
	STR.W r1, [r0, #136]
	BIC r1, r3, r7, ROR #18
	EOR r1, r1, r6, ROR #6
	STR.W r1, [r0, #68]
	BIC r1, r4, r3, ROR #4
	EOR r1, r1, r7, ROR #22
	LDR.W r3, [r0, #96]
	LDR.W r4, [r0, #24]
	LDR.W r5, [r0, #152]
	LDR.W r6, [r0, #40]
	LDR.W r7, [r0, #172]
	STR.W r1, [r0, #196]
	EOR r3, r2, r3, ROR #25
	EOR r4, r9, r4, ROR #18
	EOR r5, r12, r5, ROR #29
	EOR r6, r8, r6, ROR #27
	EOR r7, r11, r7, ROR #12
	BIC r1, r5, r4, ROR #24
	EOR r1, r1, r3, ROR #20
	STR.W r1, [r0, #40]
	BIC r1, r6, r5, ROR #1
	EOR r1, r1, r4, ROR #25
	STR.W r1, [r0, #172]
	BIC r1, r7, r6, ROR #13
	EOR r1, r1, r5, ROR #14
	STR.W r1, [r0, #96]
	BIC r1, r3, r7, ROR #30
	EOR r1, r1, r6, ROR #11
	STR.W r1, [r0, #24]
	BIC r1, r4, r3, ROR #28
	EOR r1, r1, r7, ROR #26
	LDR r9, [sp, #12]
	LDR.W r3, [r0, #0]
	LDR r4, [r0, #132]
	LDR r5, [r0, #60]
	LDR r6, [r0, #188]
	LDR r7, [r0, #116]
	STR.W r1, [r0, #152]
	EOR r3, r8, r3
	EOR r4, r10, r4, ROR #9
	EOR r5, r2, r5, ROR #23
	EOR r6, r9, r6, ROR #19
	EOR r7, r12, r7, ROR #4
	BIC r1, r6, r5, ROR #21
	EOR r1, r1, r4, ROR #21
	STR.W r1, [r0, #132]
	BIC r1, r7, r6, ROR #28
	EOR r1, r1, r5, ROR #17
	STR.W r1, [r0, #60]
	BIC r1, r3, r7, ROR #25
	EOR r1, r1, r6, ROR #21
	STR.W r1, [r0, #188]
	BIC r1, r4, r3, ROR #22
	EOR r1, r1, r7, ROR #15
	STR.W r1, [r0, #116]
	BIC r5, r5, r4
	LDR r1, [sp, #20]
	LDR r4, [r1, #8]
	EOR r3, r3, r5, ROR #10
	EOR r1, r4, r3
	LDR.W r2, [sp, #16]
	LDR.W r3, [r0, #144]
	LDR.W r4, [r0, #76]
	LDR.W r5, [r0, #160]
	LDR.W r6, [r0, #88]
	LDR.W r7, [r0, #16]
	STR.W r1, [r0, #0]
	EOR r3, r9, r3
	EOR r4, lr, r4, ROR #22
	EOR r5, r8, r5, ROR #19
	EOR r6, r10, r6, ROR #24
	EOR r7, r2, r7, ROR #31
	BIC r1, r5, r4, ROR #23
	EOR r1, r1, r3, ROR #19
	STR.W r1, [r0, #160]
	BIC r1, r6, r5, ROR #21
	EOR r1, r1, r4, ROR #12
	STR.W r1, [r0, #88]
	BIC r1, r7, r6, ROR #8
	EOR r1, r1, r5, ROR #29
	STR.W r1, [r0, #16]
	BIC r1, r3, r7, ROR #16
	EOR r1, r1, r6, ROR #24
	STR.W r1, [r0, #144]
	BIC r1, r4, r3, ROR #28
	EOR r1, r1, r7, ROR #12
	LDR.W r3, [r0, #52]
	LDR.W r4, [r0, #180]
	LDR.W r5, [r0, #104]
	LDR.W r6, [r0, #36]
	LDR.W r7, [r0, #124]
	STR.W r1, [r0, #76]
	EOR r3, r11, r3, ROR #22
	EOR r4, r2, r4, ROR #1
	EOR r5, r9, r5
	EOR r6, r12, r6, ROR #14
	EOR r7, r8, r7, ROR #12
	BIC r1, r5, r4, ROR #10
	EOR r1, r1, r3, ROR #12
	STR.W r1, [r0, #124]
	BIC r1, r6, r5, ROR #23
	EOR r1, r1, r4, ROR #1
	STR.W r1, [r0, #52]
	BIC r1, r7, r6, ROR #5
	EOR r1, r1, r5, ROR #28
	STR.W r1, [r0, #180]
	BIC r1, r3, r7, ROR #24
	EOR r1, r1, r6, ROR #29
	STR.W r1, [r0, #104]
	BIC r1, r4, r3, ROR #2
	EOR r1, r1, r7, ROR #26
	LDR r8, [sp, #4]
	LDR.W r3, [r0, #192]
	LDR.W r4, [r0, #80]
	LDR.W r5, [r0, #8]
	LDR.W r6, [r0, #140]
	LDR.W r7, [r0, #64]
	STR.W r1, [r0, #36]
	EOR r3, r12, r3, ROR #10
	EOR r4, r8, r4, ROR #31
	EOR r5, r11, r5, ROR #28
	EOR r6, r2, r6, ROR #4
	EOR r7, r9, r7, ROR #1
	BIC r1, r5, r4, ROR #19
	EOR r1, r1, r3, ROR #24
	STR.W r1, [r0, #80]
	BIC r1, r6, r5, ROR #2
	EOR r1, r1, r4, ROR #21
	STR.W r1, [r0, #8]
	BIC r1, r7, r6, ROR #21
	EOR r1, r1, r5, ROR #23
	STR.W r1, [r0, #140]
	BIC r1, r3, r7, ROR #17
	EOR r1, r1, r6, ROR #6
	STR.W r1, [r0, #64]
	BIC r1, r4, r3, ROR #5
	EOR r1, r1, r7, ROR #22
	LDR.W r3, [r0, #100]
	LDR.W r4, [r0, #28]
	LDR.W r5, [r0, #156]
	LDR.W r6, [r0, #44]
	LDR.W r7, [r0, #168]
	STR.W r1, [r0, #192]
	EOR r3, r2, r3, ROR #25
	EOR r4, r9, r4, ROR #18
	EOR r5, lr, r5, ROR #29
	EOR r6, r8, r6, ROR #27
	EOR r7, r10, r7, ROR #11
	BIC r1, r5, r4, ROR #24
	EOR r1, r1, r3, ROR #21
	STR.W r1, [r0, #44]
	BIC r1, r6, r5, ROR #1
	EOR r1, r1, r4, ROR #25
	STR.W r1, [r0, #168]
	BIC r1, r7, r6, ROR #12
	EOR r1, r1, r5, ROR #13
	STR.W r1, [r0, #100]
	BIC r1, r3, r7, ROR #30
	EOR r1, r1, r6, ROR #10
	STR.W r1, [r0, #28]
	BIC r1, r4, r3, ROR #29
	EOR r1, r1, r7, ROR #27
	LDR r9, [sp, #8]
	LDR.W r3, [r0, #4]
	LDR r4, [r0, #128]
	LDR r5, [r0, #56]
	LDR r6, [r0, #184]
	LDR r7, [r0, #112]
	STR.W r1, [r0, #156]
	EOR r3, r8, r3
	EOR r4, r11, r4, ROR #10
	EOR r5, r2, r5, ROR #23
	EOR r6, r9, r6, ROR #18
	EOR r7, lr, r7, ROR #5
	BIC r1, r6, r5, ROR #21
	EOR r1, r1, r4, ROR #20
	STR.W r1, [r0, #128]
	BIC r1, r7, r6, ROR #29
	EOR r1, r1, r5, ROR #18
	STR.W r1, [r0, #56]
	BIC r1, r3, r7, ROR #25
	EOR r1, r1, r6, ROR #22
	STR.W r1, [r0, #184]
	BIC r1, r4, r3, ROR #22
	EOR r1, r1, r7, ROR #15
	STR.W r1, [r0, #112]
	BIC r5, r5, r4, ROR #31
	LDR r1, [sp, #20]
	LDR r4, [r1, #12]
	EOR r3, r3, r5, ROR #11
	EOR lr, r4, r3
	LDR.W r3, [r0, #116]
	LDR.W r1, [r0, #72]
	LDR.W r5, [r0, #36]
	LDR r11, [r0, #196]
	LDR r12, [r0, #156]
	STR.W lr, [r0, #4]
	EOR r3, r3, r1, ROR #12
	EOR r3, r3, r5, ROR #19
	EOR r3, r3, r11, ROR #4
	EOR r3, r3, r12, ROR #26
	LDR.W r7, [r0, #128]
	LDR.W r1, [r0, #88]
	LDR.W r5, [r0, #48]
	LDR r11, [r0, #8]
	LDR r12, [r0, #172]
	EOR r7, r7, r1, ROR #20
	EOR r7, r7, r5, ROR #6
	EOR r7, r7, r11, ROR #3
	EOR r7, r7, r12, ROR #22
	ROR r3, r3, #10
	EOR r6, r3, r7, ROR #21
	LDR.W r4, [r0, #56]
	LDR.W r1, [r0, #16]
	LDR.W r5, [r0, #176]
	LDR r11, [r0, #140]
	LDR r12, [r0, #96]
	STR.W r6, [sp, #0]
	EOR r4, r4, r1, ROR #9
	EOR r4, r4, r5, ROR #30
	EOR r4, r4, r11, ROR #11
	EOR r4, r4, r12, ROR #6
	EOR r6, r3, r4, ROR #25
	LDR.W r3, [r0, #188]
	LDR.W r1, [r0, #148]
	LDR.W r5, [r0, #104]
	LDR r11, [r0, #68]
	LDR r12, [r0, #28]
	STR.W r6, [sp, #12]
	EOR r3, r3, r1, ROR #18
	EOR r3, r3, r5, ROR #31
	EOR r3, r3, r11, ROR #18
	EOR r3, r3, r12, ROR #1
	EOR r2, r3, r7, ROR #22
	LDR.W r7, [r0, #0]
	LDR.W r1, [r0, #164]
	LDR.W r5, [r0, #124]
	LDR r11, [r0, #84]
	LDR r12, [r0, #44]
	EOR r7, r7, r1, ROR #30
	EOR r7, r7, r5, ROR #19
	EOR r7, r7, r11, ROR #27
	EOR r7, r7, r12, ROR #12
	EOR r10, r7, r4, ROR #24
	LDR.W r4, [r0, #184]
	LDR.W r1, [r0, #144]
	LDR.W r5, [r0, #108]
	LDR r11, [r0, #64]
	LDR r12, [r0, #24]
	EOR r4, r4, r1, ROR #18
	EOR r4, r4, r5
	EOR r4, r4, r11, ROR #19
	EOR r4, r4, r12, ROR #1
	EOR lr, r4, r7
	LDR.W r7, [r0, #132]
	LDR.W r1, [r0, #92]
	LDR.W r5, [r0, #52]
	LDR r11, [r0, #12]
	LDR r12, [r0, #168]
	EOR r7, r7, r1, ROR #20
	EOR r7, r7, r5, ROR #7
	EOR r7, r7, r11, ROR #3
	EOR r7, r7, r12, ROR #22
	ROR r7, r7, #21
	EOR r6, r7, r4, ROR #31
	LDR.W r4, [r0, #112]
	LDR.W r1, [r0, #76]
	LDR.W r5, [r0, #32]
	LDR r11, [r0, #192]
	LDR r12, [r0, #152]
	STR.W r6, [sp, #16]
	EOR r4, r4, r1, ROR #12
	EOR r4, r4, r5, ROR #19
	EOR r4, r4, r11, ROR #4
	EOR r4, r4, r12, ROR #27
	EOR r8, r7, r4, ROR #10
	LDR.W r7, [r0, #60]
	LDR.W r1, [r0, #20]
	LDR.W r5, [r0, #180]
	LDR r11, [r0, #136]
	LDR r12, [r0, #100]
	STR.W r8, [sp, #4]
	EOR r7, r7, r1, ROR #8
	EOR r7, r7, r5, ROR #30
	EOR r7, r7, r11, ROR #11
	EOR r7, r7, r12, ROR #6
	ROR r7, r7, #25
	EOR r9, r7, r4, ROR #9
	LDR.W r4, [r0, #4]
	LDR.W r1, [r0, #160]
	LDR.W r5, [r0, #120]
	LDR r11, [r0, #80]
	LDR r12, [r0, #40]
	STR.W r9, [sp, #8]
	EOR r4, r4, r1, ROR #31
	EOR r4, r4, r5, ROR #20
	EOR r4, r4, r11, ROR #27
	EOR r4, r4, r12, ROR #13
	EOR r11, r4, r7
	EOR r12, r3, r4, ROR #31
	LDR.W r3, [r0, #188]
	LDR.W r4, [r0, #72]
	LDR.W r5, [r0, #120]
	LDR.W r6, [r0, #8]
	LDR.W r7, [r0, #96]
	STR.W r1, [r0, #120]
	EOR r3, r9, r3
	EOR r4, r12, r4, ROR #22
	EOR r5, r8, r5, ROR #20
	EOR r6, r11, r6, ROR #25
	EOR r7, r2, r7, ROR #31
	BIC r1, r5, r4, ROR #24
	EOR r1, r1, r3, ROR #20
	STR.W r1, [r0, #120]
	BIC r1, r6, r5, ROR #21
	EOR r1, r1, r4, ROR #13
	STR.W r1, [r0, #8]
	BIC r1, r7, r6, ROR #8
	EOR r1, r1, r5, ROR #29
	STR.W r1, [r0, #96]
	BIC r1, r3, r7, ROR #15
	EOR r1, r1, r6, ROR #23
	STR.W r1, [r0, #188]
	BIC r1, r4, r3, ROR #28
	EOR r1, r1, r7, ROR #11
	LDR.W r3, [r0, #132]
	LDR.W r4, [r0, #16]
	LDR.W r5, [r0, #104]
	LDR.W r6, [r0, #192]
	LDR.W r7, [r0, #40]
	STR.W r1, [r0, #72]
	EOR r3, r10, r3, ROR #21
	EOR r4, r2, r4, ROR #2
	EOR r5, r9, r5, ROR #31
	EOR r6, lr, r6, ROR #14
	EOR r7, r8, r7, ROR #13
	BIC r1, r5, r4, ROR #9
	EOR r1, r1, r3, ROR #12
	STR.W r1, [r0, #40]
	BIC r1, r6, r5, ROR #24
	EOR r1, r1, r4, ROR #1
	STR.W r1, [r0, #132]
	BIC r1, r7, r6, ROR #5
	EOR r1, r1, r5, ROR #29
	STR.W r1, [r0, #16]
	BIC r1, r3, r7, ROR #23
	EOR r1, r1, r6, ROR #28
	STR.W r1, [r0, #104]
	BIC r1, r4, r3, ROR #3
	EOR r1, r1, r7, ROR #26
	LDR r8, [sp, #0]
	LDR.W r3, [r0, #112]
	LDR.W r4, [r0, #164]
	LDR.W r5, [r0, #52]
	LDR.W r6, [r0, #140]
	LDR.W r7, [r0, #28]
	STR.W r1, [r0, #192]
	EOR r3, lr, r3, ROR #10
	EOR r4, r8, r4, ROR #30
	EOR r5, r10, r5, ROR #28
	EOR r6, r2, r6, ROR #4
	EOR r7, r9, r7, ROR #1
	BIC r1, r5, r4, ROR #19
	EOR r1, r1, r3, ROR #23
	STR.W r1, [r0, #164]
	BIC r1, r6, r5, ROR #3
	EOR r1, r1, r4, ROR #22
	STR.W r1, [r0, #52]
	BIC r1, r7, r6, ROR #20
	EOR r1, r1, r5, ROR #23
	STR.W r1, [r0, #140]
	BIC r1, r3, r7, ROR #18
	EOR r1, r1, r6, ROR #6
	STR.W r1, [r0, #28]
	BIC r1, r4, r3, ROR #4
	EOR r1, r1, r7, ROR #22
	LDR.W r3, [r0, #56]
	LDR.W r4, [r0, #148]
	LDR.W r5, [r0, #36]
	LDR.W r6, [r0, #84]
	LDR.W r7, [r0, #172]
	STR.W r1, [r0, #112]
	EOR r3, r2, r3, ROR #25
	EOR r4, r9, r4, ROR #18
	EOR r5, r12, r5, ROR #29
	EOR r6, r8, r6, ROR #27
	EOR r7, r11, r7, ROR #12
	BIC r1, r5, r4, ROR #24
	EOR r1, r1, r3, ROR #20
	STR.W r1, [r0, #84]
	BIC r1, r6, r5, ROR #1
	EOR r1, r1, r4, ROR #25
	STR.W r1, [r0, #172]
	BIC r1, r7, r6, ROR #13
	EOR r1, r1, r5, ROR #14
	STR.W r1, [r0, #56]
	BIC r1, r3, r7, ROR #30
	EOR r1, r1, r6, ROR #11
	STR.W r1, [r0, #148]
	BIC r1, r4, r3, ROR #28
	EOR r1, r1, r7, ROR #26
	LDR r9, [sp, #12]
	LDR.W r3, [r0, #0]
	LDR r4, [r0, #92]
	LDR r5, [r0, #176]
	LDR r6, [r0, #64]
	LDR r7, [r0, #156]
	STR.W r1, [r0, #36]
	EOR r3, r8, r3
	EOR r4, r10, r4, ROR #9
	EOR r5, r2, r5, ROR #23
	EOR r6, r9, r6, ROR #19
	EOR r7, r12, r7, ROR #4
	BIC r1, r6, r5, ROR #21
	EOR r1, r1, r4, ROR #21
	STR.W r1, [r0, #92]
	BIC r1, r7, r6, ROR #28
	EOR r1, r1, r5, ROR #17
	STR.W r1, [r0, #176]
	BIC r1, r3, r7, ROR #25
	EOR r1, r1, r6, ROR #21
	STR.W r1, [r0, #64]
	BIC r1, r4, r3, ROR #22
	EOR r1, r1, r7, ROR #15
	STR.W r1, [r0, #156]
	BIC r5, r5, r4
	LDR r1, [sp, #20]
	LDR r4, [r1, #16]
	EOR r3, r3, r5, ROR #10
	EOR r1, r4, r3
	LDR.W r2, [sp, #16]
	LDR.W r3, [r0, #184]
	LDR.W r4, [r0, #76]
	LDR.W r5, [r0, #124]
	LDR.W r6, [r0, #12]
	LDR.W r7, [r0, #100]
	STR.W r1, [r0, #0]
	EOR r3, r9, r3
	EOR r4, lr, r4, ROR #22
	EOR r5, r8, r5, ROR #19
	EOR r6, r10, r6, ROR #24
	EOR r7, r2, r7, ROR #31
	BIC r1, r5, r4, ROR #23
	EOR r1, r1, r3, ROR #19
	STR.W r1, [r0, #124]
	BIC r1, r6, r5, ROR #21
	EOR r1, r1, r4, ROR #12
	STR.W r1, [r0, #12]
	BIC r1, r7, r6, ROR #8
	EOR r1, r1, r5, ROR #29
	STR.W r1, [r0, #100]
	BIC r1, r3, r7, ROR #16
	EOR r1, r1, r6, ROR #24
	STR.W r1, [r0, #184]
	BIC r1, r4, r3, ROR #28
	EOR r1, r1, r7, ROR #12
	LDR.W r3, [r0, #128]
	LDR.W r4, [r0, #20]
	LDR.W r5, [r0, #108]
	LDR.W r6, [r0, #196]
	LDR.W r7, [r0, #44]
	STR.W r1, [r0, #76]
	EOR r3, r11, r3, ROR #22
	EOR r4, r2, r4, ROR #1
	EOR r5, r9, r5
	EOR r6, r12, r6, ROR #14
	EOR r7, r8, r7, ROR #12
	BIC r1, r5, r4, ROR #10
	EOR r1, r1, r3, ROR #12
	STR.W r1, [r0, #44]
	BIC r1, r6, r5, ROR #23
	EOR r1, r1, r4, ROR #1
	STR.W r1, [r0, #128]
	BIC r1, r7, r6, ROR #5
	EOR r1, r1, r5, ROR #28
	STR.W r1, [r0, #20]
	BIC r1, r3, r7, ROR #24
	EOR r1, r1, r6, ROR #29
	STR.W r1, [r0, #108]
	BIC r1, r4, r3, ROR #2
	EOR r1, r1, r7, ROR #26
	LDR r8, [sp, #4]
	LDR.W r3, [r0, #116]
	LDR.W r4, [r0, #160]
	LDR.W r5, [r0, #48]
	LDR.W r6, [r0, #136]
	LDR.W r7, [r0, #24]
	STR.W r1, [r0, #196]
	EOR r3, r12, r3, ROR #10
	EOR r4, r8, r4, ROR #31
	EOR r5, r11, r5, ROR #28
	EOR r6, r2, r6, ROR #4
	EOR r7, r9, r7, ROR #1
	BIC r1, r5, r4, ROR #19
	EOR r1, r1, r3, ROR #24
	STR.W r1, [r0, #160]
	BIC r1, r6, r5, ROR #2
	EOR r1, r1, r4, ROR #21
	STR.W r1, [r0, #48]
	BIC r1, r7, r6, ROR #21
	EOR r1, r1, r5, ROR #23
	STR.W r1, [r0, #136]
	BIC r1, r3, r7, ROR #17
	EOR r1, r1, r6, ROR #6
	STR.W r1, [r0, #24]
	BIC r1, r4, r3, ROR #5
	EOR r1, r1, r7, ROR #22
	LDR.W r3, [r0, #60]
	LDR.W r4, [r0, #144]
	LDR.W r5, [r0, #32]
	LDR.W r6, [r0, #80]
	LDR.W r7, [r0, #168]
	STR.W r1, [r0, #116]
	EOR r3, r2, r3, ROR #25
	EOR r4, r9, r4, ROR #18
	EOR r5, lr, r5, ROR #29
	EOR r6, r8, r6, ROR #27
	EOR r7, r10, r7, ROR #11
	BIC r1, r5, r4, ROR #24
	EOR r1, r1, r3, ROR #21
	STR.W r1, [r0, #80]
	BIC r1, r6, r5, ROR #1
	EOR r1, r1, r4, ROR #25
	STR.W r1, [r0, #168]
	BIC r1, r7, r6, ROR #12
	EOR r1, r1, r5, ROR #13
	STR.W r1, [r0, #60]
	BIC r1, r3, r7, ROR #30
	EOR r1, r1, r6, ROR #10
	STR.W r1, [r0, #144]
	BIC r1, r4, r3, ROR #29
	EOR r1, r1, r7, ROR #27
	LDR r9, [sp, #8]
	LDR.W r3, [r0, #4]
	LDR r4, [r0, #88]
	LDR r5, [r0, #180]
	LDR r6, [r0, #68]
	LDR r7, [r0, #152]
	STR.W r1, [r0, #32]
	EOR r3, r8, r3
	EOR r4, r11, r4, ROR #10
	EOR r5, r2, r5, ROR #23
	EOR r6, r9, r6, ROR #18
	EOR r7, lr, r7, ROR #5
	BIC r1, r6, r5, ROR #21
	EOR r1, r1, r4, ROR #20
	STR.W r1, [r0, #88]
	BIC r1, r7, r6, ROR #29
	EOR r1, r1, r5, ROR #18
	STR.W r1, [r0, #180]
	BIC r1, r3, r7, ROR #25
	EOR r1, r1, r6, ROR #22
	STR.W r1, [r0, #68]
	BIC r1, r4, r3, ROR #22
	EOR r1, r1, r7, ROR #15
	STR.W r1, [r0, #152]
	BIC r5, r5, r4, ROR #31
	LDR r1, [sp, #20]
	LDR r4, [r1, #20]
	EOR r3, r3, r5, ROR #11
	EOR lr, r4, r3
	LDR.W r3, [r0, #156]
	LDR.W r1, [r0, #72]
	LDR.W r5, [r0, #196]
	LDR r11, [r0, #112]
	LDR r12, [r0, #32]
	STR.W lr, [r0, #4]
	EOR r3, r3, r1, ROR #12
	EOR r3, r3, r5, ROR #19
	EOR r3, r3, r11, ROR #4
	EOR r3, r3, r12, ROR #26
	LDR.W r7, [r0, #88]
	LDR.W r1, [r0, #12]
	LDR.W r5, [r0, #132]
	LDR r11, [r0, #48]
	LDR r12, [r0, #172]
	EOR r7, r7, r1, ROR #20
	EOR r7, r7, r5, ROR #6
	EOR r7, r7, r11, ROR #3
	EOR r7, r7, r12, ROR #22
	ROR r3, r3, #10
	EOR r6, r3, r7, ROR #21
	LDR.W r4, [r0, #180]
	LDR.W r1, [r0, #100]
	LDR.W r5, [r0, #16]
	LDR r11, [r0, #136]
	LDR r12, [r0, #56]
	STR.W r6, [sp, #0]
	EOR r4, r4, r1, ROR #9
	EOR r4, r4, r5, ROR #30
	EOR r4, r4, r11, ROR #11
	EOR r4, r4, r12, ROR #6
	EOR r6, r3, r4, ROR #25
	LDR.W r3, [r0, #64]
	LDR.W r1, [r0, #188]
	LDR.W r5, [r0, #108]
	LDR r11, [r0, #28]
	LDR r12, [r0, #144]
	STR.W r6, [sp, #12]
	EOR r3, r3, r1, ROR #18
	EOR r3, r3, r5, ROR #31
	EOR r3, r3, r11, ROR #18
	EOR r3, r3, r12, ROR #1
	EOR r2, r3, r7, ROR #22
	LDR.W r7, [r0, #0]
	LDR.W r1, [r0, #120]
	LDR.W r5, [r0, #44]
	LDR r11, [r0, #164]
	LDR r12, [r0, #80]
	EOR r7, r7, r1, ROR #30
	EOR r7, r7, r5, ROR #19
	EOR r7, r7, r11, ROR #27
	EOR r7, r7, r12, ROR #12
	EOR r10, r7, r4, ROR #24
	LDR.W r4, [r0, #68]
	LDR.W r1, [r0, #184]
	LDR.W r5, [r0, #104]
	LDR r11, [r0, #24]
	LDR r12, [r0, #148]
	EOR r4, r4, r1, ROR #18
	EOR r4, r4, r5
	EOR r4, r4, r11, ROR #19
	EOR r4, r4, r12, ROR #1
	EOR lr, r4, r7
	LDR.W r7, [r0, #92]
	LDR.W r1, [r0, #8]
	LDR.W r5, [r0, #128]
	LDR r11, [r0, #52]
	LDR r12, [r0, #168]
	EOR r7, r7, r1, ROR #20
	EOR r7, r7, r5, ROR #7
	EOR r7, r7, r11, ROR #3
	EOR r7, r7, r12, ROR #22
	ROR r7, r7, #21
	EOR r6, r7, r4, ROR #31
	LDR.W r4, [r0, #152]
	LDR.W r1, [r0, #76]
	LDR.W r5, [r0, #192]
	LDR r11, [r0, #116]
	LDR r12, [r0, #36]
	STR.W r6, [sp, #16]
	EOR r4, r4, r1, ROR #12
	EOR r4, r4, r5, ROR #19
	EOR r4, r4, r11, ROR #4
	EOR r4, r4, r12, ROR #27
	EOR r8, r7, r4, ROR #10
	LDR.W r7, [r0, #176]
	LDR.W r1, [r0, #96]
	LDR.W r5, [r0, #20]
	LDR r11, [r0, #140]
	LDR r12, [r0, #60]
	STR.W r8, [sp, #4]
	EOR r7, r7, r1, ROR #8
	EOR r7, r7, r5, ROR #30
	EOR r7, r7, r11, ROR #11
	EOR r7, r7, r12, ROR #6
	ROR r7, r7, #25
	EOR r9, r7, r4, ROR #9
	LDR.W r4, [r0, #4]
	LDR.W r1, [r0, #124]
	LDR.W r5, [r0, #40]
	LDR r11, [r0, #160]
	LDR r12, [r0, #84]
	STR.W r9, [sp, #8]
	EOR r4, r4, r1, ROR #31
	EOR r4, r4, r5, ROR #20
	EOR r4, r4, r11, ROR #27
	EOR r4, r4, r12, ROR #13
	EOR r11, r4, r7
	EOR r12, r3, r4, ROR #31
	LDR.W r3, [r0, #64]
	LDR.W r4, [r0, #72]
	LDR.W r5, [r0, #40]
	LDR.W r6, [r0, #48]
	LDR.W r7, [r0, #56]
	STR.W r1, [r0, #40]
	EOR r3, r9, r3
	EOR r4, r12, r4, ROR #22
	EOR r5, r8, r5, ROR #20
	EOR r6, r11, r6, ROR #25
	EOR r7, r2, r7, ROR #31
	BIC r1, r5, r4, ROR #24
	EOR r1, r1, r3, ROR #20
	ROR r1, r1, #30
	STR.W r1, [r0, #40]
	BIC r1, r6, r5, ROR #21
	EOR r1, r1, r4, ROR #13
	ROR r1, r1, #9
	STR.W r1, [r0, #48]
	BIC r1, r7, r6, ROR #8
	EOR r1, r1, r5, ROR #29
	ROR r1, r1, #1
	STR.W r1, [r0, #56]
	BIC r1, r3, r7, ROR #15
	EOR r1, r1, r6, ROR #23
	ROR r1, r1, #18
	STR.W r1, [r0, #64]
	BIC r1, r4, r3, ROR #28
	EOR r1, r1, r7, ROR #11
	ROR r1, r1, #22
	LDR.W r3, [r0, #92]
	LDR.W r4, [r0, #100]
	LDR.W r5, [r0, #108]
	LDR.W r6, [r0, #116]
	LDR.W r7, [r0, #84]
	STR.W r1, [r0, #72]
	EOR r3, r10, r3, ROR #21
	EOR r4, r2, r4, ROR #2
	EOR r5, r9, r5, ROR #31
	EOR r6, lr, r6, ROR #14
	EOR r7, r8, r7, ROR #13
	BIC r1, r5, r4, ROR #9
	EOR r1, r1, r3, ROR #12
	ROR r1, r1, #20
	STR.W r1, [r0, #84]
	BIC r1, r6, r5, ROR #24
	EOR r1, r1, r4, ROR #1
	ROR r1, r1, #28
	STR.W r1, [r0, #92]
	BIC r1, r7, r6, ROR #5
	EOR r1, r1, r5, ROR #29
	ROR r1, r1, #23
	STR.W r1, [r0, #100]
	BIC r1, r3, r7, ROR #23
	EOR r1, r1, r6, ROR #28
	STR.W r1, [r0, #108]
	BIC r1, r4, r3, ROR #3
	EOR r1, r1, r7, ROR #26
	ROR r1, r1, #29
	LDR r8, [sp, #0]
	LDR.W r3, [r0, #152]
	LDR.W r4, [r0, #120]
	LDR.W r5, [r0, #128]
	LDR.W r6, [r0, #136]
	LDR.W r7, [r0, #144]
	STR.W r1, [r0, #116]
	EOR r3, lr, r3, ROR #10
	EOR r4, r8, r4, ROR #30
	EOR r5, r10, r5, ROR #28
	EOR r6, r2, r6, ROR #4
	EOR r7, r9, r7, ROR #1
	BIC r1, r5, r4, ROR #19
	EOR r1, r1, r3, ROR #23
	ROR r1, r1, #27
	STR.W r1, [r0, #120]
	BIC r1, r6, r5, ROR #3
	EOR r1, r1, r4, ROR #22
	ROR r1, r1, #24
	STR.W r1, [r0, #128]
	BIC r1, r7, r6, ROR #20
	EOR r1, r1, r5, ROR #23
	ROR r1, r1, #4
	STR.W r1, [r0, #136]
	BIC r1, r3, r7, ROR #18
	EOR r1, r1, r6, ROR #6
	ROR r1, r1, #18
	STR.W r1, [r0, #144]
	BIC r1, r4, r3, ROR #4
	EOR r1, r1, r7, ROR #22
	ROR r1, r1, #14
	LDR.W r3, [r0, #180]
	LDR.W r4, [r0, #188]
	LDR.W r5, [r0, #196]
	LDR.W r6, [r0, #164]
	LDR.W r7, [r0, #172]
	STR.W r1, [r0, #152]
	EOR r3, r2, r3, ROR #25
	EOR r4, r9, r4, ROR #18
	EOR r5, r12, r5, ROR #29
	EOR r6, r8, r6, ROR #27
	EOR r7, r11, r7, ROR #12
	BIC r1, r5, r4, ROR #24
	EOR r1, r1, r3, ROR #20
	ROR r1, r1, #13
	STR.W r1, [r0, #164]
	BIC r1, r6, r5, ROR #1
	EOR r1, r1, r4, ROR #25
	ROR r1, r1, #12
	STR.W r1, [r0, #172]
	BIC r1, r7, r6, ROR #13
	EOR r1, r1, r5, ROR #14
	ROR r1, r1, #31
	STR.W r1, [r0, #180]
	BIC r1, r3, r7, ROR #30
	EOR r1, r1, r6, ROR #11
	ROR r1, r1, #1
	STR.W r1, [r0, #188]
	BIC r1, r4, r3, ROR #28
	EOR r1, r1, r7, ROR #26
	ROR r1, r1, #5
	LDR r9, [sp, #12]
	LDR.W r3, [r0, #0]
	LDR r4, [r0, #8]
	LDR r5, [r0, #16]
	LDR r6, [r0, #24]
	LDR r7, [r0, #32]
	STR.W r1, [r0, #196]
	EOR r3, r8, r3
	EOR r4, r10, r4, ROR #9
	EOR r5, r2, r5, ROR #23
	EOR r6, r9, r6, ROR #19
	EOR r7, r12, r7, ROR #4
	BIC r1, r6, r5, ROR #21
	EOR r1, r1, r4, ROR #21
	ROR r1, r1, #21
	STR.W r1, [r0, #8]
	BIC r1, r7, r6, ROR #28
	EOR r1, r1, r5, ROR #17
	ROR r1, r1, #25
	STR.W r1, [r0, #16]
	BIC r1, r3, r7, ROR #25
	EOR r1, r1, r6, ROR #21
	STR.W r1, [r0, #24]
	BIC r1, r4, r3, ROR #22
	EOR r1, r1, r7, ROR #15
	ROR r1, r1, #10
	STR.W r1, [r0, #32]
	BIC r5, r5, r4
	LDR r1, [sp, #20]
	LDR r4, [r1, #24]
	EOR r3, r3, r5, ROR #10
	EOR r1, r4, r3
	LDR.W r2, [sp, #16]
	LDR.W r3, [r0, #68]
	LDR.W r4, [r0, #76]
	LDR.W r5, [r0, #44]
	LDR.W r6, [r0, #52]
	LDR.W r7, [r0, #60]
	STR.W r1, [r0, #0]
	EOR r3, r9, r3
	EOR r4, lr, r4, ROR #22
	EOR r5, r8, r5, ROR #19
	EOR r6, r10, r6, ROR #24
	EOR r7, r2, r7, ROR #31
	BIC r1, r5, r4, ROR #23
	EOR r1, r1, r3, ROR #19
	ROR r1, r1, #31
	STR.W r1, [r0, #44]
	BIC r1, r6, r5, ROR #21
	EOR r1, r1, r4, ROR #12
	ROR r1, r1, #10
	STR.W r1, [r0, #52]
	BIC r1, r7, r6, ROR #8
	EOR r1, r1, r5, ROR #29
	ROR r1, r1, #2
	STR.W r1, [r0, #60]
	BIC r1, r3, r7, ROR #16
	EOR r1, r1, r6, ROR #24
	ROR r1, r1, #18
	STR.W r1, [r0, #68]
	BIC r1, r4, r3, ROR #28
	EOR r1, r1, r7, ROR #12
	ROR r1, r1, #22
	LDR.W r3, [r0, #88]
	LDR.W r4, [r0, #96]
	LDR.W r5, [r0, #104]
	LDR.W r6, [r0, #112]
	LDR.W r7, [r0, #80]
	STR.W r1, [r0, #76]
	EOR r3, r11, r3, ROR #22
	EOR r4, r2, r4, ROR #1
	EOR r5, r9, r5
	EOR r6, r12, r6, ROR #14
	EOR r7, r8, r7, ROR #12
	BIC r1, r5, r4, ROR #10
	EOR r1, r1, r3, ROR #12
	ROR r1, r1, #19
	STR.W r1, [r0, #80]
	BIC r1, r6, r5, ROR #23
	EOR r1, r1, r4, ROR #1
	ROR r1, r1, #28
	STR.W r1, [r0, #88]
	BIC r1, r7, r6, ROR #5
	EOR r1, r1, r5, ROR #28
	ROR r1, r1, #23
	STR.W r1, [r0, #96]
	BIC r1, r3, r7, ROR #24
	EOR r1, r1, r6, ROR #29
	ROR r1, r1, #31
	STR.W r1, [r0, #104]
	BIC r1, r4, r3, ROR #2
	EOR r1, r1, r7, ROR #26
	ROR r1, r1, #29
	LDR r8, [sp, #4]
	LDR.W r3, [r0, #156]
	LDR.W r4, [r0, #124]
	LDR.W r5, [r0, #132]
	LDR.W r6, [r0, #140]
	LDR.W r7, [r0, #148]
	STR.W r1, [r0, #112]
	EOR r3, r12, r3, ROR #10
	EOR r4, r8, r4, ROR #31
	EOR r5, r11, r5, ROR #28
	EOR r6, r2, r6, ROR #4
	EOR r7, r9, r7, ROR #1
	BIC r1, r5, r4, ROR #19
	EOR r1, r1, r3, ROR #24
	ROR r1, r1, #27
	STR.W r1, [r0, #124]
	BIC r1, r6, r5, ROR #2
	EOR r1, r1, r4, ROR #21
	ROR r1, r1, #25
	STR.W r1, [r0, #132]
	BIC r1, r7, r6, ROR #21
	EOR r1, r1, r5, ROR #23
	ROR r1, r1, #4
	STR.W r1, [r0, #140]
	BIC r1, r3, r7, ROR #17
	EOR r1, r1, r6, ROR #6
	ROR r1, r1, #19
	STR.W r1, [r0, #148]
	BIC r1, r4, r3, ROR #5
	EOR r1, r1, r7, ROR #22
	ROR r1, r1, #14
	LDR.W r3, [r0, #176]
	LDR.W r4, [r0, #184]
	LDR.W r5, [r0, #192]
	LDR.W r6, [r0, #160]
	LDR.W r7, [r0, #168]
	STR.W r1, [r0, #156]
	EOR r3, r2, r3, ROR #25
	EOR r4, r9, r4, ROR #18
	EOR r5, lr, r5, ROR #29
	EOR r6, r8, r6, ROR #27
	EOR r7, r10, r7, ROR #11
	BIC r1, r5, r4, ROR #24
	EOR r1, r1, r3, ROR #21
	ROR r1, r1, #12
	STR.W r1, [r0, #160]
	BIC r1, r6, r5, ROR #1
	EOR r1, r1, r4, ROR #25
	ROR r1, r1, #11
	STR.W r1, [r0, #168]
	BIC r1, r7, r6, ROR #12
	EOR r1, r1, r5, ROR #13
	ROR r1, r1, #31
	STR.W r1, [r0, #176]
	BIC r1, r3, r7, ROR #30
	EOR r1, r1, r6, ROR #10
	ROR r1, r1, #1
	STR.W r1, [r0, #184]
	BIC r1, r4, r3, ROR #29
	EOR r1, r1, r7, ROR #27
	ROR r1, r1, #4
	LDR r9, [sp, #8]
	LDR.W r3, [r0, #4]
	LDR r4, [r0, #12]
	LDR r5, [r0, #20]
	LDR r6, [r0, #28]
	LDR r7, [r0, #36]
	STR.W r1, [r0, #192]
	EOR r3, r8, r3
	EOR r4, r11, r4, ROR #10
	EOR r5, r2, r5, ROR #23
	EOR r6, r9, r6, ROR #18
	EOR r7, lr, r7, ROR #5
	BIC r1, r6, r5, ROR #21
	EOR r1, r1, r4, ROR #20
	ROR r1, r1, #22
	STR.W r1, [r0, #12]
	BIC r1, r7, r6, ROR #29
	EOR r1, r1, r5, ROR #18
	ROR r1, r1, #25
	STR.W r1, [r0, #20]
	BIC r1, r3, r7, ROR #25
	EOR r1, r1, r6, ROR #22
	STR.W r1, [r0, #28]
	BIC r1, r4, r3, ROR #22
	EOR r1, r1, r7, ROR #15
	ROR r1, r1, #10
	STR.W r1, [r0, #36]
	BIC r5, r5, r4, ROR #31
	LDR r1, [sp, #20]
	LDR r4, [r1, #28]
	LDR r7, [r1, #32]!
	STR r1, [sp, #20]
	CMP r7, #255
	EOR r3, r3, r5, ROR #11
	EOR r1, r4, r3
	STR.W r1, [r0, #4]
	BNE Lepcc_keccak_permute_asm.round_loop_begin
Lepcc_keccak_permute_asm.round_loop_end:
	ADD sp, sp, #28
	POP {{lr}}
	POP.W {{r4, r5, r6, r7, r8, r9, r10, r11}}
	BX lr

"#);

unsafe extern "C" {
    pub fn epcc_keccak_permute_asm(state: *mut u32);
}
