(module
    (import "js" "mem" (memory 1)) ;; [72, ...]
    (import "js" "decode_print" (func $decode_print (param i32 i32)))
    (data (i32.const 0) "Hello, World!")
    (func (export "hello_world")
        i32.const 0
        i32.const 13
        call $decode_print
    )
)