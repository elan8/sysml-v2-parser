# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 25 (Transitions): Transition Actions"))
~~~
# SOURCE
~~~sysml
package 'Transition Actions' {
	
	attribute def VehicleStartSignal;
	attribute def VehicleOnSignal;
	attribute def VehicleOffSignal;
	
	attribute def ControllerStartSignal;
	
	part def Vehicle {
		brakePedalDepressed : ScalarValues::Boolean;
	}
	part def VehicleController;
	
	action performSelfTest { in vehicle : Vehicle; }
	
	state def VehicleStates;
		
	state vehicleStates : VehicleStates {
		in operatingVehicle : Vehicle;
		in controller : VehicleController;

		entry; then off;
		
		state off;
		accept VehicleStartSignal 
			then starting;
			
		state starting;
		accept VehicleOnSignal
			if operatingVehicle.brakePedalDepressed
			do send new ControllerStartSignal() to controller
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
  (document "25_transition_actions.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Transition Actions' {
    attribute def VehicleStartSignal;
    attribute def VehicleOnSignal;
    attribute def VehicleOffSignal;
    attribute def ControllerStartSignal;
    part def Vehicle {
        brakePedalDepressed : ScalarValues::Boolean;
    }
    part def VehicleController;
    action performSelfTest {
        in vehicle : Vehicle;
    }
    state def VehicleStates;
    state vehicleStates : VehicleStates {
        in operatingVehicle : Vehicle;
        in controller : VehicleController;
        entry;
        then off;
        state off;
        transition accept VehicleStartSignal then starting;
        state starting;
        transition accept VehicleOnSignal if operatingVehicle.brakePedalDepressed do send new ControllerStartSignal() to controller then on;
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
    (reference r0 (scope relative) (span (offset 219) (line 10) (column 25) (len 21)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 219) (line 10) (column 25) (len 12))) (segment 1 (token "Boolean") (name "Boolean") (separator colon-colon) (span (offset 233) (line 10) (column 39) (len 7)))))
    (reference r1 (scope relative) (span (offset 380) (line 18) (column 24) (len 13)) (segments (segment 0 (token "VehicleStates") (name "VehicleStates") (separator none) (span (offset 380) (line 18) (column 24) (len 13)))))
    (reference r2 (scope relative) (span (offset 481) (line 22) (column 15) (len 3)) (segments (segment 0 (token "off") (name "off") (separator none) (span (offset 481) (line 22) (column 15) (len 3)))))
    (reference r3 (scope relative) (span (offset 511) (line 25) (column 10) (len 18)) (segments (segment 0 (token "VehicleStartSignal") (name "VehicleStartSignal") (separator none) (span (offset 511) (line 25) (column 10) (len 18)))))
    (reference r4 (scope relative) (span (offset 539) (line 26) (column 9) (len 8)) (segments (segment 0 (token "starting") (name "starting") (separator none) (span (offset 539) (line 26) (column 9) (len 8)))))
    (reference r5 (scope relative) (span (offset 580) (line 29) (column 10) (len 15)) (segments (segment 0 (token "VehicleOnSignal") (name "VehicleOnSignal") (separator none) (span (offset 580) (line 29) (column 10) (len 15)))))
    (reference r6 (scope relative) (span (offset 602) (line 30) (column 7) (len 16)) (segments (segment 0 (token "operatingVehicle") (name "operatingVehicle") (separator none) (span (offset 602) (line 30) (column 7) (len 16)))))
    (reference r7 (scope relative) (span (offset 619) (line 30) (column 24) (len 19)) (segments (segment 0 (token "brakePedalDepressed") (name "brakePedalDepressed") (separator none) (span (offset 619) (line 30) (column 24) (len 19)))))
    (reference r8 (scope relative) (span (offset 654) (line 31) (column 16) (len 21)) (segments (segment 0 (token "ControllerStartSignal") (name "ControllerStartSignal") (separator none) (span (offset 654) (line 31) (column 16) (len 21)))))
    (reference r9 (scope relative) (span (offset 681) (line 31) (column 43) (len 10)) (segments (segment 0 (token "controller") (name "controller") (separator none) (span (offset 681) (line 31) (column 43) (len 10)))))
    (reference r10 (scope relative) (span (offset 700) (line 32) (column 9) (len 2)) (segments (segment 0 (token "on") (name "on") (separator none) (span (offset 700) (line 32) (column 9) (len 2)))))
    (reference r11 (scope relative) (span (offset 730) (line 35) (column 10) (len 15)) (segments (segment 0 (token "performSelfTest") (name "performSelfTest") (separator none) (span (offset 730) (line 35) (column 10) (len 15)))))
    (reference r12 (scope relative) (span (offset 880) (line 39) (column 10) (len 16)) (segments (segment 0 (token "VehicleOffSignal") (name "VehicleOffSignal") (separator none) (span (offset 880) (line 39) (column 10) (len 16)))))
    (reference r13 (scope relative) (span (offset 905) (line 40) (column 9) (len 3)) (segments (segment 0 (token "off") (name "off") (separator none) (span (offset 905) (line 40) (column 9) (len 3)))))
  )
  (root (package (name "Transition Actions") (body brace (attribute-def (declaration-name "VehicleStartSignal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "VehicleOnSignal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "VehicleOffSignal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "ControllerStartSignal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (part-def (name "Vehicle") (modifiers) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "brakePedalDepressed") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "VehicleController") (modifiers) (body semicolon)) (action-usage (keyword action) (name "performSelfTest") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (in-out-declaration))) (state-def (name "VehicleStates") (modifiers) (body semicolon)) (state-usage (name "vehicleStates") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (inout-declaration) (inout-declaration) (entry (action-keyword false) (target none) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)) (then (state (ref r2))) (state-usage (name "off") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name none) (source none) (initial false) (accept (shorthand (expression (span (offset 511) (line 25) (column 10) (len 18)) (ref r3)) (via none))) (guard none) (effect none) (target (expression (span (offset 539) (line 26) (column 9) (len 8)) (ref r4))) (body semicolon)) (state-usage (name "starting") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name none) (source none) (initial false) (accept (shorthand (expression (span (offset 580) (line 29) (column 10) (len 15)) (ref r5)) (via none))) (guard (expression (span (offset 602) (line 30) (column 7) (len 36)) (member-access (base (expression (span (offset 602) (line 30) (column 7) (len 16)) (ref r6))) (separator dot) (member (ref r7))))) (effect (send (payload (expression (span (offset 650) (line 31) (column 12) (len 27)) (constructor (type (ref r8)) (arguments)))) (type none) (via none) (to (expression (span (offset 681) (line 31) (column 43) (len 10)) (ref r9))) (body none))) (target (expression (span (offset 700) (line 32) (column 9) (len 2)) (ref r10))) (body semicolon)) (state-usage (name "on") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (entry (action-keyword false) (target (ref r11)) (declared-name none) (type none) (redefines none) (effect false) (body brace (inout-declaration))) (do (action-keyword true) (target none) (declared-name "providePower") (type none) (redefines none) (effect false) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 810) (line 36) (column 31) (len 5)) (normalized "... "))))) (exit (action-keyword true) (target none) (declared-name "applyParkingBrake") (type none) (redefines none) (effect false) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 857) (line 37) (column 38) (len 5)) (normalized "... "))))))) (transition (name none) (source none) (initial false) (accept (shorthand (expression (span (offset 880) (line 39) (column 10) (len 16)) (ref r12)) (via none))) (guard none) (effect none) (target (expression (span (offset 905) (line 40) (column 9) (len 3)) (ref r13))) (body semicolon)))))))
)
~~~
