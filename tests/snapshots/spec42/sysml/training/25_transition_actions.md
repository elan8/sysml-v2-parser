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
            }
            exit action applyParkingBrake {
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
  )
  (root (package (name "Transition Actions") (body brace (attribute-def (name "VehicleStartSignal") (multiplicity none)) (attribute-def (name "VehicleOnSignal") (multiplicity none)) (attribute-def (name "VehicleOffSignal") (multiplicity none)) (attribute-def (name "ControllerStartSignal") (multiplicity none)) (part-def (name "Vehicle") (body brace (default-reference-usage))) (part-def (name "VehicleController") (body semicolon)) (action-usage (name "performSelfTest") (short-name none)) (state-def (name "VehicleStates") (body semicolon)) (state-usage))))
)
~~~
