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
    (reference r1 (scope relative) (span (offset 134) (line 11) (column 16) (len 2)) (segments (segment 0 (token "P2") (name "P2") (separator none) (span (offset 134) (line 11) (column 16) (len 2)))))
    (reference r2 (scope relative) (span (offset 308) (line 21) (column 17) (len 2)) (segments (segment 0 (token "P3") (name "P3") (separator none) (span (offset 308) (line 21) (column 17) (len 2)))))
    (reference r3 (scope relative) (span (offset 457) (line 28) (column 21) (len 2)) (segments (segment 0 (token "P2") (name "P2") (separator none) (span (offset 457) (line 28) (column 21) (len 2)))))
  )
  (root (package (name "PrivateImportTest") (body (package (name "P1") (body (part-def (name "A") (body semicolon)))) (package (name "P2") (body (import (target (span (span (offset 90) (line 6) (column 18) (len 5))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 92) (line 6) (column 20) (len 3))) (separator (span (offset 92) (line 6) (column 20) (len 2))) (marker (span (offset 94) (line 6) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))))) (part-usage) (import (target (span (span (offset 134) (line 11) (column 16) (len 5))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 136) (line 11) (column 18) (len 3))) (separator (span (offset 136) (line 11) (column 18) (len 2))) (marker (span (offset 138) (line 11) (column 20) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "P3") (body (part-def (name "B") (body semicolon)))) (import (target (span (span (offset 308) (line 21) (column 17) (len 5))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 310) (line 21) (column 19) (len 3))) (separator (span (offset 310) (line 21) (column 19) (len 2))) (marker (span (offset 312) (line 21) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage) (package (name "P4") (body (import (target (span (span (offset 453) (line 28) (column 17) (len 9))) (all (span (offset 453) (line 28) (column 17) (len 3))) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 459) (line 28) (column 23) (len 3))) (separator (span (offset 459) (line 28) (column 23) (len 2))) (marker (span (offset 461) (line 28) (column 25) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage))))))
)
~~~
