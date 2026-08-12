# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 23 (State Definitions): State Definition Example-2"))
~~~
# SOURCE
~~~sysml
package 'State Definition Example-2' {
	
	attribute def VehicleStartSignal;
	attribute def VehicleOnSignal;
	attribute def VehicleOffSignal;
		
	state def VehicleStates {
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
  (document "23_state_definition_example_2.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'State Definition Example-2' {
    attribute def VehicleStartSignal;
    attribute def VehicleOnSignal;
    attribute def VehicleOffSignal;
    state def VehicleStates {
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
    (reference r0 (scope relative) (span (offset 185) (line 8) (column 15) (len 3)) (segments (segment 0 (token "off") (name "off") (separator none) (span (offset 185) (line 8) (column 15) (len 3)))))
  )
  (root (package (name "State Definition Example-2") (body (attribute-def) (attribute-def) (attribute-def) (state-def (name "VehicleStates") (body (entry) (then (state (ref r0))) (state-usage) (transition) (state-usage) (transition) (state-usage) (transition))))))
)
~~~
