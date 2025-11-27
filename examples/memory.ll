; Memory operations - load and store
define i32 @load_store(ptr %ptr, i32 %value) {
entry:
    ; Store value to pointer
    store i32 %value, ptr %ptr

    ; Load value back
    %loaded = load i32, ptr %ptr

    ; Return loaded value
    ret i32 %loaded
}

; Stack allocation
define i32 @stack_alloc() {
entry:
    ; Allocate space on stack
    %local = alloca i32

    ; Store to stack
    store i32 42, ptr %local

    ; Load from stack
    %result = load i32, ptr %local

    ret i32 %result
}

; 8-bit and 16-bit loads/stores
define i32 @byte_ops(ptr %ptr) {
entry:
    ; Store byte
    store i8 255, ptr %ptr

    ; Load byte (unsigned)
    %byte = load i8, ptr %ptr

    ; Zero-extend to i32
    %result = zext i8 %byte to i32

    ret i32 %result
}
