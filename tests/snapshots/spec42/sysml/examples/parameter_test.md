# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Simple Tests): ParameterTest"))
~~~
# SOURCE
~~~sysml
package ParameterTest {
	attribute def A {
		attribute x : ScalarValues::String;
		attribute y : A;
	}
	
	attribute a : A;
	
	calc def F { in p : A; in q : ScalarValues::Integer; return :  ScalarValues::Integer; }
	
	attribute f = F(a, 2);
	attribute g = F(q = 1, p = a);
	
	attribute b = new A(y=a, x=""); 
	attribute c = new A("test2");
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "parameter_test.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ParameterTest {
    attribute def A {
        attribute x : ScalarValues::String;
        attribute y : A;
    }
    attribute a : A;
    calc def F {
        in p : A;
        in q : ScalarValues::Integer;
        return : ScalarValues::Integer;
    }
    attribute f = F(a, 2);
    attribute g = F(q = 1, p = a);
    attribute b = new A(y = a, x = "");
    attribute c = new A("test2");
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 59) (line 3) (column 17) (len 20)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 59) (line 3) (column 17) (len 12))) (segment 1 (token "String") (name "String") (separator colon-colon) (span (offset 73) (line 3) (column 31) (len 6)))))
    (reference r1 (scope relative) (span (offset 97) (line 4) (column 17) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 97) (line 4) (column 17) (len 1)))))
    (reference r2 (scope relative) (span (offset 146) (line 9) (column 22) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 146) (line 9) (column 22) (len 1)))))
    (reference r3 (scope relative) (span (offset 156) (line 9) (column 32) (len 21)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 156) (line 9) (column 32) (len 12))) (segment 1 (token "Integer") (name "Integer") (separator colon-colon) (span (offset 170) (line 9) (column 46) (len 7)))))
  )
  (root (package (name "ParameterTest") (body brace (attribute-def (declaration-name "A") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "x") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "y") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (calc-def (name "F") (modifiers) (body brace (in-out (direction in) (reference false) (declaration "p") (subsets none) (type (ref r2)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 139) (line 9) (column 15) (len 9))) (in-out (direction in) (reference false) (declaration "q") (subsets none) (type (ref r3)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 149) (line 9) (column 25) (len 29))) (return-declaration (name none) (short-name none)))) (attribute-usage) (attribute-usage) (attribute-usage) (attribute-usage))))
)
~~~
