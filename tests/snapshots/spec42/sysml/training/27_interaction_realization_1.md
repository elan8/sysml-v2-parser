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
  )
  (root (package (name "Interaction Realization-1") (body (import (target (span (span (offset 54) (line 2) (column 17) (len 26))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 77) (line 2) (column 40) (len 3))) (separator (span (offset 77) (line 2) (column 40) (len 2))) (marker (span (offset 79) (line 2) (column 42) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage) (part-usage) (occurrence (portion none) (declaration "cruiseControlInteraction_a") (target none)))))
)
~~~
