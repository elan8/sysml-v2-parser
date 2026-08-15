# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 24 (States): State Decomposition-2"))
~~~
# SOURCE
~~~sysml
package 'State Decomposition-1' {
	
	attribute def VehicleStartSignal;
	attribute def VehicleOnSignal;
	attribute def VehicleOffSignal;
	
	state def VehicleStates;
		
	state vehicleStates : VehicleStates parallel {
		
		state operationalStates {
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
		
		state healthStates { 
			/* ... */
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "24_state_decomposition_2.md"
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
        state operationalStates {
            entry;
            then off;
            state off;
            transition accept VehicleStartSignal then starting;
            state starting;
            transition accept VehicleOnSignal then on;
            state on;
            transition accept VehicleOffSignal then off;
        }
        state healthStates {
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "State Decomposition-1") (body brace (attribute-def) (attribute-def) (attribute-def) (state-def (name "VehicleStates") (body semicolon)) (state-usage))))
)
~~~
