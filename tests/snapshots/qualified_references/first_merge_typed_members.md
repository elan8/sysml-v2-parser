# META
~~~sexpr
(snapshot (type recovery) (description "First/merge brace bodies retain ordered typed pin members, explicit unsupported action members, malformed recovery nodes, delimiter provenance, and valid members after recovery."))
~~~
# SOURCE
~~~sysml
package ControlMembers {
    part def Flow {
        first Actions::start then Actions::finish {
            out pin : Signals::Output;
            calc opaque;
            bogus ???;
            in resumed : Signals::Input;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "first_merge_typed_members.md"
    (diagnostics
      (diagnostic (code "unsupported_grammar_form") (severity error) (category unsupportedgrammarform) (span (offset 148) (line 5) (column 13) (len 12)) (message "spec-valid action-body member is not modeled in first/merge bodies"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 173) (line 6) (column 13) (len 23)) (message "unrecognized declaration `bogus` in first/merge body"))
    )
  )
)
~~~
# FORMAT
~~~sexpr
(stable-idempotent)
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 59) (line 3) (column 15) (len 14)) (segments (segment 0 (token "Actions") (name "Actions") (separator none) (span (offset 59) (line 3) (column 15) (len 7))) (segment 1 (token "start") (name "start") (separator colon-colon) (span (offset 68) (line 3) (column 24) (len 5)))))
    (reference r1 (scope relative) (span (offset 79) (line 3) (column 35) (len 15)) (segments (segment 0 (token "Actions") (name "Actions") (separator none) (span (offset 79) (line 3) (column 35) (len 7))) (segment 1 (token "finish") (name "finish") (separator colon-colon) (span (offset 88) (line 3) (column 44) (len 6)))))
    (reference r2 (scope relative) (span (offset 119) (line 4) (column 23) (len 15)) (segments (segment 0 (token "Signals") (name "Signals") (separator none) (span (offset 119) (line 4) (column 23) (len 7))) (segment 1 (token "Output") (name "Output") (separator colon-colon) (span (offset 128) (line 4) (column 32) (len 6)))))
    (reference r3 (scope relative) (span (offset 209) (line 7) (column 26) (len 14)) (segments (segment 0 (token "Signals") (name "Signals") (separator none) (span (offset 209) (line 7) (column 26) (len 7))) (segment 1 (token "Input") (name "Input") (separator colon-colon) (span (offset 218) (line 7) (column 35) (len 5)))))
  )
  (root (package (name "ControlMembers") (body (part-def (name "Flow") (body (first (source (expression (span (offset 59) (line 3) (column 15) (len 14)) (ref r0))) (target (expression (span (offset 79) (line 3) (column 35) (len 15)) (ref r1))) (body brace (span (span (offset 95) (line 3) (column 51) (len 139))) (open-brace (span (offset 95) (line 3) (column 51) (len 1))) (members (in-out (direction out) (declaration "pin") (type (ref r2)) (redefines none) (value none) (span (offset 109) (line 4) (column 13) (len 26))) (unsupported (production action-body-member) (code "unsupported_grammar_form") (found "calc opaque;") (span (offset 148) (line 5) (column 13) (len 12))) (malformed (code "unrecognized_declaration_in_scope") (found "bogus ???;") (span (offset 173) (line 6) (column 13) (len 23))) (in-out (direction in) (declaration "resumed") (type (ref r3)) (redefines none) (value none) (span (offset 196) (line 7) (column 13) (len 28)))) (close-brace (span (offset 233) (line 8) (column 9) (len 1))))))))))
)
~~~
