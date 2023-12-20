; fasm x86_64 assembly for linux
; program to implement hash function as specified in advent of code day 15, 2032. https://adventofcode.com/2023/day/15

format elf64 executable 3
entry _start

; code section
segment readable executable
    include "print_integer64.inc"
    include "read_to_buffer.inc"

    ; hashes a string with an alorithm specified by AOC2023 day15
    ; assumes left-to-right string buffer
    ; inputs:
    ;  - rdi: address of string buffer
    ;  - rsi: length of string buffer
    ; outputs:
    ;  - rax: hash algorithm result as 64-bit unsigned integer
    ; clobbers:
    ;  - rax (output)
    ;  - r10    
    string_hash_256:        
        ; problem exerpt:
                ; The HASH algorithm is a way to turn any string of characters into a single number in the range 0 to 255. 
                ; To run the HASH algorithm on a string, start with a current value of 0. 
                ;Then, for each character in the string starting from the beginning:

                ; Determine the ASCII code for the current character of the string.
                ; Increase the current value by the ASCII code you just determined.
                ; Set the current value to itself multiplied by 17.
                ; Set the current value to the remainder of dividing itself by 256.
                ; After following these steps for each character in the string in order, the current value is the output of the HASH algorithm.
        ;

        ; preserve registers
        push r8
        push r9
        push r15
        push r14
        push rdx
        
        ; Grab the pointer to the start of the string
        lea r8, [rdi]                       
        
        ; Grab a pointer to the last character of the string
        lea r9, [rdi + rsi]
        
        ; loop prelude        
        mov r15, 0                              ; clear register to use as accumulator
        mov r10, 0                              ; r10 will be used as a temporary register. 

        .loop_start:
            ; check loop conditions
            cmp r8, r9                     
            jge string_hash_256.loop_end        ; if r8 >= r9, break from loop. 
                                                ; Breaks from loop, if we are at the last address
            
            ; prepare for iteration
            xor r14, r14                        ; clear r14
            mov r14b, [r8]                      ; load selected character into register r14

            ; Step 1: add next digit to accumulator
            add r15, r14

            ; Step 2: r15 = r15 * 17
            mov rax, r15                        ; load multiplicand
            mov r10, 17                         ; load multiplier. 0x11
            mul r10                             ; multiply rax by r10. Result is stored in bits of rdx and rax. 
                                                ; I will not check r10 to save time - the inputs are too small to need more bits to represent that rax can hold. 
                                                ; WARNING: if this assumption is wrong, the result is undefined. Be aware of this if you are bug hunting
            mov r15, rax                        ; save result back to accumulator

            ; Step 3: set accumulator to itself modulus 256. 
            mov r10, 256                        ; load divisor
            xor rdx, rdx                        ; clear bytes 8 to 16 of division input
            mov rax, r15                        ; write accumulator to division input bytes 0 to 8
            div r10                             ; Do rdx:rax divided by r10
                                                ; Remaindier in rdx, Result in rax 
                                                ; Main result is ignored for our use
            mov r15, rdx                        ; write remandier to accumulator

            
            ; end iteration
            inc r8                              ; add 1 to r8 to get next address in string to read from

            jmp .loop_start                     ; no break conditions have been met, continue the loop.
        string_hash_256.loop_end:
        
        ; prepare return value
        mov rax, r15
        
        ; function cleanup
        pop rdx
        pop r14
        pop r15
        pop r9
        pop r8
    ret
        
    _start:
        ; load file into input_buffer
        lea rdi, [input_file_name]
        lea rsi, [input_buffer]
        mov rdx, input_buffer.capacity
        call read_to_buffer

        ; check if error returned from read_to_buffer
        cmp rax, 0
        jge @F                                  ; skip the following block if no error    
            ; exit with error code
            mov rdi, rax                        ; load return value
            mov rax, 60                         ; SYS_exit
            syscall
        @@:

        ; save count of bytes written
        mov [input_buffer.size], rax

        ; loop1 setup
        lea r12, [input_buffer]                 ; load starting address
        mov r13, 0                              ; used to store the current hash value
        _start.outer_start:
            ; loop2 setup
            mov r9, r12                         ; create a cursor with current address
            
            mov r8, r9                          ; copy starting address                          
            add r8, [input_buffer.size]         ; get offset of first bit after the input read
            sub r8, 1                           ; get the last bit read

            ; find next comma address. increases r9 until it points to a comma
            _start.comma_loop:
                cmp r9, r8
                jg _start.outer_end             ; break if beyond last valid address  
                
                xor r11, r11                    ; clear upper bits of temporary register r11 
                mov byte r11b, [r9]             ; read next character into lower r11
                
                cmp r11, ','
                je _start.end_comma_loop        ; break if comma
                
                cmp r11, 0xA                    
                je _start.end_comma_loop        ; break if newline


                ; prepare for next iteration
                inc r9                          ; increase selected address
            jmp _start.comma_loop
            _start.end_comma_loop:

            ; move past the comma
            inc r9

            ; calculate character count from current base address and selected address
            mov r11, r9                         ; load current selected address into temporary register
            sub r11, r12                        ; subtract base address
            dec r11                             ; exclude comma

            ; call string_hash_256 with args. returns in rax
            lea rdi, [r12]                      ; load address
            mov rsi, r11                        ; load count
            call string_hash_256                ; clobbers r10

            ; use result
            add r13, rax

            ; update base address to comma found for next outer iteration
            mov r12, r9

            ; repeat
            jmp _start.outer_start            
        _start.outer_end:
        
        ; print the result
        mov rdi, r13
        call print_i64
        
        ; exit 
        mov rdi, 0
        mov rax, 60
        syscall


; writeable pre-reserved data section
segment readable writeable
    ; reserve memory to read file into
    input_buffer.capacity = 1024 * 1024 * 16; define 16MB capacity. 'compile-time' constant 
    input_buffer.size       dq 0                ; declare qword to store current size in and set to 0
    input_buffer            rb input_buffer.capacity ; declare buffer with given size
    
    errno                   dd 0                ; dword to write error codes to, if any                        

; readonly pre-reserved data section
segment readable
    ; declare null-terminated file name
    input_file_name db "input.txt", 0           ; declare null-terminated file path
    input_file_name.length = $-input_file_name-1 ; declare length excluding null terminator