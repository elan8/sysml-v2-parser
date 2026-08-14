# META
~~~sexpr
(snapshot (type semantic) (description "Fuzzer crash: for loop with multiple trailing line comments in sequence causing idempotence violation"))
~~~
# SOURCE
~~~sysml
package P {
action def A {
    for
perform action doS : Dff {     for y // ndent g {
//'//ug {
// port for HTTPprin items { }
    }
    } }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz_crash_for_loop_multicomment.md"
    (diagnostics
      (diagnostic (code "recovered_action_body_element") (severity error) (category parseerror) (span (offset 31) (line 3) (column 5) (len 105)) (message "unexpected token in action body"))
      (diagnostic (code "unexpected_closing_brace") (severity error) (category parseerror) (span (offset 140) (line 9) (column 1) (len 1)) (message "unexpected closing '}'"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package P {
    action def A {
        for
perform action doS : Dff {     for y // ndent g {
//'//ug {
// port for HTTPprin items { }
    }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "P") (body (action-def (name "A") (specializes none) (body (malformed (code "recovered_action_body_element") (found "for") (span (offset 31) (line 3) (column 5) (len 105))))))))
)
~~~
