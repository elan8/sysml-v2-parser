# META
~~~sexpr
(snapshot (type semantic) (description "Coverage: individual keyword parsing paths"))
~~~
# SOURCE
~~~sysml
package CoverageIndividual {
	individual def D1;
	individual occurrence def D2;
	individual item def D3;
	individual part def D4;
	individual action def D5;
	individual state def D6;
	individual connection def D7;
	individual calc def D8;
	individual constraint def D9;
	individual requirement def D10;
	individual concern def D11;
	individual case def D12;
	individual analysis def D13;
	individual verification def D14;
	individual view def D15;
	individual viewpoint def D16;
	individual rendering def D17;

	individual p1;
	individual occurrence o1;
	individual item i1;
	individual part p2;
	individual port po1;
	individual action a1;
	individual state s1;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "coverage_individual.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package CoverageIndividual {
    individual def D1;
    individual occurrence def D2;
    individual item def D3;
    individual part def D4;
    individual action def D5;
    individual state def D6;
    individual connection def D7;
    individual calc def D8;
    individual constraint def D9;
    individual requirement def D10;
    individual concern def D11;
    individual case def D12;
    individual analysis def D13;
    individual verification def D14;
    individual view def D15;
    individual viewpoint def D16;
    individual rendering def D17;
    individual p1;
    individual occurrence o1;
    individual item i1;
    individual part p2;
    individual port po1;
    individual action a1;
    individual state s1;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "CoverageIndividual") (body brace (individual-def (modifiers)) (occurrence-def (modifiers)) (item-def (name "D3") (modifiers) (individual true) (specializes none) (body semicolon)) (part-def (name "D4") (modifiers individual) (body semicolon)) (action-def (name "D5") (modifiers) (specializes none) (body semicolon)) (state-def (name "D6") (modifiers individual) (body semicolon)) (connection-def (name "D7") (modifiers individual) (role ordinary) (specializes none) (body semicolon)) (calc-def (name "D8") (modifiers individual) (body semicolon)) (constraint-def (name "D9") (modifiers individual) (specializes none) (body semicolon)) (requirement-def (name "D10") (modifiers individual) (body semicolon)) (concern-usage (name "D11") (visibility none) (abstract false) (individual true) (definition true) (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (case-def (modifiers)) (analysis-case-def (modifiers)) (verification-case-def (name "D14") (modifiers) (body semicolon)) (view-def (name "D15") (short-name none) (modifiers individual) (specializes none) (body semicolon)) (viewpoint-def (modifiers)) (rendering-def (modifiers)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion none) (extensions)) (declaration "p1") (short-name none) (target none) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion none) (extensions)) (declaration "o1") (short-name none) (target none) (body semicolon)) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion none) (extensions)) (declaration "i1") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion none) (extensions)) (declaration-name "p2") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion none) (extensions)) (declaration-name "po1") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (action-usage (keyword action) (name "a1") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual true)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (state-usage (name "s1") (prefix (direction none) (derived false) (abstract false) (reference false) (individual true)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))))
)
~~~
