# META
~~~sexpr
  (snapshot (type semantic) (description "KerML BracketExpression is a repeatable postfix over every primary expression. Its SequenceExpressionList operand retains qualified references and binary operators structurally, so value brackets never become a declaration multiplicity."))
~~~
# SOURCE
~~~sysml
package BracketedUnitExpressions {
    part def P {
        attribute :>> elements = (new Translation( (0, shape.width/2, 0)[source] ));
        attribute a = (1.0, 2.0, 3.0)[mm];
        attribute b = new Rotation((0, 0, 1), angle[deg])[source];
        attribute c = 18 [mm];
        attribute d = 10.0 [N * m];
        attribute e = 60 [SI::mm];
        attribute f = 30 [mi / gal];
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "bracketed_unit_expressions.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package BracketedUnitExpressions {
    part def P {
        attribute :>> elements = (new Translation((0, shape.width / 2, 0)[source]));
        attribute a = (1.0, 2.0, 3.0)[mm];
        attribute b = new Rotation((0, 0, 1), angle[deg])[source];
        attribute c = 18[mm];
        attribute d = 10.0[N * m];
        attribute e = 60[SI::mm];
        attribute f = 30[mi / gal];
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 74) (line 3) (column 23) (len 8)) (segments (segment 0 (token "elements") (name "elements") (separator none) (span (offset 74) (line 3) (column 23) (len 8)))))
    (reference r1 (scope relative) (span (offset 90) (line 3) (column 39) (len 11)) (segments (segment 0 (token "Translation") (name "Translation") (separator none) (span (offset 90) (line 3) (column 39) (len 11)))))
    (reference r2 (scope relative) (span (offset 107) (line 3) (column 56) (len 5)) (segments (segment 0 (token "shape") (name "shape") (separator none) (span (offset 107) (line 3) (column 56) (len 5)))))
    (reference r3 (scope relative) (span (offset 113) (line 3) (column 62) (len 5)) (segments (segment 0 (token "width") (name "width") (separator none) (span (offset 113) (line 3) (column 62) (len 5)))))
    (reference r4 (scope relative) (span (offset 125) (line 3) (column 74) (len 6)) (segments (segment 0 (token "source") (name "source") (separator none) (span (offset 125) (line 3) (column 74) (len 6)))))
    (reference r5 (scope relative) (span (offset 175) (line 4) (column 39) (len 2)) (segments (segment 0 (token "mm") (name "mm") (separator none) (span (offset 175) (line 4) (column 39) (len 2)))))
    (reference r6 (scope relative) (span (offset 206) (line 5) (column 27) (len 8)) (segments (segment 0 (token "Rotation") (name "Rotation") (separator none) (span (offset 206) (line 5) (column 27) (len 8)))))
    (reference r7 (scope relative) (span (offset 226) (line 5) (column 47) (len 5)) (segments (segment 0 (token "angle") (name "angle") (separator none) (span (offset 226) (line 5) (column 47) (len 5)))))
    (reference r8 (scope relative) (span (offset 232) (line 5) (column 53) (len 3)) (segments (segment 0 (token "deg") (name "deg") (separator none) (span (offset 232) (line 5) (column 53) (len 3)))))
    (reference r9 (scope relative) (span (offset 238) (line 5) (column 59) (len 6)) (segments (segment 0 (token "source") (name "source") (separator none) (span (offset 238) (line 5) (column 59) (len 6)))))
    (reference r10 (scope relative) (span (offset 273) (line 6) (column 27) (len 2)) (segments (segment 0 (token "mm") (name "mm") (separator none) (span (offset 273) (line 6) (column 27) (len 2)))))
    (reference r11 (scope relative) (span (offset 306) (line 7) (column 29) (len 1)) (segments (segment 0 (token "N") (name "N") (separator none) (span (offset 306) (line 7) (column 29) (len 1)))))
    (reference r12 (scope relative) (span (offset 310) (line 7) (column 33) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 310) (line 7) (column 33) (len 1)))))
    (reference r13 (scope relative) (span (offset 340) (line 8) (column 27) (len 6)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 340) (line 8) (column 27) (len 2))) (segment 1 (token "mm") (name "mm") (separator colon-colon) (span (offset 344) (line 8) (column 31) (len 2)))))
    (reference r14 (scope relative) (span (offset 375) (line 9) (column 27) (len 2)) (segments (segment 0 (token "mi") (name "mi") (separator none) (span (offset 375) (line 9) (column 27) (len 2)))))
    (reference r15 (scope relative) (span (offset 380) (line 9) (column 32) (len 3)) (segments (segment 0 (token "gal") (name "gal") (separator none) (span (offset 380) (line 9) (column 32) (len 3)))))
  )
  (root (package (name "BracketedUnitExpressions") (body brace (part-def (name "P") (modifiers) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r0)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 85) (line 3) (column 34) (len 50)) (sequence (sequence-list (element first (expression (span (offset 86) (line 3) (column 35) (len 48)) (constructor (type (ref r1)) (arguments (argument (parameter none) (value (expression (span (offset 103) (line 3) (column 52) (len 29)) (bracket (base (expression (span (offset 103) (line 3) (column 52) (len 21)) (sequence (sequence-list (element first (expression (span (offset 104) (line 3) (column 53) (len 1)) (integer 0))) (element comma (expression (span (offset 107) (line 3) (column 56) (len 13)) (binary (operator "/") (left (expression (span (offset 107) (line 3) (column 56) (len 11)) (member-access (base (expression (span (offset 107) (line 3) (column 56) (len 5)) (ref r2))) (separator dot) (member (ref r3))))) (right (expression (span (offset 119) (line 3) (column 68) (len 1)) (integer 2)))))) (element comma (expression (span (offset 122) (line 3) (column 71) (len 1)) (integer 0))))))) (operands (sequence-list (element first (expression (span (offset 125) (line 3) (column 74) (len 6)) (ref r4))))))))))))))))))) (body semicolon)) (attribute-usage (declaration-name "a") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 159) (line 4) (column 23) (len 19)) (bracket (base (expression (span (offset 159) (line 4) (column 23) (len 15)) (sequence (sequence-list (element first (expression (span (offset 160) (line 4) (column 24) (len 3)) (real "1.0"))) (element comma (expression (span (offset 165) (line 4) (column 29) (len 3)) (real "2.0"))) (element comma (expression (span (offset 170) (line 4) (column 34) (len 3)) (real "3.0"))))))) (operands (sequence-list (element first (expression (span (offset 175) (line 4) (column 39) (len 2)) (ref r5)))))))))) (body semicolon)) (attribute-usage (declaration-name "b") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 202) (line 5) (column 23) (len 43)) (bracket (base (expression (span (offset 202) (line 5) (column 23) (len 35)) (constructor (type (ref r6)) (arguments (argument (parameter none) (value (expression (span (offset 215) (line 5) (column 36) (len 9)) (sequence (sequence-list (element first (expression (span (offset 216) (line 5) (column 37) (len 1)) (integer 0))) (element comma (expression (span (offset 219) (line 5) (column 40) (len 1)) (integer 0))) (element comma (expression (span (offset 222) (line 5) (column 43) (len 1)) (integer 1)))))))) (argument (parameter none) (value (expression (span (offset 226) (line 5) (column 47) (len 10)) (bracket (base (expression (span (offset 226) (line 5) (column 47) (len 5)) (ref r7))) (operands (sequence-list (element first (expression (span (offset 232) (line 5) (column 53) (len 3)) (ref r8))))))))))))) (operands (sequence-list (element first (expression (span (offset 238) (line 5) (column 59) (len 6)) (ref r9)))))))))) (body semicolon)) (attribute-usage (declaration-name "c") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 269) (line 6) (column 23) (len 7)) (bracket (base (expression (span (offset 269) (line 6) (column 23) (len 2)) (integer 18))) (operands (sequence-list (element first (expression (span (offset 273) (line 6) (column 27) (len 2)) (ref r10)))))))))) (body semicolon)) (attribute-usage (declaration-name "d") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 300) (line 7) (column 23) (len 12)) (bracket (base (expression (span (offset 300) (line 7) (column 23) (len 4)) (real "10.0"))) (operands (sequence-list (element first (expression (span (offset 306) (line 7) (column 29) (len 5)) (binary (operator "*") (left (expression (span (offset 306) (line 7) (column 29) (len 1)) (ref r11))) (right (expression (span (offset 310) (line 7) (column 33) (len 1)) (ref r12))))))))))))) (body semicolon)) (attribute-usage (declaration-name "e") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 336) (line 8) (column 23) (len 11)) (bracket (base (expression (span (offset 336) (line 8) (column 23) (len 2)) (integer 60))) (operands (sequence-list (element first (expression (span (offset 340) (line 8) (column 27) (len 6)) (ref r13)))))))))) (body semicolon)) (attribute-usage (declaration-name "f") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 371) (line 9) (column 23) (len 13)) (bracket (base (expression (span (offset 371) (line 9) (column 23) (len 2)) (integer 30))) (operands (sequence-list (element first (expression (span (offset 375) (line 9) (column 27) (len 8)) (binary (operator "/") (left (expression (span (offset 375) (line 9) (column 27) (len 2)) (ref r14))) (right (expression (span (offset 380) (line 9) (column 32) (len 3)) (ref r15))))))))))))) (body semicolon)))))))
)
~~~
