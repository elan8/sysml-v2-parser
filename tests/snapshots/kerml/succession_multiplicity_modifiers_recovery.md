# META
~~~sexpr
(snapshot (type malformed) (description "Duplicate succession declaration modifier slots and missing ends produce recovery nodes while later succession and feature siblings remain typed.") (name-language kerml))
~~~
# SOURCE
~~~sysml
class C {
    succession bad [*] ordered ordered first a then b;
    succession afterOrdering [*] ordered first a then b;
    succession badUnique [*] nonunique nonunique first a then b;
    succession afterUniqueness [*] nonunique first a then b;
    succession badEnd [*] ordered first a then ;
    feature afterEnd;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "succession_multiplicity_modifiers_recovery.md"
    (diagnostics
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 14) (line 2) (column 5) (len 55)) (message "unexpected keyword `succession` in calc body"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 126) (line 4) (column 5) (len 65)) (message "unexpected keyword `succession` in calc body"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 252) (line 6) (column 5) (len 49)) (message "unexpected keyword `succession` in calc body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
class C {
    succession bad [*] ordered ordered first a then b;
    succession afterOrdering[*] ordered first a then b;
    succession badUnique [*] nonunique nonunique first a then b;
    succession afterUniqueness[*] nonunique first a then b;
    succession badEnd [*] ordered first a then ;
    feature afterEnd;
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language kerml)
  (references
    (reference r0 (scope relative) (span (offset 112) (line 3) (column 48) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 112) (line 3) (column 48) (len 1)))))
    (reference r1 (scope relative) (span (offset 119) (line 3) (column 55) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 119) (line 3) (column 55) (len 1)))))
    (reference r2 (scope relative) (span (offset 238) (line 5) (column 52) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 238) (line 5) (column 52) (len 1)))))
    (reference r3 (scope relative) (span (offset 245) (line 5) (column 59) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 245) (line 5) (column 59) (len 1)))))
  )
  (root (kerml-classifier (keyword class) (abstract false) (name "C") (multiplicity none) (specializes none) (conjugates none) (body brace (malformed (code "unexpected_keyword_in_scope") (found "succession bad [*] ordered ordered first a then b;") (span (offset 14) (line 2) (column 5) (len 55))) (kerml-succession (visibility none) (all false) (name "afterOrdering") (specializations) (multiplicity (lower none) (upper (expression (span (offset 95) (line 3) (column 31) (len 1)) (infinity)))) (multiplicity-modifiers (ordering ordered) (uniqueness none)) (first-keyword true) (first (connector-end (multiplicity none) (target (ref r0)) (references none))) (then (connector-end (multiplicity none) (target (ref r1)) (references none)))) (malformed (code "unexpected_keyword_in_scope") (found "succession badUnique [*] nonunique nonunique first a then b;") (span (offset 126) (line 4) (column 5) (len 65))) (kerml-succession (visibility none) (all false) (name "afterUniqueness") (specializations) (multiplicity (lower none) (upper (expression (span (offset 219) (line 5) (column 33) (len 1)) (infinity)))) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (first-keyword true) (first (connector-end (multiplicity none) (target (ref r2)) (references none))) (then (connector-end (multiplicity none) (target (ref r3)) (references none)))) (malformed (code "unexpected_keyword_in_scope") (found "succession badEnd [*] ordered first a then ;") (span (offset 252) (line 6) (column 5) (len 49))) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "afterEnd") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)))))
)
~~~
