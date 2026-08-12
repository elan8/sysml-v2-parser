# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (03-Function-based Behavior): 3c-Function-based Behavior-structure mod-2"))
~~~
# SOURCE
~~~sysml
package '3c-Function-based Behavior-structure mod-2' {
	
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
		
		perform action {
			action 'connect trailer to vehicle' {
				// Assert that exactly one connection exists during the
				// performance of this action.
				abstract ref :>> trailerHitch[1];
			}
			then action 'disconnect trailer from vehicle' {
				// Assert that exactly no connection exists during the
				// performance of this action.
				abstract ref :>> trailerHitch[0];		
			}
		}
		
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "3c_function_based_behavior_structure_mod_2.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '3c-Function-based Behavior-structure mod-2' {
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
        perform action  {
            action 'connect trailer to vehicle' {
                ref '' :>> trailerHitch;
            }
            then action 'disconnect trailer from vehicle' {
                ref '' :>> trailerHitch;
            }
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
  (root (package (name "3c-Function-based Behavior-structure mod-2") (body (part-def (name "Vehicle") (body semicolon)) (part-def (name "VehicleFrame") (body semicolon)) (part-def (name "HitchBall") (body semicolon)) (part-def (name "TrailerCoupler") (body semicolon)) (part-def (name "Trailer") (body semicolon)) (part-def (name "TrailerFrame") (body semicolon)) (connection-def (name "TrailerHitch") (role ordinary) (specializes none) (body (end (identity (declaration (name "hitch") (span (offset 233) (line 13) (column 7) (len 5)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (references none) (redefines none) (crosses none)) (end (identity (declaration (name "coupler") (span (offset 258) (line 14) (column 7) (len 7)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (references none) (redefines none) (crosses none)))) (part-usage))))
)
~~~
