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
  )
  (root (package (name "Q") (body (part-def (name "F") (body (part-usage))) (part-usage) (part-def (name "A") (body (part-usage))) (part-def (name "B") (body (part-usage) (part-usage))) (part-def (name "C") (body (part-usage) (part-usage) (flow-usage))) (part-usage) (enum-def) (part-usage))))
)
~~~
