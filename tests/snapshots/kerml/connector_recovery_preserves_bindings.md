# META
~~~sexpr
(snapshot (type recovery) (description "KerML type-body recovery stops after the malformed connector body and preserves both following binding members.") (name-language kerml))
~~~
# SOURCE
~~~sysml
package Connectors {
    class A {
        feature a : A;
        feature b : A;
        connector c1 from a to b;
        abstract connector c2 = c1;
        connector = c2 {
            end feature references a;
            end feature references b;
        }
        binding a = b;
        binding ab of a = b;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "connector_recovery_preserves_bindings.md"
    (diagnostics
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 123) (line 6) (column 9) (len 36)) (message "unexpected keyword `abstract` in calc body"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 159) (line 7) (column 9) (len 111)) (message "unexpected keyword `connector` in calc body"))
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
  (name-language kerml)
  (references
    (reference r0 (scope relative) (span (offset 55) (line 3) (column 21) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 55) (line 3) (column 21) (len 1)))))
    (reference r1 (scope relative) (span (offset 78) (line 4) (column 21) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 78) (line 4) (column 21) (len 1)))))
    (reference r2 (scope relative) (span (offset 107) (line 5) (column 27) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 107) (line 5) (column 27) (len 1)))))
    (reference r3 (scope relative) (span (offset 112) (line 5) (column 32) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 112) (line 5) (column 32) (len 1)))))
    (reference r4 (scope relative) (span (offset 278) (line 11) (column 17) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 278) (line 11) (column 17) (len 1)))))
    (reference r5 (scope relative) (span (offset 282) (line 11) (column 21) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 282) (line 11) (column 21) (len 1)))))
    (reference r6 (scope relative) (span (offset 307) (line 12) (column 23) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 307) (line 12) (column 23) (len 1)))))
    (reference r7 (scope relative) (span (offset 311) (line 12) (column 27) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 311) (line 12) (column 27) (len 1)))))
  )
  (root (package (visibility none) (name "Connectors") (body brace (kerml-classifier (keyword class) (abstract false) (name "A") (multiplicity none) (specializes none) (conjugates none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "a") (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "b") (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-connector (all false) (name "c1") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (from (connector-end (multiplicity none) (target (ref r2)) (references none))) (to (connector-end (multiplicity none) (target (ref r3)) (references none))) (body semicolon)) (malformed (code "unexpected_keyword_in_scope") (found "abstract connector c2 = c1;") (span (offset 123) (line 6) (column 9) (len 36))) (malformed (code "unexpected_keyword_in_scope") (found "connector = c2 {") (span (offset 159) (line 7) (column 9) (len 111))) (binding (all false) (name none) (multiplicity none) (inline-ends (pair (of false) (left (connector-end (multiplicity none) (target (ref r4)) (references none))) (right (connector-end (multiplicity none) (target (ref r5)) (references none))))) (body semicolon)) (binding (all false) (name "ab") (multiplicity none) (inline-ends (pair (of true) (left (connector-end (multiplicity none) (target (ref r6)) (references none))) (right (connector-end (multiplicity none) (target (ref r7)) (references none))))) (body semicolon)))))))
)
~~~
