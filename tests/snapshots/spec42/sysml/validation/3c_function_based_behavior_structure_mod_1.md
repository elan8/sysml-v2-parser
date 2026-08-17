# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (03-Function-based Behavior): 3c-Function-based Behavior-structure mod-1"))
~~~
# SOURCE
~~~sysml
package '3c-Function-based Behavior-structure mod-1' {
	
	part def Vehicle;
	part def VehicleFrame;
	
	part def HitchBall;
	part def TrailerCoupler;
	
	part def Trailer;
	part def TrailerFrame;
	
	connection def TrailerHitch {
		end hitch : HitchBall;
		end coupler : TrailerCoupler;
	}
	
	part 'vehicle-trailer system' {
		
		part vehicle : Vehicle {
			part vehicleFrame : VehicleFrame {
				part hitch : HitchBall;
			}
		}
		
		connection trailerHitch : TrailerHitch[0..1]
			connect vehicle.vehicleFrame.hitch to trailer.trailerFrame.coupler;
		
		part trailer : Trailer {
			part trailerFrame : TrailerFrame {
				part coupler : TrailerCoupler;
			}
		}
		
		action {
			// Create a link and assign it as the TrailerHitch connection.
			// Link participants are determined from inherited ends.
			action 'connect trailer to vehicle'
				assign 'vehicle-trailer system'.trailerHitch := new TrailerHitch();
				
			// Destroy the link object.
			then action 'destroy connection of trailer to vehicle' : 
				OccurrenceFunctions::destroy {
				inout occ = 'vehicle-trailer system'.trailerHitch;
			}
				
			// Remove the link from the TrailerHitch connection.
			then action 'disconnect trailer from vehicle'
				assign 'vehicle-trailer system'.trailerHitch := null;
		}	
	}	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "3c_function_based_behavior_structure_mod_1.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '3c-Function-based Behavior-structure mod-1' {
    part def Vehicle;
    part def VehicleFrame;
    part def HitchBall;
    part def TrailerCoupler;
    part def Trailer;
    part def TrailerFrame;
    connection def TrailerHitch {
        end hitch : HitchBall;
        end coupler : TrailerCoupler;
    }
    part 'vehicle-trailer system' {
        part vehicle : Vehicle {
            part vehicleFrame : VehicleFrame {
                part hitch : HitchBall;
            }
        }
        connection trailerHitch : TrailerHitch[0..1] connect vehicle.vehicleFrame.hitch to trailer.trailerFrame.coupler;
        part trailer : Trailer {
            part trailerFrame : TrailerFrame {
                part coupler : TrailerCoupler;
            }
        }
        action {
            action 'connect trailer to vehicle'
            assign 'vehicle-trailer system'.trailerHitch := new TrailerHitch();
            then action 'destroy connection of trailer to vehicle' : OccurrenceFunctions::destroy {
                inout occ = 'vehicle-trailer system'.trailerHitch;
            }
            then action 'disconnect trailer from vehicle'
            assign 'vehicle-trailer system'.trailerHitch := null;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 241) (line 13) (column 15) (len 9)) (segments (segment 0 (token "HitchBall") (name "HitchBall") (separator none) (span (offset 241) (line 13) (column 15) (len 9)))))
    (reference r1 (scope relative) (span (offset 268) (line 14) (column 17) (len 14)) (segments (segment 0 (token "TrailerCoupler") (name "TrailerCoupler") (separator none) (span (offset 268) (line 14) (column 17) (len 14)))))
  )
  (root (package (name "3c-Function-based Behavior-structure mod-1") (body brace (part-def (name "Vehicle") (body semicolon)) (part-def (name "VehicleFrame") (body semicolon)) (part-def (name "HitchBall") (body semicolon)) (part-def (name "TrailerCoupler") (body semicolon)) (part-def (name "Trailer") (body semicolon)) (part-def (name "TrailerFrame") (body semicolon)) (connection-def (name "TrailerHitch") (role ordinary) (specializes none) (body brace (end (identity (declaration (name "hitch") (span (offset 233) (line 13) (column 7) (len 5)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (references none) (redefines none) (crosses none)) (end (identity (declaration (name "coupler") (span (offset 258) (line 14) (column 7) (len 7)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (references none) (redefines none) (crosses none)))) (part-usage (declaration-name "vehicle-trailer system") (typing none) (body brace (part-usage) (connection) (part-usage) (action-usage))))))
)
~~~
