; Comparison operations
define i32 @max(i32 %a, i32 %b) {
entry:
    %cmp = icmp sgt i32 %a, %b
    br i1 %cmp, label %return_a, label %return_b

return_a:
    ret i32 %a

return_b:
    ret i32 %b
}

define i32 @is_equal(i32 %a, i32 %b) {
entry:
    %cmp = icmp eq i32 %a, %b
    %result = zext i1 %cmp to i32
    ret i32 %result
}

define i32 @is_less_unsigned(i32 %a, i32 %b) {
entry:
    %cmp = icmp ult i32 %a, %b
    %result = zext i1 %cmp to i32
    ret i32 %result
}
