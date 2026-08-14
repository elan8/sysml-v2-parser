# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 24 (States): State Actions"))
~~~
# SOURCE
~~~sysml
package 'State Actions' {
	
	attribute def VehicleStartSignal;
	attribute def VehicleOnSignal;
	attribute def VehicleOffSignal;
	
	part def Vehicle;
	
	action performSelfTest { in vehicle : Vehicle; }
	
	state def VehicleStates { in operatingVehicle : Vehicle; }
		
	state vehicleStates : VehicleStates {
		in operatingVehicle : Vehicle;
			
		entry; then off;
		
		state off;
		accept VehicleStartSignal 
			then starting;
			
		state starting;
		accept VehicleOnSignal
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
  (document "24_state_actions.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'State Actions' {
    attribute def VehicleStartSignal;
    attribute def VehicleOnSignal;
    attribute def VehicleOffSignal;
    part def Vehicle;
    action performSelfTest {
        in vehicle : Vehicle;
    }
    state def VehicleStates {
        in operatingVehicle : Vehicle;
    }
    state vehicleStates : VehicleStates {
        in operatingVehicle : Vehicle;
        entry;
        then off;
        state off;
        transition accept VehicleStartSignal then starting;
        state starting;
        transition accept VehicleOnSignal then on;
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
  (root (package (name "State Actions") (body (attribute-def) (attribute-def) (attribute-def) (part-def (name "Vehicle") (body semicolon)) (action-usage) (state-def (name "VehicleStates") (body (inout-declaration))) (state-usage))))
)
~~~
