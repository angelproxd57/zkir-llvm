; Sum numbers from 1 to n
define i32 @sum_to_n(i32 %n) {
entry:
    br label %loop

loop:
    %i = phi i32 [ 1, %entry ], [ %next_i, %loop ]
    %sum = phi i32 [ 0, %entry ], [ %next_sum, %loop ]
    %next_sum = add i32 %sum, %i
    %next_i = add i32 %i, 1
    %done = icmp sgt i32 %next_i, %n
    br i1 %done, label %exit, label %loop

exit:
    ret i32 %next_sum
}
