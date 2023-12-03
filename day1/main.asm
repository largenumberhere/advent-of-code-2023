format elf64 executable 3

entry _start

NULL = 0
O_RDONLY = 0

segment readable executable
    include "print_integer64.inc"
    
    _start:
        ; open file name given by argv[0]
            pop r8                              ; load number of arguments
            pop rsi                             ; discard argument 0 - program name

            ; if not 2 args passed in, print message and exit
            cmp r8, 2
            je @F
                ; print Usage message
                mov rax, 1                      ; SYS_write
                mov rdi, 1                      ; stoud
                mov rsi, usage_message
                mov rdx, usage_message.length
                syscall

                ; exit with failure
                mov rax, 60
                mov rdi, 1
                syscall
            @@:

            ; open file
            mov rax, 2                          ; SYS_open
            pop rdi                             ; load file name given in program arguments 
            mov rsi, O_RDONLY                   ; read only flag
            mov rdx, NULL                       ; do not specify a mode, that only is required when using the O_CREAT flag                          
            syscall

            ; check for error opening file
            cmp rax, 0
            jnle @F
                mov r8, rax                     ; stow the error

                ; print file opening error message
                mov rax, 1
                mov rdi, 1
                mov rsi, open_error_message
                mov rdx, open_error_message.length
                syscall
            
                ; exit with file error as error code
                mov rdi, r8
                mov rax, 60
                syscall
            @@:

            ; save file descriptor
            mov [file_handle], rax

        ; read all into memory
            ; read into buffer
            mov rax, 0                          ; SYS_read
            mov rdi, [file_handle]              ; load file descriptor
            lea rsi, [buffer]                   ; load buffer address
            mov rdx, buffer.capacity            ; load capacity constant
            syscall

            ; check buffer isn't full. If it is, the file is probably too big
            cmp rax, buffer.capacity
            jnge @F 
                ; write error message
                mov rax, 1
                mov rdi, 1
                mov rsi, buffer_overflow_error_message
                mov rdx, buffer_overflow_error_message.length
                syscall 

                ; close file
                mov rax, 3
                mov rdi, [file_handle]
                syscall

                ; exit with error code 2
                mov rax, 60
                mov rdi, 2
                syscall
            @@:

            ; save buffer size
            mov [buffer.size], rax

            ; close file
            mov rax, 3
            mov rdi, [file_handle]
            syscall

        ; loop, each time read in one line
            ; loop setup
            mov r9,     0                       ; clear accumulator
            mov r10,    0                       ; holds immediate character
            mov r11,    0                       ; holds position in buffer
            mov r12,    [buffer.size]           ; load upper bound for buffer
            mov r13,    -1                      ; first 'calibration' value. -1 indicates not present
            mov r14,    -1                      ; 2nd 'calibration' value. -1 indicates not present
            ; r15 is used as temporary register

            .loop:
                ; check if still in bounds of buffer
                cmp r11, r12
                jnl .loop_end 

                ; load next character into lower r10
                lea r15, [buffer]               ; get base address of buffer
                add r15, r11                    ; add offset for current position
                mov r10b, [r15]                 ; read the address

                ; check if newline. If so, update accumulator and reset state. todo:
                cmp r10b, 0xA
                jne @F
                    ; if there is no 2nd calibration value, duplicate the 1st
                    cmp r14, -1
                    jne .yes_calib2
                        mov r14, r13
                    .yes_calib2:

                    ; multiply first calibation value by 10
                    mov rax, 10
                    mul r13                     ; rax = rax * r13. overflow into rdx should not be possible give all number are between 0 and 9

                    ; add values
                    add r14, rax

                    ; print value
                    push r8
                    push r9
                    push r10
                    push r11
                    mov rdi, r14
                    call print_i64
                    pop r11             
                    pop r10
                    pop r9
                    pop r8

                    ; add to accumulator
                    add r9, r14

                    ; clear state
                    mov r13, -1
                    mov r14, -1
                @@:

                ; check if is an english word
                ; word,     length

                ; "one"     3
                ; "two"     3
                ; "six"     3
                ; "ten"     3
  
                ; "four"    4
                ; "five"    4
                ; "nine"    4

                ; "three"   5
                ; "eight"   5
                ; "seven"   5
                
                ; calculate remaining characters
                mov r15, r12    
                sub r15, r11                        
                inc r15                             ; r15 = current_index - length + 1

                ; branch if there is at least 5 characters remaining
                cmp r15, 5
                jge .check_5

                ; branch if there is at lest 4 characters remaining
                cmp r15, 4
                jge .check_4

                ; branch if there is at least 3 characters remaining
                cmp r15, 3
                jge .check_3

                ; there is not enough characters left to check for words. Skip all of the checks
                jmp _start.end_check

                    .check_5:
                        cmp r10, 't'    ; could be three
                        jne @F
                            ; check for three
                            lea r15, [buffer]
                            add r15, r11
                            inc r15

                            cmp byte [r15], 'h'
                            jne @F

                            inc r15
                            cmp byte [r15], 'r'
                            jne @F

                            inc r15
                            cmp byte [r15], 'e'
                            jne @F

                            inc r15
                            cmp byte [r15], 'e'                            
                            jne @F

                            ; matches 'three'
                            mov r10b, '3'
                            
                           ; add r11, 4  ; update position for next loop. Value is incremented once before loop iteration ends
                            jmp _start.end_check

                        @@:

                        cmp r10, 'e'    ; could be eight
                        jne @F
                            ; check for "eight"
                            lea r15, [buffer]
                            add r15, r11
                            inc r15

                            cmp byte [r15], 'i'
                            jne @F

                            inc r15
                            cmp byte [r15], 'g'
                            jne @F

                            inc r15
                            cmp byte [r15], 'h'
                            jne @F

                            inc r15
                            cmp byte [r15], 't'
                            jne @F

                            mov r10b, '8'
                            ;add r11, 4
                            jmp _start.end_check
                        @@:

                        cmp r10, 's'    ; could be seven
                        jne @F
                            ; check for seven
                            lea r15, [buffer]
                            add r15, r11
                            inc r15
 
                            cmp byte [r15], 'e'
                            jne @F

                            inc r15
                            cmp byte [r15], 'v'
                            jne @F

                            inc r15
                            cmp byte [r15], 'e'
                            jne @F
                            inc r15
                            cmp byte [r15], 'n'
                            jne @F

                            mov r10b, '7'
                           ; add r11, 4
                            jmp _start.end_check
                        @@:
                    
                    .check_4:
                        cmp r10, 'f'    ; could be four, five
                        jne @f
                            ; check for five
                            lea r15, [buffer]
                            add r15, r11
                            inc r15

                            cmp byte [r15], 'i'
                            jne .maybe_4

                            inc r15
                            cmp byte [r15], 'v'
                            jne @F

                            inc r15
                            cmp byte [r15], 'e'
                            jne @F

                            mov r10b, '5'
                           ; add r11, 3
                            jmp _start.end_check

                            .maybe_4:
                                cmp byte [r15], 'o'
                                jne @F

                                inc r15
                                cmp byte [r15], 'u'
                                jne @F

                                inc r15
                                cmp byte [r15], 'r'
                                jne @F

                                mov r10b, '4'
                          ;      add r11, 3
                                jmp _start.end_check
                        @@:
                        
                        cmp r10, 'n'    ; could be nine
                        jne @f
                            ; check for nine
                            lea r15, [buffer]
                            add r15, r11
                            inc r15

                            ; cmp byte [r15], 'n'
                            ; jne @F
                            
                            cmp byte [r15], 'i'
                            jne @F

                            inc r15
                            cmp byte [r15], 'n'
                            jne @F

                            inc r15
                            cmp byte [r15], 'e'
                            jne @F

                            mov r10b, '9'
                         ;   add r11, 3
                            jmp _start.end_check
                        @@:
                        

                    .check_3:
                        cmp r10, 's'    ; could be six
                        jne @f
                            ; check for six
                            lea r15, [buffer]
                            add r15, r11
                            inc r15

                            cmp byte [r15], 'i'
                            jne @F

                            inc r15
                            cmp byte [r15], 'x'
                            jne @F

                            mov r10b, '6'
                         ;   add r11, 2
                            jmp _start.end_check
                        @@:
                        
                        cmp r10, 't'    ; could be two
                        jne @f
                            ; check for six
                            lea r15, [buffer]
                            add r15, r11
                            inc r15

                            cmp byte [r15], 'w'
                            jne @F

                            inc r15
                            cmp byte [r15], 'o'
                            jne @F

                            mov r10b, '2'
                          ;  add r11, 2
                            jmp _start.end_check
                        @@:
                        
                        cmp r10, 'o'    ; could be one
                        jne @f
                            ; check for 1
                            lea r15, [buffer]
                            add r15, r11
                            inc r15

                            cmp byte [r15], 'n'
                            jne @F

                            inc r15
                            cmp byte [r15], 'e'
                            jne @F

                            mov r10b, '1'
                          ;  add r11, 2
                            jmp _start.end_check
                        @@:

                    _start.end_check:

                ; check if number is between '0' and '9'
                cmp r10b, '0'
                jl @F
                cmp r10b, '9' 
                jg @F
                    ; number is between the 2 values
                        ; convert from ascii to number
                        sub r10b, '0'

                        ; check if calibration number 1 is not done, fill it and break
                        cmp r13, -1
                        jne .filled_1_end
                            ; write to it
                            mov r13, r10

                            ; break
                            jmp @F
                        .filled_1_end:

                        ; write calibation value to 2nd number, if 1st is done
                        mov r14, r10
                @@:

                inc r11                             ; update position counter for next loop
                jmp _start.loop
            _start.loop_end:

            ; print sum. clobbers many registers
            mov rdi, r9
            call print_i64

        ; close file
        mov rax, 3
        mov rdi, [file_handle]
        syscall

        ; clear file handle
        mov [file_handle] , -1       

        ; exit with 0
        mov rax, 60
        mov rdi, 0
        syscall

segment readable writeable
    buffer          rb  1024*1024                   ; reserve 1mb buffer 
    buffer.size     dq  -1                          ; default size to -1, to make it clear if it's not initalized
    buffer.capacity =   1024*1024                   ; capacity is fixed

    file_handle     dq  -1                          ; space for a file handle to be loaded directly to and from a 64-bit register. -1 will always cause an error if passed to the kernel as a fd

segment readable

    usage_message db "Usage: ./main input-file.txt", 0xA
    usage_message.length = $-usage_message

    open_error_message db "Failed to open file", 0xA
    open_error_message.length = $-open_error_message

    buffer_overflow_error_message db "File was too big for the fixed buffer :(", 0xA
    buffer_overflow_error_message.length = $-buffer_overflow_error_message 