# META
~~~sexpr
(snapshot (type semantic) (description "Pinned KerML Simple Tests/Inheritance.kerml source: the nested `feature y: A` TypeBody retains its AliasMember before the following feature redefinition (KerML textual BNF TypeBodyElement; Pilot agrees)."))
~~~
# SOURCE
~~~sysml
package Inheritance {
	class A {
		feature f;
	}
	
	class B specializes A {
		
	}
		
	feature y: A {
		alias x for B::f;
		feature g redefines f;
	}
	
	alias z for y::g;
	
	feature w subsets y;
	
	alias us for w::g;
	
	feature yy: y;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "inheritance.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Inheritance {
    class A {
        feature f;
    }
    class B specializes A {
    }
    feature y : A {
        alias x for B::f;
        feature g :>> f;
    }
    alias z for y::g;
    feature w :> y;
    alias us for w::g;
    feature yy : y;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 72) (line 6) (column 22) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 72) (line 6) (column 22) (len 1)))))
    (reference r1 (scope relative) (span (offset 115) (line 11) (column 15) (len 4)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 115) (line 11) (column 15) (len 1))) (segment 1 (token "f") (name "f") (separator colon-colon) (span (offset 118) (line 11) (column 18) (len 1)))))
    (reference r2 (scope relative) (span (offset 164) (line 15) (column 14) (len 4)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 164) (line 15) (column 14) (len 1))) (segment 1 (token "g") (name "g") (separator colon-colon) (span (offset 167) (line 15) (column 17) (len 1)))))
    (reference r3 (scope relative) (span (offset 210) (line 19) (column 15) (len 4)) (segments (segment 0 (token "w") (name "w") (separator none) (span (offset 210) (line 19) (column 15) (len 1))) (segment 1 (token "g") (name "g") (separator colon-colon) (span (offset 213) (line 19) (column 18) (len 1)))))
  )
  (root (package (name "Inheritance") (body brace (kerml-classifier (keyword class) (abstract false) (name "A") (specializes none) (body brace (kerml-feature))) (kerml-classifier (keyword class) (abstract false) (name "B") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r0)))) (body brace)) (kerml-feature (name "y") (body brace (alias (name "x") (target (ref r1)) (body semicolon)) (kerml-feature))) (alias (name "z") (target (ref r2)) (body semicolon)) (kerml-feature (name "w") (body semicolon)) (alias (name "us") (target (ref r3)) (body semicolon)) (kerml-feature (name "yy") (body semicolon)))))
)
~~~
