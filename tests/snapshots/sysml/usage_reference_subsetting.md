# META
~~~sexpr
(snapshot (type semantic) (description "Item, allocation, constraint, enumeration, action, and state usages retain authored reference subsettings for symbol and keyword forms, including an anonymous usage. Formatting and reparsing preserve the relationship and target."))
~~~
# SOURCE
~~~sysml
package ItemReferences {
    item original;
    item symbolic ::> original;
    item keyword references original;
    item ::> original;
    allocation a ::> original;
    allocation b references original;
    constraint c ::> original;
    constraint d references original;
    enum e ::> original;
    enum f references original;
    action g ::> original;
    action h references original;
    state i ::> original;
    state j references original;
    action merged ::> original : A references symbolic;
    state mergedState references original : S ::> symbolic;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "usage_reference_subsetting.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ItemReferences {
    item original;
    item symbolic ::> original;
    item keyword references original;
    item ::> original;
    allocation a ::> original;
    allocation b references original;
    constraint c ::> original;
    constraint d references original;
    enum e ::> original;
    enum f references original;
    action g ::> original;
    action h references original;
    state i ::> original;
    state j references original;
    action merged : A ::> original, symbolic;
    state mergedState : S references original, symbolic;
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 66) (line 3) (column 23) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 66) (line 3) (column 23) (len 8)))))
    (reference r1 (scope relative) (span (offset 104) (line 4) (column 29) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 104) (line 4) (column 29) (len 8)))))
    (reference r2 (scope relative) (span (offset 127) (line 5) (column 14) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 127) (line 5) (column 14) (len 8)))))
    (reference r3 (scope relative) (span (offset 158) (line 6) (column 22) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 158) (line 6) (column 22) (len 8)))))
    (reference r4 (scope relative) (span (offset 196) (line 7) (column 29) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 196) (line 7) (column 29) (len 8)))))
    (reference r5 (scope relative) (span (offset 227) (line 8) (column 22) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 227) (line 8) (column 22) (len 8)))))
    (reference r6 (scope relative) (span (offset 265) (line 9) (column 29) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 265) (line 9) (column 29) (len 8)))))
    (reference r7 (scope relative) (span (offset 290) (line 10) (column 16) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 290) (line 10) (column 16) (len 8)))))
    (reference r8 (scope relative) (span (offset 322) (line 11) (column 23) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 322) (line 11) (column 23) (len 8)))))
    (reference r9 (scope relative) (span (offset 349) (line 12) (column 18) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 349) (line 12) (column 18) (len 8)))))
    (reference r10 (scope relative) (span (offset 383) (line 13) (column 25) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 383) (line 13) (column 25) (len 8)))))
    (reference r11 (scope relative) (span (offset 409) (line 14) (column 17) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 409) (line 14) (column 17) (len 8)))))
    (reference r12 (scope relative) (span (offset 442) (line 15) (column 24) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 442) (line 15) (column 24) (len 8)))))
    (reference r13 (scope relative) (span (offset 485) (line 16) (column 34) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 485) (line 16) (column 34) (len 1)))))
    (reference r14 (scope relative) (span (offset 474) (line 16) (column 23) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 474) (line 16) (column 23) (len 8)))))
    (reference r15 (scope relative) (span (offset 498) (line 16) (column 47) (len 8)) (segments (segment 0 (token "symbolic") (name "symbolic") (separator none) (span (offset 498) (line 16) (column 47) (len 8)))))
    (reference r16 (scope relative) (span (offset 552) (line 17) (column 45) (len 1)) (segments (segment 0 (token "S") (name "S") (separator none) (span (offset 552) (line 17) (column 45) (len 1)))))
    (reference r17 (scope relative) (span (offset 541) (line 17) (column 34) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 541) (line 17) (column 34) (len 8)))))
    (reference r18 (scope relative) (span (offset 558) (line 17) (column 51) (len 8)) (segments (segment 0 (token "symbolic") (name "symbolic") (separator none) (span (offset 558) (line 17) (column 51) (len 8)))))
  )
  (root (package (visibility none) (name "ItemReferences") (body brace (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "original") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "symbolic") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references (relationship (kind references) (implied false) (targets (ref r0)))) (value none) (body semicolon)) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "keyword") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references (relationship (kind references) (implied false) (targets (ref r1)))) (value none) (body semicolon)) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration none) (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references (relationship (kind references) (implied false) (targets (ref r2)))) (value none) (body semicolon)) (allocation-usage (name "a") (short-name none) (typing none) (references (relationship (kind references) (implied false) (targets (ref r3)))) (ends) (body semicolon)) (allocation-usage (name "b") (short-name none) (typing none) (references (relationship (kind references) (implied false) (targets (ref r4)))) (ends) (body semicolon)) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "c") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (references (relationship (kind references) (implied false) (targets (ref r5)))) (body semicolon)) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "d") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (references (relationship (kind references) (implied false) (targets (ref r6)))) (body semicolon)) (enumeration-usage (name "e") (short-name none) (typing none) (references (relationship (kind references) (implied false) (targets (ref r7)))) (body semicolon)) (enumeration-usage (name "f") (short-name none) (typing none) (references (relationship (kind references) (implied false) (targets (ref r8)))) (body semicolon)) (action-usage (keyword action) (name "g") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references (relationship (kind references) (implied false) (targets (ref r9)))) (body semicolon)) (action-usage (keyword action) (name "h") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references (relationship (kind references) (implied false) (targets (ref r10)))) (body semicolon)) (state-usage (name "i") (short-name none) (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references (relationship (kind references) (implied false) (targets (ref r11)))) (body semicolon)) (state-usage (name "j") (short-name none) (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references (relationship (kind references) (implied false) (targets (ref r12)))) (body semicolon)) (action-usage (keyword action) (name "merged") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references (relationship (kind references) (implied false) (targets (ref r14) (ref r15)))) (body semicolon)) (state-usage (name "mergedState") (short-name none) (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references (relationship (kind references) (implied false) (targets (ref r17) (ref r18)))) (body semicolon)))))
)
~~~
