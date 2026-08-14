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
  )
  (root (package (name "5-State-based Behavior-2") (body (import (target (span (span (offset 53) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 65) (line 2) (column 29) (len 3))) (separator (span (offset 65) (line 2) (column 29) (len 2))) (marker (span (offset 67) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 86) (line 3) (column 17) (len 6))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 89) (line 3) (column 20) (len 3))) (separator (span (offset 89) (line 3) (column 20) (len 2))) (marker (span (offset 91) (line 3) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 110) (line 4) (column 17) (len 33))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 140) (line 4) (column 47) (len 3))) (separator (span (offset 140) (line 4) (column 47) (len 2))) (marker (span (offset 142) (line 4) (column 49) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "Definitions") (body (part-def (name "VehicleA") (body (perform (declaration "provide power") (action none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (body semicolon)) (exhibit (declaration "vehicle states") (state none)))) (part-def (name "VehicleController") (body (exhibit (declaration "controller states") (state none)))) (state-def (name "Vehicle States") (body semicolon)) (state-def (name "Controller States") (body semicolon)) (action-def (name "Perform Self Test") (specializes none) (body semicolon)) (action-def (name "Apply Parking Brake") (specializes none) (body semicolon)) (action-def (name "Sense Temperature") (specializes none) (body (in-out (direction out) (reference false) (declaration "temp") (subsets none) (type (ref r4)) (multiplicity none) (ordered false) (nonunique false) (redefines none) (value none) (span (offset 569) (line 21) (column 36) (len 27))))) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def))) (package (name "Usages") (body (import (target (span (span (offset 887) (line 34) (column 18) (len 14))) (all none) (ref r5) (shape (namespace (wildcard-suffix (span (span (offset 898) (line 34) (column 29) (len 3))) (separator (span (offset 898) (line 34) (column 29) (len 2))) (marker (span (offset 900) (line 34) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (action-usage) (action-usage) (action-usage) (state-usage) (state-usage) (part-usage))))))
)
~~~
