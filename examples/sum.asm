main:
    psh 5       ; スタックに値を積む
    psh 8
    psh 2
    cal sum     ; 合計する
    pop rax     ; スタックから結果を取り出す
    sta 0, rax  ; 結果をメモリに保存
    hlt         ; プログラムを終了する

; スタックにある値を合計する関数
sum:
    mov rax, 0          ; 合計値を初期化
loop_start:
    mov rcx, rsp        ; スタックが空ならループ終了
    nor rcx, rcx
    jmp rcx, loop_end
    pop rdx             ; スタックから値を取り出す
    add rax, rdx        ; 加算する
    jmp 1, loop_start   ; ループ先頭に戻る
loop_end:
    psh rax             ; 合計値をスタックに積む
    ret
