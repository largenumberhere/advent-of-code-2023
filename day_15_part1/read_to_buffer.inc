; read the entire contents of a file to a region in meory specified.
; requires errno to be defined as a 32-bit signed integer.
; inputs:
;  - rdi:   address to a null-terminated file name string
;  - rsi:   buffer to write to
;  - rdx:   buffer to write to capacity
; outputs on success: 
;  - rax:   positive unsigned number of bytes copied to buffer
; outputs on error:
;  - rax:   negative integer, specified bellow under `return error numbers`  
;  - errno: if the errror was caused by a syscall, its return value is written to this address 
; return error numbers:
;  - -1:    error caused by SYS_open call
;  - -2:    error caused by SYS_read call
;  - -3:    buffer capacity was too small for file
;  - -4:    error caused by SYS_close call
; clobbers: 
;  - rdi (input)
;  - rsi (input)
;  - rdx (input)
;  - rax (output)
;  - r10
;  - r11
read_to_buffer:
    ; function prologue
        ; update stack base pointer
        push rbp
        mov rbp, rsp

        ; save registers (to-do: make this not over-allocate)
        sub rsp, 8 * 5                                  ; allocate space for 4 8-byte registers
        mov [rbp - 0],  r8
        mov [rbp - 8],  r9
        mov [rbp - 24], rcx
        mov [rbp - 32], r12
        
        ; preserve inputs that aren't immediately used
        mov r8, rsi
        mov r9, rdx

    ; get handle to file. this syscall clobbers rsi, rdx, rcx, r11
    mov rax, 2                                          ; SYS_open
    mov rsi, 0                                          ; set to read only
    mov rdx, 0                                          ; set mode to null - it is not required for readonly access
    syscall                                             ; uses rdi passed in

    ; return early if error
    cmp rax, 0
    jge @F                                              ; skip following block, if rax >= 0 
        mov [errno], eax                                ; write lower rax to errno
        mov rax, -1                                     ; set return value to -1
        jmp read_to_buffer.taredown_and_exit            ; return with rax
    @@:

    ; save file descriptor
    mov r10, rax
    
    ; read into buffer. returns byte count read ;23071/ 0x5a1f
    mov rax, 0                                          ; SYS_read
    mov rdi, r10                                        ; load file handle
    lea rsi, [r8]                                       ; load buffer address
    mov rdx, r9                                         ; load capacity
    syscall

    ; return early if error
    cmp rax, 0
    jge @F                                              ; skip following block, if rax >= 0 
        mov [errno], eax                                ; write lower rax into errno
        mov rax, -2                                     ; set return value to -2
        jmp read_to_buffer.taredown_and_exit            ; return
    @@:

    ; assume entire file was not read if buffer was completely filled
    cmp rax, r9                                         
    jl @F                                               ; skip following block if rax < capacity
        mov rax, -3                                     ; set return value to -3
        jmp read_to_buffer.taredown_and_exit            ; return
    @@:

    ; stow written count
    mov r12, rax

    ; close file
    mov rax, 3                                          ; SYS_close
    mov rdi, r10                                        ; load file descriptor
    syscall

    ; return early if file close error
    cmp rax, 0
    jge @F                                              ; skip the following if rax >=0
        mov rax, -4                                     ; set return value to -4    
        jmp read_to_buffer.taredown_and_exit            ; return
    @@:

    ; prepare to exit with success
    mov rax, r12                                          ; set return value to written count
    
    read_to_buffer.taredown_and_exit:
        ; recover registers
        mov r8,     [rbp - 0]
        mov r9,     [rbp - 8]
        mov rcx,    [rbp - 24]
        mov r12,    [rbp - 32]

        add rsp, 8 * 5                                  ; deallocate stack

        ; recover base pointer
        mov rsp, rbp
        pop rbp
ret

; byte =1, word=2, dword=4, qword=8, 