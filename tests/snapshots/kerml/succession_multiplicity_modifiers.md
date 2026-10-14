# META
~~~sexpr
(snapshot (type semantic) (description "Succession declaration multiplicity modifiers retain authored ordering and uniqueness, including anonymous declarations, both keyword orders, specialization clauses, quoted names, and distinct endpoint multiplicities and references.") (name-language kerml))
~~~
# SOURCE
~~~sysml
class C {
    succession s [*] ordered nonunique first a then b;
    succession reverse [0..1] nonunique ordered first a then b;
    succession ordered first a then b;
    succession nonunique first a then b;
    succession [*] ordered nonunique first a then b;
    succession : T ordered references prior first [1] a references original.a then [0..1] b references original.b;
    private succession 'ordered' [*] nonunique first a then b;
    succession a then b;
    succession first a then b;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "succession_multiplicity_modifiers.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
class C {
    succession s[*] ordered nonunique first a then b;
    succession reverse[0..1] nonunique ordered first a then b;
    succession ordered first a then b;
    succession nonunique first a then b;
    succession [*] ordered nonunique first a then b;
    succession : T references prior ordered first [1] a references original.a then [0..1] b references original.b;
    private succession 'ordered'[*] nonunique first a then b;
    succession a then b;
    succession first a then b;
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language kerml)
  (references
    (reference r0 (scope relative) (span (offset 55) (line 2) (column 46) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 55) (line 2) (column 46) (len 1)))))
    (reference r1 (scope relative) (span (offset 62) (line 2) (column 53) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 62) (line 2) (column 53) (len 1)))))
    (reference r2 (scope relative) (span (offset 119) (line 3) (column 55) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 119) (line 3) (column 55) (len 1)))))
    (reference r3 (scope relative) (span (offset 126) (line 3) (column 62) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 126) (line 3) (column 62) (len 1)))))
    (reference r4 (scope relative) (span (offset 158) (line 4) (column 30) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 158) (line 4) (column 30) (len 1)))))
    (reference r5 (scope relative) (span (offset 165) (line 4) (column 37) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 165) (line 4) (column 37) (len 1)))))
    (reference r6 (scope relative) (span (offset 199) (line 5) (column 32) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 199) (line 5) (column 32) (len 1)))))
    (reference r7 (scope relative) (span (offset 206) (line 5) (column 39) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 206) (line 5) (column 39) (len 1)))))
    (reference r8 (scope relative) (span (offset 252) (line 6) (column 44) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 252) (line 6) (column 44) (len 1)))))
    (reference r9 (scope relative) (span (offset 259) (line 6) (column 51) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 259) (line 6) (column 51) (len 1)))))
    (reference r10 (scope relative) (span (offset 279) (line 7) (column 18) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 279) (line 7) (column 18) (len 1)))))
    (reference r11 (scope relative) (span (offset 300) (line 7) (column 39) (len 5)) (segments (segment 0 (token "prior") (name "prior") (separator none) (span (offset 300) (line 7) (column 39) (len 5)))))
    (reference r12 (scope relative) (span (offset 316) (line 7) (column 55) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 316) (line 7) (column 55) (len 1)))))
    (reference r13 (scope relative) (span (offset 329) (line 7) (column 68) (len 10)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 329) (line 7) (column 68) (len 8))) (segment 1 (token "a") (name "a") (separator dot) (span (offset 338) (line 7) (column 77) (len 1)))))
    (reference r14 (scope relative) (span (offset 352) (line 7) (column 91) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 352) (line 7) (column 91) (len 1)))))
    (reference r15 (scope relative) (span (offset 365) (line 7) (column 104) (len 10)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 365) (line 7) (column 104) (len 8))) (segment 1 (token "b") (name "b") (separator dot) (span (offset 374) (line 7) (column 113) (len 1)))))
    (reference r16 (scope relative) (span (offset 430) (line 8) (column 54) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 430) (line 8) (column 54) (len 1)))))
    (reference r17 (scope relative) (span (offset 437) (line 8) (column 61) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 437) (line 8) (column 61) (len 1)))))
    (reference r18 (scope relative) (span (offset 455) (line 9) (column 16) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 455) (line 9) (column 16) (len 1)))))
    (reference r19 (scope relative) (span (offset 462) (line 9) (column 23) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 462) (line 9) (column 23) (len 1)))))
    (reference r20 (scope relative) (span (offset 486) (line 10) (column 22) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 486) (line 10) (column 22) (len 1)))))
    (reference r21 (scope relative) (span (offset 493) (line 10) (column 29) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 493) (line 10) (column 29) (len 1)))))
  )
  (root (kerml-classifier (keyword class) (abstract false) (name "C") (multiplicity none) (specializes none) (conjugates none) (body brace (kerml-succession (visibility none) (all false) (name "s") (specializations) (multiplicity (lower none) (upper (expression (span (offset 28) (line 2) (column 19) (len 1)) (infinity)))) (multiplicity-modifiers (ordering ordered) (uniqueness nonunique)) (first-keyword true) (first (connector-end (multiplicity none) (target (ref r0)) (references none))) (then (connector-end (multiplicity none) (target (ref r1)) (references none)))) (kerml-succession (visibility none) (all false) (name "reverse") (specializations) (multiplicity (lower (expression (span (offset 89) (line 3) (column 25) (len 1)) (integer 0))) (upper (expression (span (offset 92) (line 3) (column 28) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering ordered) (uniqueness nonunique)) (first-keyword true) (first (connector-end (multiplicity none) (target (ref r2)) (references none))) (then (connector-end (multiplicity none) (target (ref r3)) (references none)))) (kerml-succession (visibility none) (all false) (name none) (specializations) (multiplicity none) (multiplicity-modifiers (ordering ordered) (uniqueness none)) (first-keyword true) (first (connector-end (multiplicity none) (target (ref r4)) (references none))) (then (connector-end (multiplicity none) (target (ref r5)) (references none)))) (kerml-succession (visibility none) (all false) (name none) (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (first-keyword true) (first (connector-end (multiplicity none) (target (ref r6)) (references none))) (then (connector-end (multiplicity none) (target (ref r7)) (references none)))) (kerml-succession (visibility none) (all false) (name none) (specializations) (multiplicity (lower none) (upper (expression (span (offset 225) (line 6) (column 17) (len 1)) (infinity)))) (multiplicity-modifiers (ordering ordered) (uniqueness nonunique)) (first-keyword true) (first (connector-end (multiplicity none) (target (ref r8)) (references none))) (then (connector-end (multiplicity none) (target (ref r9)) (references none)))) (kerml-succession (visibility none) (all false) (name none) (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (reference-subsetting (relationship (kind references) (implied false) (targets (ref r11))))) (multiplicity none) (multiplicity-modifiers (ordering ordered) (uniqueness none)) (first-keyword true) (first (connector-end (multiplicity (lower none) (upper (expression (span (offset 313) (line 7) (column 52) (len 1)) (integer 1)))) (target (ref r12)) (references (ref r13)))) (then (connector-end (multiplicity (lower (expression (span (offset 346) (line 7) (column 85) (len 1)) (integer 0))) (upper (expression (span (offset 349) (line 7) (column 88) (len 1)) (integer 1)))) (target (ref r14)) (references (ref r15))))) (kerml-succession (visibility private) (all false) (name "ordered") (specializations) (multiplicity (lower none) (upper (expression (span (offset 411) (line 8) (column 35) (len 1)) (infinity)))) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (first-keyword true) (first (connector-end (multiplicity none) (target (ref r16)) (references none))) (then (connector-end (multiplicity none) (target (ref r17)) (references none)))) (kerml-succession (visibility none) (all false) (name none) (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (first-keyword false) (first (connector-end (multiplicity none) (target (ref r18)) (references none))) (then (connector-end (multiplicity none) (target (ref r19)) (references none)))) (kerml-succession (visibility none) (all false) (name none) (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (first-keyword true) (first (connector-end (multiplicity none) (target (ref r20)) (references none))) (then (connector-end (multiplicity none) (target (ref r21)) (references none)))))))
)
~~~
