# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (05-State-based Behavior): 5-State-based Behavior-2"))
~~~
# SOURCE
~~~sysml
package '5-State-based Behavior-2' {
	private import ScalarValues::*;
	private import ISQ::*;
	private import '3a-Function-based Behavior-1'::*;
	
	package Definitions {
		part def VehicleA {
			perform action 'provide power': 'Provide Power';
			exhibit state 'vehicle states': 'Vehicle States';
		}
		
		part def VehicleController {
			exhibit state 'controller states': 'Controller States';
		}

		state def 'Vehicle States';
		state def 'Controller States';	

		action def 'Perform Self Test';
		action def 'Apply Parking Brake';
		action def 'Sense Temperature' { out temp: TemperatureValue; }
		
		attribute def 'Vehicle Start Signal';
		attribute def 'Vehicle On Signal';
		attribute def 'Vehicle Off Signal';
		
		attribute def 'Start Signal';
		attribute def 'Off Signal';
		attribute def 'Over Temp';
		attribute def 'Return to Normal';
	}
	
	package Usages {
		private import Definitions::*;
		 
		action 'perform self test': 'Perform Self Test';
		action 'apply parking brake': 'Apply Parking Brake';
		action 'sense temperature': 'Sense Temperature';
		
		state 'vehicle states': 'Vehicle States' parallel {

			state 'operational states' {
				entry; then off;
				
				/*
				 * The following uses a shorthand for a transition whose source 
				 * is the immediately preceding state.
				 */
				state off;
				accept 'Vehicle Start Signal' 
					if vehicle1_c1.'brake pedal depressed'
					do send new 'Start Signal'() to vehicle1_c1.vehicleController
					then starting;
					
				state starting;
				accept 'Vehicle On Signal'
					then on;
					
				state on {
					entry 'perform self test';
					do 'provide power';
					exit 'apply parking brake';
				}
				accept 'Vehicle Off Signal'
					then off;
			}
			
			state 'health states' {
				entry; then normal;
				do 'sense temperature' { out temp; }
				
				/*
				 * The shorthand can be used for multiple transitions after
				 * a single state.
				 */
				state normal;
				accept at vehicle1_c1.maintenanceTime
					then maintenance;
				accept when 'sense temperature'.temp > vehicle1_c1.Tmax
					do send new 'Over Temp'() to vehicle1_c1.vehicleController 
					then degraded;
				
				state maintenance;
				accept 'Return to Normal'
					then normal;
				
				state degraded;
				accept 'Return to Normal'
					then normal;
			}
		}
		
		state 'controller states': 'Controller States' parallel {
			state 'operational controller states' {
				entry; then off;
				
				state off;
				accept 'Start Signal'
					then on;
				
				state on;
				accept 'Off Signal'
					then off;
			}
		}		

		part vehicle1_c1: VehicleA {
			port fuelCmdPort {
				in fuelCmd: FuelCmd;
			}
			
			attribute 'brake pedal depressed': Boolean;		
			attribute maintenanceTime: Time::DateTime;
			attribute Tmax: TemperatureValue;
			
			perform 'provide power' :>> VehicleA::'provide power' {
				in fuelCmd = fuelCmdPort.fuelCmd;
			}
				
			exhibit 'vehicle states' :>> VehicleA::'vehicle states';
				
			part vehicleController: VehicleController {
				exhibit 'controller states' :>> VehicleController::'controller states';
			}			
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "5_state_based_behavior_2.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '5-State-based Behavior-2' {
    private import ScalarValues::*;
    private import ISQ::*;
    private import '3a-Function-based Behavior-1'::*;
    package Definitions {
        part def VehicleA {
            perform action 'provide power' : 'Provide Power';
            exhibit state 'vehicle states' : 'Vehicle States';
        }
        part def VehicleController {
            exhibit state 'controller states' : 'Controller States';
        }
        state def 'Vehicle States';
        state def 'Controller States';
        action def 'Perform Self Test';
        action def 'Apply Parking Brake';
        action def 'Sense Temperature' {
            out temp : TemperatureValue;
        }
        attribute def 'Vehicle Start Signal';
        attribute def 'Vehicle On Signal';
        attribute def 'Vehicle Off Signal';
        attribute def 'Start Signal';
        attribute def 'Off Signal';
        attribute def 'Over Temp';
        attribute def 'Return to Normal';
    }
    package Usages {
        private import Definitions::*;
        action 'perform self test' : 'Perform Self Test';
        action 'apply parking brake' : 'Apply Parking Brake';
        action 'sense temperature' : 'Sense Temperature';
        state 'vehicle states' : 'Vehicle States' {
            state 'operational states' {
                entry;
                then off;
                /*
				 * The following uses a shorthand for a transition whose source 
				 * is the immediately preceding state.
				 */
                state off;
                transition accept 'Vehicle Start Signal' if vehicle1_c1.'brake pedal depressed' do send new 'Start Signal'() to vehicle1_c1.vehicleController then starting;
                state starting;
                transition accept 'Vehicle On Signal' then on;
                state on {
                    entry 'perform self test';
                    do 'provide power';
                    exit 'apply parking brake';
                }
                transition accept 'Vehicle Off Signal' then off;
            }
            state 'health states' {
                entry;
                then normal;
                do 'sense temperature' {
                    out temp;
                }
                /*
				 * The shorthand can be used for multiple transitions after
				 * a single state.
				 */
                state normal;
                transition accept at vehicle1_c1.maintenanceTime then maintenance;
                transition accept when 'sense temperature'.temp > vehicle1_c1.Tmax do send new 'Over Temp'() to vehicle1_c1.vehicleController then degraded;
                state maintenance;
                transition accept 'Return to Normal' then normal;
                state degraded;
                transition accept 'Return to Normal' then normal;
            }
        }
        state 'controller states' : 'Controller States' {
            state 'operational controller states' {
                entry;
                then off;
                state off;
                transition accept 'Start Signal' then on;
                state on;
                transition accept 'Off Signal' then off;
            }
        }
        part vehicle1_c1 : VehicleA {
            port fuelCmdPort {
                in fuelCmd : FuelCmd;
            }
            attribute 'brake pedal depressed' : Boolean;
            attribute maintenanceTime : Time::DateTime;
            attribute Tmax : TemperatureValue;
            perform 'provide power' :>> VehicleA::'provide power' {
                in fuelCmd = fuelCmdPort.fuelCmd;
            }
            state 'vehicle states' :>> VehicleA::'vehicle states';
            part vehicleController : VehicleController {
                state 'controller states' :>> VehicleController::'controller states';
            }
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 53) (line 2) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 53) (line 2) (column 17) (len 12)))))
    (reference r1 (scope relative) (span (offset 86) (line 3) (column 17) (len 3)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 86) (line 3) (column 17) (len 3)))))
    (reference r2 (scope relative) (span (offset 110) (line 4) (column 17) (len 30)) (segments (segment 0 (token "'3a-Function-based Behavior-1'") (name "3a-Function-based Behavior-1") (separator none) (span (offset 110) (line 4) (column 17) (len 30)))))
    (reference r3 (scope relative) (span (offset 227) (line 8) (column 36) (len 15)) (segments (segment 0 (token "'Provide Power'") (name "Provide Power") (separator none) (span (offset 227) (line 8) (column 36) (len 15)))))
    (reference r4 (scope relative) (span (offset 579) (line 21) (column 46) (len 16)) (segments (segment 0 (token "TemperatureValue") (name "TemperatureValue") (separator none) (span (offset 579) (line 21) (column 46) (len 16)))))
    (reference r5 (scope relative) (span (offset 887) (line 34) (column 18) (len 11)) (segments (segment 0 (token "Definitions") (name "Definitions") (separator none) (span (offset 887) (line 34) (column 18) (len 11)))))
    (reference r6 (scope relative) (span (offset 937) (line 36) (column 31) (len 19)) (segments (segment 0 (token "'Perform Self Test'") (name "Perform Self Test") (separator none) (span (offset 937) (line 36) (column 31) (len 19)))))
    (reference r7 (scope relative) (span (offset 990) (line 37) (column 33) (len 21)) (segments (segment 0 (token "'Apply Parking Brake'") (name "Apply Parking Brake") (separator none) (span (offset 990) (line 37) (column 33) (len 21)))))
    (reference r8 (scope relative) (span (offset 1043) (line 38) (column 31) (len 19)) (segments (segment 0 (token "'Sense Temperature'") (name "Sense Temperature") (separator none) (span (offset 1043) (line 38) (column 31) (len 19)))))
    (reference r9 (scope relative) (span (offset 1093) (line 40) (column 27) (len 16)) (segments (segment 0 (token "'Vehicle States'") (name "Vehicle States") (separator none) (span (offset 1093) (line 40) (column 27) (len 16)))))
    (reference r10 (scope relative) (span (offset 1170) (line 43) (column 17) (len 3)) (segments (segment 0 (token "off") (name "off") (separator none) (span (offset 1170) (line 43) (column 17) (len 3)))))
    (reference r11 (scope relative) (span (offset 1333) (line 50) (column 12) (len 22)) (segments (segment 0 (token "'Vehicle Start Signal'") (name "Vehicle Start Signal") (separator none) (span (offset 1333) (line 50) (column 12) (len 22)))))
    (reference r12 (scope relative) (span (offset 1365) (line 51) (column 9) (len 11)) (segments (segment 0 (token "vehicle1_c1") (name "vehicle1_c1") (separator none) (span (offset 1365) (line 51) (column 9) (len 11)))))
    (reference r13 (scope relative) (span (offset 1377) (line 51) (column 21) (len 23)) (segments (segment 0 (token "'brake pedal depressed'") (name "brake pedal depressed") (separator none) (span (offset 1377) (line 51) (column 21) (len 23)))))
    (reference r14 (scope relative) (span (offset 1418) (line 52) (column 18) (len 14)) (segments (segment 0 (token "'Start Signal'") (name "Start Signal") (separator none) (span (offset 1418) (line 52) (column 18) (len 14)))))
    (reference r15 (scope relative) (span (offset 1438) (line 52) (column 38) (len 11)) (segments (segment 0 (token "vehicle1_c1") (name "vehicle1_c1") (separator none) (span (offset 1438) (line 52) (column 38) (len 11)))))
    (reference r16 (scope relative) (span (offset 1450) (line 52) (column 50) (len 17)) (segments (segment 0 (token "vehicleController") (name "vehicleController") (separator none) (span (offset 1450) (line 52) (column 50) (len 17)))))
    (reference r17 (scope relative) (span (offset 1478) (line 53) (column 11) (len 8)) (segments (segment 0 (token "starting") (name "starting") (separator none) (span (offset 1478) (line 53) (column 11) (len 8)))))
    (reference r18 (scope relative) (span (offset 1525) (line 56) (column 12) (len 19)) (segments (segment 0 (token "'Vehicle On Signal'") (name "Vehicle On Signal") (separator none) (span (offset 1525) (line 56) (column 12) (len 19)))))
    (reference r19 (scope relative) (span (offset 1555) (line 57) (column 11) (len 2)) (segments (segment 0 (token "on") (name "on") (separator none) (span (offset 1555) (line 57) (column 11) (len 2)))))
    (reference r20 (scope relative) (span (offset 1591) (line 60) (column 12) (len 19)) (segments (segment 0 (token "'perform self test'") (name "perform self test") (separator none) (span (offset 1591) (line 60) (column 12) (len 19)))))
    (reference r21 (scope relative) (span (offset 1620) (line 61) (column 9) (len 15)) (segments (segment 0 (token "'provide power'") (name "provide power") (separator none) (span (offset 1620) (line 61) (column 9) (len 15)))))
    (reference r22 (scope relative) (span (offset 1647) (line 62) (column 11) (len 21)) (segments (segment 0 (token "'apply parking brake'") (name "apply parking brake") (separator none) (span (offset 1647) (line 62) (column 11) (len 21)))))
    (reference r23 (scope relative) (span (offset 1687) (line 64) (column 12) (len 20)) (segments (segment 0 (token "'Vehicle Off Signal'") (name "Vehicle Off Signal") (separator none) (span (offset 1687) (line 64) (column 12) (len 20)))))
    (reference r24 (scope relative) (span (offset 1718) (line 65) (column 11) (len 3)) (segments (segment 0 (token "off") (name "off") (separator none) (span (offset 1718) (line 65) (column 11) (len 3)))))
    (reference r25 (scope relative) (span (offset 1775) (line 69) (column 17) (len 6)) (segments (segment 0 (token "normal") (name "normal") (separator none) (span (offset 1775) (line 69) (column 17) (len 6)))))
    (reference r26 (scope relative) (span (offset 1790) (line 70) (column 8) (len 19)) (segments (segment 0 (token "'sense temperature'") (name "sense temperature") (separator none) (span (offset 1790) (line 70) (column 8) (len 19)))))
    (reference r27 (scope relative) (span (offset 1963) (line 77) (column 15) (len 11)) (segments (segment 0 (token "vehicle1_c1") (name "vehicle1_c1") (separator none) (span (offset 1963) (line 77) (column 15) (len 11)))))
    (reference r28 (scope relative) (span (offset 1975) (line 77) (column 27) (len 15)) (segments (segment 0 (token "maintenanceTime") (name "maintenanceTime") (separator none) (span (offset 1975) (line 77) (column 27) (len 15)))))
    (reference r29 (scope relative) (span (offset 2001) (line 78) (column 11) (len 11)) (segments (segment 0 (token "maintenance") (name "maintenance") (separator none) (span (offset 2001) (line 78) (column 11) (len 11)))))
    (reference r30 (scope relative) (span (offset 2030) (line 79) (column 17) (len 19)) (segments (segment 0 (token "'sense temperature'") (name "sense temperature") (separator none) (span (offset 2030) (line 79) (column 17) (len 19)))))
    (reference r31 (scope relative) (span (offset 2050) (line 79) (column 37) (len 4)) (segments (segment 0 (token "temp") (name "temp") (separator none) (span (offset 2050) (line 79) (column 37) (len 4)))))
    (reference r32 (scope relative) (span (offset 2057) (line 79) (column 44) (len 11)) (segments (segment 0 (token "vehicle1_c1") (name "vehicle1_c1") (separator none) (span (offset 2057) (line 79) (column 44) (len 11)))))
    (reference r33 (scope relative) (span (offset 2069) (line 79) (column 56) (len 4)) (segments (segment 0 (token "Tmax") (name "Tmax") (separator none) (span (offset 2069) (line 79) (column 56) (len 4)))))
    (reference r34 (scope relative) (span (offset 2091) (line 80) (column 18) (len 11)) (segments (segment 0 (token "'Over Temp'") (name "Over Temp") (separator none) (span (offset 2091) (line 80) (column 18) (len 11)))))
    (reference r35 (scope relative) (span (offset 2108) (line 80) (column 35) (len 11)) (segments (segment 0 (token "vehicle1_c1") (name "vehicle1_c1") (separator none) (span (offset 2108) (line 80) (column 35) (len 11)))))
    (reference r36 (scope relative) (span (offset 2120) (line 80) (column 47) (len 17)) (segments (segment 0 (token "vehicleController") (name "vehicleController") (separator none) (span (offset 2120) (line 80) (column 47) (len 17)))))
    (reference r37 (scope relative) (span (offset 2149) (line 81) (column 11) (len 8)) (segments (segment 0 (token "degraded") (name "degraded") (separator none) (span (offset 2149) (line 81) (column 11) (len 8)))))
    (reference r38 (scope relative) (span (offset 2198) (line 84) (column 12) (len 18)) (segments (segment 0 (token "'Return to Normal'") (name "Return to Normal") (separator none) (span (offset 2198) (line 84) (column 12) (len 18)))))
    (reference r39 (scope relative) (span (offset 2227) (line 85) (column 11) (len 6)) (segments (segment 0 (token "normal") (name "normal") (separator none) (span (offset 2227) (line 85) (column 11) (len 6)))))
    (reference r40 (scope relative) (span (offset 2271) (line 88) (column 12) (len 18)) (segments (segment 0 (token "'Return to Normal'") (name "Return to Normal") (separator none) (span (offset 2271) (line 88) (column 12) (len 18)))))
    (reference r41 (scope relative) (span (offset 2300) (line 89) (column 11) (len 6)) (segments (segment 0 (token "normal") (name "normal") (separator none) (span (offset 2300) (line 89) (column 11) (len 6)))))
    (reference r42 (scope relative) (span (offset 2349) (line 93) (column 30) (len 19)) (segments (segment 0 (token "'Controller States'") (name "Controller States") (separator none) (span (offset 2349) (line 93) (column 30) (len 19)))))
    (reference r43 (scope relative) (span (offset 2439) (line 95) (column 17) (len 3)) (segments (segment 0 (token "off") (name "off") (separator none) (span (offset 2439) (line 95) (column 17) (len 3)))))
    (reference r44 (scope relative) (span (offset 2475) (line 98) (column 12) (len 14)) (segments (segment 0 (token "'Start Signal'") (name "Start Signal") (separator none) (span (offset 2475) (line 98) (column 12) (len 14)))))
    (reference r45 (scope relative) (span (offset 2500) (line 99) (column 11) (len 2)) (segments (segment 0 (token "on") (name "on") (separator none) (span (offset 2500) (line 99) (column 11) (len 2)))))
    (reference r46 (scope relative) (span (offset 2534) (line 102) (column 12) (len 12)) (segments (segment 0 (token "'Off Signal'") (name "Off Signal") (separator none) (span (offset 2534) (line 102) (column 12) (len 12)))))
    (reference r47 (scope relative) (span (offset 2557) (line 103) (column 11) (len 3)) (segments (segment 0 (token "off") (name "off") (separator none) (span (offset 2557) (line 103) (column 11) (len 3)))))
    (reference r48 (scope relative) (span (offset 2594) (line 107) (column 21) (len 8)) (segments (segment 0 (token "VehicleA") (name "VehicleA") (separator none) (span (offset 2594) (line 107) (column 21) (len 8)))))
    (reference r49 (scope relative) (span (offset 2699) (line 112) (column 39) (len 7)) (segments (segment 0 (token "Boolean") (name "Boolean") (separator none) (span (offset 2699) (line 112) (column 39) (len 7)))))
    (reference r50 (scope relative) (span (offset 2740) (line 113) (column 31) (len 14)) (segments (segment 0 (token "Time") (name "Time") (separator none) (span (offset 2740) (line 113) (column 31) (len 4))) (segment 1 (token "DateTime") (name "DateTime") (separator colon-colon) (span (offset 2746) (line 113) (column 37) (len 8)))))
    (reference r51 (scope relative) (span (offset 2775) (line 114) (column 20) (len 16)) (segments (segment 0 (token "TemperatureValue") (name "TemperatureValue") (separator none) (span (offset 2775) (line 114) (column 20) (len 16)))))
    (reference r52 (scope relative) (span (offset 2808) (line 116) (column 12) (len 15)) (segments (segment 0 (token "'provide power'") (name "provide power") (separator none) (span (offset 2808) (line 116) (column 12) (len 15)))))
    (reference r53 (scope relative) (span (offset 2828) (line 116) (column 32) (len 25)) (segments (segment 0 (token "VehicleA") (name "VehicleA") (separator none) (span (offset 2828) (line 116) (column 32) (len 8))) (segment 1 (token "'provide power'") (name "provide power") (separator colon-colon) (span (offset 2838) (line 116) (column 42) (len 15)))))
    (reference r54 (scope relative) (span (offset 2863) (line 117) (column 8) (len 7)) (segments (segment 0 (token "fuelCmd") (name "fuelCmd") (separator none) (span (offset 2863) (line 117) (column 8) (len 7)))))
    (reference r55 (scope relative) (span (offset 2873) (line 117) (column 18) (len 19)) (segments (segment 0 (token "fuelCmdPort") (name "fuelCmdPort") (separator none) (span (offset 2873) (line 117) (column 18) (len 11))) (segment 1 (token "fuelCmd") (name "fuelCmd") (separator dot) (span (offset 2885) (line 117) (column 30) (len 7)))))
    (reference r56 (scope relative) (span (offset 2996) (line 122) (column 28) (len 17)) (segments (segment 0 (token "VehicleController") (name "VehicleController") (separator none) (span (offset 2996) (line 122) (column 28) (len 17)))))
  )
  (root (package (name "5-State-based Behavior-2") (body brace (import (target (span (span (offset 53) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 65) (line 2) (column 29) (len 3))) (separator (span (offset 65) (line 2) (column 29) (len 2))) (marker (span (offset 67) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 86) (line 3) (column 17) (len 6))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 89) (line 3) (column 20) (len 3))) (separator (span (offset 89) (line 3) (column 20) (len 2))) (marker (span (offset 91) (line 3) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 110) (line 4) (column 17) (len 33))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 140) (line 4) (column 47) (len 3))) (separator (span (offset 140) (line 4) (column 47) (len 2))) (marker (span (offset 142) (line 4) (column 49) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "Definitions") (body brace (part-def (name "VehicleA") (modifiers) (body brace (perform (target (action (name "provide power") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (value none) (body semicolon)) (exhibit (declaration "vehicle states") (state none)))) (part-def (name "VehicleController") (modifiers) (body brace (exhibit (declaration "controller states") (state none)))) (state-def (name "Vehicle States") (modifiers) (body semicolon)) (state-def (name "Controller States") (modifiers) (body semicolon)) (action-def (name "Perform Self Test") (modifiers) (specializes none) (body semicolon)) (action-def (name "Apply Parking Brake") (modifiers) (specializes none) (body semicolon)) (action-def (name "Sense Temperature") (modifiers) (specializes none) (body brace (in-out (direction out) (kind none) (reference false) (declaration "temp") (subsets none) (type (ref r4)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 569) (line 21) (column 36) (len 27))))) (attribute-def (declaration-name "Vehicle Start Signal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "Vehicle On Signal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "Vehicle Off Signal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "Start Signal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "Off Signal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "Over Temp") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "Return to Normal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)))) (package (name "Usages") (body brace (import (target (span (span (offset 887) (line 34) (column 18) (len 14))) (all none) (ref r5) (shape (namespace (wildcard-suffix (span (span (offset 898) (line 34) (column 29) (len 3))) (separator (span (offset 898) (line 34) (column 29) (len 2))) (marker (span (offset 900) (line 34) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (action-usage (keyword action) (name "perform self test") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (action-usage (keyword action) (name "apply parking brake") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (action-usage (keyword action) (name "sense temperature") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (state-usage (name "vehicle states") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (state-usage (name "operational states") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (entry (action-keyword false) (target none) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)) (then (state (ref r10))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1186) (line 45) (column 7) (len 118)) (normalized "The following uses a shorthand for a transition whose source \nis the immediately preceding state.\n"))) (state-usage (name "off") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name none) (source none) (initial false) (accept (shorthand (expression (span (offset 1333) (line 50) (column 12) (len 22)) (ref r11)) (via none))) (guard (expression (span (offset 1365) (line 51) (column 9) (len 35)) (member-access (base (expression (span (offset 1365) (line 51) (column 9) (len 11)) (ref r12))) (separator dot) (member (ref r13))))) (effect (send (payload (expression (span (offset 1414) (line 52) (column 14) (len 20)) (constructor (type (ref r14)) (arguments)))) (type none) (via none) (to (expression (span (offset 1438) (line 52) (column 38) (len 29)) (member-access (base (expression (span (offset 1438) (line 52) (column 38) (len 11)) (ref r15))) (separator dot) (member (ref r16))))) (body none))) (target (expression (span (offset 1478) (line 53) (column 11) (len 8)) (ref r17))) (body semicolon)) (state-usage (name "starting") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name none) (source none) (initial false) (accept (shorthand (expression (span (offset 1525) (line 56) (column 12) (len 19)) (ref r18)) (via none))) (guard none) (effect none) (target (expression (span (offset 1555) (line 57) (column 11) (len 2)) (ref r19))) (body semicolon)) (state-usage (name "on") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (entry (action-keyword false) (target (ref r20)) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)) (do (action-keyword false) (target (ref r21)) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)) (exit (action-keyword false) (target (ref r22)) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)))) (transition (name none) (source none) (initial false) (accept (shorthand (expression (span (offset 1687) (line 64) (column 12) (len 20)) (ref r23)) (via none))) (guard none) (effect none) (target (expression (span (offset 1718) (line 65) (column 11) (len 3)) (ref r24))) (body semicolon)))) (state-usage (name "health states") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (entry (action-keyword false) (target none) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)) (then (state (ref r25))) (do (action-keyword false) (target (ref r26)) (declared-name none) (type none) (redefines none) (effect false) (body brace (inout-declaration))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1835) (line 72) (column 7) (len 93)) (normalized "The shorthand can be used for multiple transitions after\na single state.\n"))) (state-usage (name "normal") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name none) (source none) (initial false) (accept (time-trigger at (expression (span (offset 1963) (line 77) (column 15) (len 27)) (member-access (base (expression (span (offset 1963) (line 77) (column 15) (len 11)) (ref r27))) (separator dot) (member (ref r28)))))) (guard none) (effect none) (target (expression (span (offset 2001) (line 78) (column 11) (len 11)) (ref r29))) (body semicolon)) (transition (name none) (source none) (initial false) (accept (time-trigger when (expression (span (offset 2030) (line 79) (column 17) (len 43)) (binary (operator ">") (left (expression (span (offset 2030) (line 79) (column 17) (len 24)) (member-access (base (expression (span (offset 2030) (line 79) (column 17) (len 19)) (ref r30))) (separator dot) (member (ref r31))))) (right (expression (span (offset 2057) (line 79) (column 44) (len 16)) (member-access (base (expression (span (offset 2057) (line 79) (column 44) (len 11)) (ref r32))) (separator dot) (member (ref r33))))))))) (guard none) (effect (send (payload (expression (span (offset 2087) (line 80) (column 14) (len 17)) (constructor (type (ref r34)) (arguments)))) (type none) (via none) (to (expression (span (offset 2108) (line 80) (column 35) (len 29)) (member-access (base (expression (span (offset 2108) (line 80) (column 35) (len 11)) (ref r35))) (separator dot) (member (ref r36))))) (body none))) (target (expression (span (offset 2149) (line 81) (column 11) (len 8)) (ref r37))) (body semicolon)) (state-usage (name "maintenance") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name none) (source none) (initial false) (accept (shorthand (expression (span (offset 2198) (line 84) (column 12) (len 18)) (ref r38)) (via none))) (guard none) (effect none) (target (expression (span (offset 2227) (line 85) (column 11) (len 6)) (ref r39))) (body semicolon)) (state-usage (name "degraded") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name none) (source none) (initial false) (accept (shorthand (expression (span (offset 2271) (line 88) (column 12) (len 18)) (ref r40)) (via none))) (guard none) (effect none) (target (expression (span (offset 2300) (line 89) (column 11) (len 6)) (ref r41))) (body semicolon)))))) (state-usage (name "controller states") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r42)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (state-usage (name "operational controller states") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (entry (action-keyword false) (target none) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)) (then (state (ref r43))) (state-usage (name "off") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name none) (source none) (initial false) (accept (shorthand (expression (span (offset 2475) (line 98) (column 12) (len 14)) (ref r44)) (via none))) (guard none) (effect none) (target (expression (span (offset 2500) (line 99) (column 11) (len 2)) (ref r45))) (body semicolon)) (state-usage (name "on") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name none) (source none) (initial false) (accept (shorthand (expression (span (offset 2534) (line 102) (column 12) (len 12)) (ref r46)) (via none))) (guard none) (effect none) (target (expression (span (offset 2557) (line 103) (column 11) (len 3)) (ref r47))) (body semicolon)))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle1_c1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r48)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "fuelCmdPort") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (in-out-declaration))) (attribute-usage (declaration-name "brake pedal depressed") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r49)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "maintenanceTime") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r50)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "Tmax") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r51)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (perform (target (reference (action (ref r52)) (redefines (relationship (kind redefines) (implied false) (targets (ref r53)))))) (value none) (body brace (binding (direction in) (target (ref r54)) (value (expression (span (offset 2873) (line 117) (column 18) (len 19)) (ref r55)))))) (state-usage) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicleController") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r56)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (state-usage))))))))))
)
~~~
