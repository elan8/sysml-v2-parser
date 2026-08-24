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
        feature g redefines f;
    }
    alias z for y::g;
    feature w subsets y;
    alias us for w::g;
    feature yy : y;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 72) (line 6) (column 22) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 72) (line 6) (column 22) (len 1)))))
    (reference r1 (scope relative) (span (offset 97) (line 10) (column 13) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 97) (line 10) (column 13) (len 1)))))
    (reference r2 (scope relative) (span (offset 115) (line 11) (column 15) (len 4)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 115) (line 11) (column 15) (len 1))) (segment 1 (token "f") (name "f") (separator colon-colon) (span (offset 118) (line 11) (column 18) (len 1)))))
    (reference r3 (scope relative) (span (offset 143) (line 12) (column 23) (len 1)) (segments (segment 0 (token "f") (name "f") (separator none) (span (offset 143) (line 12) (column 23) (len 1)))))
    (reference r4 (scope relative) (span (offset 164) (line 15) (column 14) (len 4)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 164) (line 15) (column 14) (len 1))) (segment 1 (token "g") (name "g") (separator colon-colon) (span (offset 167) (line 15) (column 17) (len 1)))))
    (reference r5 (scope relative) (span (offset 191) (line 17) (column 20) (len 1)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 191) (line 17) (column 20) (len 1)))))
    (reference r6 (scope relative) (span (offset 210) (line 19) (column 15) (len 4)) (segments (segment 0 (token "w") (name "w") (separator none) (span (offset 210) (line 19) (column 15) (len 1))) (segment 1 (token "g") (name "g") (separator colon-colon) (span (offset 213) (line 19) (column 18) (len 1)))))
    (reference r7 (scope relative) (span (offset 231) (line 21) (column 14) (len 1)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 231) (line 21) (column 14) (len 1)))))
  )
  (root (package (name "Inheritance") (body brace (kerml-classifier (keyword class) (abstract false) (name "A") (specializes none) (conjugates none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "f") (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)))) (kerml-classifier (keyword class) (abstract false) (name "B") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r0)))) (conjugates none) (body brace)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "y") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body brace (alias (name "x") (target (ref r2)) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "g") (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r3)))) (references none) (crosses none) (relationships) (value none) (body semicolon)))) (alias (name "z") (target (ref r4)) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "w") (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r5)))) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)) (alias (name "us") (target (ref r6)) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "yy") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)))))
)
~~~
