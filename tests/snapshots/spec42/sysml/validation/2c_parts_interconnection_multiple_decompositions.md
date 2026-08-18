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
        part b12 : B12 {
            part c3 : C3;
            part c4 : C4;
            port :>> pf = c3.pd;
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
            port :>> pg = c1.pa;
        }
        part b22 : B22 {
            ref c2 : C2 = a11.b11.c2;
            ref c4 : C4 = a11.b12.c4;
            port :>> ph = c2.pc;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 327) (line 30) (column 12) (len 2)) (segments (segment 0 (token "A1") (name "A1") (separator none) (span (offset 327) (line 30) (column 12) (len 2)))))
    (reference r1 (scope relative) (span (offset 402) (line 36) (column 13) (len 3)) (segments (segment 0 (token "B11") (name "B11") (separator none) (span (offset 402) (line 36) (column 13) (len 3)))))
    (reference r2 (scope relative) (span (offset 420) (line 37) (column 13) (len 2)) (segments (segment 0 (token "C1") (name "C1") (separator none) (span (offset 420) (line 37) (column 13) (len 2)))))
    (reference r3 (scope relative) (span (offset 439) (line 38) (column 13) (len 2)) (segments (segment 0 (token "C2") (name "C2") (separator none) (span (offset 439) (line 38) (column 13) (len 2)))))
    (reference r4 (scope relative) (span (offset 490) (line 42) (column 13) (len 2)) (segments (segment 0 (token "pe") (name "pe") (separator none) (span (offset 490) (line 42) (column 13) (len 2)))))
    (reference r5 (scope relative) (span (offset 495) (line 42) (column 18) (len 2)) (segments (segment 0 (token "c1") (name "c1") (separator none) (span (offset 495) (line 42) (column 18) (len 2)))))
    (reference r6 (scope relative) (span (offset 498) (line 42) (column 21) (len 2)) (segments (segment 0 (token "pb") (name "pb") (separator none) (span (offset 498) (line 42) (column 21) (len 2)))))
    (reference r7 (scope relative) (span (offset 758) (line 53) (column 13) (len 3)) (segments (segment 0 (token "B12") (name "B12") (separator none) (span (offset 758) (line 53) (column 13) (len 3)))))
    (reference r8 (scope relative) (span (offset 776) (line 54) (column 13) (len 2)) (segments (segment 0 (token "C3") (name "C3") (separator none) (span (offset 776) (line 54) (column 13) (len 2)))))
    (reference r9 (scope relative) (span (offset 795) (line 55) (column 13) (len 2)) (segments (segment 0 (token "C4") (name "C4") (separator none) (span (offset 795) (line 55) (column 13) (len 2)))))
    (reference r10 (scope relative) (span (offset 815) (line 57) (column 13) (len 2)) (segments (segment 0 (token "pf") (name "pf") (separator none) (span (offset 815) (line 57) (column 13) (len 2)))))
    (reference r11 (scope relative) (span (offset 820) (line 57) (column 18) (len 2)) (segments (segment 0 (token "c3") (name "c3") (separator none) (span (offset 820) (line 57) (column 18) (len 2)))))
    (reference r12 (scope relative) (span (offset 823) (line 57) (column 21) (len 2)) (segments (segment 0 (token "pd") (name "pd") (separator none) (span (offset 823) (line 57) (column 21) (len 2)))))
    (reference r13 (scope relative) (span (offset 878) (line 63) (column 12) (len 2)) (segments (segment 0 (token "A1") (name "A1") (separator none) (span (offset 878) (line 63) (column 12) (len 2)))))
    (reference r14 (scope relative) (span (offset 957) (line 69) (column 13) (len 3)) (segments (segment 0 (token "B21") (name "B21") (separator none) (span (offset 957) (line 69) (column 13) (len 3)))))
    (reference r15 (scope relative) (span (offset 1116) (line 75) (column 12) (len 2)) (segments (segment 0 (token "C1") (name "C1") (separator none) (span (offset 1116) (line 75) (column 12) (len 2)))))
    (reference r16 (scope relative) (span (offset 1147) (line 76) (column 12) (len 2)) (segments (segment 0 (token "C3") (name "C3") (separator none) (span (offset 1147) (line 76) (column 12) (len 2)))))
    (reference r17 (scope relative) (span (offset 1211) (line 80) (column 13) (len 2)) (segments (segment 0 (token "pg") (name "pg") (separator none) (span (offset 1211) (line 80) (column 13) (len 2)))))
    (reference r18 (scope relative) (span (offset 1216) (line 80) (column 18) (len 2)) (segments (segment 0 (token "c1") (name "c1") (separator none) (span (offset 1216) (line 80) (column 18) (len 2)))))
    (reference r19 (scope relative) (span (offset 1219) (line 80) (column 21) (len 2)) (segments (segment 0 (token "pa") (name "pa") (separator none) (span (offset 1219) (line 80) (column 21) (len 2)))))
    (reference r20 (scope relative) (span (offset 1242) (line 83) (column 13) (len 3)) (segments (segment 0 (token "B22") (name "B22") (separator none) (span (offset 1242) (line 83) (column 13) (len 3)))))
    (reference r21 (scope relative) (span (offset 1259) (line 84) (column 12) (len 2)) (segments (segment 0 (token "C2") (name "C2") (separator none) (span (offset 1259) (line 84) (column 12) (len 2)))))
    (reference r22 (scope relative) (span (offset 1290) (line 85) (column 12) (len 2)) (segments (segment 0 (token "C4") (name "C4") (separator none) (span (offset 1290) (line 85) (column 12) (len 2)))))
    (reference r23 (scope relative) (span (offset 1323) (line 87) (column 13) (len 2)) (segments (segment 0 (token "ph") (name "ph") (separator none) (span (offset 1323) (line 87) (column 13) (len 2)))))
    (reference r24 (scope relative) (span (offset 1328) (line 87) (column 18) (len 2)) (segments (segment 0 (token "c2") (name "c2") (separator none) (span (offset 1328) (line 87) (column 18) (len 2)))))
    (reference r25 (scope relative) (span (offset 1331) (line 87) (column 21) (len 2)) (segments (segment 0 (token "pc") (name "pc") (separator none) (span (offset 1331) (line 87) (column 21) (len 2)))))
  )
  (root (package (name "2c-Parts Interconnection-Multiple Decompositions") (body brace (part-def (name "A1") (body semicolon)) (part-def (name "B11") (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "pe") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "B12") (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "pf") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "B21") (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "pg") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "B22") (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "ph") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "C1") (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "pa") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "pb") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "C2") (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "pc") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "C3") (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "pd") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "C4") (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "a11") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (doc) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "b11") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "c1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "c2") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (connect) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 495) (line 42) (column 18) (len 5)) (member-access (base (expression (span (offset 495) (line 42) (column 18) (len 2)) (ref r5))) (separator dot) (member (ref r6))))))) (body brace (doc))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "b12") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "c3") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "c4") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r10)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 820) (line 57) (column 18) (len 5)) (member-access (base (expression (span (offset 820) (line 57) (column 18) (len 2)) (ref r11))) (separator dot) (member (ref r12))))))) (body semicolon)))) (connect))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "a12") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (doc) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "b21") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r14)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (ref (name "c1") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (redefines none) (subsets none) (body semicolon)) (ref (name "c3") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (redefines none) (subsets none) (body semicolon)) (connect) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r17)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1216) (line 80) (column 18) (len 5)) (member-access (base (expression (span (offset 1216) (line 80) (column 18) (len 2)) (ref r18))) (separator dot) (member (ref r19))))))) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "b22") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r20)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (ref (name "c2") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r21)))) (redefines none) (subsets none) (body semicolon)) (ref (name "c4") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r22)))) (redefines none) (subsets none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r23)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1328) (line 87) (column 18) (len 5)) (member-access (base (expression (span (offset 1328) (line 87) (column 18) (len 2)) (ref r24))) (separator dot) (member (ref r25))))))) (body semicolon)))))))))
)
~~~
