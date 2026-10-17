# META
~~~sexpr
(snapshot (type semantic) (description "Issue 200 case 3: every scalar-typing usage family retains all targets, their order and authored typing operator, including rendering and framed concern declarations."))
~~~
# SOURCE
~~~sysml
package TypingLists {
    item def A;
    item def B;
    item cargo : A, B;
    item tagged typed by B, A;
    enum choices : A, B;
    requirement needs : A, B;
    concern safety : A, B;
    case c : A, B;
    analysis a : A, B;
    verification v : A, B;
    use case u : A, B;
    viewpoint vp : A, B;
    allocation assigned : A, B;
    view report {
        render rendering renderer : A, B;
        render renderer typed by B, A;
    }
    requirement def R {
        frame concern related : A, B;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "usage_typing_lists.md"
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
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 71) (line 4) (column 18) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 71) (line 4) (column 18) (len 1)))))
    (reference r1 (scope relative) (span (offset 74) (line 4) (column 21) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 74) (line 4) (column 21) (len 1)))))
    (reference r2 (scope relative) (span (offset 102) (line 5) (column 26) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 102) (line 5) (column 26) (len 1)))))
    (reference r3 (scope relative) (span (offset 105) (line 5) (column 29) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 105) (line 5) (column 29) (len 1)))))
    (reference r4 (scope relative) (span (offset 127) (line 6) (column 20) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 127) (line 6) (column 20) (len 1)))))
    (reference r5 (scope relative) (span (offset 130) (line 6) (column 23) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 130) (line 6) (column 23) (len 1)))))
    (reference r6 (scope relative) (span (offset 157) (line 7) (column 25) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 157) (line 7) (column 25) (len 1)))))
    (reference r7 (scope relative) (span (offset 160) (line 7) (column 28) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 160) (line 7) (column 28) (len 1)))))
    (reference r8 (scope relative) (span (offset 184) (line 8) (column 22) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 184) (line 8) (column 22) (len 1)))))
    (reference r9 (scope relative) (span (offset 187) (line 8) (column 25) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 187) (line 8) (column 25) (len 1)))))
    (reference r10 (scope relative) (span (offset 203) (line 9) (column 14) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 203) (line 9) (column 14) (len 1)))))
    (reference r11 (scope relative) (span (offset 206) (line 9) (column 17) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 206) (line 9) (column 17) (len 1)))))
    (reference r12 (scope relative) (span (offset 226) (line 10) (column 18) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 226) (line 10) (column 18) (len 1)))))
    (reference r13 (scope relative) (span (offset 229) (line 10) (column 21) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 229) (line 10) (column 21) (len 1)))))
    (reference r14 (scope relative) (span (offset 253) (line 11) (column 22) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 253) (line 11) (column 22) (len 1)))))
    (reference r15 (scope relative) (span (offset 256) (line 11) (column 25) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 256) (line 11) (column 25) (len 1)))))
    (reference r16 (scope relative) (span (offset 276) (line 12) (column 18) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 276) (line 12) (column 18) (len 1)))))
    (reference r17 (scope relative) (span (offset 279) (line 12) (column 21) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 279) (line 12) (column 21) (len 1)))))
    (reference r18 (scope relative) (span (offset 301) (line 13) (column 20) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 301) (line 13) (column 20) (len 1)))))
    (reference r19 (scope relative) (span (offset 304) (line 13) (column 23) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 304) (line 13) (column 23) (len 1)))))
    (reference r20 (scope relative) (span (offset 333) (line 14) (column 27) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 333) (line 14) (column 27) (len 1)))))
    (reference r21 (scope relative) (span (offset 336) (line 14) (column 30) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 336) (line 14) (column 30) (len 1)))))
    (reference r22 (scope relative) (span (offset 393) (line 16) (column 37) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 393) (line 16) (column 37) (len 1)))))
    (reference r23 (scope relative) (span (offset 396) (line 16) (column 40) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 396) (line 16) (column 40) (len 1)))))
    (reference r24 (scope relative) (span (offset 414) (line 17) (column 16) (len 8)) (segments (segment 0 (token "renderer") (name "renderer") (separator none) (span (offset 414) (line 17) (column 16) (len 8)))))
    (reference r25 (scope relative) (span (offset 432) (line 17) (column 34) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 432) (line 17) (column 34) (len 1)))))
    (reference r26 (scope relative) (span (offset 435) (line 17) (column 37) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 435) (line 17) (column 37) (len 1)))))
    (reference r27 (scope relative) (span (offset 500) (line 20) (column 33) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 500) (line 20) (column 33) (len 1)))))
    (reference r28 (scope relative) (span (offset 503) (line 20) (column 36) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 503) (line 20) (column 36) (len 1)))))
  )
  (root (package (name "TypingLists") (body brace (item-def (name "A") (modifiers) (individual false) (specializes none) (body semicolon)) (item-def (name "B") (modifiers) (individual false) (specializes none) (body semicolon)) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "cargo") (short-name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r0) (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "tagged") (short-name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r2) (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (enumeration-usage (name "choices") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4) (ref r5)))) (body semicolon)) (requirement-usage (name "needs") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6) (ref r7)))) (abstract false) (variation false) none (subsets none) (references none) (body semicolon)) (concern-usage (name "safety") (short-name none) (visibility none) (abstract false) (individual false) (definition false) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r8) (ref r9)))) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (case-usage (declaration (name "c") (short-name none) (abstract false) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r10) (ref r11)))) (subsets none)) (body semicolon)) (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "a") (short-name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r12) (ref r13)))) (subsets none) (redefines none) (body semicolon)) (verification-case-usage (declaration (name "v") (short-name none) (abstract false) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r14) (ref r15)))) (subsets none)) (body semicolon)) (use-case-usage (declaration (name "u") (short-name none) (abstract false) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r16) (ref r17)))) (subsets none)) (variation false) (body semicolon)) (viewpoint-usage (name "vp") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r18) (ref r19)))) (body semicolon)) (allocation-usage (name "assigned") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r20) (ref r21)))) (ends) (body semicolon)) (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "report") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body brace (view-rendering (form inline "renderer") (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r22) (ref r23)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (view-rendering (form reference (ref r24)) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r25) (ref r26)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))) (requirement-def (name "R") (modifiers) (body brace (frame (concern-keyword true) (name "related") (short-name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r27) (ref r28)))) (body semicolon)))))))
)
~~~
