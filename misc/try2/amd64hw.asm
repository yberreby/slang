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

    ; sys_exit code
    mov rax, 60
    ; set exit code to 0
    mov rdi, 0
    syscall
