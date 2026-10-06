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
        end port p1 : P;
        end port p2 : ~P;
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
            end port p3 : P ::> p.p1;
            end port p4 : ~P ::> p.p2;
        }
        interface i : I {
            end port p3 : P ::> p.p1;
            end port p4 : ~P ::> p.p2;
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
    (reference r6 (scope relative) (span (offset 240) (line 21) (column 13) (len 1)) (segments (segment 0 (token "P") (name "P") (separator none) (span (offset 240) (line 21) (column 13) (len 1)))))
    (reference r7 (scope relative) (span (offset 256) (line 22) (column 14) (len 1)) (segments (segment 0 (token "P") (name "P") (separator none) (span (offset 256) (line 22) (column 14) (len 1)))))
    (reference r8 (scope relative) (span (offset 283) (line 25) (column 17) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 283) (line 25) (column 17) (len 1)))))
    (reference r9 (scope relative) (span (offset 303) (line 26) (column 17) (len 1)) (segments (segment 0 (token "P") (name "P") (separator none) (span (offset 303) (line 26) (column 17) (len 1)))))
    (reference r10 (scope relative) (span (offset 309) (line 26) (column 23) (len 4)) (segments (segment 0 (token "p") (name "p") (separator none) (span (offset 309) (line 26) (column 23) (len 1))) (segment 1 (token "p1") (name "p1") (separator dot) (span (offset 311) (line 26) (column 25) (len 2)))))
    (reference r11 (scope relative) (span (offset 332) (line 27) (column 18) (len 1)) (segments (segment 0 (token "P") (name "P") (separator none) (span (offset 332) (line 27) (column 18) (len 1)))))
    (reference r12 (scope relative) (span (offset 338) (line 27) (column 24) (len 4)) (segments (segment 0 (token "p") (name "p") (separator none) (span (offset 338) (line 27) (column 24) (len 1))) (segment 1 (token "p2") (name "p2") (separator dot) (span (offset 340) (line 27) (column 26) (len 2)))))
    (reference r13 (scope relative) (span (offset 363) (line 29) (column 16) (len 1)) (segments (segment 0 (token "I") (name "I") (separator none) (span (offset 363) (line 29) (column 16) (len 1)))))
    (reference r14 (scope relative) (span (offset 383) (line 30) (column 17) (len 1)) (segments (segment 0 (token "P") (name "P") (separator none) (span (offset 383) (line 30) (column 17) (len 1)))))
    (reference r15 (scope relative) (span (offset 389) (line 30) (column 23) (len 4)) (segments (segment 0 (token "p") (name "p") (separator none) (span (offset 389) (line 30) (column 23) (len 1))) (segment 1 (token "p1") (name "p1") (separator dot) (span (offset 391) (line 30) (column 25) (len 2)))))
    (reference r16 (scope relative) (span (offset 412) (line 31) (column 18) (len 1)) (segments (segment 0 (token "P") (name "P") (separator none) (span (offset 412) (line 31) (column 18) (len 1)))))
    (reference r17 (scope relative) (span (offset 418) (line 31) (column 24) (len 4)) (segments (segment 0 (token "p") (name "p") (separator none) (span (offset 418) (line 31) (column 24) (len 1))) (segment 1 (token "p2") (name "p2") (separator dot) (span (offset 420) (line 31) (column 26) (len 2)))))
  )
  (root (package (name "ConjugationTest") (body brace (port-def (name "P") (modifiers) (specializes none) (body semicolon)) (part-def (name "B") (modifiers) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "p1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "p2") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (connection-def (name "A") (modifiers) (extensions) (specializes none) (body brace (port-usage (prefix (end (cross none)) (extensions)) (declaration-name "p1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (end (cross none)) (extensions)) (declaration-name "p2") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (interface-def (name "I") (modifiers) (specializes none) (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions) (short-name none) (identity (declaration (name "p1") (span (offset 176) (line 15) (column 7) (len 2)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (references none) (multiplicity none) (redefines none) (crosses none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions) (short-name none) (identity (declaration (name "p2") (span (offset 189) (line 16) (column 7) (len 2)))) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r5)))) (references none) (multiplicity none) (redefines none) (crosses none)))) (part-def (name "B1") (modifiers) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "p") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "p1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "p2") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (connection-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "a") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (connect) (body brace (port-usage (prefix (end (cross none)) (extensions)) (declaration-name "p3") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references (relationship (kind references) (implied false) (targets (ref r10)))) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (end (cross none)) (extensions)) (declaration-name "p4") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r11)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references (relationship (kind references) (implied false) (targets (ref r12)))) (crosses none) (intersects none) (value none) (body semicolon)))) (interface-usage (form declaration) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "i") (short-name none) (type (ref r13)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (part none) (body brace (port-usage (prefix (end (cross none)) (extensions)) (declaration-name "p3") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r14)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references (relationship (kind references) (implied false) (targets (ref r15)))) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (end (cross none)) (extensions)) (declaration-name "p4") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r16)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references (relationship (kind references) (implied false) (targets (ref r17)))) (crosses none) (intersects none) (value none) (body semicolon)))))))))
)
~~~
