# META
~~~sexpr
(snapshot (type recovery) (description "An alias body open at end of input reads (unterminated) like every other brace body, so the projection shows that no } was authored and the formatter writes none."))
~~~
# SOURCE
~~~sysml
package Unclosed {
    alias Closed for Lib::A { }
    alias Open for Lib::B {
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "unterminated_alias_body.md"
    (diagnostics
      (diagnostic (code "missing_closing_brace") (severity none) (category parseerror) (span (offset 78) (line 3) (column 28) (len 1)) (message "missing closing '}'"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package Unclosed {
    alias Closed for Lib::A {}
    alias Open for Lib::B {
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 40) (line 2) (column 22) (len 6)) (segments (segment 0 (token "Lib") (name "Lib") (separator none) (span (offset 40) (line 2) (column 22) (len 3))) (segment 1 (token "A") (name "A") (separator colon-colon) (span (offset 45) (line 2) (column 27) (len 1)))))
    (reference r1 (scope relative) (span (offset 70) (line 3) (column 20) (len 6)) (segments (segment 0 (token "Lib") (name "Lib") (separator none) (span (offset 70) (line 3) (column 20) (len 3))) (segment 1 (token "B") (name "B") (separator colon-colon) (span (offset 75) (line 3) (column 25) (len 1)))))
  )
  (root (package (name "Unclosed") (body brace (unterminated) (alias (name "Closed") (target (ref r0)) (body brace (element-count 0))) (alias (name "Open") (target (ref r1)) (body brace (unterminated) (element-count 0))))))
)
~~~
