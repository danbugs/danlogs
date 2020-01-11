;; How to GET params

(module
    (func (param i32) (param $p1 i32) (return i32)
        local.get 0
        local.get $p1
        i32.add
    )
)

;; The Stack