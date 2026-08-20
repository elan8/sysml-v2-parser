# META
~~~sexpr
(snapshot (type semantic) (description "A subsetting-family clause written more than once in one usage header names further targets of the same relationship, so the targets accumulate in source order instead of the last clause overwriting the earlier ones. The keyword and operator spellings are the same relationship, and different clause kinds stay separate. Attribute usages carry these cases because their projection names the relationship and its targets; the one case that needs a clause before the typing has to be an item usage, whose body-element projection is still a bare marker, so only the emitted form covers it."))
~~~
# SOURCE
~~~sysml
package SpecializationClauseAccumulation {
    part def D {
        attribute repeatedSubsets : T[0..*] ordered subsets a, b subsets c;
        attribute repeatedRedefines : T redefines a redefines b;
        attribute mixedSpellings : T subsets base :> latest;
        attribute differentKinds : T subsets a redefines b;
        attribute libraryShape : ActionUsage[0..*] ordered subsets step, usage subsets Metadata::metadataItems;
        item aroundTheTyping :> base : T :> latest;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "specialization_clause_accumulation.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package SpecializationClauseAccumulation {
    part def D {
        attribute repeatedSubsets : T[0..*] ordered :> a, b, c;
        attribute repeatedRedefines : T :>> a, b;
        attribute mixedSpellings : T :> base, latest;
        attribute differentKinds : T :> a :>> b;
        attribute libraryShape : ActionUsage[0..*] ordered :> step, usage, Metadata::metadataItems;
        item aroundTheTyping : T :> base, latest;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 96) (line 3) (column 37) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 96) (line 3) (column 37) (len 1)))))
    (reference r1 (scope relative) (span (offset 120) (line 3) (column 61) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 120) (line 3) (column 61) (len 1)))))
    (reference r2 (scope relative) (span (offset 123) (line 3) (column 64) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 123) (line 3) (column 64) (len 1)))))
    (reference r3 (scope relative) (span (offset 133) (line 3) (column 74) (len 1)) (segments (segment 0 (token "c") (name "c") (separator none) (span (offset 133) (line 3) (column 74) (len 1)))))
    (reference r4 (scope relative) (span (offset 174) (line 4) (column 39) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 174) (line 4) (column 39) (len 1)))))
    (reference r5 (scope relative) (span (offset 186) (line 4) (column 51) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 186) (line 4) (column 51) (len 1)))))
    (reference r6 (scope relative) (span (offset 198) (line 4) (column 63) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 198) (line 4) (column 63) (len 1)))))
    (reference r7 (scope relative) (span (offset 236) (line 5) (column 36) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 236) (line 5) (column 36) (len 1)))))
    (reference r8 (scope relative) (span (offset 246) (line 5) (column 46) (len 4)) (segments (segment 0 (token "base") (name "base") (separator none) (span (offset 246) (line 5) (column 46) (len 4)))))
    (reference r9 (scope relative) (span (offset 254) (line 5) (column 54) (len 6)) (segments (segment 0 (token "latest") (name "latest") (separator none) (span (offset 254) (line 5) (column 54) (len 6)))))
    (reference r10 (scope relative) (span (offset 297) (line 6) (column 36) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 297) (line 6) (column 36) (len 1)))))
    (reference r11 (scope relative) (span (offset 307) (line 6) (column 46) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 307) (line 6) (column 46) (len 1)))))
    (reference r12 (scope relative) (span (offset 319) (line 6) (column 58) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 319) (line 6) (column 58) (len 1)))))
    (reference r13 (scope relative) (span (offset 355) (line 7) (column 34) (len 11)) (segments (segment 0 (token "ActionUsage") (name "ActionUsage") (separator none) (span (offset 355) (line 7) (column 34) (len 11)))))
    (reference r14 (scope relative) (span (offset 389) (line 7) (column 68) (len 4)) (segments (segment 0 (token "step") (name "step") (separator none) (span (offset 389) (line 7) (column 68) (len 4)))))
    (reference r15 (scope relative) (span (offset 395) (line 7) (column 74) (len 5)) (segments (segment 0 (token "usage") (name "usage") (separator none) (span (offset 395) (line 7) (column 74) (len 5)))))
    (reference r16 (scope relative) (span (offset 409) (line 7) (column 88) (len 23)) (segments (segment 0 (token "Metadata") (name "Metadata") (separator none) (span (offset 409) (line 7) (column 88) (len 8))) (segment 1 (token "metadataItems") (name "metadataItems") (separator colon-colon) (span (offset 419) (line 7) (column 98) (len 13)))))
    (reference r17 (scope relative) (span (offset 473) (line 8) (column 40) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 473) (line 8) (column 40) (len 1)))))
    (reference r18 (scope relative) (span (offset 466) (line 8) (column 33) (len 4)) (segments (segment 0 (token "base") (name "base") (separator none) (span (offset 466) (line 8) (column 33) (len 4)))))
    (reference r19 (scope relative) (span (offset 478) (line 8) (column 45) (len 6)) (segments (segment 0 (token "latest") (name "latest") (separator none) (span (offset 478) (line 8) (column 45) (len 6)))))
  )
  (root (package (name "SpecializationClauseAccumulation") (body brace (part-def (name "D") (modifiers) (body brace (attribute-usage (declaration-name "repeatedSubsets") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets (relationship (kind subsets) (implied false) (targets (ref r1) (ref r2) (ref r3)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "repeatedRedefines") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r5) (ref r6)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "mixedSpellings") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (subsets (relationship (kind subsets) (implied false) (targets (ref r8) (ref r9)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "differentKinds") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (subsets (relationship (kind subsets) (implied false) (targets (ref r11)))) (redefines (relationship (kind redefines) (implied false) (targets (ref r12)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "libraryShape") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (subsets (relationship (kind subsets) (implied false) (targets (ref r14) (ref r15) (ref r16)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "aroundTheTyping") (short-name none) (type (ref r17)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r18) (ref r19)))) (redefines none) (value none) (body semicolon)))))))
)
~~~
