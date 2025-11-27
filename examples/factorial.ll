; Iterative factorial (ZK-friendly)
define i32 @factorial(i32 %n) {
entry:
    ; Check if n <= 1
    %cmp = icmp sle i32 %n, 1
    br i1 %cmp, label %return_one, label %loop_init

return_one:
    ret i32 1

loop_init:
    br label %loop

loop:
    %i = phi i32 [ 2, %loop_init ], [ %next_i, %loop ]
    %result = phi i32 [ 1, %loop_init ], [ %next_result, %loop ]

    ; result = result * i
    %next_result = mul i32 %result, %i

    ; i = i + 1
    %next_i = add i32 %i, 1

    ; Check if done (i > n)
    %done = icmp sgt i32 %next_i, %n
    br i1 %done, label %exit, label %loop

exit:
    ret i32 %next_result
}
