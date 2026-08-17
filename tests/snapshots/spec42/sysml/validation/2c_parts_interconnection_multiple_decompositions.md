# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (02-Parts Interconnection): 2c-Parts Interconnection-Multiple Decompositions"))
~~~
# SOURCE
~~~sysml
package '2c-Parts Interconnection-Multiple Decompositions' {
	
	part def A1;
	
	part def B11 {
		port pe;
	}
	part def B12 {
		port pf;
	}
	part def B21 {
		port pg;
	}
	part def B22 {
		port ph;
	}
	
	part def C1 {
		port pa;
		port pb;
	}	
	part def C2 {
		port pc;
	}
	part def C3 {
		port pd;
	}
	part def C4;
	
	part a11: A1 {
	doc
	/*
	 * Decomposition 1 - Subsystems b11, b12
	 */
	
		part b11: B11 {
			part c1: C1;			
			part c2: C2;
			
			connect c1.pa to c2.pc;
			
			port :>> pe = c1.pb {
				doc
				/*
				 * This combines the definition of a port with a binding
				 * connector. (It is the same notation used to bind a
				 * attribute to a attribute property or a reference to a reference
				 * property.)
				 */
			}
		}
		
		part b12: B12 {
			part c3: C3;			
			part c4: C4;
			
			port :>> pf = c3.pd;
		}
		
		connect b11.pe to b12.pf;
	}
	
	part a12: A1 {
		doc
		/*
		 * Decomposition 2 - Assemblies b21, b22
		 */
	
		part b21: B21 {
			/*
			 * The c-level entities are already composite parts within
			 * a11, so they cannot also be composite parts within a12.
			 */
			 
			ref c1: C1 = a11.b11.c1;			
			ref c3: C3 = a11.b12.c3;
			
			connect c1.pb to c3.pd;
			
			port :>> pg = c1.pa;
		}
		
		part b22: B22 {
			ref c2: C2 = a11.b11.c2;			
			ref c4: C4 = a11.b12.c4;
			
			port :>> ph = c2.pc;
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "2c_parts_interconnection_multiple_decompositions.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '2c-Parts Interconnection-Multiple Decompositions' {
    part def A1;
    part def B11 {
        port pe;
    }
    part def B12 {
        port pf;
    }
    part def B21 {
        port pg;
    }
    part def B22 {
        port ph;
    }
    part def C1 {
        port pa;
        port pb;
    }
    part def C2 {
        port pc;
    }
    part def C3 {
        port pd;
    }
    part def C4;
    part a11 : A1 {
        doc
        /*
	 * Decomposition 1 - Subsystems b11, b12
	 */
        part b11 : B11 {
            part c1 : C1;
            part c2 : C2;
            connect c1.pa to c2.pc;
            port  :>> pe = c1.pb {
                doc
                /*
				 * This combines the definition of a port with a binding
				 * connector. (It is the same notation used to bind a
				 * attribute to a attribute property or a reference to a reference
				 * property.)
				 */
            }
        }
        part b12 : B12 {
            part c3 : C3;
            part c4 : C4;
            port  :>> pf = c3.pd;
        }
        connect b11.pe to b12.pf;
    }
    part a12 : A1 {
        doc
        /*
		 * Decomposition 2 - Assemblies b21, b22
		 */
        part b21 : B21 {
            ref c1 : C1 = a11.b11.c1;
            ref c3 : C3 = a11.b12.c3;
            connect c1.pb to c3.pd;
            port  :>> pg = c1.pa;
        }
        part b22 : B22 {
            ref c2 : C2 = a11.b11.c2;
            ref c4 : C4 = a11.b12.c4;
            port  :>> ph = c2.pc;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 327) (line 30) (column 12) (len 2)) (segments (segment 0 (token "A1") (name "A1") (separator none) (span (offset 327) (line 30) (column 12) (len 2)))))
    (reference r1 (scope relative) (span (offset 878) (line 63) (column 12) (len 2)) (segments (segment 0 (token "A1") (name "A1") (separator none) (span (offset 878) (line 63) (column 12) (len 2)))))
  )
  (root (package (name "2c-Parts Interconnection-Multiple Decompositions") (body brace (part-def (name "A1") (body semicolon)) (part-def (name "B11") (body brace (port-usage (declaration-name "pe") (direction none) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "B12") (body brace (port-usage (declaration-name "pf") (direction none) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "B21") (body brace (port-usage (declaration-name "pg") (direction none) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "B22") (body brace (port-usage (declaration-name "ph") (direction none) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "C1") (body brace (port-usage (declaration-name "pa") (direction none) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (declaration-name "pb") (direction none) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "C2") (body brace (port-usage (declaration-name "pc") (direction none) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "C3") (body brace (port-usage (declaration-name "pd") (direction none) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "C4") (body semicolon)) (part-usage (declaration-name "a11") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (body brace (doc) (part-usage) (part-usage) (connect))) (part-usage (declaration-name "a12") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (body brace (doc) (part-usage) (part-usage))))))
)
~~~
