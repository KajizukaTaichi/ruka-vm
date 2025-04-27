main:
    psh 5       ; スタックに値を積む
    psh 8
    psh 2
    cal sum     ; 合計する
    pop ar      ; スタックから結果を取り出す
    mov ba, 8   ; 保存先アドレス: 8
    sta ba, ar  ; 結果をメモリに保存
    hlt         ; プログラムを終了する



; スタックにある値を合計する関数
sum:
    mov ar, 0           ; 合計値を初期化
loop_start:
    mov cr, sp
    nor cr, cr          ; スタックは空か？
    jmp cr, loop_end    ; そうならループ終了
loop_body:
    pop dr              ; スタックから値を取り出す
    add ar, dr          ; 加算する
    jmp 1, loop_start   ; ループ先頭に戻る
loop_end:
    psh ar              ; 合計値をスタックに積む
    ret
