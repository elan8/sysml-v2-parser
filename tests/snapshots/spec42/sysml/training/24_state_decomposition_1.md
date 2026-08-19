# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 24 (States): State Decomposition-1"))
~~~
# SOURCE
~~~sysml
package 'State Decomposition-1' {
	
	attribute def VehicleStartSignal;
	attribute def VehicleOnSignal;
	attribute def VehicleOffSignal;
	
	state def VehicleStates;
		
	state vehicleStates : VehicleStates {
		entry; then off;
		
		state off;
		accept VehicleStartSignal 
			then starting;
			
		state starting;
		accept VehicleOnSignal
			then on;
			
		state on;
		accept VehicleOffSignal
			then off;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "24_state_decomposition_1.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'State Decomposition-1' {
    attribute def VehicleStartSignal;
    attribute def VehicleOnSignal;
    attribute def VehicleOffSignal;
    state def VehicleStates;
    state vehicleStates : VehicleStates {
        entry;
        then off;
        state off;
        transition accept VehicleStartSignal then starting;
        state starting;
        transition accept VehicleOnSignal then on;
        state on;
        transition accept VehicleOffSignal then off;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "State Decomposition-1") (body brace (attribute-def (declaration-name "VehicleStartSignal") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "VehicleOnSignal") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "VehicleOffSignal") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (state-def (name "VehicleStates") (body semicolon)) (state-usage))))
)
~~~
