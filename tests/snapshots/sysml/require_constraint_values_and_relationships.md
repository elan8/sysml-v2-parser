# META
~~~sexpr
(snapshot (type semantic) (description "Required and assumed constraint declarations retain every ordered specialization and their separate ValuePart, including symbolic and keyword relationships, repeated targets, assignment and default values; reference shorthands remain references."))
~~~
# SOURCE
~~~sysml
requirement def R {
    require constraint bound :> base = true;
    assume constraint assigned :>> inherited := false;
    require constraint defaulted subsets base default true;
    assume #goal constraint named references target crosses other = value;
    require #goal tagged ::> a => b : C, D redefines previous default := value;
    require constraint repeated references a references b crosses c crosses d;
    require constraint mixed :> a : T references b :>> c = true;
    require constraint references target = true;
    assume constraint = true;
    require constraint := false;
    require constraint default true;
    require existing;
    assume P::existing;
}
constraint def Nested {
    require constraint local references target = true;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "require_constraint_values_and_relationships.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
requirement def R {
    require constraint bound :> base = true;
    assume constraint assigned :>> inherited := false;
    require constraint defaulted subsets base default true;
    assume #goal constraint named references target crosses other = value;
    require #goal tagged ::> a => b : C, D redefines previous default := value;
    require constraint repeated references a references b crosses c crosses d;
    require constraint mixed :> a : T references b :>> c = true;
    require constraint references target = true;
    assume constraint = true;
    require constraint := false;
    require constraint default true;
    require existing;
    assume P::existing;
}

constraint def Nested {
    require constraint local references target = true;
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 52) (line 2) (column 33) (len 4)) (segments (segment 0 (token "base") (name "base") (separator none) (span (offset 52) (line 2) (column 33) (len 4)))))
    (reference r1 (scope relative) (span (offset 100) (line 3) (column 36) (len 9)) (segments (segment 0 (token "inherited") (name "inherited") (separator none) (span (offset 100) (line 3) (column 36) (len 9)))))
    (reference r2 (scope relative) (span (offset 161) (line 4) (column 42) (len 4)) (segments (segment 0 (token "base") (name "base") (separator none) (span (offset 161) (line 4) (column 42) (len 4)))))
    (reference r3 (scope relative) (span (offset 225) (line 5) (column 46) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 225) (line 5) (column 46) (len 6)))))
    (reference r4 (scope relative) (span (offset 240) (line 5) (column 61) (len 5)) (segments (segment 0 (token "other") (name "other") (separator none) (span (offset 240) (line 5) (column 61) (len 5)))))
    (reference r5 (scope relative) (span (offset 192) (line 5) (column 13) (len 4)) (segments (segment 0 (token "goal") (name "goal") (separator none) (span (offset 192) (line 5) (column 13) (len 4)))))
    (reference r6 (scope relative) (span (offset 248) (line 5) (column 69) (len 5)) (segments (segment 0 (token "value") (name "value") (separator none) (span (offset 248) (line 5) (column 69) (len 5)))))
    (reference r7 (scope relative) (span (offset 284) (line 6) (column 30) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 284) (line 6) (column 30) (len 1)))))
    (reference r8 (scope relative) (span (offset 289) (line 6) (column 35) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 289) (line 6) (column 35) (len 1)))))
    (reference r9 (scope relative) (span (offset 293) (line 6) (column 39) (len 1)) (segments (segment 0 (token "C") (name "C") (separator none) (span (offset 293) (line 6) (column 39) (len 1)))))
    (reference r10 (scope relative) (span (offset 296) (line 6) (column 42) (len 1)) (segments (segment 0 (token "D") (name "D") (separator none) (span (offset 296) (line 6) (column 42) (len 1)))))
    (reference r11 (scope relative) (span (offset 308) (line 6) (column 54) (len 8)) (segments (segment 0 (token "previous") (name "previous") (separator none) (span (offset 308) (line 6) (column 54) (len 8)))))
    (reference r12 (scope relative) (span (offset 268) (line 6) (column 14) (len 4)) (segments (segment 0 (token "goal") (name "goal") (separator none) (span (offset 268) (line 6) (column 14) (len 4)))))
    (reference r13 (scope relative) (span (offset 328) (line 6) (column 74) (len 5)) (segments (segment 0 (token "value") (name "value") (separator none) (span (offset 328) (line 6) (column 74) (len 5)))))
    (reference r14 (scope relative) (span (offset 378) (line 7) (column 44) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 378) (line 7) (column 44) (len 1)))))
    (reference r15 (scope relative) (span (offset 391) (line 7) (column 57) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 391) (line 7) (column 57) (len 1)))))
    (reference r16 (scope relative) (span (offset 401) (line 7) (column 67) (len 1)) (segments (segment 0 (token "c") (name "c") (separator none) (span (offset 401) (line 7) (column 67) (len 1)))))
    (reference r17 (scope relative) (span (offset 411) (line 7) (column 77) (len 1)) (segments (segment 0 (token "d") (name "d") (separator none) (span (offset 411) (line 7) (column 77) (len 1)))))
    (reference r18 (scope relative) (span (offset 446) (line 8) (column 33) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 446) (line 8) (column 33) (len 1)))))
    (reference r19 (scope relative) (span (offset 450) (line 8) (column 37) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 450) (line 8) (column 37) (len 1)))))
    (reference r20 (scope relative) (span (offset 463) (line 8) (column 50) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 463) (line 8) (column 50) (len 1)))))
    (reference r21 (scope relative) (span (offset 469) (line 8) (column 56) (len 1)) (segments (segment 0 (token "c") (name "c") (separator none) (span (offset 469) (line 8) (column 56) (len 1)))))
    (reference r22 (scope relative) (span (offset 513) (line 9) (column 35) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 513) (line 9) (column 35) (len 6)))))
    (reference r23 (scope relative) (span (offset 640) (line 13) (column 13) (len 8)) (segments (segment 0 (token "existing") (name "existing") (separator none) (span (offset 640) (line 13) (column 13) (len 8)))))
    (reference r24 (scope relative) (span (offset 661) (line 14) (column 12) (len 11)) (segments (segment 0 (token "P") (name "P") (separator none) (span (offset 661) (line 14) (column 12) (len 1))) (segment 1 (token "existing") (name "existing") (separator colon-colon) (span (offset 664) (line 14) (column 15) (len 8)))))
    (reference r25 (scope relative) (span (offset 740) (line 17) (column 41) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 740) (line 17) (column 41) (len 6)))))
  )
  (root (requirement-def (name "R") (modifiers) (body brace (require-constraint (kind require) (constraint-keyword true) (name "bound") (target none) (specializations (subsetting (relationship (kind subsets) (implied false) (targets (ref r0))) (value none))) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 59) (line 2) (column 40) (len 4)) (boolean true))))) (body semicolon)) (require-constraint (kind assume) (constraint-keyword true) (name "assigned") (target none) (specializations (redefinition (relationship (kind redefines) (implied false) (targets (ref r1))))) (value (feature-value (kind assign) (default false) (expression (expression (span (offset 113) (line 3) (column 49) (len 5)) (boolean false))))) (body semicolon)) (require-constraint (kind require) (constraint-keyword true) (name "defaulted") (target none) (specializations (subsetting (relationship (kind subsets) (implied false) (targets (ref r2))) (value none))) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 174) (line 4) (column 55) (len 4)) (boolean true))))) (body semicolon)) (require-constraint (kind assume) (constraint-keyword true) (name "named") (target none) (specializations (reference-subsetting (relationship (kind references) (implied false) (targets (ref r3)))) (cross-subsetting (relationship (kind crosses) (implied false) (targets (ref r4))))) (extensions (ref r5)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 248) (line 5) (column 69) (len 5)) (ref r6))))) (body semicolon)) (require-constraint (kind require) (constraint-keyword false) (name "tagged") (target none) (specializations (reference-subsetting (relationship (kind references) (implied false) (targets (ref r7)))) (cross-subsetting (relationship (kind crosses) (implied false) (targets (ref r8)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9) (ref r10)))) (redefinition (relationship (kind redefines) (implied false) (targets (ref r11))))) (extensions (ref r12)) (value (feature-value (kind assign) (default true) (expression (expression (span (offset 328) (line 6) (column 74) (len 5)) (ref r13))))) (body semicolon)) (require-constraint (kind require) (constraint-keyword true) (name "repeated") (target none) (specializations (reference-subsetting (relationship (kind references) (implied false) (targets (ref r14)))) (reference-subsetting (relationship (kind references) (implied false) (targets (ref r15)))) (cross-subsetting (relationship (kind crosses) (implied false) (targets (ref r16)))) (cross-subsetting (relationship (kind crosses) (implied false) (targets (ref r17))))) (value none) (body semicolon)) (require-constraint (kind require) (constraint-keyword true) (name "mixed") (target none) (specializations (subsetting (relationship (kind subsets) (implied false) (targets (ref r18))) (value none)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r19)))) (reference-subsetting (relationship (kind references) (implied false) (targets (ref r20)))) (redefinition (relationship (kind redefines) (implied false) (targets (ref r21))))) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 473) (line 8) (column 60) (len 4)) (boolean true))))) (body semicolon)) (require-constraint (kind require) (constraint-keyword true) (name none) (target none) (specializations (reference-subsetting (relationship (kind references) (implied false) (targets (ref r22))))) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 522) (line 9) (column 44) (len 4)) (boolean true))))) (body semicolon)) (require-constraint (kind assume) (constraint-keyword true) (name none) (target none) (specializations) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 552) (line 10) (column 25) (len 4)) (boolean true))))) (body semicolon)) (require-constraint (kind require) (constraint-keyword true) (name none) (target none) (specializations) (value (feature-value (kind assign) (default false) (expression (expression (span (offset 584) (line 11) (column 27) (len 5)) (boolean false))))) (body semicolon)) (require-constraint (kind require) (constraint-keyword true) (name none) (target none) (specializations) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 622) (line 12) (column 32) (len 4)) (boolean true))))) (body semicolon)) (require-constraint (kind require) (constraint-keyword false) (name none) (target (ref r23)) (specializations) (value none) (body semicolon)) (require-constraint (kind assume) (constraint-keyword false) (name none) (target (ref r24)) (specializations) (value none) (body semicolon)))) (constraint-def (name "Nested") (modifiers) (specializes none) (body brace (require-constraint (kind require) (constraint-keyword true) (name "local") (target none) (specializations (reference-subsetting (relationship (kind references) (implied false) (targets (ref r25))))) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 749) (line 17) (column 50) (len 4)) (boolean true))))) (body semicolon)))))
)
~~~
