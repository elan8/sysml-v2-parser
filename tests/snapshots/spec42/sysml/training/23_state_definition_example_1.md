# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 23 (State Definitions): State Definition Example-1"))
~~~
# SOURCE
~~~sysml
package 'State Definition Example-1' {
	
	attribute def VehicleStartSignal;
	attribute def VehicleOnSignal;
	attribute def VehicleOffSignal;
		
	state def VehicleStates {
		entry; then off;
		
		state off;
		
		transition off_to_starting
			first off
			accept VehicleStartSignal 
			then starting;
			
		state starting;
		
		transition starting_to_on
			first starting
			accept VehicleOnSignal
			then on;
			
		state on;
		
		transition on_to_off
			first on
			accept VehicleOffSignal
			then off;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "23_state_definition_example_1.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'State Definition Example-1' {
    attribute def VehicleStartSignal;
    attribute def VehicleOnSignal;
    attribute def VehicleOffSignal;
    state def VehicleStates {
        entry;
        then off;
        state off;
        transition off_to_starting first off accept VehicleStartSignal then starting;
        state starting;
        transition starting_to_on first starting accept VehicleOnSignal then on;
        state on;
        transition on_to_off first on accept VehicleOffSignal then off;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 185) (line 8) (column 15) (len 3)) (segments (segment 0 (token "off") (name "off") (separator none) (span (offset 185) (line 8) (column 15) (len 3)))))
  )
  (root (package (name "State Definition Example-1") (body (attribute-def) (attribute-def) (attribute-def) (state-def (name "VehicleStates") (body (entry (action-keyword false) (target none) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)) (then (state (ref r0))) (state-usage) (transition) (state-usage) (transition) (state-usage) (transition))))))
)
~~~
