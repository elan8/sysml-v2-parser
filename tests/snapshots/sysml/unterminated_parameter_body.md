# META
~~~sexpr
(snapshot (type recovery) (description "A parameter's brace body open at end of input keeps its members and records (unterminated), so the formatter writes no } for it. The closed parameter before it keeps its }."))
~~~
# SOURCE
~~~sysml
action def Compute {
    in closed : Real { doc /* closed */ }
    in open : Real {
        @Tool {
            name
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "unterminated_parameter_body.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 112) (line 5) (column 13) (len 4)) (message "unrecognized declaration `name` in metadata body"))
      (diagnostic (code "missing_closing_brace") (severity none) (category parseerror) (span (offset 116) (line 5) (column 17) (len 1)) (message "missing closing '}'"))
    )
  )
)
~~~
# FORMAT
~~~sysml
action def Compute {
    in closed : Real {
        doc
        /* closed */
    }
    in open : Real {
        @Tool {
            name
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 37) (line 2) (column 17) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 37) (line 2) (column 17) (len 4)))))
    (reference r1 (scope relative) (span (offset 77) (line 3) (column 15) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 77) (line 3) (column 15) (len 4)))))
    (reference r2 (scope relative) (span (offset 93) (line 4) (column 10) (len 4)) (segments (segment 0 (token "Tool") (name "Tool") (separator none) (span (offset 93) (line 4) (column 10) (len 4)))))
  )
  (root (action-def (name "Compute") (modifiers) (specializes none) (body brace (unterminated) (in-out (direction in) (kind none) (reference false) (declaration "closed") (short-name none) (subsets none) (type (ref r0)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (body brace (doc (name none) (short-name none) (locale none) (body (span (offset 50) (line 2) (column 30) (len 8)) (normalized "closed ")))) (span (offset 25) (line 2) (column 5) (len 37))) (in-out (direction in) (kind none) (reference false) (declaration "open") (short-name none) (subsets none) (type (ref r1)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (body brace (unterminated) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r2)) (about) (body brace (unterminated) (malformed (code "unrecognized_declaration_in_scope") (found "name") (span (offset 112) (line 5) (column 13) (len 4)))))) (span (offset 67) (line 3) (column 5) (len 49))))))
)
~~~
