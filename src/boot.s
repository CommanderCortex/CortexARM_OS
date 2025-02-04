.section .text._start
.global _start 
.type _start, @function
_start: 
        adr x7, {}     // stack start
        mov x8, {}     // stack size
        add x7, x7, x8 // stack end
        mov sp, x7     // set stack pointer
        
        //Enable floating pointer:
        mrs x7, cpacr_el1 // Read CPACR_EL1
        orr x7, x7, #(3 << 20) // Set bits 20 and 21
        msr cpacr_el1, x7 // Write CPACR_EL1

        adr x0, _start
        adr x1, _rela_start
        adr x2, _rela_end
        bl _relocate_binary

        bl main // call main

_relocate_binary:
        ldr x3, [x1]
        add x3, x3, x0
        str x3, [x1]
        add x1, x1, #8
        cmp x1, x2
        bne _relocate_binary
        ret
