# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 27 (Occurrences): Event Occurrence Example"))
~~~
# SOURCE
~~~sysml
package 'Event Occurrence Example' {	
	part def Driver;
	part def CruiseController;
	part def Speedometer;
	part def Engine;
	part def Vehicle;
	
	part driver : Driver {
		event occurrence setSpeedSent;
	}
	
	part vehicle : Vehicle {
	
		part cruiseController : CruiseController {
			event occurrence setSpeedReceived;		
			then event occurrence sensedSpeedReceived;		
			then event occurrence fuelCommandSent;
		}
		
		part speedometer : Speedometer {
			event occurrence sensedSpeedSent;
		}
		
		part engine : Engine {
			event occurrence fuelCommandReceived;
		}
	
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "27_event_occurrence_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Event Occurrence Example' {
    part def Driver;
    part def CruiseController;
    part def Speedometer;
    part def Engine;
    part def Vehicle;
    part driver : Driver {
        event occurrence setSpeedSent;
    }
    part vehicle : Vehicle {
        part cruiseController : CruiseController {
            event occurrence setSpeedReceived;
            then event occurrence sensedSpeedReceived;
            then event occurrence fuelCommandSent;
        }
        part speedometer : Speedometer {
            event occurrence sensedSpeedSent;
        }
        part engine : Engine {
            event occurrence fuelCommandReceived;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "Event Occurrence Example") (body (part-def (name "Driver") (body semicolon)) (part-def (name "CruiseController") (body semicolon)) (part-def (name "Speedometer") (body semicolon)) (part-def (name "Engine") (body semicolon)) (part-def (name "Vehicle") (body semicolon)) (part-usage) (part-usage))))
)
~~~
