# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (05-State-based Behavior): 5-State-based Behavior-1a"))
~~~
# SOURCE
~~~sysml
package '5-State-based Behavior-1a' {
	private import ScalarValues::*;
	private import ISQ::*;
	
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

		action def 'Provide Power';
		action def 'Perform Self Test';
		action def 'Apply Parking Brake';
		action def 'Sense Temperature' { out temp: TemperatureValue; }
		
		attribute def FuelCmd;
		
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
		 
		action 'provide power': 'Provide Power';
		action 'perform self test': 'Perform Self Test';
		action 'apply parking brake': 'Apply Parking Brake';
		action 'sense temperature': 'Sense Temperature';
		
		state 'vehicle states': 'Vehicle States' parallel {
			/*
			 * This is a usage of the state definition 'Vehicle States'.
			 * Note that it depends specifically on on the part 'vehicle1_c1'.
			 */		 

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
					 * The notation "'new Start Signal'()" constructs a specific instance of the
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
			doc
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
  (document "5_state_based_behavior_1a.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '5-State-based Behavior-1a' {
    private import ScalarValues::*;
    private import ISQ::*;
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
        action def 'Provide Power';
        action def 'Perform Self Test';
        action def 'Apply Parking Brake';
        action def 'Sense Temperature' {
            out temp : TemperatureValue;
        }
        attribute def FuelCmd;
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
        action 'provide power' : 'Provide Power';
        action 'perform self test' : 'Perform Self Test';
        action 'apply parking brake' : 'Apply Parking Brake';
        action 'sense temperature' : 'Sense Temperature';
        state 'vehicle states' : 'Vehicle States' {
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
                transition 'off-starting' first off accept 'Vehicle Start Signal' if vehicle1_c1.'brake pedal depressed' do send new 'Start Signal'() to vehicle1_c1.vehicleController then starting {}
                state starting;
                transition 'starting-on' first starting accept 'Vehicle On Signal' then on;
                state on {
                    entry 'perform self test';
                    do 'provide power';
                    exit 'apply parking brake';
                }
                transition 'on-off' first on accept 'Vehicle Off Signal' then off;
            }
            state 'health states' {
                entry action initial;
                do 'sense temperature' {
                    out temp;
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
            attribute 'brake pedal depressed' : Boolean;
            attribute maintenanceTime : Time::DateTime;
            attribute Tmax : TemperatureValue;
            perform 'provide power' :>> VehicleA::'provide power' {
                doc
                /*
			 * In the context of the 'vehicle1_c1' part, the 'provide power' action
			 * that is enabled in 'vehicle states' gets its input from the 'fuelCmdPort'.
			 */
                in fuelCmd = fuelCmdPort.fuelCmd;
            }
            state  :>> VehicleA::'vehicle states' {
            }
            part vehicleController : VehicleController {
                state  :>> VehicleController::'controller states';
            }
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 54) (line 2) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 54) (line 2) (column 17) (len 12)))))
    (reference r1 (scope relative) (span (offset 87) (line 3) (column 17) (len 3)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 87) (line 3) (column 17) (len 3)))))
    (reference r2 (scope relative) (span (offset 362) (line 12) (column 36) (len 15)) (segments (segment 0 (token "'Provide Power'") (name "Provide Power") (separator none) (span (offset 362) (line 12) (column 36) (len 15)))))
    (reference r3 (scope relative) (span (offset 1241) (line 47) (column 18) (len 11)) (segments (segment 0 (token "Definitions") (name "Definitions") (separator none) (span (offset 1241) (line 47) (column 18) (len 11)))))
  )
  (root (package (name "5-State-based Behavior-1a") (body (import (target (span (span (offset 54) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 66) (line 2) (column 29) (len 3))) (separator (span (offset 66) (line 2) (column 29) (len 2))) (marker (span (offset 68) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 87) (line 3) (column 17) (len 6))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 90) (line 3) (column 20) (len 3))) (separator (span (offset 90) (line 3) (column 20) (len 2))) (marker (span (offset 92) (line 3) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "Definitions") (body (part-def (name "VehicleA") (body (perform (declaration "provide power") (action none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (body semicolon)) (exhibit (declaration "vehicle states") (state none)))) (part-def (name "VehicleController") (body (exhibit (declaration "controller states") (state none)))) (state-def (name "Vehicle States") (body semicolon)) (state-def (name "Controller States") (body semicolon)) (action-def) (action-def) (action-def) (action-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def))) (package (name "Usages") (body (import (target (span (span (offset 1241) (line 47) (column 18) (len 14))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 1252) (line 47) (column 29) (len 3))) (separator (span (offset 1252) (line 47) (column 29) (len 2))) (marker (span (offset 1254) (line 47) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (action-usage) (action-usage) (action-usage) (action-usage) (state-usage) (state-usage) (part-usage))))))
)
~~~
