# META
~~~sexpr
(snapshot (type semantic) (description "KerML successions in a type body with every SuccessionDeclaration shape: ends only, the `first` keyword with no declaration, a named declaration, and a declaration of specializations alone, with a multiplicity directly after `then` (#163). Documentation whose Identification is a short name alone keeps its own body (#164)."))
~~~
# SOURCE
~~~sysml
package Sequences {
    class Finishing {
        doc <a> /* Documentation comment on Finishing */
        doc named /* Named documentation */
        doc <s> both /* Short and declared name */
        doc /* Plain documentation */
        feature paint;
        feature dry;
        feature ship;
        succession paint then dry;
        succession first paint then dry;
        succession ordered first [0..1] paint then [1] dry;
        succession redefines p_before_d : Before [1] first paint then dry;
        succession first [1] dry then[0..1] ship;
        private succession all [*] paint then [*] ship;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "succession_declarations_and_named_documentation.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Sequences {
    class Finishing {
        doc <a>
        /* Documentation comment on Finishing */
        doc named
        /* Named documentation */
        doc <s> both
        /* Short and declared name */
        doc
        /* Plain documentation */
        feature paint;
        feature dry;
        feature ship;
        succession paint then dry;
        succession first paint then dry;
        succession ordered first [0..1] paint then [1] dry;
        succession redefines p_before_d : Before[1] first paint then dry;
        succession first [1] dry then [0..1] ship;
        private succession all [*] paint then [*] ship;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 317) (line 10) (column 20) (len 5)) (segments (segment 0 (token "paint") (name "paint") (separator none) (span (offset 317) (line 10) (column 20) (len 5)))))
    (reference r1 (scope relative) (span (offset 328) (line 10) (column 31) (len 3)) (segments (segment 0 (token "dry") (name "dry") (separator none) (span (offset 328) (line 10) (column 31) (len 3)))))
    (reference r2 (scope relative) (span (offset 358) (line 11) (column 26) (len 5)) (segments (segment 0 (token "paint") (name "paint") (separator none) (span (offset 358) (line 11) (column 26) (len 5)))))
    (reference r3 (scope relative) (span (offset 369) (line 11) (column 37) (len 3)) (segments (segment 0 (token "dry") (name "dry") (separator none) (span (offset 369) (line 11) (column 37) (len 3)))))
    (reference r4 (scope relative) (span (offset 414) (line 12) (column 41) (len 5)) (segments (segment 0 (token "paint") (name "paint") (separator none) (span (offset 414) (line 12) (column 41) (len 5)))))
    (reference r5 (scope relative) (span (offset 429) (line 12) (column 56) (len 3)) (segments (segment 0 (token "dry") (name "dry") (separator none) (span (offset 429) (line 12) (column 56) (len 3)))))
    (reference r6 (scope relative) (span (offset 463) (line 13) (column 30) (len 10)) (segments (segment 0 (token "p_before_d") (name "p_before_d") (separator none) (span (offset 463) (line 13) (column 30) (len 10)))))
    (reference r7 (scope relative) (span (offset 476) (line 13) (column 43) (len 6)) (segments (segment 0 (token "Before") (name "Before") (separator none) (span (offset 476) (line 13) (column 43) (len 6)))))
    (reference r8 (scope relative) (span (offset 493) (line 13) (column 60) (len 5)) (segments (segment 0 (token "paint") (name "paint") (separator none) (span (offset 493) (line 13) (column 60) (len 5)))))
    (reference r9 (scope relative) (span (offset 504) (line 13) (column 71) (len 3)) (segments (segment 0 (token "dry") (name "dry") (separator none) (span (offset 504) (line 13) (column 71) (len 3)))))
    (reference r10 (scope relative) (span (offset 538) (line 14) (column 30) (len 3)) (segments (segment 0 (token "dry") (name "dry") (separator none) (span (offset 538) (line 14) (column 30) (len 3)))))
    (reference r11 (scope relative) (span (offset 553) (line 14) (column 45) (len 4)) (segments (segment 0 (token "ship") (name "ship") (separator none) (span (offset 553) (line 14) (column 45) (len 4)))))
    (reference r12 (scope relative) (span (offset 594) (line 15) (column 36) (len 5)) (segments (segment 0 (token "paint") (name "paint") (separator none) (span (offset 594) (line 15) (column 36) (len 5)))))
    (reference r13 (scope relative) (span (offset 609) (line 15) (column 51) (len 4)) (segments (segment 0 (token "ship") (name "ship") (separator none) (span (offset 609) (line 15) (column 51) (len 4)))))
  )
  (root (package (name "Sequences") (body brace (kerml-classifier (keyword class) (abstract false) (name "Finishing") (multiplicity none) (specializes none) (conjugates none) (body brace (doc (name none) (short-name "a") (locale none) (body (span (offset 60) (line 3) (column 19) (len 36)) (normalized "Documentation comment on Finishing "))) (doc (name "named") (short-name none) (locale none) (body (span (offset 119) (line 4) (column 21) (len 21)) (normalized "Named documentation "))) (doc (name "both") (short-name "s") (locale none) (body (span (offset 166) (line 5) (column 24) (len 25)) (normalized "Short and declared name "))) (doc (name none) (short-name none) (locale none) (body (span (offset 208) (line 6) (column 15) (len 21)) (normalized "Plain documentation "))) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "paint") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "dry") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "ship") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-succession (visibility none) (all false) (name none) (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (first-keyword false) (first (connector-end (multiplicity none) (target (ref r0)) (references none))) (then (connector-end (multiplicity none) (target (ref r1)) (references none)))) (kerml-succession (visibility none) (all false) (name none) (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (first-keyword true) (first (connector-end (multiplicity none) (target (ref r2)) (references none))) (then (connector-end (multiplicity none) (target (ref r3)) (references none)))) (kerml-succession (visibility none) (all false) (name none) (specializations) (multiplicity none) (multiplicity-modifiers (ordering ordered) (uniqueness none)) (first-keyword true) (first (connector-end (multiplicity (lower (expression (span (offset 408) (line 12) (column 35) (len 1)) (integer 0))) (upper (expression (span (offset 411) (line 12) (column 38) (len 1)) (integer 1)))) (target (ref r4)) (references none))) (then (connector-end (multiplicity (lower none) (upper (expression (span (offset 426) (line 12) (column 53) (len 1)) (integer 1)))) (target (ref r5)) (references none)))) (kerml-succession (visibility none) (all false) (name none) (specializations (redefinition (relationship (kind redefines) (implied false) (targets (ref r6)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7))))) (multiplicity (lower none) (upper (expression (span (offset 484) (line 13) (column 51) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (first-keyword true) (first (connector-end (multiplicity none) (target (ref r8)) (references none))) (then (connector-end (multiplicity none) (target (ref r9)) (references none)))) (kerml-succession (visibility none) (all false) (name none) (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (first-keyword true) (first (connector-end (multiplicity (lower none) (upper (expression (span (offset 535) (line 14) (column 27) (len 1)) (integer 1)))) (target (ref r10)) (references none))) (then (connector-end (multiplicity (lower (expression (span (offset 547) (line 14) (column 39) (len 1)) (integer 0))) (upper (expression (span (offset 550) (line 14) (column 42) (len 1)) (integer 1)))) (target (ref r11)) (references none)))) (kerml-succession (visibility private) (all true) (name none) (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (first-keyword false) (first (connector-end (multiplicity (lower none) (upper (expression (span (offset 591) (line 15) (column 33) (len 1)) (infinity)))) (target (ref r12)) (references none))) (then (connector-end (multiplicity (lower none) (upper (expression (span (offset 606) (line 15) (column 48) (len 1)) (infinity)))) (target (ref r13)) (references none)))))))))
)
~~~
