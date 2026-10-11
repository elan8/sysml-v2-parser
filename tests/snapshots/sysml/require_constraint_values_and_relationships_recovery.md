# META
~~~sexpr
(snapshot (type malformed) (description "Missing constraint value expressions and relationship targets become diagnostics and recovery nodes while later required constraints remain typed; unsupported intersects syntax is diagnosed instead of silently consumed."))
~~~
# SOURCE
~~~sysml
requirement def R {
    require constraint missingValue :> base = ;
    require constraint afterValue references target = true;
    assume constraint missingTarget references ;
    require constraint afterTarget crosses other default true;
    require constraint unsupported intersects target;
    require constraint afterUnsupported = true;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "require_constraint_values_and_relationships_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_requirement_body_element") (severity error) (category parseerror) (span (offset 24) (line 2) (column 5) (len 43)) (message "unexpected token in requirement body"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 132) (line 4) (column 5) (len 49)) (message "unexpected keyword `assume` in requirement body"))
      (diagnostic (code "recovered_requirement_body_element") (severity error) (category parseerror) (span (offset 244) (line 6) (column 5) (len 49)) (message "unexpected token in requirement body"))
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
    (reference r0 (scope relative) (span (offset 113) (line 3) (column 46) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 113) (line 3) (column 46) (len 6)))))
    (reference r1 (scope relative) (span (offset 220) (line 5) (column 44) (len 5)) (segments (segment 0 (token "other") (name "other") (separator none) (span (offset 220) (line 5) (column 44) (len 5)))))
  )
  (root (requirement-def (name "R") (modifiers) (body brace (malformed (code "recovered_requirement_body_element") (found "require constraint missingValue :> base = ;") (span (offset 24) (line 2) (column 5) (len 43))) (require-constraint (kind require) (constraint-keyword true) (name "afterValue") (target none) (specializations (reference-subsetting (relationship (kind references) (implied false) (targets (ref r0))))) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 122) (line 3) (column 55) (len 4)) (boolean true))))) (body semicolon)) (malformed (code "unexpected_keyword_in_scope") (found "assume constraint missingTarget references ;") (span (offset 132) (line 4) (column 5) (len 49))) (require-constraint (kind require) (constraint-keyword true) (name "afterTarget") (target none) (specializations (cross-subsetting (relationship (kind crosses) (implied false) (targets (ref r1))))) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 234) (line 5) (column 58) (len 4)) (boolean true))))) (body semicolon)) (malformed (code "recovered_requirement_body_element") (found "require constraint unsupported intersects target;") (span (offset 244) (line 6) (column 5) (len 49))) (require-constraint (kind require) (constraint-keyword true) (name "afterUnsupported") (target none) (specializations) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 336) (line 7) (column 43) (len 4)) (boolean true))))) (body semicolon)))))
)
~~~
