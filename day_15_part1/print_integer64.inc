; for x86_64 linux fasm
; includes 4 procedures
; for writing a number to stdout:
; - print_i64
; - print_u64
; for writing a number to a buffer:
; - write_i64
; - write_u64


; write the number to stdout
; inputs:
; - rdi: 64-bit signed number
; outputs:
; - rax: result of sys_write call
; clobbers: 
; - r10
; - r11
print_i64:
    ; save registers' state
    push rsi
    push rdi
    push rdx

    mov rsi, rsp                                        ; get the address of the start of stack
    sub rsp, 32                                         ; allocate at least 21 bytes + 1 for newline of space for buffer in stack
    call write_i64                                      ; writes the printable digits for the signed integer in rdi into the address's destination in rsi
    mov byte [rsi+21], 0xA                              ; write newline character to the end

    ; print the number
    mov rax, 1                                          ; sys_write
    mov rdi, 1                                          ; stdout
    mov rdx, 22                                         ; max size of 64-bit number as decimal has 21 digits + sign
    syscall                                             ; call kernel using rax, rdi, rdx and rsi. rax is overwritten

    add rsp, 32                                         ; dellocate extra stack space for buffer

    ; recover registers's state
    pop rdx
    pop rdi
    pop rsi
ret     

; write the number to stdout
; inputs:
; - rdi: 64-bit un-signed integer
; outputs:
; - rax: result of sys_write call
; clobbers:
; - r10
; - r11
print_u64:
    ; save registers' state
    push rsi
    push rdi
    push rdx

    mov rsi, rsp                                        ; copy stack address
    sub rsp, 32                                         ; allocate at least 21 bytes + 1 for newline on stack for a buffer
    call write_u64                                      ; convert the number in rdi to ascii digits in rsi. Clobbers r10, r11

    ; print value at adress stored in rsi 
    mov rax, 1                                          ; sys_write
    mov rdi, 1                                          ; stdout
    mov rdx, 22                                         ; max size of 64-bit number as decimal has 21 digits + sign
    syscall

    add rsp, 32                                         ; deallocate buffer

    ; recover registers's state
    pop rdx
    pop rdi
    pop rsi
ret

; calculate the decimal representation of an unsigned number and write it to a buffer
; inputs:
; - rdi: 64-bit un-signed integer value
; - rsi: buffer to write to. 64-bit effective address for beginning of 21-bytes. If null terminated, must be size 22, null terminator is not added for you.
; output: none
; clobbers: 
; - r10
; - r11
write_u64:
     ; save all registers
    push rax
    push r9
    push rdi
    ; push r10
    ; push r11

    ; load address to write to
    mov r9, rsi                                         ; move address to preferred register
    add r9, 20                                          ; go to just after end of string
    
    ; loop setup
    xor r11, r11                                        ; current loop itteration = 0
    mov r10, 10                                         ; use base 10
    
.loop_start:
    ; perform division
    mov rax, rdi                                        ; load current integer value               
    xor rdx, rdx                                        ; clear upper bits of division input
    div r10                                             ; rax, rdx(remaindier) = rdx:rax / r12
    mov rdi, rax                                        ; save current value

    ; write remaindier digit
    add rdx, '0'                                        ; convert to ascii digit
    mov byte [r9], dl                                   ; save lower bit of rdx (the rest should be 0)

    ; update loop data
    inc r11                                             ; update loop counter
    dec r9                                              ; update address to write to
    
    ; repeat until r11 > 21
    cmp r11, 21
    jng .loop_start

;loop_end:
    ; include sign
    mov byte [rsi], ' '                                 ; do not add '+' or '-' if number is unsigned

    ; recover all registers
    ; pop r11
    ; pop r10
    pop rdi
    pop r9
    pop rax
ret

; calculate the decimal representation of a signed number and write it to a buffer
; inputs:
; - rdi: 64-bit signed integer value. Must be between or equal to 9_223_372_036_854_775_807 and -9_223_372_036_854_775_808
; - rsi: buffer to write to. 64-bit effective address for beginning of 21-bytes. If null terminated, must be size 22, null terminator is not added for you.
; clobbers: 
; - r10 
; - r11
write_i64:
    test rdi, rdi                                       ; check the sign of the number
    jns .is_not_negaitve                                ; if signed, go to is_not_negative
    ; fall through to is_negative
; is_negative:
    neg rdi                                             ; convert rdi to unsigned number for algorithm of proc_calcu64 to work
    call write_i64                                      ; run the unsigned algorithm
    mov byte [rsi], '-'                                 ; write negative sign
ret                                                     ; return from proc_calci64

.is_not_negaitve:                                
    call write_u64                                      ; fill the buffer with the number
    mov byte [rsi], '+'                                 ; write positive sign
ret