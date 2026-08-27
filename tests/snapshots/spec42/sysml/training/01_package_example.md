# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 01 (Packages): Package Example"))
~~~
# SOURCE
~~~sysml
package 'Package Example' {
	public import ISQ::TorqueValue;
	private import ScalarValues::*;
	 
	private part def Automobile;
	
	public alias Car for Automobile;	                         
	alias Torque for ISQ::TorqueValue;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "01_package_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Package Example' {
    public import ISQ::TorqueValue;
    private import ScalarValues::*;
    private part def Automobile;
    public alias Car for Automobile;
    alias Torque for ISQ::TorqueValue;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 43) (line 2) (column 16) (len 16)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 43) (line 2) (column 16) (len 3))) (segment 1 (token "TorqueValue") (name "TorqueValue") (separator colon-colon) (span (offset 48) (line 2) (column 21) (len 11)))))
    (reference r1 (scope relative) (span (offset 77) (line 3) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 77) (line 3) (column 17) (len 12)))))
    (reference r2 (scope relative) (span (offset 151) (line 7) (column 23) (len 10)) (segments (segment 0 (token "Automobile") (name "Automobile") (separator none) (span (offset 151) (line 7) (column 23) (len 10)))))
    (reference r3 (scope relative) (span (offset 207) (line 8) (column 19) (len 16)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 207) (line 8) (column 19) (len 3))) (segment 1 (token "TorqueValue") (name "TorqueValue") (separator colon-colon) (span (offset 212) (line 8) (column 24) (len 11)))))
  )
  (root (package (name "Package Example") (body brace (import (target (span (span (offset 43) (line 2) (column 16) (len 16))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 77) (line 3) (column 17) (len 15))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 89) (line 3) (column 29) (len 3))) (separator (span (offset 89) (line 3) (column 29) (len 2))) (marker (span (offset 91) (line 3) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Automobile") (modifiers) (body semicolon)) (alias (name "Car") (target (ref r2)) (body semicolon)) (alias (name "Torque") (target (ref r3)) (body semicolon)))))
)
~~~
