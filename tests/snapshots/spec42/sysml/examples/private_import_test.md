# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Import Tests): PrivateImportTest"))
~~~
# SOURCE
~~~sysml
package PrivateImportTest {
	package P1 {
		part def A;
	}
	package P2 {
		private import P1::*;
	}

	part x: P1::A;
	
	public import P2::*;
	// This should fail.
	// A is not visible, because the import in P2 is private.
	// part y: A;
	// part y1: P2::A;
	
	package P3 {
		part def B;
	}
	
	private import P3::*;
	
	// This should not fail.
	// Private import only restricts visibility outside the package.
	part z: B;
	
	package P4 {
		public import all P2::*;
		
		// This should not fail because "import all" overrides private import.
		part z1: A;
	}	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "private_import_test.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package PrivateImportTest {
    package P1 {
        part def A;
    }
    package P2 {
        private import P1::*;
    }
    part x : P1::A;
    public import P2::*;
    package P3 {
        part def B;
    }
    private import P3::*;
    part z : B;
    package P4 {
        public import all P2::*;
        part z1 : A;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 90) (line 6) (column 18) (len 2)) (segments (segment 0 (token "P1") (name "P1") (separator none) (span (offset 90) (line 6) (column 18) (len 2)))))
    (reference r1 (scope relative) (span (offset 110) (line 9) (column 10) (len 5)) (segments (segment 0 (token "P1") (name "P1") (separator none) (span (offset 110) (line 9) (column 10) (len 2))) (segment 1 (token "A") (name "A") (separator colon-colon) (span (offset 114) (line 9) (column 14) (len 1)))))
    (reference r2 (scope relative) (span (offset 134) (line 11) (column 16) (len 2)) (segments (segment 0 (token "P2") (name "P2") (separator none) (span (offset 134) (line 11) (column 16) (len 2)))))
    (reference r3 (scope relative) (span (offset 308) (line 21) (column 17) (len 2)) (segments (segment 0 (token "P3") (name "P3") (separator none) (span (offset 308) (line 21) (column 17) (len 2)))))
    (reference r4 (scope relative) (span (offset 418) (line 25) (column 10) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 418) (line 25) (column 10) (len 1)))))
    (reference r5 (scope relative) (span (offset 457) (line 28) (column 21) (len 2)) (segments (segment 0 (token "P2") (name "P2") (separator none) (span (offset 457) (line 28) (column 21) (len 2)))))
    (reference r6 (scope relative) (span (offset 551) (line 31) (column 12) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 551) (line 31) (column 12) (len 1)))))
  )
  (root (package (name "PrivateImportTest") (body brace (package (name "P1") (body brace (part-def (name "A") (modifiers) (body semicolon)))) (package (name "P2") (body brace (import (target (span (span (offset 90) (line 6) (column 18) (len 5))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 92) (line 6) (column 20) (len 3))) (separator (span (offset 92) (line 6) (column 20) (len 2))) (marker (span (offset 94) (line 6) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "x") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (import (target (span (span (offset 134) (line 11) (column 16) (len 5))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 136) (line 11) (column 18) (len 3))) (separator (span (offset 136) (line 11) (column 18) (len 2))) (marker (span (offset 138) (line 11) (column 20) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "P3") (body brace (part-def (name "B") (modifiers) (body semicolon)))) (import (target (span (span (offset 308) (line 21) (column 17) (len 5))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 310) (line 21) (column 19) (len 3))) (separator (span (offset 310) (line 21) (column 19) (len 2))) (marker (span (offset 312) (line 21) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "z") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (package (name "P4") (body brace (import (target (span (span (offset 453) (line 28) (column 17) (len 9))) (all (span (offset 453) (line 28) (column 17) (len 3))) (ref r5) (shape (namespace (wildcard-suffix (span (span (offset 459) (line 28) (column 23) (len 3))) (separator (span (offset 459) (line 28) (column 23) (len 2))) (marker (span (offset 461) (line 28) (column 25) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "z1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))))))
)
~~~
