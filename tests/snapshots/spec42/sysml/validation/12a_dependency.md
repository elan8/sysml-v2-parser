# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (12-Dependency Relationships): 12a-Dependency"))
~~~
# SOURCE
~~~sysml
package '12a-Dependency' {
	
	package 'Application Layer';
	package 'Service Layer';
	package 'Data Layer';
	
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
  (document "12a_dependency.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '12a-Dependency' {
    package 'Application Layer';
    package 'Service Layer';
    package 'Data Layer';
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
    (reference r0 (scope relative) (span (offset 131) (line 7) (column 22) (len 19)) (segments (segment 0 (token "'Application Layer'") (name "Application Layer") (separator none) (span (offset 131) (line 7) (column 22) (len 19)))))
    (reference r1 (scope relative) (span (offset 154) (line 7) (column 45) (len 15)) (segments (segment 0 (token "'Service Layer'") (name "Service Layer") (separator none) (span (offset 154) (line 7) (column 45) (len 15)))))
    (reference r2 (scope relative) (span (offset 188) (line 8) (column 18) (len 15)) (segments (segment 0 (token "'Service Layer'") (name "Service Layer") (separator none) (span (offset 188) (line 8) (column 18) (len 15)))))
    (reference r3 (scope relative) (span (offset 207) (line 8) (column 37) (len 12)) (segments (segment 0 (token "'Data Layer'") (name "Data Layer") (separator none) (span (offset 207) (line 8) (column 37) (len 12)))))
    (reference r4 (scope relative) (span (offset 279) (line 14) (column 13) (len 1)) (segments (segment 0 (token "z") (name "z") (separator none) (span (offset 279) (line 14) (column 13) (len 1)))))
    (reference r5 (scope relative) (span (offset 284) (line 14) (column 18) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 284) (line 14) (column 18) (len 1)))))
    (reference r6 (scope relative) (span (offset 287) (line 14) (column 21) (len 1)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 287) (line 14) (column 21) (len 1)))))
  )
  (root (package (name "12a-Dependency") (body (package (name "Application Layer") (body semicolon)) (package (name "Service Layer") (body semicolon)) (package (name "Data Layer") (body semicolon)) (dependency (clients (ref r0)) (suppliers (ref r1))) (dependency (clients (ref r2)) (suppliers (ref r3))) (attribute-def) (attribute-def) (attribute-def) (dependency (clients (ref r4)) (suppliers (ref r5) (ref r6))))))
)
~~~
