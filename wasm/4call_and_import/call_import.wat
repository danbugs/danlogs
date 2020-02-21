;; CALLING FUNCTIONS INSIDE WAT
;; IMPORTING FUNCTIONS

(module
    (import "imports" "console.log" (func $log (param i32)))
    (func (param i32) (result i32)
        local.get 0
    )
    (func (export "add") (param i32) (param $p1 i32)
        local.get 0
        call 1 ;; 1243
        local.get $p1 ;; 2
        i32.add
        call $log
    )
)