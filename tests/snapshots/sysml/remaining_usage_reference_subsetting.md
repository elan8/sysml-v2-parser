# META
~~~sexpr
(snapshot (type semantic) (description "Calculation, interface, and rendering usages retain both reference-subsetting spellings in named and anonymous declarations. Formatting preserves targets across typing, value, and connect clauses."))
~~~
# SOURCE
~~~sysml
package RemainingReferences {
    calc c1 ::> original;
    calc c2 references original;
    calc ::> original;
    calc references original;
    calc c3 ::> original : Calculation references other;
    interface i1 ::> original;
    interface i2 references original;
    interface ::> original;
    interface references original;
    interface i3 : Interface ::> original connect a to b;
    rendering r1 ::> original;
    rendering r2 references original;
    rendering ::> original;
    rendering references original;
    rendering r3 references original : Rendering ::> other = value;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "remaining_usage_reference_subsetting.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package RemainingReferences {
    calc c1 ::> original;
    calc c2 references original;
    calc ::> original;
    calc references original;
    calc c3 : Calculation ::> original, other;
    interface i1 ::> original;
    interface i2 references original;
    interface ::> original;
    interface references original;
    interface i3 : Interface ::> original connect a to b;
    rendering r1 ::> original;
    rendering r2 references original;
    rendering ::> original;
    rendering references original;
    rendering r3 : Rendering references original, other = value;
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 46) (line 2) (column 17) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 46) (line 2) (column 17) (len 8)))))
    (reference r1 (scope relative) (span (offset 79) (line 3) (column 24) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 79) (line 3) (column 24) (len 8)))))
    (reference r2 (scope relative) (span (offset 102) (line 4) (column 14) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 102) (line 4) (column 14) (len 8)))))
    (reference r3 (scope relative) (span (offset 132) (line 5) (column 21) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 132) (line 5) (column 21) (len 8)))))
    (reference r4 (scope relative) (span (offset 169) (line 6) (column 28) (len 11)) (segments (segment 0 (token "Calculation") (name "Calculation") (separator none) (span (offset 169) (line 6) (column 28) (len 11)))))
    (reference r5 (scope relative) (span (offset 158) (line 6) (column 17) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 158) (line 6) (column 17) (len 8)))))
    (reference r6 (scope relative) (span (offset 192) (line 6) (column 51) (len 5)) (segments (segment 0 (token "other") (name "other") (separator none) (span (offset 192) (line 6) (column 51) (len 5)))))
    (reference r7 (scope relative) (span (offset 220) (line 7) (column 22) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 220) (line 7) (column 22) (len 8)))))
    (reference r8 (scope relative) (span (offset 258) (line 8) (column 29) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 258) (line 8) (column 29) (len 8)))))
    (reference r9 (scope relative) (span (offset 286) (line 9) (column 19) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 286) (line 9) (column 19) (len 8)))))
    (reference r10 (scope relative) (span (offset 321) (line 10) (column 26) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 321) (line 10) (column 26) (len 8)))))
    (reference r11 (scope relative) (span (offset 350) (line 11) (column 20) (len 9)) (segments (segment 0 (token "Interface") (name "Interface") (separator none) (span (offset 350) (line 11) (column 20) (len 9)))))
    (reference r12 (scope relative) (span (offset 364) (line 11) (column 34) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 364) (line 11) (column 34) (len 8)))))
    (reference r13 (scope relative) (span (offset 381) (line 11) (column 51) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 381) (line 11) (column 51) (len 1)))))
    (reference r14 (scope relative) (span (offset 386) (line 11) (column 56) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 386) (line 11) (column 56) (len 1)))))
    (reference r15 (scope relative) (span (offset 410) (line 12) (column 22) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 410) (line 12) (column 22) (len 8)))))
    (reference r16 (scope relative) (span (offset 448) (line 13) (column 29) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 448) (line 13) (column 29) (len 8)))))
    (reference r17 (scope relative) (span (offset 476) (line 14) (column 19) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 476) (line 14) (column 19) (len 8)))))
    (reference r18 (scope relative) (span (offset 511) (line 15) (column 26) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 511) (line 15) (column 26) (len 8)))))
    (reference r19 (scope relative) (span (offset 560) (line 16) (column 40) (len 9)) (segments (segment 0 (token "Rendering") (name "Rendering") (separator none) (span (offset 560) (line 16) (column 40) (len 9)))))
    (reference r20 (scope relative) (span (offset 582) (line 16) (column 62) (len 5)) (segments (segment 0 (token "value") (name "value") (separator none) (span (offset 582) (line 16) (column 62) (len 5)))))
    (reference r21 (scope relative) (span (offset 549) (line 16) (column 29) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 549) (line 16) (column 29) (len 8)))))
    (reference r22 (scope relative) (span (offset 574) (line 16) (column 54) (len 5)) (segments (segment 0 (token "other") (name "other") (separator none) (span (offset 574) (line 16) (column 54) (len 5)))))
  )
  (root (package (visibility none) (name "RemainingReferences") (body brace (calc-usage (name "c1") (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (references (relationship (kind references) (implied false) (targets (ref r0)))) (body semicolon)) (calc-usage (name "c2") (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (references (relationship (kind references) (implied false) (targets (ref r1)))) (body semicolon)) (calc-usage (name none) (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (references (relationship (kind references) (implied false) (targets (ref r2)))) (body semicolon)) (calc-usage (name none) (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (references (relationship (kind references) (implied false) (targets (ref r3)))) (body semicolon)) (calc-usage (name "c3") (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (references (relationship (kind references) (implied false) (targets (ref r5) (ref r6)))) (body semicolon)) (interface-usage (form declaration) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "i1") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (references (relationship (kind references) (implied false) (targets (ref r7)))) (part none) (body semicolon)) (interface-usage (form declaration) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "i2") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (references (relationship (kind references) (implied false) (targets (ref r8)))) (part none) (body semicolon)) (interface-usage (form declaration) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name none) (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (references (relationship (kind references) (implied false) (targets (ref r9)))) (part none) (body semicolon)) (interface-usage (form declaration) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name none) (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (references (relationship (kind references) (implied false) (targets (ref r10)))) (part none) (body semicolon)) (interface-usage (form typed-connect) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "i3") (short-name none) (type (ref r11)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (references (relationship (kind references) (implied false) (targets (ref r12)))) (part (binary (from (interface-end (multiplicity none) (target (ref r13)))) (to (interface-end (multiplicity none) (target (ref r14)))))) (body semicolon)) (rendering-usage (abstract false) (name "r1") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (references (relationship (kind references) (implied false) (targets (ref r15)))) (body semicolon)) (rendering-usage (abstract false) (name "r2") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (references (relationship (kind references) (implied false) (targets (ref r16)))) (body semicolon)) (rendering-usage (abstract false) (name none) (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (references (relationship (kind references) (implied false) (targets (ref r17)))) (body semicolon)) (rendering-usage (abstract false) (name none) (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (references (relationship (kind references) (implied false) (targets (ref r18)))) (body semicolon)) (rendering-usage (abstract false) (name "r3") (short-name none) (type (ref r19)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 582) (line 16) (column 62) (len 5)) (ref r20))))) (references (relationship (kind references) (implied false) (targets (ref r21) (ref r22)))) (body semicolon)))))
)
~~~
