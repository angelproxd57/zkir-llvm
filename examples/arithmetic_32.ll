; 32-bit arithmetic operations showcase
define i32 @test_arithmetic(i32 %a, i32 %b) {
entry:
    ; Addition
    %sum = add i32 %a, %b

    ; Subtraction
    %diff = sub i32 %sum, %b

    ; Multiplication
    %prod = mul i32 %a, %b

    ; Division (signed)
    %quot = sdiv i32 %prod, %b

    ; Remainder (signed)
    %rem = srem i32 %a, %b

    ; Bitwise AND
    %and_result = and i32 %a, %b

    ; Bitwise OR
    %or_result = or i32 %a, %b

    ; Bitwise XOR
    %xor_result = xor i32 %a, %b

    ; Shift left
    %shl_result = shl i32 %a, 2

    ; Logical shift right
    %lshr_result = lshr i32 %a, 1

    ret i32 %shl_result
}
