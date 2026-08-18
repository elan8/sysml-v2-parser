# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Simple Tests): ConjugationTest"))
~~~
# SOURCE
~~~sysml
package ConjugationTest {
	port def P;
	
	part def B {
		port p1: P;
		port p2: ~P;
	}
	
	connection def A {
		end port p1: P;
		end port p2: ~P;
	}
	
	interface def I {
		end p1: P;
		end p2: ~P;
	}
	
	part def B1 {
		part p {
			port p1: P;
			port p2: ~P;		
		}
	
		connection a: A {
			end port p3: P ::> p.p1;
			end port p4: ~P ::> p.p2;
		}
		interface i: I {
			end port p3: P ::> p.p1;
			end port p4: ~P ::> p.p2;
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "conjugation_test.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ConjugationTest {
    port def P;
    part def B {
        port p1 : P;
        port p2 : ~P;
    }
    connection def A {
        end p1 : P;
        end p2 : ~P;
    }
    interface def I {
        end p1 : P;
        end p2 : ~P;
    }
    part def B1 {
        part p {
            port p1 : P;
            port p2 : ~P;
        }
        connection a : A {
            end p3 : P ::> p.p1;
            end p4 : ~P ::> p.p2;
        }
        interface i : I {
            end p3 : P ::> p.p1;
            end p4 : ~P ::> p.p2;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 66) (line 5) (column 12) (len 1)) (segments (segment 0 (token "P") (name "P") (separator none) (span (offset 66) (line 5) (column 12) (len 1)))))
    (reference r1 (scope relative) (span (offset 81) (line 6) (column 13) (len 1)) (segments (segment 0 (token "P") (name "P") (separator none) (span (offset 81) (line 6) (column 13) (len 1)))))
    (reference r2 (scope relative) (span (offset 124) (line 10) (column 16) (len 1)) (segments (segment 0 (token "P") (name "P") (separator none) (span (offset 124) (line 10) (column 16) (len 1)))))
    (reference r3 (scope relative) (span (offset 143) (line 11) (column 17) (len 1)) (segments (segment 0 (token "P") (name "P") (separator none) (span (offset 143) (line 11) (column 17) (len 1)))))
    (reference r4 (scope relative) (span (offset 180) (line 15) (column 11) (len 1)) (segments (segment 0 (token "P") (name "P") (separator none) (span (offset 180) (line 15) (column 11) (len 1)))))
    (reference r5 (scope relative) (span (offset 194) (line 16) (column 12) (len 1)) (segments (segment 0 (token "P") (name "P") (separator none) (span (offset 194) (line 16) (column 12) (len 1)))))
  )
  (root (package (name "ConjugationTest") (body brace (port-def (name "P") (specializes none) (body semicolon)) (part-def (name "B") (body brace (port-usage (declaration-name "p1") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (declaration-name "p2") (direction none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r1)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (connection-def (name "A") (modifiers) (role ordinary) (specializes none) (body brace (end (short-name none) (identity (declaration (name "p1") (span (offset 120) (line 10) (column 12) (len 2)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (references none) (redefines none) (crosses none)) (end (short-name none) (identity (declaration (name "p2") (span (offset 138) (line 11) (column 12) (len 2)))) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r3)))) (references none) (redefines none) (crosses none)))) (interface-def (name "I") (modifiers) (specializes none) (body brace (end (short-name none) (identity (declaration (name "p1") (span (offset 176) (line 15) (column 7) (len 2)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (references none) (redefines none) (crosses none)) (end (short-name none) (identity (declaration (name "p2") (span (offset 189) (line 16) (column 7) (len 2)))) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r5)))) (references none) (redefines none) (crosses none)))) (part-def (name "B1") (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "p") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (port-usage) (port-usage))) (connection) (interface-usage))))))
)
~~~
