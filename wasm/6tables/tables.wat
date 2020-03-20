(module
    (table 2 funcref) ;; anyfunc
    (elem (i32.const 0) $f1 $f2)
    (func $f1 (result i32)
        i32.const 1
    )
    (func $f2 (result i32)
        i32.const 2
    )
    (type $result_i32 (func (result i32)))
    ;; exports.call_by_index(0)
    (func (export "call_by_index") (param i32) (result i32)
        local.get 0
        call_indirect (type $result_i32)
    )
)