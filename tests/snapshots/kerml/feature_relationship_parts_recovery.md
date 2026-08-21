# META
~~~sexpr
(snapshot (type recovery) (description "A malformed TypeFeaturingPart fails the containing feature transaction, retains its exact recovery text, and resumes at the next feature member."))
~~~
# SOURCE
~~~sysml
package P {
    class C {
        feature broken : T featured by ;
        feature retained : T featured by A;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "feature_relationship_parts_recovery.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 34) (line 3) (column 9) (len 41)) (message "unrecognized declaration `feature` in calc body"))
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
    (reference r0 (scope relative) (span (offset 94) (line 4) (column 28) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 94) (line 4) (column 28) (len 1)))))
    (reference r1 (scope relative) (span (offset 108) (line 4) (column 42) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 108) (line 4) (column 42) (len 1)))))
  )
  (root (package (name "P") (body brace (kerml-classifier (keyword class) (abstract false) (name "C") (specializes none) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "feature broken : T featured by ;") (span (offset 34) (line 3) (column 9) (len 41))) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "retained") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships (featured-by (ref r1))) (value none) (body semicolon)))))))
)
~~~
