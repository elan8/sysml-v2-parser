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
    (reference r0 (scope relative) (span (offset 190) (line 9) (column 24) (len 13)) (segments (segment 0 (token "VehicleStates") (name "VehicleStates") (separator none) (span (offset 190) (line 9) (column 24) (len 13)))))
    (reference r1 (scope relative) (span (offset 220) (line 10) (column 15) (len 3)) (segments (segment 0 (token "off") (name "off") (separator none) (span (offset 220) (line 10) (column 15) (len 3)))))
    (reference r2 (scope relative) (span (offset 250) (line 13) (column 10) (len 18)) (segments (segment 0 (token "VehicleStartSignal") (name "VehicleStartSignal") (separator none) (span (offset 250) (line 13) (column 10) (len 18)))))
    (reference r3 (scope relative) (span (offset 278) (line 14) (column 9) (len 8)) (segments (segment 0 (token "starting") (name "starting") (separator none) (span (offset 278) (line 14) (column 9) (len 8)))))
    (reference r4 (scope relative) (span (offset 319) (line 17) (column 10) (len 15)) (segments (segment 0 (token "VehicleOnSignal") (name "VehicleOnSignal") (separator none) (span (offset 319) (line 17) (column 10) (len 15)))))
    (reference r5 (scope relative) (span (offset 343) (line 18) (column 9) (len 2)) (segments (segment 0 (token "on") (name "on") (separator none) (span (offset 343) (line 18) (column 9) (len 2)))))
    (reference r6 (scope relative) (span (offset 372) (line 21) (column 10) (len 16)) (segments (segment 0 (token "VehicleOffSignal") (name "VehicleOffSignal") (separator none) (span (offset 372) (line 21) (column 10) (len 16)))))
    (reference r7 (scope relative) (span (offset 397) (line 22) (column 9) (len 3)) (segments (segment 0 (token "off") (name "off") (separator none) (span (offset 397) (line 22) (column 9) (len 3)))))
  )
  (root (package (name "State Decomposition-1") (body brace (attribute-def (declaration-name "VehicleStartSignal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "VehicleOnSignal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "VehicleOffSignal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (state-def (name "VehicleStates") (modifiers) (body semicolon)) (state-usage (name "vehicleStates") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (entry (action-keyword false) (target none) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)) (then (state (ref r1))) (state-usage (name "off") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name none) (source none) (initial false) (accept (shorthand (expression (span (offset 250) (line 13) (column 10) (len 18)) (ref r2)) (via none))) (guard none) (effect none) (target (expression (span (offset 278) (line 14) (column 9) (len 8)) (ref r3))) (body semicolon)) (state-usage (name "starting") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name none) (source none) (initial false) (accept (shorthand (expression (span (offset 319) (line 17) (column 10) (len 15)) (ref r4)) (via none))) (guard none) (effect none) (target (expression (span (offset 343) (line 18) (column 9) (len 2)) (ref r5))) (body semicolon)) (state-usage (name "on") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name none) (source none) (initial false) (accept (shorthand (expression (span (offset 372) (line 21) (column 10) (len 16)) (ref r6)) (via none))) (guard none) (effect none) (target (expression (span (offset 397) (line 22) (column 9) (len 3)) (ref r7))) (body semicolon)))))))
)
~~~
