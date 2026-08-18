# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 02 (Part Definitions): Part Definition Example"))
~~~
# SOURCE
~~~sysml
package 'Part Definition Example' {
	private import ScalarValues::*;
	
	part def Vehicle {
		attribute mass : Real;
		attribute status : VehicleStatus;
		
		part eng : Engine;
		
		ref part driver : Person;
	}
	
	attribute def VehicleStatus {
		attribute gearSetting : Integer;
		attribute acceleratorPosition : Real;
	}
	
	part def Engine;	
	part def Person;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "02_part_definition_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Part Definition Example' {
    private import ScalarValues::*;
    part def Vehicle {
        attribute mass : Real;
        attribute status : VehicleStatus;
        part eng : Engine;
        ref part driver : Person;
    }
    attribute def VehicleStatus {
        attribute gearSetting : Integer;
        attribute acceleratorPosition : Real;
    }
    part def Engine;
    part def Person;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 52) (line 2) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 52) (line 2) (column 17) (len 12)))))
    (reference r1 (scope relative) (span (offset 110) (line 5) (column 20) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 110) (line 5) (column 20) (len 4)))))
    (reference r2 (scope relative) (span (offset 137) (line 6) (column 22) (len 13)) (segments (segment 0 (token "VehicleStatus") (name "VehicleStatus") (separator none) (span (offset 137) (line 6) (column 22) (len 13)))))
    (reference r3 (scope relative) (span (offset 168) (line 8) (column 14) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 168) (line 8) (column 14) (len 6)))))
    (reference r4 (scope relative) (span (offset 199) (line 10) (column 21) (len 6)) (segments (segment 0 (token "Person") (name "Person") (separator none) (span (offset 199) (line 10) (column 21) (len 6)))))
    (reference r5 (scope relative) (span (offset 269) (line 14) (column 27) (len 7)) (segments (segment 0 (token "Integer") (name "Integer") (separator none) (span (offset 269) (line 14) (column 27) (len 7)))))
    (reference r6 (scope relative) (span (offset 312) (line 15) (column 35) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 312) (line 15) (column 35) (len 4)))))
  )
  (root (package (name "Part Definition Example") (body brace (import (target (span (span (offset 52) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 64) (line 2) (column 29) (len 3))) (separator (span (offset 64) (line 2) (column 29) (len 2))) (marker (span (offset 66) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Vehicle") (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "status") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "eng") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "driver") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (attribute-def (declaration-name "VehicleStatus") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "gearSetting") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "acceleratorPosition") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Engine") (body semicolon)) (part-def (name "Person") (body semicolon)))))
)
~~~
