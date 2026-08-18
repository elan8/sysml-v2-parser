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
    (reference r0 (scope relative) (span (offset 57) (line 4) (column 16) (len 21)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 57) (line 4) (column 16) (len 12))) (segment 1 (token "Integer") (name "Integer") (separator colon-colon) (span (offset 71) (line 4) (column 30) (len 7)))))
    (reference r1 (scope relative) (span (offset 114) (line 7) (column 17) (len 1)) (segments (segment 0 (token "P") (name "P") (separator none) (span (offset 114) (line 7) (column 17) (len 1)))))
    (reference r2 (scope relative) (span (offset 127) (line 8) (column 11) (len 1)) (segments (segment 0 (token "P") (name "P") (separator none) (span (offset 127) (line 8) (column 11) (len 1)))))
    (reference r3 (scope relative) (span (offset 158) (line 11) (column 9) (len 1)) (segments (segment 0 (token "n") (name "n") (separator none) (span (offset 158) (line 11) (column 9) (len 1)))))
    (reference r4 (scope relative) (span (offset 170) (line 12) (column 9) (len 1)) (segments (segment 0 (token "n") (name "n") (separator none) (span (offset 170) (line 12) (column 9) (len 1)))))
    (reference r5 (scope relative) (span (offset 188) (line 13) (column 12) (len 1)) (segments (segment 0 (token "n") (name "n") (separator none) (span (offset 188) (line 13) (column 12) (len 1)))))
    (reference r6 (scope relative) (span (offset 227) (line 16) (column 16) (len 21)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 227) (line 16) (column 16) (len 12))) (segment 1 (token "Integer") (name "Integer") (separator colon-colon) (span (offset 241) (line 16) (column 30) (len 7)))))
    (reference r7 (scope relative) (span (offset 266) (line 17) (column 17) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 266) (line 17) (column 17) (len 1)))))
  )
  (root (package (name "MultiplicityTest") (body brace (part-def (name "P") (body semicolon)) (attribute-def (declaration-name "n") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 81) (line 4) (column 40) (len 1)) (integer 5))))) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "a") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 94) (line 6) (column 9) (len 1)) (integer 1))) (upper (expression (span (offset 94) (line 6) (column 9) (len 1)) (integer 1)))) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "b") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity (lower (expression (span (offset 106) (line 7) (column 9) (len 1)) (integer 0))) (upper (expression (span (offset 109) (line 7) (column 12) (len 1)) (integer 2)))) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "c") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity (lower (expression (span (offset 129) (line 8) (column 13) (len 1)) (integer 2))) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "d") (short-name none) (typing none) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "e") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 158) (line 11) (column 9) (len 1)) (ref r3))) (upper (expression (span (offset 158) (line 11) (column 9) (len 1)) (ref r3)))) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "f") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 170) (line 12) (column 9) (len 1)) (ref r4))) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "g") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 185) (line 13) (column 9) (len 1)) (integer 1))) (upper (expression (span (offset 188) (line 13) (column 12) (len 1)) (ref r5)))) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (attribute-def (declaration-name "A") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "i") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "x") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
