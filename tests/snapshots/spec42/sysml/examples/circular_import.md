# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Import Tests): CircularImport"))
~~~
# SOURCE
~~~sysml
package CircularImport {

	package P1 {
		public import P2::*;
		part def A;
	}
	package P2 {
		public import P1::*;
		part def B;
	}
	package Test1 {
		public import P1::*;
		part x: A;
		part y: B;
	}
	package Test2 {
		public import P2::*;
		part x: A;
		part y: B;
	}
	
	part x: P1::A;
	
	// The following should not fail.
	part y: P1::B;
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "circular_import.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package CircularImport {
    package P1 {
        public import P2::*;
        part def A;
    }
    package P2 {
        public import P1::*;
        part def B;
    }
    package Test1 {
        public import P1::*;
        part x : A;
        part y : B;
    }
    package Test2 {
        public import P2::*;
        part x : A;
        part y : B;
    }
    part x : P1::A;
    part y : P1::B;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 56) (line 4) (column 17) (len 2)) (segments (segment 0 (token "P2") (name "P2") (separator none) (span (offset 56) (line 4) (column 17) (len 2)))))
    (reference r1 (scope relative) (span (offset 110) (line 8) (column 17) (len 2)) (segments (segment 0 (token "P1") (name "P1") (separator none) (span (offset 110) (line 8) (column 17) (len 2)))))
    (reference r2 (scope relative) (span (offset 167) (line 12) (column 17) (len 2)) (segments (segment 0 (token "P1") (name "P1") (separator none) (span (offset 167) (line 12) (column 17) (len 2)))))
    (reference r3 (scope relative) (span (offset 236) (line 17) (column 17) (len 2)) (segments (segment 0 (token "P2") (name "P2") (separator none) (span (offset 236) (line 17) (column 17) (len 2)))))
  )
  (root (package (name "CircularImport") (body (package (name "P1") (body (import (target (span (span (offset 56) (line 4) (column 17) (len 5))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 58) (line 4) (column 19) (len 3))) (separator (span (offset 58) (line 4) (column 19) (len 2))) (marker (span (offset 60) (line 4) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "A") (body semicolon)))) (package (name "P2") (body (import (target (span (span (offset 110) (line 8) (column 17) (len 5))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 112) (line 8) (column 19) (len 3))) (separator (span (offset 112) (line 8) (column 19) (len 2))) (marker (span (offset 114) (line 8) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "B") (body semicolon)))) (package (name "Test1") (body (import (target (span (span (offset 167) (line 12) (column 17) (len 5))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 169) (line 12) (column 19) (len 3))) (separator (span (offset 169) (line 12) (column 19) (len 2))) (marker (span (offset 171) (line 12) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage) (part-usage))) (package (name "Test2") (body (import (target (span (span (offset 236) (line 17) (column 17) (len 5))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 238) (line 17) (column 19) (len 3))) (separator (span (offset 238) (line 17) (column 19) (len 2))) (marker (span (offset 240) (line 17) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage) (part-usage))) (part-usage) (part-usage))))
)
~~~
