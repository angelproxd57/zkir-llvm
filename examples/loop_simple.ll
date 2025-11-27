; Simple loop - sum from 1 to N
define i32 @sum_to_n(i32 %n) {
entry:
    br label %loop

loop:
    %i = phi i32 [ 1, %entry ], [ %next_i, %loop_body ]
    %sum = phi i32 [ 0, %entry ], [ %next_sum, %loop_body ]

    ; Check if done
    %cmp = icmp sgt i32 %i, %n
    br i1 %cmp, label %exit, label %loop_body

loop_body:
    ; sum = sum + i
    %next_sum = add i32 %sum, %i

    ; i = i + 1
    %next_i = add i32 %i, 1

    br label %loop

exit:
    ret i32 %sum
}
