.section .text._start
.global _start
.type _start, @function
_start:
        adr x7, {}     // stack start
        mov x8, {}     // stack size
        add x7, x7, x8 // stack end
        mov sp, x7     // set stack pointer
        
        bl main
