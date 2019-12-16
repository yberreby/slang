section .data

    message db "Hello, world!", 10

section .text

global _start
_start:
    ; sys_write
    mov rax, 1
    ; STDOUT
    mov rdi, 1
    ; message content
    mov rsi, message
    ; length
    mov rdx, 14
    syscall
    jmp exit_ok


exit_ok:
    ; sys_exit code
    mov rax, 60
    ; set exit code to 0
    mov r8, 7
    mov r9, 42
    add r8, r9
_my_breakpoint:
    mov rdi, r8
    push rdi
_breakpoint_2:
    syscall
