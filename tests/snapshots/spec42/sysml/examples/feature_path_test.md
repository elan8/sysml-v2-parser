# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Simple Tests): FeaturePathTest"))
~~~
# SOURCE
~~~sysml
package Q {
  part def F {
  	part a : A;
  }
  
  part f : F;
  
  part def A {
    part g = f.a;
  }
  
  part def B {
  	part f : F;
  	part a : A;
  }
  
  part def C {
	part b : B {
	  connect f.a to a.g;
	  bind f.a = a.g;
	}
  
	part c subsets b.f {
	  	part aa subsets a;
	}
	
	flow b.f.a to c.aa;
  }
  
  part e1 {
  	attribute x : E;
  	// Ensure that "e1" resolves correctly.
  	bind e1.x = E::e2;
  }
  
  enum def E {
  	enum e1;
  	enum e2;
  }
  
  part g = new A().g.g.g;
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "feature_path_test.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Q {
    part def F {
        part a : A;
    }
    part f : F;
    part def A {
        part g = f.a;
    }
    part def B {
        part f : F;
        part a : A;
    }
    part def C {
        part b : B {
            connect f.a to a.g;
            bind f.a = a.g;
        }
        part c :> b.f {
            part aa :> a;
        }
        flow from b.f.a to c.aa;
    }
    part e1 {
        attribute x : E;
        bind e1.x = E::e2;
    }
    enum def E {
        e1;
        e2;
    }
    part g = new A().g.g.g;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 39) (line 3) (column 13) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 39) (line 3) (column 13) (len 1)))))
    (reference r1 (scope relative) (span (offset 60) (line 6) (column 12) (len 1)) (segments (segment 0 (token "F") (name "F") (separator none) (span (offset 60) (line 6) (column 12) (len 1)))))
    (reference r2 (scope relative) (span (offset 94) (line 9) (column 14) (len 1)) (segments (segment 0 (token "f") (name "f") (separator none) (span (offset 94) (line 9) (column 14) (len 1)))))
    (reference r3 (scope relative) (span (offset 96) (line 9) (column 16) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 96) (line 9) (column 16) (len 1)))))
    (reference r4 (scope relative) (span (offset 133) (line 13) (column 13) (len 1)) (segments (segment 0 (token "F") (name "F") (separator none) (span (offset 133) (line 13) (column 13) (len 1)))))
    (reference r5 (scope relative) (span (offset 148) (line 14) (column 13) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 148) (line 14) (column 13) (len 1)))))
    (reference r6 (scope relative) (span (offset 183) (line 18) (column 11) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 183) (line 18) (column 11) (len 1)))))
    (reference r7 (scope relative) (span (offset 251) (line 23) (column 17) (len 3)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 251) (line 23) (column 17) (len 1))) (segment 1 (token "f") (name "f") (separator dot) (span (offset 253) (line 23) (column 19) (len 1)))))
    (reference r8 (scope relative) (span (offset 277) (line 24) (column 21) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 277) (line 24) (column 21) (len 1)))))
    (reference r9 (scope relative) (span (offset 342) (line 31) (column 18) (len 1)) (segments (segment 0 (token "E") (name "E") (separator none) (span (offset 342) (line 31) (column 18) (len 1)))))
    (reference r10 (scope relative) (span (offset 478) (line 41) (column 16) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 478) (line 41) (column 16) (len 1)))))
    (reference r11 (scope relative) (span (offset 482) (line 41) (column 20) (len 1)) (segments (segment 0 (token "g") (name "g") (separator none) (span (offset 482) (line 41) (column 20) (len 1)))))
    (reference r12 (scope relative) (span (offset 484) (line 41) (column 22) (len 1)) (segments (segment 0 (token "g") (name "g") (separator none) (span (offset 484) (line 41) (column 22) (len 1)))))
    (reference r13 (scope relative) (span (offset 486) (line 41) (column 24) (len 1)) (segments (segment 0 (token "g") (name "g") (separator none) (span (offset 486) (line 41) (column 24) (len 1)))))
  )
  (root (package (name "Q") (body brace (part-def (name "F") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "a") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "f") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-def (name "A") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "g") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 94) (line 9) (column 14) (len 3)) (member-access (base (expression (span (offset 94) (line 9) (column 14) (len 1)) (ref r2))) (separator dot) (member (ref r3))))))) (body semicolon)))) (part-def (name "B") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "f") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "a") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "C") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "b") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (connect) (bind))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "c") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r7))) (value none))) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "aa") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r8))) (value none))) (redefines none) (value none) (body semicolon)))) (flow-usage))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "e1") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name "x") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (bind))) (enum-def (name "E") (body brace (enum-value (name "e1") (short-name none) (value none) (body semicolon) (span (offset 435) (line 37) (column 4) (len 8))) (enum-value (name "e2") (short-name none) (value none) (body semicolon) (span (offset 447) (line 38) (column 4) (len 8))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "g") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 474) (line 41) (column 12) (len 13)) (member-access (base (expression (span (offset 474) (line 41) (column 12) (len 11)) (member-access (base (expression (span (offset 474) (line 41) (column 12) (len 9)) (member-access (base (expression (span (offset 474) (line 41) (column 12) (len 7)) (constructor (type (ref r10)) (arguments)))) (separator dot) (member (ref r11))))) (separator dot) (member (ref r12))))) (separator dot) (member (ref r13))))))) (body semicolon)))))
)
~~~
