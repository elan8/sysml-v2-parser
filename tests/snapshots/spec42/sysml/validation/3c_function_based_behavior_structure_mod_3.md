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
    (reference r0 (scope relative) (span (offset 208) (line 10) (column 17) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 208) (line 10) (column 17) (len 7)))))
    (reference r1 (scope relative) (span (offset 240) (line 11) (column 23) (len 12)) (segments (segment 0 (token "VehicleFrame") (name "VehicleFrame") (separator none) (span (offset 240) (line 11) (column 23) (len 12)))))
    (reference r2 (scope relative) (span (offset 271) (line 12) (column 17) (len 9)) (segments (segment 0 (token "HitchBall") (name "HitchBall") (separator none) (span (offset 271) (line 12) (column 17) (len 9)))))
    (reference r3 (scope relative) (span (offset 307) (line 16) (column 17) (len 7)) (segments (segment 0 (token "Trailer") (name "Trailer") (separator none) (span (offset 307) (line 16) (column 17) (len 7)))))
    (reference r4 (scope relative) (span (offset 339) (line 17) (column 23) (len 12)) (segments (segment 0 (token "TrailerFrame") (name "TrailerFrame") (separator none) (span (offset 339) (line 17) (column 23) (len 12)))))
    (reference r5 (scope relative) (span (offset 372) (line 18) (column 19) (len 14)) (segments (segment 0 (token "TrailerCoupler") (name "TrailerCoupler") (separator none) (span (offset 372) (line 18) (column 19) (len 14)))))
    (reference r6 (scope relative) (span (offset 410) (line 19) (column 22) (len 9)) (segments (segment 0 (token "HitchBall") (name "HitchBall") (separator none) (span (offset 410) (line 19) (column 22) (len 9)))))
  )
  (root (package (name "3c-Function-based Behavior-structure mod-3") (body brace (part-def (name "Vehicle") (body semicolon)) (part-def (name "VehicleFrame") (body semicolon)) (part-def (name "HitchBall") (body semicolon)) (part-def (name "Trailer") (body semicolon)) (part-def (name "TrailerFrame") (body semicolon)) (part-def (name "TrailerCoupler") (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicleFrame") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "hitch") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "trailer") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "trailerFrame") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "coupler") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "hitch") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))))))) (action-usage (name "") (short-name none) (body brace (action-usage (name "connect trailer to vehicle") (short-name none) (body absent)) (assign) (then-action) (assign))))))
)
~~~
