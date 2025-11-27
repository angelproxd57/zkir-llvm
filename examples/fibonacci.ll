; Iterative Fibonacci (ZK-friendly - no recursion)
define i32 @fib(i32 %n) {
entry:
    %cmp0 = icmp eq i32 %n, 0
    br i1 %cmp0, label %return_zero, label %check_one

check_one:
    %cmp1 = icmp eq i32 %n, 1
    br i1 %cmp1, label %return_one, label %loop_init

return_zero:
    ret i32 0

return_one:
    ret i32 1

loop_init:
    br label %loop

loop:
    %i = phi i32 [ 2, %loop_init ], [ %next_i, %loop ]
    %prev = phi i32 [ 0, %loop_init ], [ %curr, %loop ]
    %curr = phi i32 [ 1, %loop_init ], [ %next, %loop ]
    %next = add i32 %prev, %curr
    %next_i = add i32 %i, 1
    %done = icmp sgt i32 %next_i, %n
    br i1 %done, label %exit, label %loop

exit:
    ret i32 %next
}
