;; GLOBALS
;; (module
;;     (import "js" "print" (func $print (param i32)))
;;     (global $g1 (import "js" "g1") (mut i32))
;;     (global $g i32 (i32.const 1))
;;     (func $getG1 (export "getG1")
;;         global.get $g1
;;         call $print
;;     )
;;     (func $setG1 (export "setG1") (param i32)
;;         local.get 0
;;         global.set $g1
;;         call $getG1    
;;     )
;; )

;; MEMORY
(module
    (import "js" "print" (func $print (param i32)))
    (import "js" "mem" (memory 1))
    ;; (memory 1)
    (func (export "populateMem")
        i32.const 0
        i32.const 123
        i32.store

        i32.const 10
        i32.const 1230
        i32.store
    )

    (func (export "getAt") (param i32)
        local.get 0
        i32.load
        call $print
    )
)