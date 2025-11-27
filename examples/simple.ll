; Simple addition function
define i32 @add(i32 %a, i32 %b) {
entry:
    %result = add i32 %a, %b
    ret i32 %result
}

; Main entry point
define i32 @main() {
entry:
    %x = call i32 @add(i32 10, i32 20)
    ret i32 %x
}
