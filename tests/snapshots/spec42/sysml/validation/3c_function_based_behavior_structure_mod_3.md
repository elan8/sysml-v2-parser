# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (03-Function-based Behavior): 3c-Function-based Behavior-structure mod-3"))
~~~
# SOURCE
~~~sysml
package '3c-Function-based Behavior-structure mod-3' {
	
	part def Vehicle;
	part def VehicleFrame;
	part def HitchBall;
	part def Trailer;
	part def TrailerFrame;
	part def TrailerCoupler;
	
	part vehicle : Vehicle {
		part vehicleFrame : VehicleFrame {
			part hitch : HitchBall;
		}
	}
	
	part trailer : Trailer {
		part trailerFrame : TrailerFrame {
			part coupler : TrailerCoupler {
				ref part hitch : HitchBall;
			}
		}		
	}
			
	action {
		// Insert the vehicle HitchBall into the TrailerCoupler.
		action 'connect trailer to vehicle'
			assign trailer.trailerFrame.coupler.hitch := vehicle.vehicleFrame.hitch;
		
		// Remove the HitchBall from the TrailerCoupler.
		then action 'disconnect trailer from vehicle'
			assign trailer.trailerFrame.coupler.hitch := null;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "3c_function_based_behavior_structure_mod_3.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '3c-Function-based Behavior-structure mod-3' {
    part def Vehicle;
    part def VehicleFrame;
    part def HitchBall;
    part def Trailer;
    part def TrailerFrame;
    part def TrailerCoupler;
    part vehicle : Vehicle {
        part vehicleFrame : VehicleFrame {
            part hitch : HitchBall;
        }
    }
    part trailer : Trailer {
        part trailerFrame : TrailerFrame {
            part coupler : TrailerCoupler {
                ref part hitch : HitchBall;
            }
        }
    }
    action {
        action 'connect trailer to vehicle'
        assign trailer.trailerFrame.coupler.hitch := vehicle.vehicleFrame.hitch;
        then action 'disconnect trailer from vehicle'
        assign trailer.trailerFrame.coupler.hitch := null;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "3c-Function-based Behavior-structure mod-3") (body brace (part-def (name "Vehicle") (body semicolon)) (part-def (name "VehicleFrame") (body semicolon)) (part-def (name "HitchBall") (body semicolon)) (part-def (name "Trailer") (body semicolon)) (part-def (name "TrailerFrame") (body semicolon)) (part-def (name "TrailerCoupler") (body semicolon)) (part-usage) (part-usage) (action-usage))))
)
~~~
