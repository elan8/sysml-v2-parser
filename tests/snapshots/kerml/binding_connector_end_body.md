# META
~~~sexpr
(snapshot (type provenance) (description "KerML BindingConnectorDeclaration may omit its inline binary pair and place connector ends in TypeBody. A three-ended binding remains one typed connector with three ordered body-owned end features; inline declared/anonymous and `all` alternatives retain their distinct syntax. KerML textual BNF 875-881; Pilot KerML.xtext 870-881."))
~~~
# SOURCE
~~~sysml
package BindingConnectorEnds {
    classifier Thing;
    classifier Holder {
        feature a : Thing;
        feature b : Thing;
        feature c : Thing;
        binding tern {
            end feature e1 :>> a;
            end feature e2 :>> b;
            end feature e3 :>> c;
        }
        binding pair of a = b;
        binding of b = c;
        binding b = c;
        binding often = a;
        binding all of c = a;
        binding all c = a;
        binding declaredOnly[1];
        binding all;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "binding_connector_end_body.md"
    (diagnostics
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
    (reference r0 (scope relative) (span (offset 97) (line 4) (column 21) (len 5)) (segments (segment 0 (token "Thing") (name "Thing") (separator none) (span (offset 97) (line 4) (column 21) (len 5)))))
    (reference r1 (scope relative) (span (offset 124) (line 5) (column 21) (len 5)) (segments (segment 0 (token "Thing") (name "Thing") (separator none) (span (offset 124) (line 5) (column 21) (len 5)))))
    (reference r2 (scope relative) (span (offset 151) (line 6) (column 21) (len 5)) (segments (segment 0 (token "Thing") (name "Thing") (separator none) (span (offset 151) (line 6) (column 21) (len 5)))))
    (reference r3 (scope relative) (span (offset 212) (line 8) (column 32) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 212) (line 8) (column 32) (len 1)))))
    (reference r4 (scope relative) (span (offset 246) (line 9) (column 32) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 246) (line 9) (column 32) (len 1)))))
    (reference r5 (scope relative) (span (offset 280) (line 10) (column 32) (len 1)) (segments (segment 0 (token "c") (name "c") (separator none) (span (offset 280) (line 10) (column 32) (len 1)))))
    (reference r6 (scope relative) (span (offset 317) (line 12) (column 25) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 317) (line 12) (column 25) (len 1)))))
    (reference r7 (scope relative) (span (offset 321) (line 12) (column 29) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 321) (line 12) (column 29) (len 1)))))
    (reference r8 (scope relative) (span (offset 343) (line 13) (column 20) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 343) (line 13) (column 20) (len 1)))))
    (reference r9 (scope relative) (span (offset 347) (line 13) (column 24) (len 1)) (segments (segment 0 (token "c") (name "c") (separator none) (span (offset 347) (line 13) (column 24) (len 1)))))
    (reference r10 (scope relative) (span (offset 366) (line 14) (column 17) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 366) (line 14) (column 17) (len 1)))))
    (reference r11 (scope relative) (span (offset 370) (line 14) (column 21) (len 1)) (segments (segment 0 (token "c") (name "c") (separator none) (span (offset 370) (line 14) (column 21) (len 1)))))
    (reference r12 (scope relative) (span (offset 389) (line 15) (column 17) (len 5)) (segments (segment 0 (token "often") (name "often") (separator none) (span (offset 389) (line 15) (column 17) (len 5)))))
    (reference r13 (scope relative) (span (offset 397) (line 15) (column 25) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 397) (line 15) (column 25) (len 1)))))
    (reference r14 (scope relative) (span (offset 423) (line 16) (column 24) (len 1)) (segments (segment 0 (token "c") (name "c") (separator none) (span (offset 423) (line 16) (column 24) (len 1)))))
    (reference r15 (scope relative) (span (offset 427) (line 16) (column 28) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 427) (line 16) (column 28) (len 1)))))
    (reference r16 (scope relative) (span (offset 450) (line 17) (column 21) (len 1)) (segments (segment 0 (token "c") (name "c") (separator none) (span (offset 450) (line 17) (column 21) (len 1)))))
    (reference r17 (scope relative) (span (offset 454) (line 17) (column 25) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 454) (line 17) (column 25) (len 1)))))
  )
  (root (package (name "BindingConnectorEnds") (body brace (kerml-classifier (keyword classifier) (abstract false) (name "Thing") (specializes none) (conjugates none) (body semicolon)) (kerml-classifier (keyword classifier) (abstract false) (name "Holder") (specializes none) (conjugates none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "a") (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "b") (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "c") (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (binding (all false) (name "tern") (multiplicity none) (inline-ends none) (body brace (kerml-feature (prefix (head end) (constant false) (cross none) (metadata)) (kind feature) (member false) (all false) (name "e1") (specializations (redefinition (relationship (kind redefines) (implied false) (targets (ref r3))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head end) (constant false) (cross none) (metadata)) (kind feature) (member false) (all false) (name "e2") (specializations (redefinition (relationship (kind redefines) (implied false) (targets (ref r4))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head end) (constant false) (cross none) (metadata)) (kind feature) (member false) (all false) (name "e3") (specializations (redefinition (relationship (kind redefines) (implied false) (targets (ref r5))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)))) (binding (all false) (name "pair") (multiplicity none) (inline-ends (pair (of true) (left (connector-end (multiplicity none) (target (ref r6)) (references none))) (right (connector-end (multiplicity none) (target (ref r7)) (references none))))) (body semicolon)) (binding (all false) (name none) (multiplicity none) (inline-ends (pair (of true) (left (connector-end (multiplicity none) (target (ref r8)) (references none))) (right (connector-end (multiplicity none) (target (ref r9)) (references none))))) (body semicolon)) (binding (all false) (name none) (multiplicity none) (inline-ends (pair (of false) (left (connector-end (multiplicity none) (target (ref r10)) (references none))) (right (connector-end (multiplicity none) (target (ref r11)) (references none))))) (body semicolon)) (binding (all false) (name none) (multiplicity none) (inline-ends (pair (of false) (left (connector-end (multiplicity none) (target (ref r12)) (references none))) (right (connector-end (multiplicity none) (target (ref r13)) (references none))))) (body semicolon)) (binding (all true) (name none) (multiplicity none) (inline-ends (pair (of true) (left (connector-end (multiplicity none) (target (ref r14)) (references none))) (right (connector-end (multiplicity none) (target (ref r15)) (references none))))) (body semicolon)) (binding (all true) (name none) (multiplicity none) (inline-ends (pair (of false) (left (connector-end (multiplicity none) (target (ref r16)) (references none))) (right (connector-end (multiplicity none) (target (ref r17)) (references none))))) (body semicolon)) (binding (all false) (name "declaredOnly") (multiplicity (lower (expression (span (offset 486) (line 18) (column 30) (len 1)) (integer 1))) (upper (expression (span (offset 486) (line 18) (column 30) (len 1)) (integer 1)))) (inline-ends none) (body semicolon)) (binding (all true) (name none) (multiplicity none) (inline-ends none) (body semicolon)))))))
)
~~~
