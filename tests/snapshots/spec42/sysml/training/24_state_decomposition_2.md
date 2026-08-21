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
            /* ... */
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 190) (line 9) (column 24) (len 13)) (segments (segment 0 (token "VehicleStates") (name "VehicleStates") (separator none) (span (offset 190) (line 9) (column 24) (len 13)))))
    (reference r1 (scope relative) (span (offset 261) (line 12) (column 16) (len 3)) (segments (segment 0 (token "off") (name "off") (separator none) (span (offset 261) (line 12) (column 16) (len 3)))))
    (reference r2 (scope relative) (span (offset 294) (line 15) (column 11) (len 18)) (segments (segment 0 (token "VehicleStartSignal") (name "VehicleStartSignal") (separator none) (span (offset 294) (line 15) (column 11) (len 18)))))
    (reference r3 (scope relative) (span (offset 323) (line 16) (column 10) (len 8)) (segments (segment 0 (token "starting") (name "starting") (separator none) (span (offset 323) (line 16) (column 10) (len 8)))))
    (reference r4 (scope relative) (span (offset 367) (line 19) (column 11) (len 15)) (segments (segment 0 (token "VehicleOnSignal") (name "VehicleOnSignal") (separator none) (span (offset 367) (line 19) (column 11) (len 15)))))
    (reference r5 (scope relative) (span (offset 392) (line 20) (column 10) (len 2)) (segments (segment 0 (token "on") (name "on") (separator none) (span (offset 392) (line 20) (column 10) (len 2)))))
    (reference r6 (scope relative) (span (offset 424) (line 23) (column 11) (len 16)) (segments (segment 0 (token "VehicleOffSignal") (name "VehicleOffSignal") (separator none) (span (offset 424) (line 23) (column 11) (len 16)))))
    (reference r7 (scope relative) (span (offset 450) (line 24) (column 10) (len 3)) (segments (segment 0 (token "off") (name "off") (separator none) (span (offset 450) (line 24) (column 10) (len 3)))))
  )
  (root (package (name "State Decomposition-1") (body brace (attribute-def (declaration-name "VehicleStartSignal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "VehicleOnSignal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "VehicleOffSignal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (state-def (name "VehicleStates") (modifiers) (body semicolon)) (state-usage (name "vehicleStates") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (state-usage (name "operationalStates") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (entry (action-keyword false) (target none) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)) (then (state (ref r1))) (state-usage (name "off") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name none) (source none) (initial false) (accept (shorthand (expression (span (offset 294) (line 15) (column 11) (len 18)) (ref r2)) (via none))) (guard none) (effect none) (target (expression (span (offset 323) (line 16) (column 10) (len 8)) (ref r3))) (body semicolon)) (state-usage (name "starting") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name none) (source none) (initial false) (accept (shorthand (expression (span (offset 367) (line 19) (column 11) (len 15)) (ref r4)) (via none))) (guard none) (effect none) (target (expression (span (offset 392) (line 20) (column 10) (len 2)) (ref r5))) (body semicolon)) (state-usage (name "on") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name none) (source none) (initial false) (accept (shorthand (expression (span (offset 424) (line 23) (column 11) (len 16)) (ref r6)) (via none))) (guard none) (effect none) (target (expression (span (offset 450) (line 24) (column 10) (len 3)) (ref r7))) (body semicolon)))) (state-usage (name "healthStates") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 491) (line 28) (column 6) (len 5)) (normalized "... "))))))))))
)
~~~
