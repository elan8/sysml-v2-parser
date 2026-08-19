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
    (reference r2 (scope relative) (span (offset 262) (line 14) (column 27) (len 16)) (segments (segment 0 (token "CruiseController") (name "CruiseController") (separator none) (span (offset 262) (line 14) (column 27) (len 16)))))
    (reference r3 (scope relative) (span (offset 439) (line 20) (column 22) (len 11)) (segments (segment 0 (token "Speedometer") (name "Speedometer") (separator none) (span (offset 439) (line 20) (column 22) (len 11)))))
    (reference r4 (scope relative) (span (offset 513) (line 24) (column 17) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 513) (line 24) (column 17) (len 6)))))
  )
  (root (package (name "Event Occurrence Example") (body brace (part-def (name "Driver") (body semicolon)) (part-def (name "CruiseController") (body semicolon)) (part-def (name "Speedometer") (body semicolon)) (part-def (name "Engine") (body semicolon)) (part-def (name "Vehicle") (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "driver") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "setSpeedSent") (short-name none) (target none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "cruiseController") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "setSpeedReceived") (short-name none) (target none) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "sensedSpeedReceived") (short-name none) (target none) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "fuelCommandSent") (short-name none) (target none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "speedometer") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "sensedSpeedSent") (short-name none) (target none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "fuelCommandReceived") (short-name none) (target none) (body semicolon)))))))))
)
~~~
