# META
~~~sexpr
(snapshot (type malformed) (description "A non-fatal redundant-semicolon warning before a genuine malformed member must not hide the later error or allow strict parsing to succeed. The valid following part remains in the recovered tree."))
~~~
# SOURCE
~~~sysml
package WarningThenError {
    part def Engine {};
    totally ???;
    part following : Engine;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "redundant_semicolon_before_error.md"
    (diagnostics
      (diagnostic (code "redundant_semicolon_after_body") (severity warning) (category parseerror) (span (offset 49) (line 2) (column 23) (len 1)) (message "possible redundant ';' after a closing '}': parsed as an empty anonymous usage"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 55) (line 3) (column 5) (len 12)) (message "unrecognized declaration `totally` in package body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package WarningThenError {
    part def Engine {
    }
    ;
    totally ???;
    part following : Engine;
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 89) (line 4) (column 22) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 89) (line 4) (column 22) (len 6)))))
  )
  (root (package (visibility none) (name "WarningThenError") (body brace (part-def (name "Engine") (modifiers) (body brace)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (malformed (code "unrecognized_declaration_in_scope") (found "totally ???;") (span (offset 55) (line 3) (column 5) (len 12))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "following") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))
)
~~~
