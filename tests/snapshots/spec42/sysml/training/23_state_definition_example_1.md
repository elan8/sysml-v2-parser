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
    (reference r1 (scope relative) (span (offset 247) (line 13) (column 10) (len 3)) (segments (segment 0 (token "off") (name "off") (separator none) (span (offset 247) (line 13) (column 10) (len 3)))))
    (reference r2 (scope relative) (span (offset 261) (line 14) (column 11) (len 18)) (segments (segment 0 (token "VehicleStartSignal") (name "VehicleStartSignal") (separator none) (span (offset 261) (line 14) (column 11) (len 18)))))
    (reference r3 (scope relative) (span (offset 289) (line 15) (column 9) (len 8)) (segments (segment 0 (token "starting") (name "starting") (separator none) (span (offset 289) (line 15) (column 9) (len 8)))))
    (reference r4 (scope relative) (span (offset 361) (line 20) (column 10) (len 8)) (segments (segment 0 (token "starting") (name "starting") (separator none) (span (offset 361) (line 20) (column 10) (len 8)))))
    (reference r5 (scope relative) (span (offset 380) (line 21) (column 11) (len 15)) (segments (segment 0 (token "VehicleOnSignal") (name "VehicleOnSignal") (separator none) (span (offset 380) (line 21) (column 11) (len 15)))))
    (reference r6 (scope relative) (span (offset 404) (line 22) (column 9) (len 2)) (segments (segment 0 (token "on") (name "on") (separator none) (span (offset 404) (line 22) (column 9) (len 2)))))
    (reference r7 (scope relative) (span (offset 459) (line 27) (column 10) (len 2)) (segments (segment 0 (token "on") (name "on") (separator none) (span (offset 459) (line 27) (column 10) (len 2)))))
    (reference r8 (scope relative) (span (offset 472) (line 28) (column 11) (len 16)) (segments (segment 0 (token "VehicleOffSignal") (name "VehicleOffSignal") (separator none) (span (offset 472) (line 28) (column 11) (len 16)))))
    (reference r9 (scope relative) (span (offset 497) (line 29) (column 9) (len 3)) (segments (segment 0 (token "off") (name "off") (separator none) (span (offset 497) (line 29) (column 9) (len 3)))))
  )
  (root (package (name "State Definition Example-1") (body brace (attribute-def (declaration-name "VehicleStartSignal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "VehicleOnSignal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "VehicleOffSignal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (state-def (name "VehicleStates") (modifiers) (body brace (entry (action-keyword false) (target none) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)) (then (state (ref r0))) (state-usage (name "off") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name "off_to_starting") (source (expression (span (offset 247) (line 13) (column 10) (len 3)) (ref r1))) (initial false) (accept (shorthand (expression (span (offset 261) (line 14) (column 11) (len 18)) (ref r2)) (via none))) (guard none) (effect none) (target (expression (span (offset 289) (line 15) (column 9) (len 8)) (ref r3))) (body semicolon)) (state-usage (name "starting") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name "starting_to_on") (source (expression (span (offset 361) (line 20) (column 10) (len 8)) (ref r4))) (initial false) (accept (shorthand (expression (span (offset 380) (line 21) (column 11) (len 15)) (ref r5)) (via none))) (guard none) (effect none) (target (expression (span (offset 404) (line 22) (column 9) (len 2)) (ref r6))) (body semicolon)) (state-usage (name "on") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name "on_to_off") (source (expression (span (offset 459) (line 27) (column 10) (len 2)) (ref r7))) (initial false) (accept (shorthand (expression (span (offset 472) (line 28) (column 11) (len 16)) (ref r8)) (via none))) (guard none) (effect none) (target (expression (span (offset 497) (line 29) (column 9) (len 3)) (ref r9))) (body semicolon)))))))
)
~~~
