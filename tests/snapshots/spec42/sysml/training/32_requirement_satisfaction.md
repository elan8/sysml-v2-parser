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
            port  :>> clutchPort;
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
    (reference r3 (scope relative) (span (offset 437) (line 20) (column 25) (len 10)) (segments (segment 0 (token "vehicle_c1") (name "vehicle_c1") (separator none) (span (offset 437) (line 20) (column 25) (len 10)))))
  )
  (root (package (name "Requirement Satisfaction") (body brace (import (target (span (span (offset 53) (line 2) (column 17) (len 28))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 78) (line 2) (column 42) (len 3))) (separator (span (offset 78) (line 2) (column 42) (len 2))) (marker (span (offset 80) (line 2) (column 44) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 99) (line 3) (column 17) (len 23))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 119) (line 3) (column 37) (len 3))) (separator (span (offset 119) (line 3) (column 37) (len 2))) (marker (span (offset 121) (line 3) (column 39) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (action-usage) (part-usage (declaration-name "vehicle_c1") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (body brace (perform) (part-usage))) (part-usage (declaration-name "Vehicle c1 Design Context") (typing none) (body brace (ref (name "vehicle_design") (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing none) (redefines none) (subsets (relationship (kind subsets) (implied false) (targets (ref r3)))) (body semicolon)) (satisfy) (satisfy))))))
)
~~~
