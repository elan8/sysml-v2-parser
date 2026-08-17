# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 26 (State Exhibition): State Exhibition Example"))
~~~
# SOURCE
~~~sysml
package 'State Exhibition Example' {
	private import 'Transition Actions'::*;
	
	part vehicle : Vehicle {
		
		part vehicleController : VehicleController;
		
		exhibit vehicleStates {
			in operatingVehicle = vehicle;
			in controller = vehicleController;
		}

	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "26_state_exhibition_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'State Exhibition Example' {
    private import 'Transition Actions'::*;
    part vehicle : Vehicle {
        part vehicleController : VehicleController;
        state vehicleStates {
            in operatingVehicle = vehicle;
            in controller = vehicleController;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 53) (line 2) (column 17) (len 20)) (segments (segment 0 (token "'Transition Actions'") (name "Transition Actions") (separator none) (span (offset 53) (line 2) (column 17) (len 20)))))
    (reference r1 (scope relative) (span (offset 96) (line 4) (column 17) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 96) (line 4) (column 17) (len 7)))))
  )
  (root (package (name "State Exhibition Example") (body brace (import (target (span (span (offset 53) (line 2) (column 17) (len 23))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 73) (line 2) (column 37) (len 3))) (separator (span (offset 73) (line 2) (column 37) (len 2))) (marker (span (offset 75) (line 2) (column 39) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage (declaration-name "vehicle") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity-modifiers (ordered false) (nonunique false)) (body brace (part-usage) (state-usage))))))
)
~~~
