# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 27 (Occurrences): Interaction Realization-1"))
~~~
# SOURCE
~~~sysml
package 'Interaction Realization-1' {
	private import 'Interaction Example-1'::*;
	
	part driver_a : Driver {
		action driverBehavior {
			action sendSetSpeed send new SetSpeed() to vehicle_a;
		}
	}
	
	part vehicle_a : Vehicle {
		part cruiseController_a : CruiseController {
			action controllerBehavior {
				action receiveSetSpeed accept SetSpeed via vehicle_a;
				then action receiveSensedSpeed accept SensedSpeed via cruiseController_a;
				then action sendFuelCommand send new FuelCommand() to engine_a;
			}
		}
		
		part speedometer_a : Speedometer {
			action speedometerBehavior {
				action sendSensedSpeed send new SensedSpeed() to cruiseController_a;
			}
		}
		
		part engine_a : Engine {
			action engineBehavior {
				action receiveFuelCommand accept FuelCommand via engine_a;
			}
		}
	}
	
	occurrence cruiseControlInteraction_a : CruiseControlInteraction {
		part :>> driver :>> driver_a {
			event driverBehavior.sendSetSpeed[1] :>> setSpeedSent;
		}
		
		part :>> vehicle :>> vehicle_a {
			part :>> cruiseController :>> cruiseController_a {
				event controllerBehavior.receiveSetSpeed[1] :>> setSpeedReceived;
				event controllerBehavior.receiveSensedSpeed[1] :>> sensedSpeedReceived;
				event controllerBehavior.sendFuelCommand[1] :>> fuelCommandSent;
			}
			part :>> speedometer :>> speedometer_a {
				event speedometerBehavior.sendSensedSpeed[1] :>> sensedSpeedSent;
			}
			part :>> engine :>> engine_a {
				event engineBehavior.receiveFuelCommand[1] :>> fuelCommandReceived;
			}
		}
		
		message :>> setSpeedMessage = driver_a.driverBehavior.sendSetSpeed.sentMessage;
		message :>> sensedSpeedMessage = vehicle_a.speedometer_a.speedometerBehavior.sendSensedSpeed.sentMessage;
		message :>> fuelCommandMessage = vehicle_a.cruiseController_a.controllerBehavior.sendFuelCommand.sentMessage;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "27_interaction_realization_1.md"
    (diagnostics
      (diagnostic (code "recovered_action_body_element") (severity error) (category parseerror) (span (offset 335) (line 13) (column 28) (len 35)) (message "unexpected token in action body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 335) (line 13) (column 28) (len 35)) (message "suppressed 5 cascading recovered diagnostics after earlier recovery errors"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Interaction Realization-1' {
    private import 'Interaction Example-1'::*;
    part driver_a : Driver {
        action driverBehavior {
            action sendSetSpeed send new SetSpeed() to vehicle_a;
        }
    }
    part vehicle_a : Vehicle {
        part cruiseController_a : CruiseController {
            action controllerBehavior {
                action receiveSetSpeed
                accept SetSpeed via vehicle_a;
                then action receiveSensedSpeed
                accept SensedSpeed via cruiseController_a;
                then action sendFuelCommand send new FuelCommand() to engine_a;
            }
        }
        part speedometer_a : Speedometer {
            action speedometerBehavior {
                action sendSensedSpeed send new SensedSpeed() to cruiseController_a;
            }
        }
        part engine_a : Engine {
            action engineBehavior {
                action receiveFuelCommand
                accept FuelCommand via engine_a;
            }
        }
    }
    occurrence cruiseControlInteraction_a : CruiseControlInteraction {
        part driver :>> driver_a {
            event driverBehavior.sendSetSpeed[1] :>> setSpeedSent;
        }
        part vehicle :>> vehicle_a {
            part cruiseController :>> cruiseController_a {
                event controllerBehavior.receiveSetSpeed[1] :>> setSpeedReceived;
                event controllerBehavior.receiveSensedSpeed[1] :>> sensedSpeedReceived;
                event controllerBehavior.sendFuelCommand[1] :>> fuelCommandSent;
            }
            part speedometer :>> speedometer_a {
                event speedometerBehavior.sendSensedSpeed[1] :>> sensedSpeedSent;
            }
            part engine :>> engine_a {
                event engineBehavior.receiveFuelCommand[1] :>> fuelCommandReceived;
            }
        }
        message :>> setSpeedMessage = driver_a.driverBehavior.sendSetSpeed.sentMessage;
        message :>> sensedSpeedMessage = vehicle_a.speedometer_a.speedometerBehavior.sendSensedSpeed.sentMessage;
        message :>> fuelCommandMessage = vehicle_a.cruiseController_a.controllerBehavior.sendFuelCommand.sentMessage;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 54) (line 2) (column 17) (len 23)) (segments (segment 0 (token "'Interaction Example-1'") (name "Interaction Example-1") (separator none) (span (offset 54) (line 2) (column 17) (len 23)))))
    (reference r1 (scope relative) (span (offset 101) (line 4) (column 18) (len 6)) (segments (segment 0 (token "Driver") (name "Driver") (separator none) (span (offset 101) (line 4) (column 18) (len 6)))))
    (reference r2 (scope relative) (span (offset 220) (line 10) (column 19) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 220) (line 10) (column 19) (len 7)))))
    (reference r3 (scope relative) (span (offset 258) (line 11) (column 29) (len 16)) (segments (segment 0 (token "CruiseController") (name "CruiseController") (separator none) (span (offset 258) (line 11) (column 29) (len 16)))))
    (reference r4 (scope relative) (span (offset 547) (line 19) (column 24) (len 11)) (segments (segment 0 (token "Speedometer") (name "Speedometer") (separator none) (span (offset 547) (line 19) (column 24) (len 11)))))
    (reference r5 (scope relative) (span (offset 696) (line 25) (column 19) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 696) (line 25) (column 19) (len 6)))))
    (reference r6 (scope relative) (span (offset 899) (line 33) (column 23) (len 8)) (segments (segment 0 (token "driver_a") (name "driver_a") (separator none) (span (offset 899) (line 33) (column 23) (len 8)))))
    (reference r7 (scope relative) (span (offset 919) (line 34) (column 10) (len 27)) (segments (segment 0 (token "driverBehavior") (name "driverBehavior") (separator none) (span (offset 919) (line 34) (column 10) (len 14))) (segment 1 (token "sendSetSpeed") (name "sendSetSpeed") (separator dot) (span (offset 934) (line 34) (column 25) (len 12)))))
    (reference r8 (scope relative) (span (offset 998) (line 37) (column 24) (len 9)) (segments (segment 0 (token "vehicle_a") (name "vehicle_a") (separator none) (span (offset 998) (line 37) (column 24) (len 9)))))
    (reference r9 (scope relative) (span (offset 1043) (line 38) (column 34) (len 18)) (segments (segment 0 (token "cruiseController_a") (name "cruiseController_a") (separator none) (span (offset 1043) (line 38) (column 34) (len 18)))))
    (reference r10 (scope relative) (span (offset 1074) (line 39) (column 11) (len 34)) (segments (segment 0 (token "controllerBehavior") (name "controllerBehavior") (separator none) (span (offset 1074) (line 39) (column 11) (len 18))) (segment 1 (token "receiveSetSpeed") (name "receiveSetSpeed") (separator dot) (span (offset 1093) (line 39) (column 30) (len 15)))))
    (reference r11 (scope relative) (span (offset 1144) (line 40) (column 11) (len 37)) (segments (segment 0 (token "controllerBehavior") (name "controllerBehavior") (separator none) (span (offset 1144) (line 40) (column 11) (len 18))) (segment 1 (token "receiveSensedSpeed") (name "receiveSensedSpeed") (separator dot) (span (offset 1163) (line 40) (column 30) (len 18)))))
    (reference r12 (scope relative) (span (offset 1220) (line 41) (column 11) (len 34)) (segments (segment 0 (token "controllerBehavior") (name "controllerBehavior") (separator none) (span (offset 1220) (line 41) (column 11) (len 18))) (segment 1 (token "sendFuelCommand") (name "sendFuelCommand") (separator dot) (span (offset 1239) (line 41) (column 30) (len 15)))))
    (reference r13 (scope relative) (span (offset 1312) (line 43) (column 29) (len 13)) (segments (segment 0 (token "speedometer_a") (name "speedometer_a") (separator none) (span (offset 1312) (line 43) (column 29) (len 13)))))
    (reference r14 (scope relative) (span (offset 1338) (line 44) (column 11) (len 35)) (segments (segment 0 (token "speedometerBehavior") (name "speedometerBehavior") (separator none) (span (offset 1338) (line 44) (column 11) (len 19))) (segment 1 (token "sendSensedSpeed") (name "sendSensedSpeed") (separator dot) (span (offset 1358) (line 44) (column 31) (len 15)))))
    (reference r15 (scope relative) (span (offset 1426) (line 46) (column 24) (len 8)) (segments (segment 0 (token "engine_a") (name "engine_a") (separator none) (span (offset 1426) (line 46) (column 24) (len 8)))))
    (reference r16 (scope relative) (span (offset 1447) (line 47) (column 11) (len 33)) (segments (segment 0 (token "engineBehavior") (name "engineBehavior") (separator none) (span (offset 1447) (line 47) (column 11) (len 14))) (segment 1 (token "receiveFuelCommand") (name "receiveFuelCommand") (separator dot) (span (offset 1462) (line 47) (column 26) (len 18)))))
  )
  (root (package (name "Interaction Realization-1") (body brace (import (target (span (span (offset 54) (line 2) (column 17) (len 26))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 77) (line 2) (column 40) (len 3))) (separator (span (offset 77) (line 2) (column 40) (len 2))) (marker (span (offset 79) (line 2) (column 42) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "driver_a") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (action-usage))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle_a") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "cruiseController_a") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (action-usage))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "speedometer_a") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (action-usage))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine_a") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (action-usage))))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "cruiseControlInteraction_a") (short-name none) (target none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "driver") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r6)))) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "") (short-name none) (target (ref r7)) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r8)))) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "cruiseController") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r9)))) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "") (short-name none) (target (ref r10)) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "") (short-name none) (target (ref r11)) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "") (short-name none) (target (ref r12)) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "speedometer") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r13)))) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "") (short-name none) (target (ref r14)) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r15)))) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "") (short-name none) (target (ref r16)) (body semicolon)))))) (malformed (code "recovered_occurrence_body_element") (found "message :>> setSpeedMessage = driver_a.driverBehavior.sendSe") (span (offset 1523) (line 51) (column 3) (len 82))) (malformed (code "recovered_occurrence_body_element") (found "message :>> sensedSpeedMessage = vehicle_a.speedometer_a.spe") (span (offset 1605) (line 52) (column 3) (len 108))) (malformed (code "recovered_occurrence_body_element") (found "message :>> fuelCommandMessage = vehicle_a.cruiseController_") (span (offset 1713) (line 53) (column 3) (len 111))))))))
)
~~~
