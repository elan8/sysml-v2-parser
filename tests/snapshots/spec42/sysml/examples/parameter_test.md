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
    attribute def a : A;
    calc def F {
        in p : A;
        in q : ScalarValues::Integer;
        return : ScalarValues::Integer;
    }
    attribute def f = F(a, 2);
    attribute def g = F(q = 1, p = a);
    attribute def b = new A(y = a, x = "");
    attribute def c = new A("test2");
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "ParameterTest") (body brace (attribute-def (name "A") (multiplicity none)) (attribute-def (name "a") (multiplicity none)) (calc-def (name "F") (body brace (in-out-declaration) (in-out-declaration) (return-declaration (name none) (short-name none)))) (attribute-def (name "f") (multiplicity none)) (attribute-def (name "g") (multiplicity none)) (attribute-def (name "b") (multiplicity none)) (attribute-def (name "c") (multiplicity none)))))
)
~~~
