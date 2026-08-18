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
    (reference r0 (scope relative) (span (offset 161) (line 8) (column 16) (len 6)) (segments (segment 0 (token "Driver") (name "Driver") (separator none) (span (offset 161) (line 8) (column 16) (len 6)))))
    (reference r1 (scope relative) (span (offset 224) (line 12) (column 17) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 224) (line 12) (column 17) (len 7)))))
  )
  (root (package (name "Event Occurrence Example") (body brace (part-def (name "Driver") (body semicolon)) (part-def (name "CruiseController") (body semicolon)) (part-def (name "Speedometer") (body semicolon)) (part-def (name "Engine") (body semicolon)) (part-def (name "Vehicle") (body semicolon)) (part-usage (declaration-name "driver") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity-modifiers (ordered false) (nonunique false)) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "setSpeedSent") (short-name none) (target none)))) (part-usage (declaration-name "vehicle") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity-modifiers (ordered false) (nonunique false)) (body brace (part-usage) (part-usage) (part-usage))))))
)
~~~
