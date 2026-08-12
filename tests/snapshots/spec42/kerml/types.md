# META
~~~sexpr
(snapshot (type semantic) (description "KerML Simple Tests: Types"))
~~~
# SOURCE
~~~sysml
package Types {
	abstract type A specializes Base::Anything;
	type all x specializes A, Base::things;
	
	// This Type has exactly one instance.
	type Singleton[1] specializes Base::Anything;
	
	type Super specializes Base::Anything {
	    private package P {
	        type Sub specializes Super;
	    }
	    protected feature f : P::Sub;
	}
	
	type B :> Base::Anything;
	
	specialization Gen subtype A specializes B;
	specialization subtype x :> Base::things;
	
	type Original specializes Base::Anything {
	    in feature Input; 
	}
	type Conjugate1 specializes Base::Anything;
	type Conjugate2 specializes Base::Anything;
	conjugation c1 conjugate Conjugate1 conjugates Original; 
	conjugation c2 conjugate Conjugate2 ~ Original; 
	
	type Conjugate3 conjugates Original;
	type Conjugate4 ~ Conjugate1;
	
	type C :> B disjoint from A;
	
	type D :> Base::Anything unions A, B;
	type E :> Base::Anything intersects A, B;
	type F :> Base::Anything differences A, B;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "types.md"
    (diagnostics
      (diagnostic (code "recovered_package_body_element") (severity error) (category parseerror) (span (offset 17) (line 2) (column 2) (len 946)) (message "unexpected token in package body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package Types {
    abstract type A specializes Base::Anything;
	type all x specializes A, Base::things;
	
	// This Type has exactly one instance.
	type Singleton[1] specializes Base::Anything;
	
	type Super specializes Base::Anything {
	    private package P {
	        type Sub specializes Super;
	    }
	    protected feature f : P::Sub;
	}
	
	type B :> Base::Anything;
	
	specialization Gen subtype A specializes B;
	specialization subtype x :> Base::things;
	
	type Original specializes Base::Anything {
	    in feature Input; 
	}
	type Conjugate1 specializes Base::Anything;
	type Conjugate2 specializes Base::Anything;
	conjugation c1 conjugate Conjugate1 conjugates Original; 
	conjugation c2 conjugate Conjugate2 ~ Original; 
	
	type Conjugate3 conjugates Original;
	type Conjugate4 ~ Conjugate1;
	
	type C :> B disjoint from A;
	
	type D :> Base::Anything unions A, B;
	type E :> Base::Anything intersects A, B;
	type F :> Base::Anything differences A, B;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "Types") (body (malformed (code "recovered_package_body_element") (found "abstract type A specializes Base::Anything;") (span (offset 17) (line 2) (column 2) (len 946))))))
)
~~~
