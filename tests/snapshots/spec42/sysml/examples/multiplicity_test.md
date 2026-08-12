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
  )
  (root (package (name "MultiplicityTest") (body (part-def (name "P") (body semicolon)) (attribute-def) (part-usage) (part-usage) (part-usage) (part-usage) (part-usage) (part-usage) (part-usage) (attribute-def))))
)
~~~
