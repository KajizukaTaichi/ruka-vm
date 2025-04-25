psh 1
psh 2
psh 3
cal 6
sta 0, rax
hlt
pop rdx
add rax, rdx
jmp rsp, 6
ret
