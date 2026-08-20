# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (05-State-based Behavior): 5-State-based Behavior-1"))
~~~
# SOURCE
~~~sysml
package '5-State-based Behavior-1' {
	private import ScalarValues::*;
	private import ISQ::*;
	private import '3a-Function-based Behavior-1'::*;
	
	package Definitions {
		part def VehicleA {
			/*
			 * The following declare that 'VehicleA' performs a
			 * 'provide power' action and exhibits some 'vehicle states',
			 * without giving details about these behaviors.
			 */
			perform action 'provide power': 'Provide Power';
			exhibit state 'vehicle states': 'Vehicle States';
		}
		
		part def VehicleController {
			exhibit state 'controller states': 'Controller States';
		}

		/*
		 * Black box specifications for state definitions may also have
		 * input and output parameters, like activities, though none
		 * are used here.
		 */

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
		
		/*
		 * These actions are used enabled in the state usage 
		 * 'vehicle states', in addition to 'provide power'.
		 */
		 
		action 'perform self test': 'Perform Self Test';
		action 'apply parking brake': 'Apply Parking Brake';
		action 'sense temperature': 'Sense Temperature';
		
		state 'vehicle states': 'Vehicle States' parallel {
			/*
			 * This is a usage of the state definition 'Vehicle States'.
			 * Note that it depends specifically on on the part 'vehicle1_c1'.
			 */
		
			ref vehicle : VehicleA;

			state 'operational states' {
			doc
			/*
			 * The state definition for this usage is implicit.
			 */
			
				entry action initial {
				doc
				/*
				 * This empty entry action acts like a start pseudo state.
				 */
				}
				
				transition initial then off;
			    
				state off;
				
				transition 'off-starting'
					first off
					accept 'Vehicle Start Signal' 
					if vehicle1_c1.'brake pedal depressed'
					do send new 'Start Signal'() to vehicle1_c1.vehicleController
					then starting {
					/*
					 * The transition definition for a transition usage is always implicit.
					 * "accept" marks the trigger, "if" the guard and "do" the effect.
					 * 
					 * The notation "new 'Start Signal'()" constructs a specific instance of the
					 * 'Start Signal' attribute def to be sent to the 'vehicleController'. If the
					 * attribute def had properties, their values would be given as arguments
					 * inside the parentheses.
					 */						
					}
					
				state starting;
				
				transition 'starting-on'
					first starting
					accept 'Vehicle On Signal'
					then on;
				
				state on {
					/*
					 * A state may have a "entry" action that is performed on entry into
					 * the state, a "do" action that is performed while in the state
					 * and an "exit" action that is performed on exit from the state.
					 */
				
					entry 'perform self test';
					do 'provide power';
					exit 'apply parking brake';
				}
				
				transition 'on-off'
					first on
					accept 'Vehicle Off Signal'
					then off;
			}
			
			state 'health states' {
				/*
				 * 'health states' is concurrent with 'operational states', because the
				 * containing state usage is "parallel".
				 */
			
				entry action initial;
				do 'sense temperature' { out temp; 
					/*
					 * State-behavior actions may have input and output parameters.
					 */
				 }
				
				transition initial then normal;
				
				state normal;
				
				transition 'normal-maintenance'
					first normal
					accept at vehicle1_c1.maintenanceTime
					then maintenance;
				
				transition 'normal-degraded'
					first normal
					accept when 'sense temperature'.temp > vehicle1_c1.Tmax
					do send new 'Over Temp'() to vehicle1_c1.vehicleController 
					then degraded;
				
				state maintenance;
				
				transition 'maintenance-normal'
					first maintenance
					accept 'Return to Normal'
					then normal;
				
				state degraded;
				
				transition 'degraded-normal'
					first degraded
					accept 'Return to Normal'
					then normal;
			}
		}
		
		state 'controller states': 'Controller States' parallel {
			state 'operational controller states' {
				entry action initial; 
				
				transition initial then off;
				
				state off;
				
				transition 'off-on'
					first off
					accept 'Start Signal'
					then on;
				
				state on;
				
				transition 'on-off'
					first on
					accept 'Off Signal'
					then off;
			}
		}		

		part vehicle1_c1: VehicleA {
			port fuelCmdPort {
				in fuelCmd: FuelCmd;
			}
			
			/*
			 * These attribute properties are used in the specification for
			 * 'vehicle states'.
			 */
			attribute 'brake pedal depressed': Boolean;		
			attribute maintenanceTime: Time::DateTime;
			attribute Tmax: TemperatureValue;
			
			perform 'provide power' :>> VehicleA::'provide power' {
				/*
				 * In the context of the 'vehicle1_c1' part, the 'provide power' action
				 * that is enabled in 'vehicle states' gets its input from the 'fuelCmdPort'.
				 */
			
				in fuelCmd = fuelCmdPort.fuelCmd;
			}
			
			exhibit 'vehicle states' :>> VehicleA::'vehicle states' {
				/*
				 * This allocates the state usage 'vehicle states' as the detailed
				 * state-based behavior for 'vehicle1_c1' that fills in the generic
				 * declaration in 'VehicleA'.
				 */
			}
				
			//*
			// The above is semantically equivalent to:
			
			ref state 'vehicle states' :> Usages::'vehicle states', exhibitedStates
				:>> VehicleA::'vehicle states';		
				
			// For a composite state performance within the vehicle, replace the above with:
			
			state 'vehicle states' :>> Usages::'vehicle states', VehicleA::'vehicle states';
			*/

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
  (document "5_state_based_behavior_1.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '5-State-based Behavior-1' {
    private import ScalarValues::*;
    private import ISQ::*;
    private import '3a-Function-based Behavior-1'::*;
    package Definitions {
        part def VehicleA {
            /*
			 * The following declare that 'VehicleA' performs a
			 * 'provide power' action and exhibits some 'vehicle states',
			 * without giving details about these behaviors.
			 */
            perform action 'provide power' : 'Provide Power';
            exhibit state 'vehicle states' : 'Vehicle States';
        }
        part def VehicleController {
            exhibit state 'controller states' : 'Controller States';
        }
        /*
		 * Black box specifications for state definitions may also have
		 * input and output parameters, like activities, though none
		 * are used here.
		 */
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
        /*
		 * These actions are used enabled in the state usage 
		 * 'vehicle states', in addition to 'provide power'.
		 */
        action 'perform self test' : 'Perform Self Test';
        action 'apply parking brake' : 'Apply Parking Brake';
        action 'sense temperature' : 'Sense Temperature';
        state 'vehicle states' : 'Vehicle States' {
            /*
			 * This is a usage of the state definition 'Vehicle States'.
			 * Note that it depends specifically on on the part 'vehicle1_c1'.
			 */
            ref vehicle : VehicleA;
            state 'operational states' {
                doc
                /*
			 * The state definition for this usage is implicit.
			 */
                entry action initial {
                    doc
                    /*
				 * This empty entry action acts like a start pseudo state.
				 */
                }
                transition initial then off;
                state off;
                transition 'off-starting' first off accept 'Vehicle Start Signal' if vehicle1_c1.'brake pedal depressed' do send new 'Start Signal'() to vehicle1_c1.vehicleController then starting {
                    /*
					 * The transition definition for a transition usage is always implicit.
					 * "accept" marks the trigger, "if" the guard and "do" the effect.
					 * 
					 * The notation "new 'Start Signal'()" constructs a specific instance of the
					 * 'Start Signal' attribute def to be sent to the 'vehicleController'. If the
					 * attribute def had properties, their values would be given as arguments
					 * inside the parentheses.
					 */
                }
                state starting;
                transition 'starting-on' first starting accept 'Vehicle On Signal' then on;
                state on {
                    /*
					 * A state may have a "entry" action that is performed on entry into
					 * the state, a "do" action that is performed while in the state
					 * and an "exit" action that is performed on exit from the state.
					 */
                    entry 'perform self test';
                    do 'provide power';
                    exit 'apply parking brake';
                }
                transition 'on-off' first on accept 'Vehicle Off Signal' then off;
            }
            state 'health states' {
                /*
				 * 'health states' is concurrent with 'operational states', because the
				 * containing state usage is "parallel".
				 */
                entry action initial;
                do 'sense temperature' {
                    out temp;
                    /*
					 * State-behavior actions may have input and output parameters.
					 */
                }
                transition initial then normal;
                state normal;
                transition 'normal-maintenance' first normal accept at vehicle1_c1.maintenanceTime then maintenance;
                transition 'normal-degraded' first normal accept when 'sense temperature'.temp > vehicle1_c1.Tmax do send new 'Over Temp'() to vehicle1_c1.vehicleController then degraded;
                state maintenance;
                transition 'maintenance-normal' first maintenance accept 'Return to Normal' then normal;
                state degraded;
                transition 'degraded-normal' first degraded accept 'Return to Normal' then normal;
            }
        }
        state 'controller states' : 'Controller States' {
            state 'operational controller states' {
                entry action initial;
                transition initial then off;
                state off;
                transition 'off-on' first off accept 'Start Signal' then on;
                state on;
                transition 'on-off' first on accept 'Off Signal' then off;
            }
        }
        part vehicle1_c1 : VehicleA {
            port fuelCmdPort {
                in fuelCmd : FuelCmd;
            }
            /*
			 * These attribute properties are used in the specification for
			 * 'vehicle states'.
			 */
            attribute 'brake pedal depressed' : Boolean;
            attribute maintenanceTime : Time::DateTime;
            attribute Tmax : TemperatureValue;
            perform 'provide power' :>> VehicleA::'provide power' {
                /*
				 * In the context of the 'vehicle1_c1' part, the 'provide power' action
				 * that is enabled in 'vehicle states' gets its input from the 'fuelCmdPort'.
				 */
                in fuelCmd = fuelCmdPort.fuelCmd;
            }
            state 'vehicle states' :>> VehicleA::'vehicle states' {
                /*
				 * This allocates the state usage 'vehicle states' as the detailed
				 * state-based behavior for 'vehicle1_c1' that fills in the generic
				 * declaration in 'VehicleA'.
				 */
            }
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
    (reference r3 (scope relative) (span (offset 412) (line 13) (column 36) (len 15)) (segments (segment 0 (token "'Provide Power'") (name "Provide Power") (separator none) (span (offset 412) (line 13) (column 36) (len 15)))))
    (reference r4 (scope relative) (span (offset 925) (line 32) (column 46) (len 16)) (segments (segment 0 (token "TemperatureValue") (name "TemperatureValue") (separator none) (span (offset 925) (line 32) (column 46) (len 16)))))
    (reference r5 (scope relative) (span (offset 1233) (line 45) (column 18) (len 11)) (segments (segment 0 (token "Definitions") (name "Definitions") (separator none) (span (offset 1233) (line 45) (column 18) (len 11)))))
    (reference r6 (scope relative) (span (offset 4731) (line 189) (column 21) (len 8)) (segments (segment 0 (token "VehicleA") (name "VehicleA") (separator none) (span (offset 4731) (line 189) (column 21) (len 8)))))
    (reference r7 (scope relative) (span (offset 4940) (line 198) (column 39) (len 7)) (segments (segment 0 (token "Boolean") (name "Boolean") (separator none) (span (offset 4940) (line 198) (column 39) (len 7)))))
    (reference r8 (scope relative) (span (offset 4981) (line 199) (column 31) (len 14)) (segments (segment 0 (token "Time") (name "Time") (separator none) (span (offset 4981) (line 199) (column 31) (len 4))) (segment 1 (token "DateTime") (name "DateTime") (separator colon-colon) (span (offset 4987) (line 199) (column 37) (len 8)))))
    (reference r9 (scope relative) (span (offset 5016) (line 200) (column 20) (len 16)) (segments (segment 0 (token "TemperatureValue") (name "TemperatureValue") (separator none) (span (offset 5016) (line 200) (column 20) (len 16)))))
    (reference r10 (scope relative) (span (offset 5049) (line 202) (column 12) (len 15)) (segments (segment 0 (token "'provide power'") (name "provide power") (separator none) (span (offset 5049) (line 202) (column 12) (len 15)))))
    (reference r11 (scope relative) (span (offset 5069) (line 202) (column 32) (len 25)) (segments (segment 0 (token "VehicleA") (name "VehicleA") (separator none) (span (offset 5069) (line 202) (column 32) (len 8))) (segment 1 (token "'provide power'") (name "provide power") (separator colon-colon) (span (offset 5079) (line 202) (column 42) (len 15)))))
    (reference r12 (scope relative) (span (offset 5281) (line 208) (column 8) (len 7)) (segments (segment 0 (token "fuelCmd") (name "fuelCmd") (separator none) (span (offset 5281) (line 208) (column 8) (len 7)))))
    (reference r13 (scope relative) (span (offset 5291) (line 208) (column 18) (len 19)) (segments (segment 0 (token "fuelCmdPort") (name "fuelCmdPort") (separator none) (span (offset 5291) (line 208) (column 18) (len 11))) (segment 1 (token "fuelCmd") (name "fuelCmd") (separator dot) (span (offset 5303) (line 208) (column 30) (len 7)))))
    (reference r14 (scope relative) (span (offset 5966) (line 230) (column 28) (len 17)) (segments (segment 0 (token "VehicleController") (name "VehicleController") (separator none) (span (offset 5966) (line 230) (column 28) (len 17)))))
  )
  (root (package (name "5-State-based Behavior-1") (body brace (import (target (span (span (offset 53) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 65) (line 2) (column 29) (len 3))) (separator (span (offset 65) (line 2) (column 29) (len 2))) (marker (span (offset 67) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 86) (line 3) (column 17) (len 6))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 89) (line 3) (column 20) (len 3))) (separator (span (offset 89) (line 3) (column 20) (len 2))) (marker (span (offset 91) (line 3) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 110) (line 4) (column 17) (len 33))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 140) (line 4) (column 47) (len 3))) (separator (span (offset 140) (line 4) (column 47) (len 2))) (marker (span (offset 142) (line 4) (column 49) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "Definitions") (body brace (part-def (name "VehicleA") (modifiers) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 197) (line 8) (column 6) (len 177)) (normalized "The following declare that 'VehicleA' performs a\n'provide power' action and exhibits some 'vehicle states',\nwithout giving details about these behaviors.\n"))) (perform (declaration "provide power") (action none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (body semicolon)) (exhibit (declaration "vehicle states") (state none)))) (part-def (name "VehicleController") (modifiers) (body brace (exhibit (declaration "controller states") (state none)))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 588) (line 21) (column 5) (len 153)) (normalized "Black box specifications for state definitions may also have\ninput and output parameters, like activities, though none\nare used here.\n"))) (state-def (name "Vehicle States") (modifiers) (body semicolon)) (state-def (name "Controller States") (modifiers) (body semicolon)) (action-def (name "Perform Self Test") (modifiers) (specializes none) (body semicolon)) (action-def (name "Apply Parking Brake") (modifiers) (specializes none) (body semicolon)) (action-def (name "Sense Temperature") (modifiers) (specializes none) (body brace (in-out (direction out) (reference false) (declaration "temp") (subsets none) (type (ref r4)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 915) (line 32) (column 36) (len 27))))) (attribute-def (declaration-name "Vehicle Start Signal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "Vehicle On Signal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "Vehicle Off Signal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "Start Signal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "Off Signal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "Over Temp") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "Return to Normal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)))) (package (name "Usages") (body brace (import (target (span (span (offset 1233) (line 45) (column 18) (len 14))) (all none) (ref r5) (shape (namespace (wildcard-suffix (span (span (offset 1244) (line 45) (column 29) (len 3))) (separator (span (offset 1244) (line 45) (column 29) (len 2))) (marker (span (offset 1246) (line 45) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1256) (line 47) (column 5) (len 115)) (normalized "These actions are used enabled in the state usage \n'vehicle states', in addition to 'provide power'.\n"))) (action-usage (name "perform self test") (short-name none) (body semicolon)) (action-usage (name "apply parking brake") (short-name none) (body semicolon)) (action-usage (name "sense temperature") (short-name none) (body semicolon)) (state-usage) (state-usage) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle1_c1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "fuelCmdPort") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (in-out-declaration))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 4803) (line 194) (column 6) (len 96)) (normalized "These attribute properties are used in the specification for\n'vehicle states'.\n"))) (attribute-usage (declaration-name "brake pedal depressed") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "maintenanceTime") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "Tmax") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (perform (declaration "") (action (ref r10)) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r11)))) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 5103) (line 203) (column 7) (len 164)) (normalized "In the context of the 'vehicle1_c1' part, the 'provide power' action\nthat is enabled in 'vehicle states' gets its input from the 'fuelCmdPort'.\n"))) (binding (direction in) (target (ref r12)) (value (expression (span (offset 5291) (line 208) (column 18) (len 19)) (ref r13)))))) (state-usage) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicleController") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r14)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (state-usage))))))))))
)
~~~
