# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 32 (Requirements): Requirement Satisfaction"))
~~~
# SOURCE
~~~sysml
package 'Requirement Satisfaction' {
	private import 'Requirement Definitions'::*;
	private import 'Requirement Groups'::*;
	
	action 'provide power' {
		action 'generate torque' { }
	}
	
	part vehicle_c1 : Vehicle {
		perform 'provide power';
			
		part engine_v1: Engine {
			port :>> clutchPort;
			perform 'provide power'.'generate torque' :>> generateTorque;
		}	
	}
	
	part 'Vehicle c1 Design Context' {
		
		ref vehicle_design :> vehicle_c1;
	
		satisfy vehicleSpecification by vehicle_design;
		satisfy engineSpecification by vehicle_design.engine_v1;
	
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "32_requirement_satisfaction.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Requirement Satisfaction' {
    private import 'Requirement Definitions'::*;
    private import 'Requirement Groups'::*;
    action 'provide power' {
        action 'generate torque' {
        }
    }
    part vehicle_c1 : Vehicle {
        perform 'provide power';
        part engine_v1 : Engine {
            port :>> clutchPort;
            perform 'provide power'.'generate torque' :>> generateTorque;
        }
    }
    part 'Vehicle c1 Design Context' {
        ref vehicle_design :> vehicle_c1;
        satisfy vehicleSpecification by vehicle_design;
        satisfy engineSpecification by vehicle_design.engine_v1;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 53) (line 2) (column 17) (len 25)) (segments (segment 0 (token "'Requirement Definitions'") (name "Requirement Definitions") (separator none) (span (offset 53) (line 2) (column 17) (len 25)))))
    (reference r1 (scope relative) (span (offset 99) (line 3) (column 17) (len 20)) (segments (segment 0 (token "'Requirement Groups'") (name "Requirement Groups") (separator none) (span (offset 99) (line 3) (column 17) (len 20)))))
    (reference r2 (scope relative) (span (offset 207) (line 9) (column 20) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 207) (line 9) (column 20) (len 7)))))
    (reference r3 (scope relative) (span (offset 266) (line 12) (column 19) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 266) (line 12) (column 19) (len 6)))))
    (reference r4 (scope relative) (span (offset 287) (line 13) (column 13) (len 10)) (segments (segment 0 (token "clutchPort") (name "clutchPort") (separator none) (span (offset 287) (line 13) (column 13) (len 10)))))
    (reference r5 (scope relative) (span (offset 437) (line 20) (column 25) (len 10)) (segments (segment 0 (token "vehicle_c1") (name "vehicle_c1") (separator none) (span (offset 437) (line 20) (column 25) (len 10)))))
    (reference r6 (scope relative) (span (offset 461) (line 22) (column 11) (len 20)) (segments (segment 0 (token "vehicleSpecification") (name "vehicleSpecification") (separator none) (span (offset 461) (line 22) (column 11) (len 20)))))
    (reference r7 (scope relative) (span (offset 485) (line 22) (column 35) (len 14)) (segments (segment 0 (token "vehicle_design") (name "vehicle_design") (separator none) (span (offset 485) (line 22) (column 35) (len 14)))))
    (reference r8 (scope relative) (span (offset 511) (line 23) (column 11) (len 19)) (segments (segment 0 (token "engineSpecification") (name "engineSpecification") (separator none) (span (offset 511) (line 23) (column 11) (len 19)))))
    (reference r9 (scope relative) (span (offset 534) (line 23) (column 34) (len 24)) (segments (segment 0 (token "vehicle_design") (name "vehicle_design") (separator none) (span (offset 534) (line 23) (column 34) (len 14))) (segment 1 (token "engine_v1") (name "engine_v1") (separator dot) (span (offset 549) (line 23) (column 49) (len 9)))))
  )
  (root (package (name "Requirement Satisfaction") (body brace (import (target (span (span (offset 53) (line 2) (column 17) (len 28))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 78) (line 2) (column 42) (len 3))) (separator (span (offset 78) (line 2) (column 42) (len 2))) (marker (span (offset 80) (line 2) (column 44) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 99) (line 3) (column 17) (len 23))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 119) (line 3) (column 37) (len 3))) (separator (span (offset 119) (line 3) (column 37) (len 2))) (marker (span (offset 121) (line 3) (column 39) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (action-usage (name "provide power") (short-name none) (body brace (action-usage (name "generate torque") (short-name none) (body brace)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle_c1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (perform) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine_v1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (perform))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "Vehicle c1 Design Context") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (ref (name "vehicle_design") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing none) (redefines none) (subsets (relationship (kind subsets) (implied false) (targets (ref r5)))) (body semicolon)) (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r6))) (typing none) (multiplicity none) (ordered false) (nonunique false) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r7)) (body semicolon)) (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r8))) (typing none) (multiplicity none) (ordered false) (nonunique false) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r9)) (body semicolon)))))))
)
~~~
