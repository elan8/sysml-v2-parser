# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Import Tests): QualifiedNameImportTest"))
~~~
# SOURCE
~~~sysml
package QualifiedNameImportTest {
	package P1 {
		part def A;
	}
	package P2 {
		package P2a {
			public import P1::*;
		}
		// The following should not fail.
		// A is a member of P2a because of the import.
		part x: P2a::A;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "qualified_name_import_test.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package QualifiedNameImportTest {
    package P1 {
        part def A;
    }
    package P2 {
        package P2a {
            public import P1::*;
        }
        part x : P2a::A;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 112) (line 7) (column 18) (len 2)) (segments (segment 0 (token "P1") (name "P1") (separator none) (span (offset 112) (line 7) (column 18) (len 2)))))
  )
  (root (package (name "QualifiedNameImportTest") (body (package (name "P1") (body (part-def (name "A") (body semicolon)))) (package (name "P2") (body (package (name "P2a") (body (import (target (span (span (offset 112) (line 7) (column 18) (len 5))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 114) (line 7) (column 20) (len 3))) (separator (span (offset 114) (line 7) (column 20) (len 2))) (marker (span (offset 116) (line 7) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))))) (part-usage))))))
)
~~~
