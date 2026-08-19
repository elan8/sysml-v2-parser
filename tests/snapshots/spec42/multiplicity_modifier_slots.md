# META
~~~sexpr
(snapshot (type semantic) (description "MultiplicityPart's ordering and uniqueness keyword slots: every spelling is retained with its authored span and reproduced in its authored order, so an authored default differs from omission; readonly and variable, which no pinned production spells, reach recovery without consuming the valid sibling that follows (spec42 Gap 52)."))
~~~
# SOURCE
~~~sysml
package MultiplicityModifiers {
    attribute omitted : Real[0..*];
    attribute isOrdered : Real[0..*] ordered;
    attribute isNonordered : Real[0..*] nonordered;
    attribute isNonunique : Real[0..*] nonunique;
    attribute isUnique : Real[0..*] unique;
    attribute orderedFirst : Real[0..*] ordered nonunique;
    attribute nonuniqueFirst : Real[0..*] nonunique ordered;
    attribute leadingSlots [0..*] ordered nonunique : Real;
    attribute notAModifier : Real[0..*] orderedBy;
    readonly attribute rejectedReadonly : Real;
    attribute afterReadonly : Real;
    variable attribute rejectedVariable : Real;
    attribute afterVariable : Real;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "multiplicity_modifier_slots.md"
    (diagnostics
      (diagnostic (code "unsupported_grammar_form") (severity warning) (category unsupportedgrammarform) (span (offset 444) (line 10) (column 5) (len 46)) (message "the spec-valid extended-library declaration production is retained but not structurally implemented"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 495) (line 11) (column 5) (len 48)) (message "unrecognized declaration `readonly` in package body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 579) (line 13) (column 5) (len 48)) (message "unrecognized declaration `variable` in package body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package MultiplicityModifiers {
    attribute def omitted : Real[0..*];
    attribute def isOrdered : Real[0..*] ordered;
    attribute def isNonordered : Real[0..*] nonordered;
    attribute def isNonunique : Real[0..*] nonunique;
    attribute def isUnique : Real[0..*] unique;
    attribute def orderedFirst : Real[0..*] ordered nonunique;
    attribute def nonuniqueFirst : Real[0..*] nonunique ordered;
    attribute leadingSlots : Real[0..*] ordered nonunique;
    attribute notAModifier : Real[0..*] orderedBy;
    readonly attribute rejectedReadonly : Real;
    attribute def afterReadonly : Real;
    variable attribute rejectedVariable : Real;
    attribute def afterVariable : Real;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 56) (line 2) (column 25) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 56) (line 2) (column 25) (len 4)))))
    (reference r1 (scope relative) (span (offset 94) (line 3) (column 27) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 94) (line 3) (column 27) (len 4)))))
    (reference r2 (scope relative) (span (offset 143) (line 4) (column 30) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 143) (line 4) (column 30) (len 4)))))
    (reference r3 (scope relative) (span (offset 194) (line 5) (column 29) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 194) (line 5) (column 29) (len 4)))))
    (reference r4 (scope relative) (span (offset 241) (line 6) (column 26) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 241) (line 6) (column 26) (len 4)))))
    (reference r5 (scope relative) (span (offset 289) (line 7) (column 30) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 289) (line 7) (column 30) (len 4)))))
    (reference r6 (scope relative) (span (offset 350) (line 8) (column 32) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 350) (line 8) (column 32) (len 4)))))
    (reference r7 (scope relative) (span (offset 569) (line 12) (column 31) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 569) (line 12) (column 31) (len 4)))))
    (reference r8 (scope relative) (span (offset 653) (line 14) (column 31) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 653) (line 14) (column 31) (len 4)))))
  )
  (root (package (name "MultiplicityModifiers") (body brace (attribute-def (declaration-name "omitted") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity (lower (expression (span (offset 61) (line 2) (column 30) (len 1)) (integer 0))) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "isOrdered") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity (lower (expression (span (offset 99) (line 3) (column 32) (len 1)) (integer 0))) (upper unbounded)) (multiplicity-modifiers (ordering ordered) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "isNonordered") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity (lower (expression (span (offset 148) (line 4) (column 35) (len 1)) (integer 0))) (upper unbounded)) (multiplicity-modifiers (ordering nonordered) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "isNonunique") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity (lower (expression (span (offset 199) (line 5) (column 34) (len 1)) (integer 0))) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "isUnique") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity (lower (expression (span (offset 246) (line 6) (column 31) (len 1)) (integer 0))) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness unique)) (value none) (body semicolon)) (attribute-def (declaration-name "orderedFirst") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity (lower (expression (span (offset 294) (line 7) (column 35) (len 1)) (integer 0))) (upper unbounded)) (multiplicity-modifiers (ordering ordered) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "nonuniqueFirst") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity (lower (expression (span (offset 355) (line 8) (column 37) (len 1)) (integer 0))) (upper unbounded)) (multiplicity-modifiers (ordering ordered) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-usage) (extended-library-declaration) (malformed (code "unrecognized_declaration_in_scope") (found "readonly attribute rejectedReadonly : Real;") (span (offset 495) (line 11) (column 5) (len 48))) (attribute-def (declaration-name "afterReadonly") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (malformed (code "unrecognized_declaration_in_scope") (found "variable attribute rejectedVariable : Real;") (span (offset 579) (line 13) (column 5) (len 48))) (attribute-def (declaration-name "afterVariable") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)))))
)
~~~
