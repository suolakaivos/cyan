.data
.globl hello
hello:
.string "Hello, world!"

.text
.global main
main:
    mov dword [%eax], 0x1

