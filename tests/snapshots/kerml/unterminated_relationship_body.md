# META
~~~sexpr
(snapshot (type recovery) (description "An explicit KerML relationship whose annotation body is open at end of input keeps its members and records (unterminated), so the formatter writes no } for it.") (name-language kerml))
~~~
# SOURCE
~~~sysml
package Unclosed {
    specialization Closed subtype A specializes B { doc /* closed */ }
    specialization Open subtype C specializes D {
        doc /* open */
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "unterminated_relationship_body.md"
    (diagnostics
      (diagnostic (code "missing_closing_brace") (severity none) (category parseerror) (span (offset 162) (line 4) (column 23) (len 1)) (message "missing closing '}'"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package Unclosed {
    specialization Closed subtype A specializes B {
        doc
        /* closed */
    }
    specialization Open subtype C specializes D {
        doc
        /* open */
~~~
# AST
~~~sexpr
(parsed-document
  (name-language kerml)
  (references
    (reference r0 (scope relative) (span (offset 53) (line 2) (column 35) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 53) (line 2) (column 35) (len 1)))))
    (reference r1 (scope relative) (span (offset 67) (line 2) (column 49) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 67) (line 2) (column 49) (len 1)))))
    (reference r2 (scope relative) (span (offset 122) (line 3) (column 33) (len 1)) (segments (segment 0 (token "C") (name "C") (separator none) (span (offset 122) (line 3) (column 33) (len 1)))))
    (reference r3 (scope relative) (span (offset 136) (line 3) (column 47) (len 1)) (segments (segment 0 (token "D") (name "D") (separator none) (span (offset 136) (line 3) (column 47) (len 1)))))
  )
  (root (package (name "Unclosed") (body brace (unterminated) (kerml-relationship (keyword subtype) (declaration-keyword true) (source (ref r0)) (target (ref r1))) (kerml-relationship (keyword subtype) (declaration-keyword true) (source (ref r2)) (target (ref r3))))))
)
~~~
