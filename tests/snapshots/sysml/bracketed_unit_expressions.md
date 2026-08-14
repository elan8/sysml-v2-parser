# META
~~~sexpr
(snapshot (type semantic) (description "The [unit] annotation applies to parenthesized tuples, invocation results, and feature references in expression position (the Domain Geometry coordinate-frame idiom), not just scalar literals (spec42 Gap 49c)."))
~~~
# SOURCE
~~~sysml
package BracketedUnitExpressions {
    part def P {
        attribute :>> elements = (new Translation( (0, shape.width/2, 0)[source] ));
        attribute a = (1.0, 2.0, 3.0)[mm];
        attribute b = new Rotation((0, 0, 1), angle[deg])[source];
        attribute c = 18 [mm];
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
        attribute :>> elements = (new Translation((0, shape.width / 2, 0) [source]));
        attribute a = (1.0, 2.0, 3.0) [mm];
        attribute b = new Rotation((0, 0, 1), angle [deg]) [source];
        attribute c = 18 [mm];
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
    (reference r4 (scope relative) (span (offset 206) (line 5) (column 27) (len 8)) (segments (segment 0 (token "Rotation") (name "Rotation") (separator none) (span (offset 206) (line 5) (column 27) (len 8)))))
    (reference r5 (scope relative) (span (offset 226) (line 5) (column 47) (len 5)) (segments (segment 0 (token "angle") (name "angle") (separator none) (span (offset 226) (line 5) (column 47) (len 5)))))
  )
  (root (package (name "BracketedUnitExpressions") (body (part-def (name "P") (body (attribute-usage (declaration-name none) (direction none) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r0)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 85) (line 3) (column 34) (len 50)) (parenthesized (expression (span (offset 86) (line 3) (column 35) (len 48)) (constructor (type (ref r1)) (arguments (argument (parameter none) (value (expression (span (offset 103) (line 3) (column 52) (len 29)) (literal-with-unit (value (expression (span (offset 103) (line 3) (column 52) (len 21)) (tuple (expression (span (offset 104) (line 3) (column 53) (len 1)) (integer 0)) (expression (span (offset 107) (line 3) (column 56) (len 13)) (binary (operator "/") (left (expression (span (offset 107) (line 3) (column 56) (len 11)) (member-access (base (expression (span (offset 107) (line 3) (column 56) (len 5)) (ref r2))) (separator dot) (member (ref r3))))) (right (expression (span (offset 119) (line 3) (column 68) (len 1)) (integer 2))))) (expression (span (offset 122) (line 3) (column 71) (len 1)) (integer 0))))) (unit (expression (span (offset 125) (line 3) (column 74) (len 6)) (bracket (expression (span (offset 125) (line 3) (column 74) (len 6)) (unit "source"))))))))))))))))) (body semicolon)) (attribute-usage (declaration-name "a") (direction none) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 159) (line 4) (column 23) (len 19)) (literal-with-unit (value (expression (span (offset 159) (line 4) (column 23) (len 15)) (tuple (expression (span (offset 160) (line 4) (column 24) (len 3)) (real "1.0")) (expression (span (offset 165) (line 4) (column 29) (len 3)) (real "2.0")) (expression (span (offset 170) (line 4) (column 34) (len 3)) (real "3.0"))))) (unit (expression (span (offset 175) (line 4) (column 39) (len 2)) (bracket (expression (span (offset 175) (line 4) (column 39) (len 2)) (unit "mm")))))))))) (body semicolon)) (attribute-usage (declaration-name "b") (direction none) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 202) (line 5) (column 23) (len 43)) (literal-with-unit (value (expression (span (offset 202) (line 5) (column 23) (len 35)) (constructor (type (ref r4)) (arguments (argument (parameter none) (value (expression (span (offset 215) (line 5) (column 36) (len 9)) (tuple (expression (span (offset 216) (line 5) (column 37) (len 1)) (integer 0)) (expression (span (offset 219) (line 5) (column 40) (len 1)) (integer 0)) (expression (span (offset 222) (line 5) (column 43) (len 1)) (integer 1)))))) (argument (parameter none) (value (expression (span (offset 226) (line 5) (column 47) (len 10)) (literal-with-unit (value (expression (span (offset 226) (line 5) (column 47) (len 5)) (ref r5))) (unit (expression (span (offset 232) (line 5) (column 53) (len 3)) (bracket (expression (span (offset 232) (line 5) (column 53) (len 3)) (unit "deg"))))))))))))) (unit (expression (span (offset 238) (line 5) (column 59) (len 6)) (bracket (expression (span (offset 238) (line 5) (column 59) (len 6)) (unit "source")))))))))) (body semicolon)) (attribute-usage (declaration-name "c") (direction none) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 269) (line 6) (column 23) (len 7)) (literal-with-unit (value (expression (span (offset 269) (line 6) (column 23) (len 2)) (integer 18))) (unit (expression (span (offset 273) (line 6) (column 27) (len 2)) (bracket (expression (span (offset 273) (line 6) (column 27) (len 2)) (unit "mm")))))))))) (body semicolon)))))))
)
~~~
