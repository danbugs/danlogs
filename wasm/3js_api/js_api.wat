(module
    (func $add (param i32) (param $p1 i32) (result i32)
        local.get 0
        local.get $p1
        i32.add
    )
    (export "add" (func $add))
)