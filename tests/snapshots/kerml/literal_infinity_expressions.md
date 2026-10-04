# META
~~~sexpr
(snapshot (type semantic) (description "KerML LiteralExpression includes LiteralInfinity (`*`), so `*` is an expression node wherever an expression is written: as a multiplicity bound and as a feature value. MultiplicityRange authors an optional lower bound and a required upper bound, so `[3]` has no lower bound while `[3..3]` has both, and `*` beside an operand is still multiplication or exponentiation."))
~~~
# SOURCE
~~~sysml
package LiteralInfinity {
    feature unbounded [*];
    feature atLeastOne [1..*];
    feature exactlyThree [3];
    feature threeToThree [3..3];
    feature infinite = *;
    feature product = 2 * 3;
    feature power = 2 ** 3;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "literal_infinity_expressions.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package LiteralInfinity {
    feature unbounded[*];
    feature atLeastOne[1..*];
    feature exactlyThree[3];
    feature threeToThree[3..3];
    feature infinite = *;
    feature product = 2 * 3;
    feature power = 2 ** 3;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "LiteralInfinity") (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "unbounded") (specializations) (multiplicity (lower none) (upper (expression (span (offset 49) (line 2) (column 24) (len 1)) (infinity)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "atLeastOne") (specializations) (multiplicity (lower (expression (span (offset 77) (line 3) (column 25) (len 1)) (integer 1))) (upper (expression (span (offset 80) (line 3) (column 28) (len 1)) (infinity)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "exactlyThree") (specializations) (multiplicity (lower none) (upper (expression (span (offset 110) (line 4) (column 27) (len 1)) (integer 3)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "threeToThree") (specializations) (multiplicity (lower (expression (span (offset 140) (line 5) (column 27) (len 1)) (integer 3))) (upper (expression (span (offset 143) (line 5) (column 30) (len 1)) (integer 3)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "infinite") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 170) (line 6) (column 24) (len 1)) (infinity))))) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "product") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 195) (line 7) (column 23) (len 5)) (binary (operator "*") (left (expression (span (offset 195) (line 7) (column 23) (len 1)) (integer 2))) (right (expression (span (offset 199) (line 7) (column 27) (len 1)) (integer 3)))))))) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "power") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 222) (line 8) (column 21) (len 6)) (binary (operator "**") (left (expression (span (offset 222) (line 8) (column 21) (len 1)) (integer 2))) (right (expression (span (offset 227) (line 8) (column 26) (len 1)) (integer 3)))))))) (body semicolon)))))
)
~~~
