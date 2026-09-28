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
    (reference r2 (scope relative) (span (offset 342) (line 19) (column 18) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 342) (line 19) (column 18) (len 7)))))
    (reference r3 (scope relative) (span (offset 375) (line 20) (column 24) (len 12)) (segments (segment 0 (token "VehicleFrame") (name "VehicleFrame") (separator none) (span (offset 375) (line 20) (column 24) (len 12)))))
    (reference r4 (scope relative) (span (offset 407) (line 21) (column 18) (len 9)) (segments (segment 0 (token "HitchBall") (name "HitchBall") (separator none) (span (offset 407) (line 21) (column 18) (len 9)))))
    (reference r5 (scope relative) (span (offset 458) (line 25) (column 29) (len 12)) (segments (segment 0 (token "TrailerHitch") (name "TrailerHitch") (separator none) (span (offset 458) (line 25) (column 29) (len 12)))))
    (reference r6 (scope relative) (span (offset 488) (line 26) (column 12) (len 26)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 488) (line 26) (column 12) (len 7))) (segment 1 (token "vehicleFrame") (name "vehicleFrame") (separator dot) (span (offset 496) (line 26) (column 20) (len 12))) (segment 2 (token "hitch") (name "hitch") (separator dot) (span (offset 509) (line 26) (column 33) (len 5)))))
    (reference r7 (scope relative) (span (offset 518) (line 26) (column 42) (len 28)) (segments (segment 0 (token "trailer") (name "trailer") (separator none) (span (offset 518) (line 26) (column 42) (len 7))) (segment 1 (token "trailerFrame") (name "trailerFrame") (separator dot) (span (offset 526) (line 26) (column 50) (len 12))) (segment 2 (token "coupler") (name "coupler") (separator dot) (span (offset 539) (line 26) (column 63) (len 7)))))
    (reference r8 (scope relative) (span (offset 568) (line 28) (column 18) (len 7)) (segments (segment 0 (token "Trailer") (name "Trailer") (separator none) (span (offset 568) (line 28) (column 18) (len 7)))))
    (reference r9 (scope relative) (span (offset 601) (line 29) (column 24) (len 12)) (segments (segment 0 (token "TrailerFrame") (name "TrailerFrame") (separator none) (span (offset 601) (line 29) (column 24) (len 12)))))
    (reference r10 (scope relative) (span (offset 635) (line 30) (column 20) (len 14)) (segments (segment 0 (token "TrailerCoupler") (name "TrailerCoupler") (separator none) (span (offset 635) (line 30) (column 20) (len 14)))))
  )
  (root (package (name "3c-Function-based Behavior-structure mod-1") (body brace (part-def (name "Vehicle") (modifiers) (body semicolon)) (part-def (name "VehicleFrame") (modifiers) (body semicolon)) (part-def (name "HitchBall") (modifiers) (body semicolon)) (part-def (name "TrailerCoupler") (modifiers) (body semicolon)) (part-def (name "Trailer") (modifiers) (body semicolon)) (part-def (name "TrailerFrame") (modifiers) (body semicolon)) (connection-def (name "TrailerHitch") (modifiers) (extensions) (specializes none) (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions) (short-name none) (identity (declaration (name "hitch") (span (offset 233) (line 13) (column 7) (len 5)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (references none) (multiplicity none) (redefines none) (crosses none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions) (short-name none) (identity (declaration (name "coupler") (span (offset 258) (line 14) (column 7) (len 7)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (references none) (multiplicity none) (redefines none) (crosses none)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle-trailer system") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicleFrame") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "hitch") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))))) (connection-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "trailerHitch") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity (lower (expression (span (offset 471) (line 25) (column 42) (len 1)) (integer 0))) (upper (expression (span (offset 474) (line 25) (column 45) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (connect (expression (span (offset 488) (line 26) (column 12) (len 26)) (ref r6)) (expression (span (offset 518) (line 26) (column 42) (len 28)) (ref r7))) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "trailer") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "trailerFrame") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "coupler") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))))) (action-usage))))))
)
~~~
