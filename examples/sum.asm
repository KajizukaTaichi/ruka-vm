main:
    psh 1   ; スタックに値を積む
    psh 2
    psh 3
    cal sum     ; 合計する
    sta 0, rax  ; 結果をメモリに保存
    hlt         ; プログラムを終了する

; スタックにある値を合計する関数
sum:
    mov rax, 0      ; 合計値を初期化
loop:
    pop rdx         ; スタックから値を取り出す
    add rax, rdx    ; 加算する (合計値はraxに格納)
    jmp rsp, loop   ; スタックが空になるまで繰り返す
    ret
