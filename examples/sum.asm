main:
    psh 1
    psh 2
    psh 3
    cal sum
    sta 0, rax
    hlt

sum:
    pop rdx
    add rax, rdx
    jmp rsp, sum
    ret
