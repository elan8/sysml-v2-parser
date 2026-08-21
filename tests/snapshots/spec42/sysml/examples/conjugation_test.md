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
      (diagnostic (code "recovered_connection_def_body_element") (severity error) (category parseerror) (span (offset 111) (line 10) (column 3) (len 18)) (message "unexpected token in connection definition body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 111) (line 10) (column 3) (len 18)) (message "suppressed 5 cascading recovered diagnostics after earlier recovery errors"))
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
        end port p1: P;
        end port p2: ~P;
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
            end port p3: P ::> p.p1;
            end port p4: ~P ::> p.p2;
        }
        interface i : I {
            end port p3: P ::> p.p1;
            end port p4: ~P ::> p.p2;
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
    (reference r2 (scope relative) (span (offset 180) (line 15) (column 11) (len 1)) (segments (segment 0 (token "P") (name "P") (separator none) (span (offset 180) (line 15) (column 11) (len 1)))))
    (reference r3 (scope relative) (span (offset 194) (line 16) (column 12) (len 1)) (segments (segment 0 (token "P") (name "P") (separator none) (span (offset 194) (line 16) (column 12) (len 1)))))
    (reference r4 (scope relative) (span (offset 240) (line 21) (column 13) (len 1)) (segments (segment 0 (token "P") (name "P") (separator none) (span (offset 240) (line 21) (column 13) (len 1)))))
    (reference r5 (scope relative) (span (offset 256) (line 22) (column 14) (len 1)) (segments (segment 0 (token "P") (name "P") (separator none) (span (offset 256) (line 22) (column 14) (len 1)))))
  )
  (root (package (name "ConjugationTest") (body brace (port-def (name "P") (modifiers) (specializes none) (body semicolon)) (part-def (name "B") (modifiers) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "p1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "p2") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (connection-def (name "A") (modifiers) (role ordinary) (specializes none) (body brace (malformed (code "recovered_connection_def_body_element") (found "end port p1: P;") (span (offset 111) (line 10) (column 3) (len 18))) (malformed (code "recovered_connection_def_body_element") (found "end port p2: ~P;") (span (offset 129) (line 11) (column 3) (len 18))))) (interface-def (name "I") (modifiers) (specializes none) (body brace (end (introducer bare) (short-name none) (identity (declaration (name "p1") (span (offset 176) (line 15) (column 7) (len 2)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (references none) (multiplicity none) (redefines none) (crosses none) (nested-usage none)) (end (introducer bare) (short-name none) (identity (declaration (name "p2") (span (offset 189) (line 16) (column 7) (len 2)))) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r3)))) (references none) (multiplicity none) (redefines none) (crosses none) (nested-usage none)))) (part-def (name "B1") (modifiers) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "p") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "p1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "p2") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (connection) (interface-usage (form declaration) (part none) (body brace (malformed (code "recovered_interface_usage_body_element") (found "end port p3: P ::> p.p1;") (span (offset 370) (line 30) (column 4) (len 28))) (malformed (code "recovered_interface_usage_body_element") (found "end port p4: ~P ::> p.p2;") (span (offset 398) (line 31) (column 4) (len 28))))))))))
)
~~~
