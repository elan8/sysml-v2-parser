# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Import Tests): AliasImport"))
~~~
# SOURCE
~~~sysml
package AliasImport {
	package Definitions {
	    part def Vehicle;
	    
	    alias Car for Vehicle;
	}
	
	package Usages {
	    private import Definitions::Car;
	
	    part vehicle : Car;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "alias_import.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package AliasImport {
    package Definitions {
        part def Vehicle;
        alias Car for Vehicle;
    }
    package Usages {
        private import Definitions::Car;
        part vehicle : Car;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 93) (line 5) (column 20) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 93) (line 5) (column 20) (len 7)))))
    (reference r1 (scope relative) (span (offset 145) (line 9) (column 21) (len 16)) (segments (segment 0 (token "Definitions") (name "Definitions") (separator none) (span (offset 145) (line 9) (column 21) (len 11))) (segment 1 (token "Car") (name "Car") (separator colon-colon) (span (offset 158) (line 9) (column 34) (len 3)))))
    (reference r2 (scope relative) (span (offset 185) (line 11) (column 21) (len 3)) (segments (segment 0 (token "Car") (name "Car") (separator none) (span (offset 185) (line 11) (column 21) (len 3)))))
  )
  (root (package (name "AliasImport") (body brace (package (name "Definitions") (body brace (part-def (name "Vehicle") (body semicolon)) (alias (name "Car") (target (ref r0)) (body semicolon)))) (package (name "Usages") (body brace (import (target (span (span (offset 145) (line 9) (column 21) (len 16))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))))))
)
~~~
