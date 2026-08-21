# META
~~~sexpr
(snapshot (type semantic) (description "Exact VariantUsage contexts from Simple Tests/VariabilityTest.sysml:9-30, including an inline typed part, an untyped reference with a body, and `variant action` members in an ActionDefBody."))
~~~
# SOURCE
~~~sysml
package VariabilityTest {
    part def P {
        attribute a;
    }
    part def Q :> P;
    attribute def B;
    variation part def V :> P {
        variant part x : Q {
            attribute b : B :>> a;
        }
    }
    part q : Q;
    variation part v : P {
        variant q {
            attribute b : B :>> a;
        }
    }
    part y : P = v::q;
    variation action def A {
        variant action a1;
        variant action a2;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "upstream_variant_variability_test.md"
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
  (references
    (reference r0 (scope relative) (span (offset 169) (line 8) (column 26) (len 1)) (segments (segment 0 (token "Q") (name "Q") (separator none) (span (offset 169) (line 8) (column 26) (len 1)))))
    (reference r1 (scope relative) (span (offset 199) (line 9) (column 27) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 199) (line 9) (column 27) (len 1)))))
    (reference r2 (scope relative) (span (offset 205) (line 9) (column 33) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 205) (line 9) (column 33) (len 1)))))
    (reference r3 (scope relative) (span (offset 237) (line 12) (column 14) (len 1)) (segments (segment 0 (token "Q") (name "Q") (separator none) (span (offset 237) (line 12) (column 14) (len 1)))))
    (reference r4 (scope relative) (span (offset 263) (line 13) (column 24) (len 1)) (segments (segment 0 (token "P") (name "P") (separator none) (span (offset 263) (line 13) (column 24) (len 1)))))
    (reference r5 (scope relative) (span (offset 283) (line 14) (column 17) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 283) (line 14) (column 17) (len 1)))))
    (reference r6 (scope relative) (span (offset 313) (line 15) (column 27) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 313) (line 15) (column 27) (len 1)))))
    (reference r7 (scope relative) (span (offset 319) (line 15) (column 33) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 319) (line 15) (column 33) (len 1)))))
    (reference r8 (scope relative) (span (offset 351) (line 18) (column 14) (len 1)) (segments (segment 0 (token "P") (name "P") (separator none) (span (offset 351) (line 18) (column 14) (len 1)))))
    (reference r9 (scope relative) (span (offset 355) (line 18) (column 18) (len 4)) (segments (segment 0 (token "v") (name "v") (separator none) (span (offset 355) (line 18) (column 18) (len 1))) (segment 1 (token "q") (name "q") (separator colon-colon) (span (offset 358) (line 18) (column 21) (len 1)))))
  )
  (root (package (name "VariabilityTest") (body brace (part-def (name "P") (modifiers) (body brace (attribute-usage (declaration-name "a") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Q") (modifiers) (body semicolon)) (attribute-def (declaration-name "B") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (part-def (name "V") (modifiers (variation (span (offset 116) (line 7) (column 5) (len 9)))) (body brace (variant-usage (target none) (usage (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "x") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name "b") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (references none) (crosses none) (intersects none) (value none) (body semicolon))))) (body absent)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "q") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance variation) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "v") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (variant-usage (target (ref r5)) (usage none) (body brace (attribute-usage (declaration-name "b") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r7)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "y") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 355) (line 18) (column 18) (len 4)) (ref r9))))) (body semicolon)) (action-def (name "A") (modifiers (variation (span (offset 365) (line 19) (column 5) (len 9)))) (specializes none) (body brace (variant-usage (target none) (usage (action-usage (name "a1") (short-name none) (body semicolon))) (body absent)) (variant-usage (target none) (usage (action-usage (name "a2") (short-name none) (body semicolon))) (body absent)))))))
)
~~~
