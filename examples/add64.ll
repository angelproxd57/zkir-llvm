; 64-bit addition using register pairs
define i64 @add64(i64 %a, i64 %b) {
entry:
    %sum = add i64 %a, %b
    ret i64 %sum
}

; 64-bit subtraction
define i64 @sub64(i64 %a, i64 %b) {
entry:
    %diff = sub i64 %a, %b
    ret i64 %diff
}
