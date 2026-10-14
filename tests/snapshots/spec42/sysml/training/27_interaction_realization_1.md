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
        part :>> driver, driver_a {
            event driverBehavior.sendSetSpeed[1] :>> setSpeedSent;
        }
        part :>> vehicle, vehicle_a {
            part :>> cruiseController, cruiseController_a {
                event controllerBehavior.receiveSetSpeed[1] :>> setSpeedReceived;
                event controllerBehavior.receiveSensedSpeed[1] :>> sensedSpeedReceived;
                event controllerBehavior.sendFuelCommand[1] :>> fuelCommandSent;
            }
            part :>> speedometer, speedometer_a {
                event speedometerBehavior.sendSensedSpeed[1] :>> sensedSpeedSent;
            }
            part :>> engine, engine_a {
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
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 54) (line 2) (column 17) (len 23)) (segments (segment 0 (token "'Interaction Example-1'") (name "Interaction Example-1") (separator none) (span (offset 54) (line 2) (column 17) (len 23)))))
    (reference r1 (scope relative) (span (offset 101) (line 4) (column 18) (len 6)) (segments (segment 0 (token "Driver") (name "Driver") (separator none) (span (offset 101) (line 4) (column 18) (len 6)))))
    (reference r2 (scope relative) (span (offset 220) (line 10) (column 19) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 220) (line 10) (column 19) (len 7)))))
    (reference r3 (scope relative) (span (offset 258) (line 11) (column 29) (len 16)) (segments (segment 0 (token "CruiseController") (name "CruiseController") (separator none) (span (offset 258) (line 11) (column 29) (len 16)))))
    (reference r4 (scope relative) (span (offset 547) (line 19) (column 24) (len 11)) (segments (segment 0 (token "Speedometer") (name "Speedometer") (separator none) (span (offset 547) (line 19) (column 24) (len 11)))))
    (reference r5 (scope relative) (span (offset 696) (line 25) (column 19) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 696) (line 25) (column 19) (len 6)))))
    (reference r6 (scope relative) (span (offset 888) (line 33) (column 12) (len 6)) (segments (segment 0 (token "driver") (name "driver") (separator none) (span (offset 888) (line 33) (column 12) (len 6)))))
    (reference r7 (scope relative) (span (offset 899) (line 33) (column 23) (len 8)) (segments (segment 0 (token "driver_a") (name "driver_a") (separator none) (span (offset 899) (line 33) (column 23) (len 8)))))
    (reference r8 (scope relative) (span (offset 919) (line 34) (column 10) (len 27)) (segments (segment 0 (token "driverBehavior") (name "driverBehavior") (separator none) (span (offset 919) (line 34) (column 10) (len 14))) (segment 1 (token "sendSetSpeed") (name "sendSetSpeed") (separator dot) (span (offset 934) (line 34) (column 25) (len 12)))))
    (reference r9 (scope relative) (span (offset 986) (line 37) (column 12) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 986) (line 37) (column 12) (len 7)))))
    (reference r10 (scope relative) (span (offset 998) (line 37) (column 24) (len 9)) (segments (segment 0 (token "vehicle_a") (name "vehicle_a") (separator none) (span (offset 998) (line 37) (column 24) (len 9)))))
    (reference r11 (scope relative) (span (offset 1022) (line 38) (column 13) (len 16)) (segments (segment 0 (token "cruiseController") (name "cruiseController") (separator none) (span (offset 1022) (line 38) (column 13) (len 16)))))
    (reference r12 (scope relative) (span (offset 1043) (line 38) (column 34) (len 18)) (segments (segment 0 (token "cruiseController_a") (name "cruiseController_a") (separator none) (span (offset 1043) (line 38) (column 34) (len 18)))))
    (reference r13 (scope relative) (span (offset 1074) (line 39) (column 11) (len 34)) (segments (segment 0 (token "controllerBehavior") (name "controllerBehavior") (separator none) (span (offset 1074) (line 39) (column 11) (len 18))) (segment 1 (token "receiveSetSpeed") (name "receiveSetSpeed") (separator dot) (span (offset 1093) (line 39) (column 30) (len 15)))))
    (reference r14 (scope relative) (span (offset 1144) (line 40) (column 11) (len 37)) (segments (segment 0 (token "controllerBehavior") (name "controllerBehavior") (separator none) (span (offset 1144) (line 40) (column 11) (len 18))) (segment 1 (token "receiveSensedSpeed") (name "receiveSensedSpeed") (separator dot) (span (offset 1163) (line 40) (column 30) (len 18)))))
    (reference r15 (scope relative) (span (offset 1220) (line 41) (column 11) (len 34)) (segments (segment 0 (token "controllerBehavior") (name "controllerBehavior") (separator none) (span (offset 1220) (line 41) (column 11) (len 18))) (segment 1 (token "sendFuelCommand") (name "sendFuelCommand") (separator dot) (span (offset 1239) (line 41) (column 30) (len 15)))))
    (reference r16 (scope relative) (span (offset 1296) (line 43) (column 13) (len 11)) (segments (segment 0 (token "speedometer") (name "speedometer") (separator none) (span (offset 1296) (line 43) (column 13) (len 11)))))
    (reference r17 (scope relative) (span (offset 1312) (line 43) (column 29) (len 13)) (segments (segment 0 (token "speedometer_a") (name "speedometer_a") (separator none) (span (offset 1312) (line 43) (column 29) (len 13)))))
    (reference r18 (scope relative) (span (offset 1338) (line 44) (column 11) (len 35)) (segments (segment 0 (token "speedometerBehavior") (name "speedometerBehavior") (separator none) (span (offset 1338) (line 44) (column 11) (len 19))) (segment 1 (token "sendSensedSpeed") (name "sendSensedSpeed") (separator dot) (span (offset 1358) (line 44) (column 31) (len 15)))))
    (reference r19 (scope relative) (span (offset 1415) (line 46) (column 13) (len 6)) (segments (segment 0 (token "engine") (name "engine") (separator none) (span (offset 1415) (line 46) (column 13) (len 6)))))
    (reference r20 (scope relative) (span (offset 1426) (line 46) (column 24) (len 8)) (segments (segment 0 (token "engine_a") (name "engine_a") (separator none) (span (offset 1426) (line 46) (column 24) (len 8)))))
    (reference r21 (scope relative) (span (offset 1447) (line 47) (column 11) (len 33)) (segments (segment 0 (token "engineBehavior") (name "engineBehavior") (separator none) (span (offset 1447) (line 47) (column 11) (len 14))) (segment 1 (token "receiveFuelCommand") (name "receiveFuelCommand") (separator dot) (span (offset 1462) (line 47) (column 26) (len 18)))))
    (reference r22 (scope relative) (span (offset 1535) (line 51) (column 15) (len 15)) (segments (segment 0 (token "setSpeedMessage") (name "setSpeedMessage") (separator none) (span (offset 1535) (line 51) (column 15) (len 15)))))
    (reference r23 (scope relative) (span (offset 1553) (line 51) (column 33) (len 8)) (segments (segment 0 (token "driver_a") (name "driver_a") (separator none) (span (offset 1553) (line 51) (column 33) (len 8)))))
    (reference r24 (scope relative) (span (offset 1562) (line 51) (column 42) (len 14)) (segments (segment 0 (token "driverBehavior") (name "driverBehavior") (separator none) (span (offset 1562) (line 51) (column 42) (len 14)))))
    (reference r25 (scope relative) (span (offset 1577) (line 51) (column 57) (len 12)) (segments (segment 0 (token "sendSetSpeed") (name "sendSetSpeed") (separator none) (span (offset 1577) (line 51) (column 57) (len 12)))))
    (reference r26 (scope relative) (span (offset 1590) (line 51) (column 70) (len 11)) (segments (segment 0 (token "sentMessage") (name "sentMessage") (separator none) (span (offset 1590) (line 51) (column 70) (len 11)))))
    (reference r27 (scope relative) (span (offset 1617) (line 52) (column 15) (len 18)) (segments (segment 0 (token "sensedSpeedMessage") (name "sensedSpeedMessage") (separator none) (span (offset 1617) (line 52) (column 15) (len 18)))))
    (reference r28 (scope relative) (span (offset 1638) (line 52) (column 36) (len 9)) (segments (segment 0 (token "vehicle_a") (name "vehicle_a") (separator none) (span (offset 1638) (line 52) (column 36) (len 9)))))
    (reference r29 (scope relative) (span (offset 1648) (line 52) (column 46) (len 13)) (segments (segment 0 (token "speedometer_a") (name "speedometer_a") (separator none) (span (offset 1648) (line 52) (column 46) (len 13)))))
    (reference r30 (scope relative) (span (offset 1662) (line 52) (column 60) (len 19)) (segments (segment 0 (token "speedometerBehavior") (name "speedometerBehavior") (separator none) (span (offset 1662) (line 52) (column 60) (len 19)))))
    (reference r31 (scope relative) (span (offset 1682) (line 52) (column 80) (len 15)) (segments (segment 0 (token "sendSensedSpeed") (name "sendSensedSpeed") (separator none) (span (offset 1682) (line 52) (column 80) (len 15)))))
    (reference r32 (scope relative) (span (offset 1698) (line 52) (column 96) (len 11)) (segments (segment 0 (token "sentMessage") (name "sentMessage") (separator none) (span (offset 1698) (line 52) (column 96) (len 11)))))
    (reference r33 (scope relative) (span (offset 1725) (line 53) (column 15) (len 18)) (segments (segment 0 (token "fuelCommandMessage") (name "fuelCommandMessage") (separator none) (span (offset 1725) (line 53) (column 15) (len 18)))))
    (reference r34 (scope relative) (span (offset 1746) (line 53) (column 36) (len 9)) (segments (segment 0 (token "vehicle_a") (name "vehicle_a") (separator none) (span (offset 1746) (line 53) (column 36) (len 9)))))
    (reference r35 (scope relative) (span (offset 1756) (line 53) (column 46) (len 18)) (segments (segment 0 (token "cruiseController_a") (name "cruiseController_a") (separator none) (span (offset 1756) (line 53) (column 46) (len 18)))))
    (reference r36 (scope relative) (span (offset 1775) (line 53) (column 65) (len 18)) (segments (segment 0 (token "controllerBehavior") (name "controllerBehavior") (separator none) (span (offset 1775) (line 53) (column 65) (len 18)))))
    (reference r37 (scope relative) (span (offset 1794) (line 53) (column 84) (len 15)) (segments (segment 0 (token "sendFuelCommand") (name "sendFuelCommand") (separator none) (span (offset 1794) (line 53) (column 84) (len 15)))))
    (reference r38 (scope relative) (span (offset 1810) (line 53) (column 100) (len 11)) (segments (segment 0 (token "sentMessage") (name "sentMessage") (separator none) (span (offset 1810) (line 53) (column 100) (len 11)))))
  )
  (root (package (name "Interaction Realization-1") (body brace (import (target (span (span (offset 54) (line 2) (column 17) (len 26))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 77) (line 2) (column 40) (len 3))) (separator (span (offset 77) (line 2) (column 40) (len 2))) (marker (span (offset 79) (line 2) (column 42) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "driver_a") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (action-usage))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle_a") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "cruiseController_a") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (action-usage))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "speedometer_a") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (action-usage))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine_a") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (action-usage))))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "cruiseControlInteraction_a") (short-name none) (target none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r6) (ref r7)))) (references none) (crosses none) (intersects none) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration none) (short-name none) (target (ref r8)) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r9) (ref r10)))) (references none) (crosses none) (intersects none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r11) (ref r12)))) (references none) (crosses none) (intersects none) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration none) (short-name none) (target (ref r13)) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration none) (short-name none) (target (ref r14)) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration none) (short-name none) (target (ref r15)) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r16) (ref r17)))) (references none) (crosses none) (intersects none) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration none) (short-name none) (target (ref r18)) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r19) (ref r20)))) (references none) (crosses none) (intersects none) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration none) (short-name none) (target (ref r21)) (body semicolon)))))) (flow-usage (kind message) (visibility none) (declaration (declared (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r22)))) (references none) (crosses none) (intersects none)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1553) (line 51) (column 33) (len 48)) (member-access (base (expression (span (offset 1553) (line 51) (column 33) (len 36)) (member-access (base (expression (span (offset 1553) (line 51) (column 33) (len 23)) (member-access (base (expression (span (offset 1553) (line 51) (column 33) (len 8)) (ref r23))) (separator dot) (member (ref r24))))) (separator dot) (member (ref r25))))) (separator dot) (member (ref r26))))))) (payloads) (endpoints none))) (body (body semicolon))) (flow-usage (kind message) (visibility none) (declaration (declared (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r27)))) (references none) (crosses none) (intersects none)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1638) (line 52) (column 36) (len 71)) (member-access (base (expression (span (offset 1638) (line 52) (column 36) (len 59)) (member-access (base (expression (span (offset 1638) (line 52) (column 36) (len 43)) (member-access (base (expression (span (offset 1638) (line 52) (column 36) (len 23)) (member-access (base (expression (span (offset 1638) (line 52) (column 36) (len 9)) (ref r28))) (separator dot) (member (ref r29))))) (separator dot) (member (ref r30))))) (separator dot) (member (ref r31))))) (separator dot) (member (ref r32))))))) (payloads) (endpoints none))) (body (body semicolon))) (flow-usage (kind message) (visibility none) (declaration (declared (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r33)))) (references none) (crosses none) (intersects none)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1746) (line 53) (column 36) (len 75)) (member-access (base (expression (span (offset 1746) (line 53) (column 36) (len 63)) (member-access (base (expression (span (offset 1746) (line 53) (column 36) (len 47)) (member-access (base (expression (span (offset 1746) (line 53) (column 36) (len 28)) (member-access (base (expression (span (offset 1746) (line 53) (column 36) (len 9)) (ref r34))) (separator dot) (member (ref r35))))) (separator dot) (member (ref r36))))) (separator dot) (member (ref r37))))) (separator dot) (member (ref r38))))))) (payloads) (endpoints none))) (body (body semicolon))))))))
)
~~~
