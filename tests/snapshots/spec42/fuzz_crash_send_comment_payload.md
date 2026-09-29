# META
~~~sexpr
(snapshot (type semantic) (description "Fuzzer crash: send node with comment-only payload causing semicolon absorption"))
~~~
# SOURCE
~~~sysml
package P {
action def A {
    for
in send// nd port for HTT3prin  pq  for y  // nd port for HTT3prin items { }
  send pq   }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz_crash_send_comment_payload.md"
    (diagnostics
      (diagnostic (code "missing_semicolon") (severity error) (category parseerror) (span (offset 31) (line 3) (column 5) (len 4)) (message "missing semicolon before next declaration"))
      (diagnostic (code "recovered_action_body_element") (severity error) (category parseerror) (span (offset 35) (line 4) (column 1) (len 79)) (message "unexpected token in action body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package P {
    action def A {
        for
        in send// nd port for HTT3prin  pq  for y  // nd port for HTT3prin items { }
        send pq
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "P") (body brace (action-def (name "A") (modifiers) (specializes none) (body brace (malformed (code "missing_semicolon") (found "for") (span (offset 31) (line 3) (column 5) (len 4))) (malformed (code "recovered_action_body_element") (found "in send// nd port for HTT3prin  pq  for y  // nd port for HT") (span (offset 35) (line 4) (column 1) (len 79))) (action-usage (keyword send) (name none) (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body absent)))))))
)
~~~
