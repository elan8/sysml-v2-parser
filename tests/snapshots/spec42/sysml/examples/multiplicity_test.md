# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Simple Tests): MultiplicityTest"))
~~~
# SOURCE
~~~sysml
package MultiplicityTest {
	
	part def P;
	attribute n : ScalarValues::Integer = 5;
	
	part a[1];
	part b[0..2] : P;
	part c : P[2..*];
	part d[*];
	
	part e[n];
	part f[n..*];
	part g[1..n];

	attribute def A {
		attribute i :ScalarValues::Integer;
		attribute x : A[i];
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "multiplicity_test.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package MultiplicityTest {
    part def P;
    attribute def n : ScalarValues::Integer = 5;
    part a[1];
    part b : P[0..2];
    part c : P[2..*];
    part d[*];
    part e[n];
    part f[n..*];
    part g[1..n];
    attribute def A {
        attribute i : ScalarValues::Integer;
        attribute x : A[i];
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 114) (line 7) (column 17) (len 1)) (segments (segment 0 (token "P") (name "P") (separator none) (span (offset 114) (line 7) (column 17) (len 1)))))
    (reference r1 (scope relative) (span (offset 127) (line 8) (column 11) (len 1)) (segments (segment 0 (token "P") (name "P") (separator none) (span (offset 127) (line 8) (column 11) (len 1)))))
  )
  (root (package (name "MultiplicityTest") (body brace (part-def (name "P") (body semicolon)) (attribute-def (name "n") (multiplicity none)) (part-usage (declaration-name "a") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body semicolon)) (part-usage (declaration-name "b") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity-modifiers (ordered false) (nonunique false)) (body semicolon)) (part-usage (declaration-name "c") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity-modifiers (ordered false) (nonunique false)) (body semicolon)) (part-usage (declaration-name "d") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body semicolon)) (part-usage (declaration-name "e") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body semicolon)) (part-usage (declaration-name "f") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body semicolon)) (part-usage (declaration-name "g") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body semicolon)) (attribute-def (name "A") (multiplicity none)))))
)
~~~
