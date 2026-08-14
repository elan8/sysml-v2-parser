# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Simple Tests): DependencyTest"))
~~~
# SOURCE
~~~sysml
package DependencyTest {
	
	package System {
		package 'Application Layer';
		package 'Service Layer';
		package 'Data Layer';
	}
	
	private import System::*;
	
	dependency Use from 'Application Layer' to 'Service Layer';
	dependency from 'Service Layer' to 'Data Layer';
	
	attribute x;
	attribute y;
	attribute z;
	
	dependency z to x, y;
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "dependency_test.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package DependencyTest {
    package System {
        package 'Application Layer';
        package 'Service Layer';
        package 'Data Layer';
    }
    private import System::*;
    dependency Use from 'Application Layer' to 'Service Layer';
    dependency from 'Service Layer' to 'Data Layer';
    attribute def x;
    attribute def y;
    attribute def z;
    dependency from z to x, y;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 148) (line 9) (column 17) (len 6)) (segments (segment 0 (token "System") (name "System") (separator none) (span (offset 148) (line 9) (column 17) (len 6)))))
    (reference r1 (scope relative) (span (offset 182) (line 11) (column 22) (len 19)) (segments (segment 0 (token "'Application Layer'") (name "Application Layer") (separator none) (span (offset 182) (line 11) (column 22) (len 19)))))
    (reference r2 (scope relative) (span (offset 205) (line 11) (column 45) (len 15)) (segments (segment 0 (token "'Service Layer'") (name "Service Layer") (separator none) (span (offset 205) (line 11) (column 45) (len 15)))))
    (reference r3 (scope relative) (span (offset 239) (line 12) (column 18) (len 15)) (segments (segment 0 (token "'Service Layer'") (name "Service Layer") (separator none) (span (offset 239) (line 12) (column 18) (len 15)))))
    (reference r4 (scope relative) (span (offset 258) (line 12) (column 37) (len 12)) (segments (segment 0 (token "'Data Layer'") (name "Data Layer") (separator none) (span (offset 258) (line 12) (column 37) (len 12)))))
    (reference r5 (scope relative) (span (offset 330) (line 18) (column 13) (len 1)) (segments (segment 0 (token "z") (name "z") (separator none) (span (offset 330) (line 18) (column 13) (len 1)))))
    (reference r6 (scope relative) (span (offset 335) (line 18) (column 18) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 335) (line 18) (column 18) (len 1)))))
    (reference r7 (scope relative) (span (offset 338) (line 18) (column 21) (len 1)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 338) (line 18) (column 21) (len 1)))))
  )
  (root (package (name "DependencyTest") (body (package (name "System") (body (package (name "Application Layer") (body semicolon)) (package (name "Service Layer") (body semicolon)) (package (name "Data Layer") (body semicolon)))) (import (target (span (span (offset 148) (line 9) (column 17) (len 9))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 154) (line 9) (column 23) (len 3))) (separator (span (offset 154) (line 9) (column 23) (len 2))) (marker (span (offset 156) (line 9) (column 25) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (dependency (clients (ref r1)) (suppliers (ref r2))) (dependency (clients (ref r3)) (suppliers (ref r4))) (attribute-def) (attribute-def) (attribute-def) (dependency (clients (ref r5)) (suppliers (ref r6) (ref r7))))))
)
~~~
