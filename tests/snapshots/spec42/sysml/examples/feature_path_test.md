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
    (reference r0 (scope relative) (span (offset 60) (line 6) (column 12) (len 1)) (segments (segment 0 (token "F") (name "F") (separator none) (span (offset 60) (line 6) (column 12) (len 1)))))
    (reference r1 (scope relative) (span (offset 342) (line 31) (column 18) (len 1)) (segments (segment 0 (token "E") (name "E") (separator none) (span (offset 342) (line 31) (column 18) (len 1)))))
  )
  (root (package (name "Q") (body brace (part-def (name "F") (body brace (part-usage))) (part-usage (declaration-name "f") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity-modifiers (ordered false) (nonunique false)) (body semicolon)) (part-def (name "A") (body brace (part-usage))) (part-def (name "B") (body brace (part-usage) (part-usage))) (part-def (name "C") (body brace (part-usage) (part-usage) (flow-usage))) (part-usage (declaration-name "e1") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body brace (attribute-usage (declaration-name "x") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (bind))) (enum-def (name "E") (body brace (enum-value (name "e1") (short-name none) (value none) (body semicolon) (span (offset 435) (line 37) (column 4) (len 8))) (enum-value (name "e2") (short-name none) (value none) (body semicolon) (span (offset 447) (line 38) (column 4) (len 8))))) (part-usage (declaration-name "g") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body semicolon)))))
)
~~~
