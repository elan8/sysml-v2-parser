# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 25 (Transitions): Transition Actions"))
~~~
# SOURCE
~~~sysml
package 'Transition Actions' {
	
	attribute def VehicleStartSignal;
	attribute def VehicleOnSignal;
	attribute def VehicleOffSignal;
	
	attribute def ControllerStartSignal;
	
	part def Vehicle {
		brakePedalDepressed : ScalarValues::Boolean;
	}
	part def VehicleController;
	
	action performSelfTest { in vehicle : Vehicle; }
	
	state def VehicleStates;
		
	state vehicleStates : VehicleStates {
		in operatingVehicle : Vehicle;
		in controller : VehicleController;

		entry; then off;
		
		state off;
		accept VehicleStartSignal 
			then starting;
			
		state starting;
		accept VehicleOnSignal
			if operatingVehicle.brakePedalDepressed
			do send new ControllerStartSignal() to controller
			then on;
			
		state on {
			entry performSelfTest{ in vehicle = operatingVehicle; }
			do action providePower { /* ... */ }
			exit action applyParkingBrake { /* ... */ }
		}
		accept VehicleOffSignal
			then off;

	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "25_transition_actions.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Transition Actions' {
    attribute def VehicleStartSignal;
    attribute def VehicleOnSignal;
    attribute def VehicleOffSignal;
    attribute def ControllerStartSignal;
    part def Vehicle {
        brakePedalDepressed : ScalarValues::Boolean;
    }
    part def VehicleController;
    action performSelfTest {
        in vehicle : Vehicle;
    }
    state def VehicleStates;
    state vehicleStates : VehicleStates {
        in operatingVehicle : Vehicle;
        in controller : VehicleController;
        entry;
        then off;
        state off;
        transition accept VehicleStartSignal then starting;
        state starting;
        transition accept VehicleOnSignal if operatingVehicle.brakePedalDepressed do send new ControllerStartSignal() to controller then on;
        state on {
            entry performSelfTest {
                in vehicle = operatingVehicle;
            }
            do action providePower {
                /* ... */
            }
            exit action applyParkingBrake {
                /* ... */
            }
        }
        transition accept VehicleOffSignal then off;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 219) (line 10) (column 25) (len 21)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 219) (line 10) (column 25) (len 12))) (segment 1 (token "Boolean") (name "Boolean") (separator colon-colon) (span (offset 233) (line 10) (column 39) (len 7)))))
  )
  (root (package (name "Transition Actions") (body brace (attribute-def (declaration-name "VehicleStartSignal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "VehicleOnSignal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "VehicleOffSignal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "ControllerStartSignal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (part-def (name "Vehicle") (modifiers) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "brakePedalDepressed") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "VehicleController") (modifiers) (body semicolon)) (action-usage (name "performSelfTest") (short-name none) (body brace (in-out-declaration))) (state-def (name "VehicleStates") (modifiers) (body semicolon)) (state-usage))))
)
~~~
