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
    (reference r1 (scope relative) (span (offset 215) (line 11) (column 10) (len 18)) (segments (segment 0 (token "VehicleStartSignal") (name "VehicleStartSignal") (separator none) (span (offset 215) (line 11) (column 10) (len 18)))))
    (reference r2 (scope relative) (span (offset 243) (line 12) (column 9) (len 8)) (segments (segment 0 (token "starting") (name "starting") (separator none) (span (offset 243) (line 12) (column 9) (len 8)))))
    (reference r3 (scope relative) (span (offset 284) (line 15) (column 10) (len 15)) (segments (segment 0 (token "VehicleOnSignal") (name "VehicleOnSignal") (separator none) (span (offset 284) (line 15) (column 10) (len 15)))))
    (reference r4 (scope relative) (span (offset 308) (line 16) (column 9) (len 2)) (segments (segment 0 (token "on") (name "on") (separator none) (span (offset 308) (line 16) (column 9) (len 2)))))
    (reference r5 (scope relative) (span (offset 337) (line 19) (column 10) (len 16)) (segments (segment 0 (token "VehicleOffSignal") (name "VehicleOffSignal") (separator none) (span (offset 337) (line 19) (column 10) (len 16)))))
    (reference r6 (scope relative) (span (offset 362) (line 20) (column 9) (len 3)) (segments (segment 0 (token "off") (name "off") (separator none) (span (offset 362) (line 20) (column 9) (len 3)))))
  )
  (root (package (name "State Definition Example-2") (body brace (attribute-def (declaration-name "VehicleStartSignal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "VehicleOnSignal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "VehicleOffSignal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (state-def (name "VehicleStates") (modifiers) (body brace (entry (action-keyword false) (target none) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)) (then (state (ref r0))) (state-usage) (transition (name none) (source none) (initial false) (accept (shorthand (expression (span (offset 215) (line 11) (column 10) (len 18)) (ref r1)) (via none))) (guard none) (effect none) (target (expression (span (offset 243) (line 12) (column 9) (len 8)) (ref r2))) (body semicolon)) (state-usage) (transition (name none) (source none) (initial false) (accept (shorthand (expression (span (offset 284) (line 15) (column 10) (len 15)) (ref r3)) (via none))) (guard none) (effect none) (target (expression (span (offset 308) (line 16) (column 9) (len 2)) (ref r4))) (body semicolon)) (state-usage) (transition (name none) (source none) (initial false) (accept (shorthand (expression (span (offset 337) (line 19) (column 10) (len 16)) (ref r5)) (via none))) (guard none) (effect none) (target (expression (span (offset 362) (line 20) (column 9) (len 3)) (ref r6))) (body semicolon)))))))
)
~~~
