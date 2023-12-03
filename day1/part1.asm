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
           
            .loop:
                ; check if still in bounds of buffer
                cmp r11, r12
                jnl .loop_end 

                ; load next character int lower r10
                lea r15, [buffer]               ; get base address of buffer
                add r15, r11                    ; add offset for current position
                mov r10b, [r15]                 ; read the address

 

                ; check if newline. If so, update accumulator and reset state. todo:
                cmp r10b, 0xA
                jne @F
                    ; to-do: work out the calibration value for the line
                    
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
                jmp .loop
            .loop_end:

            ; print sum
            mov rdi, r9
            call print_i64

        ; close file
        mov rax, 3
        mov rdi, [file_handle]
        syscall

        ; clear file handle
        mov [file_handle] , -1

        ; print final result 
        

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