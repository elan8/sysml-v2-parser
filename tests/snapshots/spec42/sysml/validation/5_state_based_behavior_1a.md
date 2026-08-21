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
        /*
		 * These actions are used enabled in the state usage 
		 * 'vehicle states', in addition to 'provide power'.
		 */
        action 'provide power' : 'Provide Power';
        action 'perform self test' : 'Perform Self Test';
        action 'apply parking brake' : 'Apply Parking Brake';
        action 'sense temperature' : 'Sense Temperature';
        state 'vehicle states' : 'Vehicle States' {
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
                transition 'off-starting' first off accept 'Vehicle Start Signal' if vehicle1_c1.'brake pedal depressed' do send new 'Start Signal'() to vehicle1_c1.vehicleController then starting {
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
                doc
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
    (reference r0 (scope relative) (span (offset 54) (line 2) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 54) (line 2) (column 17) (len 12)))))
    (reference r1 (scope relative) (span (offset 87) (line 3) (column 17) (len 3)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 87) (line 3) (column 17) (len 3)))))
    (reference r2 (scope relative) (span (offset 362) (line 12) (column 36) (len 15)) (segments (segment 0 (token "'Provide Power'") (name "Provide Power") (separator none) (span (offset 362) (line 12) (column 36) (len 15)))))
    (reference r3 (scope relative) (span (offset 905) (line 32) (column 46) (len 16)) (segments (segment 0 (token "TemperatureValue") (name "TemperatureValue") (separator none) (span (offset 905) (line 32) (column 46) (len 16)))))
    (reference r4 (scope relative) (span (offset 1241) (line 47) (column 18) (len 11)) (segments (segment 0 (token "Definitions") (name "Definitions") (separator none) (span (offset 1241) (line 47) (column 18) (len 11)))))
    (reference r5 (scope relative) (span (offset 1412) (line 54) (column 27) (len 15)) (segments (segment 0 (token "'Provide Power'") (name "Provide Power") (separator none) (span (offset 1412) (line 54) (column 27) (len 15)))))
    (reference r6 (scope relative) (span (offset 1459) (line 55) (column 31) (len 19)) (segments (segment 0 (token "'Perform Self Test'") (name "Perform Self Test") (separator none) (span (offset 1459) (line 55) (column 31) (len 19)))))
    (reference r7 (scope relative) (span (offset 1512) (line 56) (column 33) (len 21)) (segments (segment 0 (token "'Apply Parking Brake'") (name "Apply Parking Brake") (separator none) (span (offset 1512) (line 56) (column 33) (len 21)))))
    (reference r8 (scope relative) (span (offset 1565) (line 57) (column 31) (len 19)) (segments (segment 0 (token "'Sense Temperature'") (name "Sense Temperature") (separator none) (span (offset 1565) (line 57) (column 31) (len 19)))))
    (reference r9 (scope relative) (span (offset 1615) (line 59) (column 27) (len 16)) (segments (segment 0 (token "'Vehicle States'") (name "Vehicle States") (separator none) (span (offset 1615) (line 59) (column 27) (len 16)))))
    (reference r10 (scope relative) (span (offset 1922) (line 71) (column 18) (len 7)) (segments (segment 0 (token "initial") (name "initial") (separator none) (span (offset 1922) (line 71) (column 18) (len 7)))))
    (reference r11 (scope relative) (span (offset 2057) (line 78) (column 29) (len 3)) (segments (segment 0 (token "off") (name "off") (separator none) (span (offset 2057) (line 78) (column 29) (len 3)))))
    (reference r12 (scope relative) (span (offset 2131) (line 83) (column 12) (len 3)) (segments (segment 0 (token "off") (name "off") (separator none) (span (offset 2131) (line 83) (column 12) (len 3)))))
    (reference r13 (scope relative) (span (offset 2147) (line 84) (column 13) (len 22)) (segments (segment 0 (token "'Vehicle Start Signal'") (name "Vehicle Start Signal") (separator none) (span (offset 2147) (line 84) (column 13) (len 22)))))
    (reference r14 (scope relative) (span (offset 2179) (line 85) (column 9) (len 11)) (segments (segment 0 (token "vehicle1_c1") (name "vehicle1_c1") (separator none) (span (offset 2179) (line 85) (column 9) (len 11)))))
    (reference r15 (scope relative) (span (offset 2191) (line 85) (column 21) (len 23)) (segments (segment 0 (token "'brake pedal depressed'") (name "brake pedal depressed") (separator none) (span (offset 2191) (line 85) (column 21) (len 23)))))
    (reference r16 (scope relative) (span (offset 2232) (line 86) (column 18) (len 14)) (segments (segment 0 (token "'Start Signal'") (name "Start Signal") (separator none) (span (offset 2232) (line 86) (column 18) (len 14)))))
    (reference r17 (scope relative) (span (offset 2252) (line 86) (column 38) (len 11)) (segments (segment 0 (token "vehicle1_c1") (name "vehicle1_c1") (separator none) (span (offset 2252) (line 86) (column 38) (len 11)))))
    (reference r18 (scope relative) (span (offset 2264) (line 86) (column 50) (len 17)) (segments (segment 0 (token "vehicleController") (name "vehicleController") (separator none) (span (offset 2264) (line 86) (column 50) (len 17)))))
    (reference r19 (scope relative) (span (offset 2292) (line 87) (column 11) (len 8)) (segments (segment 0 (token "starting") (name "starting") (separator none) (span (offset 2292) (line 87) (column 11) (len 8)))))
    (reference r20 (scope relative) (span (offset 2837) (line 102) (column 12) (len 8)) (segments (segment 0 (token "starting") (name "starting") (separator none) (span (offset 2837) (line 102) (column 12) (len 8)))))
    (reference r21 (scope relative) (span (offset 2858) (line 103) (column 13) (len 19)) (segments (segment 0 (token "'Vehicle On Signal'") (name "Vehicle On Signal") (separator none) (span (offset 2858) (line 103) (column 13) (len 19)))))
    (reference r22 (scope relative) (span (offset 2888) (line 104) (column 11) (len 2)) (segments (segment 0 (token "on") (name "on") (separator none) (span (offset 2888) (line 104) (column 11) (len 2)))))
    (reference r23 (scope relative) (span (offset 3160) (line 113) (column 12) (len 19)) (segments (segment 0 (token "'perform self test'") (name "perform self test") (separator none) (span (offset 3160) (line 113) (column 12) (len 19)))))
    (reference r24 (scope relative) (span (offset 3189) (line 114) (column 9) (len 15)) (segments (segment 0 (token "'provide power'") (name "provide power") (separator none) (span (offset 3189) (line 114) (column 9) (len 15)))))
    (reference r25 (scope relative) (span (offset 3216) (line 115) (column 11) (len 21)) (segments (segment 0 (token "'apply parking brake'") (name "apply parking brake") (separator none) (span (offset 3216) (line 115) (column 11) (len 21)))))
    (reference r26 (scope relative) (span (offset 3285) (line 119) (column 12) (len 2)) (segments (segment 0 (token "on") (name "on") (separator none) (span (offset 3285) (line 119) (column 12) (len 2)))))
    (reference r27 (scope relative) (span (offset 3300) (line 120) (column 13) (len 20)) (segments (segment 0 (token "'Vehicle Off Signal'") (name "Vehicle Off Signal") (separator none) (span (offset 3300) (line 120) (column 13) (len 20)))))
    (reference r28 (scope relative) (span (offset 3331) (line 121) (column 11) (len 3)) (segments (segment 0 (token "off") (name "off") (separator none) (span (offset 3331) (line 121) (column 11) (len 3)))))
    (reference r29 (scope relative) (span (offset 3529) (line 130) (column 18) (len 7)) (segments (segment 0 (token "initial") (name "initial") (separator none) (span (offset 3529) (line 130) (column 18) (len 7)))))
    (reference r30 (scope relative) (span (offset 3545) (line 131) (column 8) (len 19)) (segments (segment 0 (token "'sense temperature'") (name "sense temperature") (separator none) (span (offset 3545) (line 131) (column 8) (len 19)))))
    (reference r31 (scope relative) (span (offset 3703) (line 137) (column 29) (len 6)) (segments (segment 0 (token "normal") (name "normal") (separator none) (span (offset 3703) (line 137) (column 29) (len 6)))))
    (reference r32 (scope relative) (span (offset 3786) (line 142) (column 12) (len 6)) (segments (segment 0 (token "normal") (name "normal") (separator none) (span (offset 3786) (line 142) (column 12) (len 6)))))
    (reference r33 (scope relative) (span (offset 3808) (line 143) (column 16) (len 11)) (segments (segment 0 (token "vehicle1_c1") (name "vehicle1_c1") (separator none) (span (offset 3808) (line 143) (column 16) (len 11)))))
    (reference r34 (scope relative) (span (offset 3820) (line 143) (column 28) (len 15)) (segments (segment 0 (token "maintenanceTime") (name "maintenanceTime") (separator none) (span (offset 3820) (line 143) (column 28) (len 15)))))
    (reference r35 (scope relative) (span (offset 3846) (line 144) (column 11) (len 11)) (segments (segment 0 (token "maintenance") (name "maintenance") (separator none) (span (offset 3846) (line 144) (column 11) (len 11)))))
    (reference r36 (scope relative) (span (offset 3908) (line 147) (column 12) (len 6)) (segments (segment 0 (token "normal") (name "normal") (separator none) (span (offset 3908) (line 147) (column 12) (len 6)))))
    (reference r37 (scope relative) (span (offset 3932) (line 148) (column 18) (len 19)) (segments (segment 0 (token "'sense temperature'") (name "sense temperature") (separator none) (span (offset 3932) (line 148) (column 18) (len 19)))))
    (reference r38 (scope relative) (span (offset 3952) (line 148) (column 38) (len 4)) (segments (segment 0 (token "temp") (name "temp") (separator none) (span (offset 3952) (line 148) (column 38) (len 4)))))
    (reference r39 (scope relative) (span (offset 3959) (line 148) (column 45) (len 11)) (segments (segment 0 (token "vehicle1_c1") (name "vehicle1_c1") (separator none) (span (offset 3959) (line 148) (column 45) (len 11)))))
    (reference r40 (scope relative) (span (offset 3971) (line 148) (column 57) (len 4)) (segments (segment 0 (token "Tmax") (name "Tmax") (separator none) (span (offset 3971) (line 148) (column 57) (len 4)))))
    (reference r41 (scope relative) (span (offset 3993) (line 149) (column 18) (len 11)) (segments (segment 0 (token "'Over Temp'") (name "Over Temp") (separator none) (span (offset 3993) (line 149) (column 18) (len 11)))))
    (reference r42 (scope relative) (span (offset 4010) (line 149) (column 35) (len 11)) (segments (segment 0 (token "vehicle1_c1") (name "vehicle1_c1") (separator none) (span (offset 4010) (line 149) (column 35) (len 11)))))
    (reference r43 (scope relative) (span (offset 4022) (line 149) (column 47) (len 17)) (segments (segment 0 (token "vehicleController") (name "vehicleController") (separator none) (span (offset 4022) (line 149) (column 47) (len 17)))))
    (reference r44 (scope relative) (span (offset 4051) (line 150) (column 11) (len 8)) (segments (segment 0 (token "degraded") (name "degraded") (separator none) (span (offset 4051) (line 150) (column 11) (len 8)))))
    (reference r45 (scope relative) (span (offset 4141) (line 155) (column 12) (len 11)) (segments (segment 0 (token "maintenance") (name "maintenance") (separator none) (span (offset 4141) (line 155) (column 12) (len 11)))))
    (reference r46 (scope relative) (span (offset 4165) (line 156) (column 13) (len 18)) (segments (segment 0 (token "'Return to Normal'") (name "Return to Normal") (separator none) (span (offset 4165) (line 156) (column 13) (len 18)))))
    (reference r47 (scope relative) (span (offset 4194) (line 157) (column 11) (len 6)) (segments (segment 0 (token "normal") (name "normal") (separator none) (span (offset 4194) (line 157) (column 11) (len 6)))))
    (reference r48 (scope relative) (span (offset 4276) (line 162) (column 12) (len 8)) (segments (segment 0 (token "degraded") (name "degraded") (separator none) (span (offset 4276) (line 162) (column 12) (len 8)))))
    (reference r49 (scope relative) (span (offset 4297) (line 163) (column 13) (len 18)) (segments (segment 0 (token "'Return to Normal'") (name "Return to Normal") (separator none) (span (offset 4297) (line 163) (column 13) (len 18)))))
    (reference r50 (scope relative) (span (offset 4326) (line 164) (column 11) (len 6)) (segments (segment 0 (token "normal") (name "normal") (separator none) (span (offset 4326) (line 164) (column 11) (len 6)))))
    (reference r51 (scope relative) (span (offset 4375) (line 168) (column 30) (len 19)) (segments (segment 0 (token "'Controller States'") (name "Controller States") (separator none) (span (offset 4375) (line 168) (column 30) (len 19)))))
    (reference r52 (scope relative) (span (offset 4466) (line 170) (column 18) (len 7)) (segments (segment 0 (token "initial") (name "initial") (separator none) (span (offset 4466) (line 170) (column 18) (len 7)))))
    (reference r53 (scope relative) (span (offset 4509) (line 172) (column 29) (len 3)) (segments (segment 0 (token "off") (name "off") (separator none) (span (offset 4509) (line 172) (column 29) (len 3)))))
    (reference r54 (scope relative) (span (offset 4574) (line 177) (column 12) (len 3)) (segments (segment 0 (token "off") (name "off") (separator none) (span (offset 4574) (line 177) (column 12) (len 3)))))
    (reference r55 (scope relative) (span (offset 4590) (line 178) (column 13) (len 14)) (segments (segment 0 (token "'Start Signal'") (name "Start Signal") (separator none) (span (offset 4590) (line 178) (column 13) (len 14)))))
    (reference r56 (scope relative) (span (offset 4615) (line 179) (column 11) (len 2)) (segments (segment 0 (token "on") (name "on") (separator none) (span (offset 4615) (line 179) (column 11) (len 2)))))
    (reference r57 (scope relative) (span (offset 4678) (line 184) (column 12) (len 2)) (segments (segment 0 (token "on") (name "on") (separator none) (span (offset 4678) (line 184) (column 12) (len 2)))))
    (reference r58 (scope relative) (span (offset 4693) (line 185) (column 13) (len 12)) (segments (segment 0 (token "'Off Signal'") (name "Off Signal") (separator none) (span (offset 4693) (line 185) (column 13) (len 12)))))
    (reference r59 (scope relative) (span (offset 4716) (line 186) (column 11) (len 3)) (segments (segment 0 (token "off") (name "off") (separator none) (span (offset 4716) (line 186) (column 11) (len 3)))))
    (reference r60 (scope relative) (span (offset 4753) (line 190) (column 21) (len 8)) (segments (segment 0 (token "VehicleA") (name "VehicleA") (separator none) (span (offset 4753) (line 190) (column 21) (len 8)))))
    (reference r61 (scope relative) (span (offset 4962) (line 199) (column 39) (len 7)) (segments (segment 0 (token "Boolean") (name "Boolean") (separator none) (span (offset 4962) (line 199) (column 39) (len 7)))))
    (reference r62 (scope relative) (span (offset 5003) (line 200) (column 31) (len 14)) (segments (segment 0 (token "Time") (name "Time") (separator none) (span (offset 5003) (line 200) (column 31) (len 4))) (segment 1 (token "DateTime") (name "DateTime") (separator colon-colon) (span (offset 5009) (line 200) (column 37) (len 8)))))
    (reference r63 (scope relative) (span (offset 5038) (line 201) (column 20) (len 16)) (segments (segment 0 (token "TemperatureValue") (name "TemperatureValue") (separator none) (span (offset 5038) (line 201) (column 20) (len 16)))))
    (reference r64 (scope relative) (span (offset 5071) (line 203) (column 12) (len 15)) (segments (segment 0 (token "'provide power'") (name "provide power") (separator none) (span (offset 5071) (line 203) (column 12) (len 15)))))
    (reference r65 (scope relative) (span (offset 5091) (line 203) (column 32) (len 25)) (segments (segment 0 (token "VehicleA") (name "VehicleA") (separator none) (span (offset 5091) (line 203) (column 32) (len 8))) (segment 1 (token "'provide power'") (name "provide power") (separator colon-colon) (span (offset 5101) (line 203) (column 42) (len 15)))))
    (reference r66 (scope relative) (span (offset 5306) (line 210) (column 8) (len 7)) (segments (segment 0 (token "fuelCmd") (name "fuelCmd") (separator none) (span (offset 5306) (line 210) (column 8) (len 7)))))
    (reference r67 (scope relative) (span (offset 5316) (line 210) (column 18) (len 19)) (segments (segment 0 (token "fuelCmdPort") (name "fuelCmdPort") (separator none) (span (offset 5316) (line 210) (column 18) (len 11))) (segment 1 (token "fuelCmd") (name "fuelCmd") (separator dot) (span (offset 5328) (line 210) (column 30) (len 7)))))
    (reference r68 (scope relative) (span (offset 5991) (line 232) (column 28) (len 17)) (segments (segment 0 (token "VehicleController") (name "VehicleController") (separator none) (span (offset 5991) (line 232) (column 28) (len 17)))))
  )
  (root (package (name "5-State-based Behavior-1a") (body brace (import (target (span (span (offset 54) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 66) (line 2) (column 29) (len 3))) (separator (span (offset 66) (line 2) (column 29) (len 2))) (marker (span (offset 68) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 87) (line 3) (column 17) (len 6))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 90) (line 3) (column 20) (len 3))) (separator (span (offset 90) (line 3) (column 20) (len 2))) (marker (span (offset 92) (line 3) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "Definitions") (body brace (part-def (name "VehicleA") (modifiers) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 147) (line 7) (column 6) (len 177)) (normalized "The following declare that 'VehicleA' performs a\n'provide power' action and exhibits some 'vehicle states',\nwithout giving details about these behaviors.\n"))) (perform (target (action (name "provide power") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (value none) (body semicolon)) (exhibit (declaration "vehicle states") (state none)))) (part-def (name "VehicleController") (modifiers) (body brace (exhibit (declaration "controller states") (state none)))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 538) (line 20) (column 5) (len 153)) (normalized "Black box specifications for state definitions may also have\ninput and output parameters, like activities, though none\nare used here.\n"))) (state-def (name "Vehicle States") (modifiers) (body semicolon)) (state-def (name "Controller States") (modifiers) (body semicolon)) (action-def (name "Provide Power") (modifiers) (specializes none) (body semicolon)) (action-def (name "Perform Self Test") (modifiers) (specializes none) (body semicolon)) (action-def (name "Apply Parking Brake") (modifiers) (specializes none) (body semicolon)) (action-def (name "Sense Temperature") (modifiers) (specializes none) (body brace (in-out (direction out) (reference false) (declaration "temp") (subsets none) (type (ref r3)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 895) (line 32) (column 36) (len 27))))) (attribute-def (declaration-name "FuelCmd") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "Vehicle Start Signal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "Vehicle On Signal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "Vehicle Off Signal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "Start Signal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "Off Signal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "Over Temp") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "Return to Normal") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)))) (package (name "Usages") (body brace (import (target (span (span (offset 1241) (line 47) (column 18) (len 14))) (all none) (ref r4) (shape (namespace (wildcard-suffix (span (span (offset 1252) (line 47) (column 29) (len 3))) (separator (span (offset 1252) (line 47) (column 29) (len 2))) (marker (span (offset 1254) (line 47) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1264) (line 49) (column 5) (len 115)) (normalized "These actions are used enabled in the state usage \n'vehicle states', in addition to 'provide power'.\n"))) (action-usage (name "provide power") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (action-usage (name "perform self test") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (action-usage (name "apply parking brake") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (action-usage (name "sense temperature") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (state-usage (name "vehicle states") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1648) (line 60) (column 6) (len 139)) (normalized "This is a usage of the state definition 'Vehicle States'.\nNote that it depends specifically on on the part 'vehicle1_c1'.\n"))) (state-usage (name "operational states") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (doc (name none) (locale none) (body (span (offset 1838) (line 67) (column 6) (len 60)) (normalized "The state definition for this usage is implicit.\n"))) (entry (action-keyword true) (target (ref r10)) (declared-name none) (type none) (redefines none) (effect false) (body brace (doc (name none) (locale none) (body (span (offset 1946) (line 73) (column 7) (len 69)) (normalized "This empty entry action acts like a start pseudo state.\n"))))) (transition (name "initial") (source none) (initial false) (accept none) (guard none) (effect none) (target (expression (span (offset 2057) (line 78) (column 29) (len 3)) (ref r11))) (body semicolon)) (state-usage (name "off") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name "off-starting") (source (expression (span (offset 2131) (line 83) (column 12) (len 3)) (ref r12))) (initial false) (accept (shorthand (expression (span (offset 2147) (line 84) (column 13) (len 22)) (ref r13)) (via none))) (guard (expression (span (offset 2179) (line 85) (column 9) (len 35)) (member-access (base (expression (span (offset 2179) (line 85) (column 9) (len 11)) (ref r14))) (separator dot) (member (ref r15))))) (effect (send (payload (expression (span (offset 2228) (line 86) (column 14) (len 20)) (constructor (type (ref r16)) (arguments)))) (type none) (via none) (to (expression (span (offset 2252) (line 86) (column 38) (len 29)) (member-access (base (expression (span (offset 2252) (line 86) (column 38) (len 11)) (ref r17))) (separator dot) (member (ref r18))))) (body none))) (target (expression (span (offset 2292) (line 87) (column 11) (len 8)) (ref r19))) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 2310) (line 88) (column 8) (len 441)) (normalized "The transition definition for a transition usage is always implicit.\n\"accept\" marks the trigger, \"if\" the guard and \"do\" the effect.\n\nThe notation \"'new Start Signal'()\" constructs a specific instance of the\n'Start Signal' attribute def to be sent to the 'vehicleController'. If the\nattribute def had properties, their values would be given as arguments\ninside the parentheses.\n"))))) (state-usage (name "starting") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name "starting-on") (source (expression (span (offset 2837) (line 102) (column 12) (len 8)) (ref r20))) (initial false) (accept (shorthand (expression (span (offset 2858) (line 103) (column 13) (len 19)) (ref r21)) (via none))) (guard none) (effect none) (target (expression (span (offset 2888) (line 104) (column 11) (len 2)) (ref r22))) (body semicolon)) (state-usage (name "on") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 2919) (line 107) (column 8) (len 222)) (normalized "A state may have a \"entry\" action that is performed on entry into\nthe state, a \"do\" action that is performed while in the state\nand an \"exit\" action that is performed on exit from the state.\n"))) (entry (action-keyword false) (target (ref r23)) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)) (do (action-keyword false) (target (ref r24)) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)) (exit (action-keyword false) (target (ref r25)) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)))) (transition (name "on-off") (source (expression (span (offset 3285) (line 119) (column 12) (len 2)) (ref r26))) (initial false) (accept (shorthand (expression (span (offset 3300) (line 120) (column 13) (len 20)) (ref r27)) (via none))) (guard none) (effect none) (target (expression (span (offset 3331) (line 121) (column 11) (len 3)) (ref r28))) (body semicolon)))) (state-usage (name "health states") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 3378) (line 125) (column 7) (len 127)) (normalized "'health states' is concurrent with 'operational states', because the\ncontaining state usage is \"parallel\".\n"))) (entry (action-keyword true) (target (ref r29)) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)) (do (action-keyword false) (target (ref r30)) (declared-name none) (type none) (redefines none) (effect false) (body brace (inout-declaration) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 3585) (line 132) (column 8) (len 76)) (normalized "State-behavior actions may have input and output parameters.\n"))))) (transition (name "initial") (source none) (initial false) (accept none) (guard none) (effect none) (target (expression (span (offset 3703) (line 137) (column 29) (len 6)) (ref r31))) (body semicolon)) (state-usage (name "normal") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name "normal-maintenance") (source (expression (span (offset 3786) (line 142) (column 12) (len 6)) (ref r32))) (initial false) (accept (time-trigger at (expression (span (offset 3808) (line 143) (column 16) (len 27)) (member-access (base (expression (span (offset 3808) (line 143) (column 16) (len 11)) (ref r33))) (separator dot) (member (ref r34)))))) (guard none) (effect none) (target (expression (span (offset 3846) (line 144) (column 11) (len 11)) (ref r35))) (body semicolon)) (transition (name "normal-degraded") (source (expression (span (offset 3908) (line 147) (column 12) (len 6)) (ref r36))) (initial false) (accept (time-trigger when (expression (span (offset 3932) (line 148) (column 18) (len 43)) (binary (operator ">") (left (expression (span (offset 3932) (line 148) (column 18) (len 24)) (member-access (base (expression (span (offset 3932) (line 148) (column 18) (len 19)) (ref r37))) (separator dot) (member (ref r38))))) (right (expression (span (offset 3959) (line 148) (column 45) (len 16)) (member-access (base (expression (span (offset 3959) (line 148) (column 45) (len 11)) (ref r39))) (separator dot) (member (ref r40))))))))) (guard none) (effect (send (payload (expression (span (offset 3989) (line 149) (column 14) (len 17)) (constructor (type (ref r41)) (arguments)))) (type none) (via none) (to (expression (span (offset 4010) (line 149) (column 35) (len 29)) (member-access (base (expression (span (offset 4010) (line 149) (column 35) (len 11)) (ref r42))) (separator dot) (member (ref r43))))) (body none))) (target (expression (span (offset 4051) (line 150) (column 11) (len 8)) (ref r44))) (body semicolon)) (state-usage (name "maintenance") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name "maintenance-normal") (source (expression (span (offset 4141) (line 155) (column 12) (len 11)) (ref r45))) (initial false) (accept (shorthand (expression (span (offset 4165) (line 156) (column 13) (len 18)) (ref r46)) (via none))) (guard none) (effect none) (target (expression (span (offset 4194) (line 157) (column 11) (len 6)) (ref r47))) (body semicolon)) (state-usage (name "degraded") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name "degraded-normal") (source (expression (span (offset 4276) (line 162) (column 12) (len 8)) (ref r48))) (initial false) (accept (shorthand (expression (span (offset 4297) (line 163) (column 13) (len 18)) (ref r49)) (via none))) (guard none) (effect none) (target (expression (span (offset 4326) (line 164) (column 11) (len 6)) (ref r50))) (body semicolon)))))) (state-usage (name "controller states") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r51)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (state-usage (name "operational controller states") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (entry (action-keyword true) (target (ref r52)) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)) (transition (name "initial") (source none) (initial false) (accept none) (guard none) (effect none) (target (expression (span (offset 4509) (line 172) (column 29) (len 3)) (ref r53))) (body semicolon)) (state-usage (name "off") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name "off-on") (source (expression (span (offset 4574) (line 177) (column 12) (len 3)) (ref r54))) (initial false) (accept (shorthand (expression (span (offset 4590) (line 178) (column 13) (len 14)) (ref r55)) (via none))) (guard none) (effect none) (target (expression (span (offset 4615) (line 179) (column 11) (len 2)) (ref r56))) (body semicolon)) (state-usage (name "on") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name "on-off") (source (expression (span (offset 4678) (line 184) (column 12) (len 2)) (ref r57))) (initial false) (accept (shorthand (expression (span (offset 4693) (line 185) (column 13) (len 12)) (ref r58)) (via none))) (guard none) (effect none) (target (expression (span (offset 4716) (line 186) (column 11) (len 3)) (ref r59))) (body semicolon)))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle1_c1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r60)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "fuelCmdPort") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (in-out-declaration))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 4825) (line 195) (column 6) (len 96)) (normalized "These attribute properties are used in the specification for\n'vehicle states'.\n"))) (attribute-usage (declaration-name "brake pedal depressed") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r61)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "maintenanceTime") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r62)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "Tmax") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r63)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (perform (target (reference (action (ref r64)) (redefines (relationship (kind redefines) (implied false) (targets (ref r65)))))) (value none) (body brace (doc (name none) (locale none) (body (span (offset 5131) (line 205) (column 6) (len 161)) (normalized "In the context of the 'vehicle1_c1' part, the 'provide power' action\nthat is enabled in 'vehicle states' gets its input from the 'fuelCmdPort'.\n"))) (binding (direction in) (target (ref r66)) (value (expression (span (offset 5316) (line 210) (column 18) (len 19)) (ref r67)))))) (state-usage) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicleController") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r68)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (state-usage))))))))))
)
~~~
