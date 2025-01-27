.section .text._start
.global _start
.type _start, @function
_start:
        adr x7, {}     // stack start
        mov x8, {}     // stack size
        add x7, x7, x8 // stack end
        mov sp, x7     // set stack pointer
        
        //Enable floating pointer:
        mrs x7, cpacr_el1
        orr x7, x7, #(3 << 20)
        msr cpacr_el1, x7
                
        bl main
