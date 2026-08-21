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
    (reference r6 (scope relative) (span (offset 1408) (line 52) (column 31) (len 19)) (segments (segment 0 (token "'Perform Self Test'") (name "Perform Self Test") (separator none) (span (offset 1408) (line 52) (column 31) (len 19)))))
    (reference r7 (scope relative) (span (offset 1461) (line 53) (column 33) (len 21)) (segments (segment 0 (token "'Apply Parking Brake'") (name "Apply Parking Brake") (separator none) (span (offset 1461) (line 53) (column 33) (len 21)))))
    (reference r8 (scope relative) (span (offset 1514) (line 54) (column 31) (len 19)) (segments (segment 0 (token "'Sense Temperature'") (name "Sense Temperature") (separator none) (span (offset 1514) (line 54) (column 31) (len 19)))))
    (reference r9 (scope relative) (span (offset 1564) (line 56) (column 27) (len 16)) (segments (segment 0 (token "'Vehicle States'") (name "Vehicle States") (separator none) (span (offset 1564) (line 56) (column 27) (len 16)))))
    (reference r10 (scope relative) (span (offset 1759) (line 62) (column 18) (len 8)) (segments (segment 0 (token "VehicleA") (name "VehicleA") (separator none) (span (offset 1759) (line 62) (column 18) (len 8)))))
    (reference r11 (scope relative) (span (offset 1898) (line 70) (column 18) (len 7)) (segments (segment 0 (token "initial") (name "initial") (separator none) (span (offset 1898) (line 70) (column 18) (len 7)))))
    (reference r12 (scope relative) (span (offset 2033) (line 77) (column 29) (len 3)) (segments (segment 0 (token "off") (name "off") (separator none) (span (offset 2033) (line 77) (column 29) (len 3)))))
    (reference r13 (scope relative) (span (offset 2107) (line 82) (column 12) (len 3)) (segments (segment 0 (token "off") (name "off") (separator none) (span (offset 2107) (line 82) (column 12) (len 3)))))
    (reference r14 (scope relative) (span (offset 2123) (line 83) (column 13) (len 22)) (segments (segment 0 (token "'Vehicle Start Signal'") (name "Vehicle Start Signal") (separator none) (span (offset 2123) (line 83) (column 13) (len 22)))))
    (reference r15 (scope relative) (span (offset 2155) (line 84) (column 9) (len 11)) (segments (segment 0 (token "vehicle1_c1") (name "vehicle1_c1") (separator none) (span (offset 2155) (line 84) (column 9) (len 11)))))
    (reference r16 (scope relative) (span (offset 2167) (line 84) (column 21) (len 23)) (segments (segment 0 (token "'brake pedal depressed'") (name "brake pedal depressed") (separator none) (span (offset 2167) (line 84) (column 21) (len 23)))))
    (reference r17 (scope relative) (span (offset 2208) (line 85) (column 18) (len 14)) (segments (segment 0 (token "'Start Signal'") (name "Start Signal") (separator none) (span (offset 2208) (line 85) (column 18) (len 14)))))
    (reference r18 (scope relative) (span (offset 2228) (line 85) (column 38) (len 11)) (segments (segment 0 (token "vehicle1_c1") (name "vehicle1_c1") (separator none) (span (offset 2228) (line 85) (column 38) (len 11)))))
    (reference r19 (scope relative) (span (offset 2240) (line 85) (column 50) (len 17)) (segments (segment 0 (token "vehicleController") (name "vehicleController") (separator none) (span (offset 2240) (line 85) (column 50) (len 17)))))
    (reference r20 (scope relative) (span (offset 2268) (line 86) (column 11) (len 8)) (segments (segment 0 (token "starting") (name "starting") (separator none) (span (offset 2268) (line 86) (column 11) (len 8)))))
    (reference r21 (scope relative) (span (offset 2814) (line 101) (column 12) (len 8)) (segments (segment 0 (token "starting") (name "starting") (separator none) (span (offset 2814) (line 101) (column 12) (len 8)))))
    (reference r22 (scope relative) (span (offset 2835) (line 102) (column 13) (len 19)) (segments (segment 0 (token "'Vehicle On Signal'") (name "Vehicle On Signal") (separator none) (span (offset 2835) (line 102) (column 13) (len 19)))))
    (reference r23 (scope relative) (span (offset 2865) (line 103) (column 11) (len 2)) (segments (segment 0 (token "on") (name "on") (separator none) (span (offset 2865) (line 103) (column 11) (len 2)))))
    (reference r24 (scope relative) (span (offset 3137) (line 112) (column 12) (len 19)) (segments (segment 0 (token "'perform self test'") (name "perform self test") (separator none) (span (offset 3137) (line 112) (column 12) (len 19)))))
    (reference r25 (scope relative) (span (offset 3166) (line 113) (column 9) (len 15)) (segments (segment 0 (token "'provide power'") (name "provide power") (separator none) (span (offset 3166) (line 113) (column 9) (len 15)))))
    (reference r26 (scope relative) (span (offset 3193) (line 114) (column 11) (len 21)) (segments (segment 0 (token "'apply parking brake'") (name "apply parking brake") (separator none) (span (offset 3193) (line 114) (column 11) (len 21)))))
    (reference r27 (scope relative) (span (offset 3262) (line 118) (column 12) (len 2)) (segments (segment 0 (token "on") (name "on") (separator none) (span (offset 3262) (line 118) (column 12) (len 2)))))
    (reference r28 (scope relative) (span (offset 3277) (line 119) (column 13) (len 20)) (segments (segment 0 (token "'Vehicle Off Signal'") (name "Vehicle Off Signal") (separator none) (span (offset 3277) (line 119) (column 13) (len 20)))))
    (reference r29 (scope relative) (span (offset 3308) (line 120) (column 11) (len 3)) (segments (segment 0 (token "off") (name "off") (separator none) (span (offset 3308) (line 120) (column 11) (len 3)))))
    (reference r30 (scope relative) (span (offset 3506) (line 129) (column 18) (len 7)) (segments (segment 0 (token "initial") (name "initial") (separator none) (span (offset 3506) (line 129) (column 18) (len 7)))))
    (reference r31 (scope relative) (span (offset 3522) (line 130) (column 8) (len 19)) (segments (segment 0 (token "'sense temperature'") (name "sense temperature") (separator none) (span (offset 3522) (line 130) (column 8) (len 19)))))
    (reference r32 (scope relative) (span (offset 3681) (line 136) (column 29) (len 6)) (segments (segment 0 (token "normal") (name "normal") (separator none) (span (offset 3681) (line 136) (column 29) (len 6)))))
    (reference r33 (scope relative) (span (offset 3764) (line 141) (column 12) (len 6)) (segments (segment 0 (token "normal") (name "normal") (separator none) (span (offset 3764) (line 141) (column 12) (len 6)))))
    (reference r34 (scope relative) (span (offset 3786) (line 142) (column 16) (len 11)) (segments (segment 0 (token "vehicle1_c1") (name "vehicle1_c1") (separator none) (span (offset 3786) (line 142) (column 16) (len 11)))))
    (reference r35 (scope relative) (span (offset 3798) (line 142) (column 28) (len 15)) (segments (segment 0 (token "maintenanceTime") (name "maintenanceTime") (separator none) (span (offset 3798) (line 142) (column 28) (len 15)))))
    (reference r36 (scope relative) (span (offset 3824) (line 143) (column 11) (len 11)) (segments (segment 0 (token "maintenance") (name "maintenance") (separator none) (span (offset 3824) (line 143) (column 11) (len 11)))))
    (reference r37 (scope relative) (span (offset 3886) (line 146) (column 12) (len 6)) (segments (segment 0 (token "normal") (name "normal") (separator none) (span (offset 3886) (line 146) (column 12) (len 6)))))
    (reference r38 (scope relative) (span (offset 3910) (line 147) (column 18) (len 19)) (segments (segment 0 (token "'sense temperature'") (name "sense temperature") (separator none) (span (offset 3910) (line 147) (column 18) (len 19)))))
    (reference r39 (scope relative) (span (offset 3930) (line 147) (column 38) (len 4)) (segments (segment 0 (token "temp") (name "temp") (separator none) (span (offset 3930) (line 147) (column 38) (len 4)))))
    (reference r40 (scope relative) (span (offset 3937) (line 147) (column 45) (len 11)) (segments (segment 0 (token "vehicle1_c1") (name "vehicle1_c1") (separator none) (span (offset 3937) (line 147) (column 45) (len 11)))))
    (reference r41 (scope relative) (span (offset 3949) (line 147) (column 57) (len 4)) (segments (segment 0 (token "Tmax") (name "Tmax") (separator none) (span (offset 3949) (line 147) (column 57) (len 4)))))
    (reference r42 (scope relative) (span (offset 3971) (line 148) (column 18) (len 11)) (segments (segment 0 (token "'Over Temp'") (name "Over Temp") (separator none) (span (offset 3971) (line 148) (column 18) (len 11)))))
    (reference r43 (scope relative) (span (offset 3988) (line 148) (column 35) (len 11)) (segments (segment 0 (token "vehicle1_c1") (name "vehicle1_c1") (separator none) (span (offset 3988) (line 148) (column 35) (len 11)))))
    (reference r44 (scope relative) (span (offset 4000) (line 148) (column 47) (len 17)) (segments (segment 0 (token "vehicleController") (name "vehicleController") (separator none) (span (offset 4000) (line 148) (column 47) (len 17)))))
    (reference r45 (scope relative) (span (offset 4029) (line 149) (column 11) (len 8)) (segments (segment 0 (token "degraded") (name "degraded") (separator none) (span (offset 4029) (line 149) (column 11) (len 8)))))
    (reference r46 (scope relative) (span (offset 4119) (line 154) (column 12) (len 11)) (segments (segment 0 (token "maintenance") (name "maintenance") (separator none) (span (offset 4119) (line 154) (column 12) (len 11)))))
    (reference r47 (scope relative) (span (offset 4143) (line 155) (column 13) (len 18)) (segments (segment 0 (token "'Return to Normal'") (name "Return to Normal") (separator none) (span (offset 4143) (line 155) (column 13) (len 18)))))
    (reference r48 (scope relative) (span (offset 4172) (line 156) (column 11) (len 6)) (segments (segment 0 (token "normal") (name "normal") (separator none) (span (offset 4172) (line 156) (column 11) (len 6)))))
    (reference r49 (scope relative) (span (offset 4254) (line 161) (column 12) (len 8)) (segments (segment 0 (token "degraded") (name "degraded") (separator none) (span (offset 4254) (line 161) (column 12) (len 8)))))
    (reference r50 (scope relative) (span (offset 4275) (line 162) (column 13) (len 18)) (segments (segment 0 (token "'Return to Normal'") (name "Return to Normal") (separator none) (span (offset 4275) (line 162) (column 13) (len 18)))))
    (reference r51 (scope relative) (span (offset 4304) (line 163) (column 11) (len 6)) (segments (segment 0 (token "normal") (name "normal") (separator none) (span (offset 4304) (line 163) (column 11) (len 6)))))
    (reference r52 (scope relative) (span (offset 4353) (line 167) (column 30) (len 19)) (segments (segment 0 (token "'Controller States'") (name "Controller States") (separator none) (span (offset 4353) (line 167) (column 30) (len 19)))))
    (reference r53 (scope relative) (span (offset 4444) (line 169) (column 18) (len 7)) (segments (segment 0 (token "initial") (name "initial") (separator none) (span (offset 4444) (line 169) (column 18) (len 7)))))
    (reference r54 (scope relative) (span (offset 4487) (line 171) (column 29) (len 3)) (segments (segment 0 (token "off") (name "off") (separator none) (span (offset 4487) (line 171) (column 29) (len 3)))))
    (reference r55 (scope relative) (span (offset 4552) (line 176) (column 12) (len 3)) (segments (segment 0 (token "off") (name "off") (separator none) (span (offset 4552) (line 176) (column 12) (len 3)))))
    (reference r56 (scope relative) (span (offset 4568) (line 177) (column 13) (len 14)) (segments (segment 0 (token "'Start Signal'") (name "Start Signal") (separator none) (span (offset 4568) (line 177) (column 13) (len 14)))))
    (reference r57 (scope relative) (span (offset 4593) (line 178) (column 11) (len 2)) (segments (segment 0 (token "on") (name "on") (separator none) (span (offset 4593) (line 178) (column 11) (len 2)))))
    (reference r58 (scope relative) (span (offset 4656) (line 183) (column 12) (len 2)) (segments (segment 0 (token "on") (name "on") (separator none) (span (offset 4656) (line 183) (column 12) (len 2)))))
    (reference r59 (scope relative) (span (offset 4671) (line 184) (column 13) (len 12)) (segments (segment 0 (token "'Off Signal'") (name "Off Signal") (separator none) (span (offset 4671) (line 184) (column 13) (len 12)))))
    (reference r60 (scope relative) (span (offset 4694) (line 185) (column 11) (len 3)) (segments (segment 0 (token "off") (name "off") (separator none) (span (offset 4694) (line 185) (column 11) (len 3)))))
    (reference r61 (scope relative) (span (offset 4731) (line 189) (column 21) (len 8)) (segments (segment 0 (token "VehicleA") (name "VehicleA") (separator none) (span (offset 4731) (line 189) (column 21) (len 8)))))
    (reference r62 (scope relative) (span (offset 4940) (line 198) (column 39) (len 7)) (segments (segment 0 (token "Boolean") (name "Boolean") (separator none) (span (offset 4940) (line 198) (column 39) (len 7)))))
    (reference r63 (scope relative) (span (offset 4981) (line 199) (column 31) (len 14)) (segments (segment 0 (token "Time") (name "Time") (separator none) (span (offset 4981) (line 199) (column 31) (len 4))) (segment 1 (token "DateTime") (name "DateTime") (separator colon-colon) (span (offset 4987) (line 199) (column 37) (len 8)))))
    (reference r64 (scope relative) (span (offset 5016) (line 200) (column 20) (len 16)) (segments (segment 0 (token "TemperatureValue") (name "TemperatureValue") (separator none) (span (offset 5016) (line 200) (column 20) (len 16)))))
    (reference r65 (scope relative) (span (offset 5049) (line 202) (column 12) (len 15)) (segments (segment 0 (token "'provide power'") (name "provide power") (separator none) (span (offset 5049) (line 202) (column 12) (len 15)))))
    (reference r66 (scope relative) (span (offset 5069) (line 202) (column 32) (len 25)) (segments (segment 0 (token "VehicleA") (name "VehicleA") (separator none) (span (offset 5069) (line 202) (column 32) (len 8))) (segment 1 (token "'provide power'") (name "provide power") (separator colon-colon) (span (offset 5079) (line 202) (column 42) (len 15)))))
    (reference r67 (scope relative) (span (offset 5281) (line 208) (column 8) (len 7)) (segments (segment 0 (token "fuelCmd") (name "fuelCmd") (separator none) (span (offset 5281) (line 208) (column 8) (len 7)))))
    (reference r68 (scope relative) (span (offset 5291) (line 208) (column 18) (len 19)) (segments (segment 0 (token "fuelCmdPort") (name "fuelCmdPort") (separator none) (span (offset 5291) (line 208) (column 18) (len 11))) (segment 1 (token "fuelCmd") (name "fuelCmd") (separator dot) (span (offset 5303) (line 208) (column 30) (len 7)))))
    (reference r69 (scope relative) (span (offset 5966) (line 230) (column 28) (len 17)) (segments (segment 0 (token "VehicleController") (name "VehicleController") (separator none) (span (offset 5966) (line 230) (column 28) (len 17)))))
  )
  (root (package (name "5-State-based Behavior-1") (body brace (import (target (span (span (offset 53) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 65) (line 2) (column 29) (len 3))) (separator (span (offset 65) (line 2) (column 29) (len 2))) (marker (span (offset 67) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 86) (line 3) (column 17) (len 6))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 89) (line 3) (column 20) (len 3))) (separator (span (offset 89) (line 3) (column 20) (len 2))) (marker (span (offset 91) (line 3) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 110) (line 4) (column 17) (len 33))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 140) (line 4) (column 47) (len 3))) (separator (span (offset 140) (line 4) (column 47) (len 2))) (marker (span (offset 142) (line 4) (column 49) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "Definitions") (body brace (part-def (name "VehicleA") (modifiers) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 197) (line 8) (column 6) (len 177)) (normalized "The following declare that 'VehicleA' performs a\n'provide power' action and exhibits some 'vehicle states',\nwithout giving details about these behaviors.\n"))) (perform (declaration "provide power") (action none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (body semicolon)) (exhibit (declaration "vehicle states") (state none)))) (part-def (name "VehicleController") (modifiers) (body brace (exhibit (declaration "controller states") (state none)))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 588) (line 21) (column 5) (len 153)) (normalized "Black box specifications for state definitions may also have\ninput and output parameters, like activities, though none\nare used here.\n"))) (state-def (name "Vehicle States") (modifiers) (body semicolon)) (state-def (name "Controller States") (modifiers) (body semicolon)) (action-def (name "Perform Self Test") (modifiers) (specializes none) (body semicolon)) (action-def (name "Apply Parking Brake") (modifiers) (specializes none) (body semicolon)) (action-def (name "Sense Temperature") (modifiers) (specializes none) (body brace (in-out (direction out) (reference false) (declaration "temp") (subsets none) (type (ref r4)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 915) (line 32) (column 36) (len 27))))) (attribute-def (declaration-name "Vehicle Start Signal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "Vehicle On Signal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "Vehicle Off Signal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "Start Signal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "Off Signal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "Over Temp") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "Return to Normal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)))) (package (name "Usages") (body brace (import (target (span (span (offset 1233) (line 45) (column 18) (len 14))) (all none) (ref r5) (shape (namespace (wildcard-suffix (span (span (offset 1244) (line 45) (column 29) (len 3))) (separator (span (offset 1244) (line 45) (column 29) (len 2))) (marker (span (offset 1246) (line 45) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1256) (line 47) (column 5) (len 115)) (normalized "These actions are used enabled in the state usage \n'vehicle states', in addition to 'provide power'.\n"))) (action-usage (name "perform self test") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (action-usage (name "apply parking brake") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (action-usage (name "sense temperature") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (state-usage (name "vehicle states") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1597) (line 57) (column 6) (len 139)) (normalized "This is a usage of the state definition 'Vehicle States'.\nNote that it depends specifically on on the part 'vehicle1_c1'.\n"))) (ref (name "vehicle") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)) (state-usage (name "operational states") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (doc (name none) (locale none) (body (span (offset 1814) (line 66) (column 6) (len 60)) (normalized "The state definition for this usage is implicit.\n"))) (entry (action-keyword true) (target (ref r11)) (declared-name none) (type none) (redefines none) (effect false) (body brace (doc (name none) (locale none) (body (span (offset 1922) (line 72) (column 7) (len 69)) (normalized "This empty entry action acts like a start pseudo state.\n"))))) (transition (name "initial") (source none) (initial false) (accept none) (guard none) (effect none) (target (expression (span (offset 2033) (line 77) (column 29) (len 3)) (ref r12))) (body semicolon)) (state-usage (name "off") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name "off-starting") (source (expression (span (offset 2107) (line 82) (column 12) (len 3)) (ref r13))) (initial false) (accept (shorthand (expression (span (offset 2123) (line 83) (column 13) (len 22)) (ref r14)) (via none))) (guard (expression (span (offset 2155) (line 84) (column 9) (len 35)) (member-access (base (expression (span (offset 2155) (line 84) (column 9) (len 11)) (ref r15))) (separator dot) (member (ref r16))))) (effect (send (payload (expression (span (offset 2204) (line 85) (column 14) (len 20)) (constructor (type (ref r17)) (arguments)))) (type none) (via none) (to (expression (span (offset 2228) (line 85) (column 38) (len 29)) (member-access (base (expression (span (offset 2228) (line 85) (column 38) (len 11)) (ref r18))) (separator dot) (member (ref r19))))) (body none))) (target (expression (span (offset 2268) (line 86) (column 11) (len 8)) (ref r20))) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 2286) (line 87) (column 8) (len 441)) (normalized "The transition definition for a transition usage is always implicit.\n\"accept\" marks the trigger, \"if\" the guard and \"do\" the effect.\n\nThe notation \"new 'Start Signal'()\" constructs a specific instance of the\n'Start Signal' attribute def to be sent to the 'vehicleController'. If the\nattribute def had properties, their values would be given as arguments\ninside the parentheses.\n"))))) (state-usage (name "starting") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name "starting-on") (source (expression (span (offset 2814) (line 101) (column 12) (len 8)) (ref r21))) (initial false) (accept (shorthand (expression (span (offset 2835) (line 102) (column 13) (len 19)) (ref r22)) (via none))) (guard none) (effect none) (target (expression (span (offset 2865) (line 103) (column 11) (len 2)) (ref r23))) (body semicolon)) (state-usage (name "on") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 2896) (line 106) (column 8) (len 222)) (normalized "A state may have a \"entry\" action that is performed on entry into\nthe state, a \"do\" action that is performed while in the state\nand an \"exit\" action that is performed on exit from the state.\n"))) (entry (action-keyword false) (target (ref r24)) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)) (do (action-keyword false) (target (ref r25)) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)) (exit (action-keyword false) (target (ref r26)) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)))) (transition (name "on-off") (source (expression (span (offset 3262) (line 118) (column 12) (len 2)) (ref r27))) (initial false) (accept (shorthand (expression (span (offset 3277) (line 119) (column 13) (len 20)) (ref r28)) (via none))) (guard none) (effect none) (target (expression (span (offset 3308) (line 120) (column 11) (len 3)) (ref r29))) (body semicolon)))) (state-usage (name "health states") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 3355) (line 124) (column 7) (len 127)) (normalized "'health states' is concurrent with 'operational states', because the\ncontaining state usage is \"parallel\".\n"))) (entry (action-keyword true) (target (ref r30)) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)) (do (action-keyword false) (target (ref r31)) (declared-name none) (type none) (redefines none) (effect false) (body brace (inout-declaration) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 3562) (line 131) (column 8) (len 76)) (normalized "State-behavior actions may have input and output parameters.\n"))))) (transition (name "initial") (source none) (initial false) (accept none) (guard none) (effect none) (target (expression (span (offset 3681) (line 136) (column 29) (len 6)) (ref r32))) (body semicolon)) (state-usage (name "normal") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name "normal-maintenance") (source (expression (span (offset 3764) (line 141) (column 12) (len 6)) (ref r33))) (initial false) (accept (time-trigger at (expression (span (offset 3786) (line 142) (column 16) (len 27)) (member-access (base (expression (span (offset 3786) (line 142) (column 16) (len 11)) (ref r34))) (separator dot) (member (ref r35)))))) (guard none) (effect none) (target (expression (span (offset 3824) (line 143) (column 11) (len 11)) (ref r36))) (body semicolon)) (transition (name "normal-degraded") (source (expression (span (offset 3886) (line 146) (column 12) (len 6)) (ref r37))) (initial false) (accept (time-trigger when (expression (span (offset 3910) (line 147) (column 18) (len 43)) (binary (operator ">") (left (expression (span (offset 3910) (line 147) (column 18) (len 24)) (member-access (base (expression (span (offset 3910) (line 147) (column 18) (len 19)) (ref r38))) (separator dot) (member (ref r39))))) (right (expression (span (offset 3937) (line 147) (column 45) (len 16)) (member-access (base (expression (span (offset 3937) (line 147) (column 45) (len 11)) (ref r40))) (separator dot) (member (ref r41))))))))) (guard none) (effect (send (payload (expression (span (offset 3967) (line 148) (column 14) (len 17)) (constructor (type (ref r42)) (arguments)))) (type none) (via none) (to (expression (span (offset 3988) (line 148) (column 35) (len 29)) (member-access (base (expression (span (offset 3988) (line 148) (column 35) (len 11)) (ref r43))) (separator dot) (member (ref r44))))) (body none))) (target (expression (span (offset 4029) (line 149) (column 11) (len 8)) (ref r45))) (body semicolon)) (state-usage (name "maintenance") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name "maintenance-normal") (source (expression (span (offset 4119) (line 154) (column 12) (len 11)) (ref r46))) (initial false) (accept (shorthand (expression (span (offset 4143) (line 155) (column 13) (len 18)) (ref r47)) (via none))) (guard none) (effect none) (target (expression (span (offset 4172) (line 156) (column 11) (len 6)) (ref r48))) (body semicolon)) (state-usage (name "degraded") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name "degraded-normal") (source (expression (span (offset 4254) (line 161) (column 12) (len 8)) (ref r49))) (initial false) (accept (shorthand (expression (span (offset 4275) (line 162) (column 13) (len 18)) (ref r50)) (via none))) (guard none) (effect none) (target (expression (span (offset 4304) (line 163) (column 11) (len 6)) (ref r51))) (body semicolon)))))) (state-usage (name "controller states") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r52)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (state-usage (name "operational controller states") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (entry (action-keyword true) (target (ref r53)) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)) (transition (name "initial") (source none) (initial false) (accept none) (guard none) (effect none) (target (expression (span (offset 4487) (line 171) (column 29) (len 3)) (ref r54))) (body semicolon)) (state-usage (name "off") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name "off-on") (source (expression (span (offset 4552) (line 176) (column 12) (len 3)) (ref r55))) (initial false) (accept (shorthand (expression (span (offset 4568) (line 177) (column 13) (len 14)) (ref r56)) (via none))) (guard none) (effect none) (target (expression (span (offset 4593) (line 178) (column 11) (len 2)) (ref r57))) (body semicolon)) (state-usage (name "on") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name "on-off") (source (expression (span (offset 4656) (line 183) (column 12) (len 2)) (ref r58))) (initial false) (accept (shorthand (expression (span (offset 4671) (line 184) (column 13) (len 12)) (ref r59)) (via none))) (guard none) (effect none) (target (expression (span (offset 4694) (line 185) (column 11) (len 3)) (ref r60))) (body semicolon)))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle1_c1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r61)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "fuelCmdPort") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (in-out-declaration))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 4803) (line 194) (column 6) (len 96)) (normalized "These attribute properties are used in the specification for\n'vehicle states'.\n"))) (attribute-usage (declaration-name "brake pedal depressed") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r62)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "maintenanceTime") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r63)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "Tmax") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r64)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (perform (declaration "") (action (ref r65)) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r66)))) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 5103) (line 203) (column 7) (len 164)) (normalized "In the context of the 'vehicle1_c1' part, the 'provide power' action\nthat is enabled in 'vehicle states' gets its input from the 'fuelCmdPort'.\n"))) (binding (direction in) (target (ref r67)) (value (expression (span (offset 5291) (line 208) (column 18) (len 19)) (ref r68)))))) (state-usage) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicleController") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r69)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (state-usage))))))))))
)
~~~
