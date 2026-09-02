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
                /* ... */
            }
            exit action applyParkingBrake {
                /* ... */
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
    (reference r0 (scope relative) (span (offset 289) (line 13) (column 24) (len 13)) (segments (segment 0 (token "VehicleStates") (name "VehicleStates") (separator none) (span (offset 289) (line 13) (column 24) (len 13)))))
    (reference r1 (scope relative) (span (offset 356) (line 16) (column 15) (len 3)) (segments (segment 0 (token "off") (name "off") (separator none) (span (offset 356) (line 16) (column 15) (len 3)))))
    (reference r2 (scope relative) (span (offset 386) (line 19) (column 10) (len 18)) (segments (segment 0 (token "VehicleStartSignal") (name "VehicleStartSignal") (separator none) (span (offset 386) (line 19) (column 10) (len 18)))))
    (reference r3 (scope relative) (span (offset 414) (line 20) (column 9) (len 8)) (segments (segment 0 (token "starting") (name "starting") (separator none) (span (offset 414) (line 20) (column 9) (len 8)))))
    (reference r4 (scope relative) (span (offset 455) (line 23) (column 10) (len 15)) (segments (segment 0 (token "VehicleOnSignal") (name "VehicleOnSignal") (separator none) (span (offset 455) (line 23) (column 10) (len 15)))))
    (reference r5 (scope relative) (span (offset 479) (line 24) (column 9) (len 2)) (segments (segment 0 (token "on") (name "on") (separator none) (span (offset 479) (line 24) (column 9) (len 2)))))
    (reference r6 (scope relative) (span (offset 509) (line 27) (column 10) (len 15)) (segments (segment 0 (token "performSelfTest") (name "performSelfTest") (separator none) (span (offset 509) (line 27) (column 10) (len 15)))))
    (reference r7 (scope relative) (span (offset 659) (line 31) (column 10) (len 16)) (segments (segment 0 (token "VehicleOffSignal") (name "VehicleOffSignal") (separator none) (span (offset 659) (line 31) (column 10) (len 16)))))
    (reference r8 (scope relative) (span (offset 684) (line 32) (column 9) (len 3)) (segments (segment 0 (token "off") (name "off") (separator none) (span (offset 684) (line 32) (column 9) (len 3)))))
  )
  (root (package (name "State Actions") (body brace (attribute-def (declaration-name "VehicleStartSignal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "VehicleOnSignal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "VehicleOffSignal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (part-def (name "Vehicle") (modifiers) (body semicolon)) (action-usage (keyword action) (name "performSelfTest") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (in-out-declaration))) (state-def (name "VehicleStates") (modifiers) (body brace (inout-declaration))) (state-usage (name "vehicleStates") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (inout-declaration) (entry (action-keyword false) (target none) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)) (then (state (ref r1))) (state-usage (name "off") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name none) (source none) (initial false) (accept (shorthand (expression (span (offset 386) (line 19) (column 10) (len 18)) (ref r2)) (via none))) (guard none) (effect none) (target (expression (span (offset 414) (line 20) (column 9) (len 8)) (ref r3))) (body semicolon)) (state-usage (name "starting") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name none) (source none) (initial false) (accept (shorthand (expression (span (offset 455) (line 23) (column 10) (len 15)) (ref r4)) (via none))) (guard none) (effect none) (target (expression (span (offset 479) (line 24) (column 9) (len 2)) (ref r5))) (body semicolon)) (state-usage (name "on") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (entry (action-keyword false) (target (ref r6)) (declared-name none) (type none) (redefines none) (effect false) (body brace (inout-declaration))) (do (action-keyword true) (target none) (declared-name "providePower") (type none) (redefines none) (effect false) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 589) (line 28) (column 31) (len 5)) (normalized "... "))))) (exit (action-keyword true) (target none) (declared-name "applyParkingBrake") (type none) (redefines none) (effect false) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 636) (line 29) (column 38) (len 5)) (normalized "... "))))))) (transition (name none) (source none) (initial false) (accept (shorthand (expression (span (offset 659) (line 31) (column 10) (len 16)) (ref r7)) (via none))) (guard none) (effect none) (target (expression (span (offset 684) (line 32) (column 9) (len 3)) (ref r8))) (body semicolon)))))))
)
~~~
