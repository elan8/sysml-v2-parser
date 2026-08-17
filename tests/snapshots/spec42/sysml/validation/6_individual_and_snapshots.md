# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (06-Individual and Snapshots): 6-Individual and Snapshots"))
~~~
# SOURCE
~~~sysml
package '6-Individual and Snapshots' {
	private import ScalarValues::Real;
	private import Time::DateTime;
	private import ISQ::*;
	
	package 'Part Definitions' {	
		part def 'Temporal-Spatial Reference' {
			attribute referenceTime : DateTime;
			attribute referenceCoordinateSystem;
		}
		
		/*
		 * Note that space and time coordinatization have not
		 * been fully specified yet.
		 */
		
		part def VehicleRoadContext {
			attribute t : TimeValue;
		}
		
		part def VehicleA {
			attribute mass : MassValue;
			attribute position : Real;
			attribute velocity : Real;
			attribute acceleration : Real;
			exhibit state vehicleStates {
				entry; then on;
				state on;
				then off;
				state off;
			}
		}
		
		part def Road {
			attribute angle : Real;
			attribute surfaceFriction : Real;
		}
	}
	
	package 'Individual Definitions' {
		private import 'Part Definitions'::*;
		
		/*
		 * An individual definition restricts the instances of a part def to
		 * those that are portions of the same life ("identity").
		 */
		 
		individual def 'Temporal-Spatial Reference_ID1' :> 'Temporal-Spatial Reference';
		individual def VehicleRoadContext_ID1 :> VehicleRoadContext;
		individual def VehicleA_ID1 :> VehicleA;
		individual def Road_ID1 :> Road;
	
	}
	
	package Values {	
		attribute t0 : TimeValue;
		attribute t1 : TimeValue;
		attribute tn : TimeValue;
		
		attribute m : MassValue;
		
		attribute p0 : Real;
		attribute p1 : Real;
		attribute pn : Real;
		
		attribute v0 : Real;
		attribute v1 : Real;
		attribute vn : Real;
		
		attribute a0 : Real;
		attribute a1 : Real;
		attribute an : Real;
		
		attribute theta0 : Real;
		attribute theta1 : Real;
		attribute thetan : Real;
		
		attribute sf0 : Real;
		attribute sf1 : Real;
		attribute sfn : Real;
	}
	
	package 'Individuals and Snapshots' {
		private import 'Individual Definitions'::*;
		private import Values::*;
		
		individual reference : 'Temporal-Spatial Reference_ID1' {
			/*
			 * An individual usage must be typed by an individual definition,
			 * representing the condition of that individual during some or all
			 * of its life.
			 */
		
			snapshot context_t0 : VehicleRoadContext_ID1 {
				:>> t = t0 {
					/*
					 * This is a concise notation for showing the redefinition
					 * of a attribute property.
					 */
				}
				
				snapshot vehicle_ID1_t0 : VehicleA_ID1 {
					/*
					 * A snapshot is a kind of individual usage restricted to
					 * a single instant of time.
					 */
				
					:>> mass = m;
					:>> position = p0;
					:>> velocity = v0;
					:>> acceleration = a0;
					
					exhibit vehicleStates.on {
						/*
						 * This asserts that the snapshot exhibits the referenced 
						 * state, which means that the vehicle must me in the state 
						 * at the time of the snapshot.
						 */
					}
				}
				
				snapshot road_ID1_t0 : Road_ID1 {
					:>> angle = theta0;
					:>> surfaceFriction = sf0;
				}
			}
			
			snapshot context_t1 : VehicleRoadContext_ID1 {
				:>> t = t1;
				
				snapshot vehicle_ID1_t1 : VehicleA_ID1 {
					:>> mass = m;
					:>> position = p1;
					:>> velocity = v1;
					:>> acceleration = a1;
					
					exhibit vehicleStates.on;
				}
				
				snapshot road_ID1_t1 : Road_ID1 {
					:>> angle = theta1;
					:>> surfaceFriction = sf1;
				}
			}
			
			// ...
			
			snapshot context_tn : VehicleRoadContext_ID1 {
				:>> t = tn;
				
				snapshot vehicle_ID1_tn : VehicleA_ID1 {
					:>> mass = m;
					:>> position = pn;
					:>> velocity = vn;
					:>> acceleration = an;
					
					exhibit vehicleStates.off;
				}
				
				snapshot road_ID1_tn : Road_ID1 {
					:>> angle = theta1;
					:>> surfaceFriction = sfn;
				}
			}
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "6_individual_and_snapshots.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '6-Individual and Snapshots' {
    private import ScalarValues::Real;
    private import Time::DateTime;
    private import ISQ::*;
    package 'Part Definitions' {
        part def 'Temporal-Spatial Reference' {
            attribute referenceTime : DateTime;
            attribute referenceCoordinateSystem;
        }
        part def VehicleRoadContext {
            attribute t : TimeValue;
        }
        part def VehicleA {
            attribute mass : MassValue;
            attribute position : Real;
            attribute velocity : Real;
            attribute acceleration : Real;
            exhibit state vehicleStates {
                entry;
                then on;
                state on;
                then off;
                state off;
            }
        }
        part def Road {
            attribute angle : Real;
            attribute surfaceFriction : Real;
        }
    }
    package 'Individual Definitions' {
        private import 'Part Definitions'::*;
        individual def 'Temporal-Spatial Reference_ID1' :> 'Temporal-Spatial Reference';
        individual def VehicleRoadContext_ID1 :> VehicleRoadContext;
        individual def VehicleA_ID1 :> VehicleA;
        individual def Road_ID1 :> Road;
    }
    package Values {
        attribute def t0 : TimeValue;
        attribute def t1 : TimeValue;
        attribute def tn : TimeValue;
        attribute def m : MassValue;
        attribute def p0 : Real;
        attribute def p1 : Real;
        attribute def pn : Real;
        attribute def v0 : Real;
        attribute def v1 : Real;
        attribute def vn : Real;
        attribute def a0 : Real;
        attribute def a1 : Real;
        attribute def an : Real;
        attribute def theta0 : Real;
        attribute def theta1 : Real;
        attribute def thetan : Real;
        attribute def sf0 : Real;
        attribute def sf1 : Real;
        attribute def sfn : Real;
    }
    package 'Individuals and Snapshots' {
        private import 'Individual Definitions'::*;
        private import Values::*;
        individual reference : 'Temporal-Spatial Reference_ID1' {
            snapshot context_t0 : VehicleRoadContext_ID1 {
                attribute :>> t = t0 {
                }
                snapshot vehicle_ID1_t0 : VehicleA_ID1 {
                    attribute :>> mass = m;
                    attribute :>> position = p0;
                    attribute :>> velocity = v0;
                    attribute :>> acceleration = a0;
                    exhibit vehicleStates.on {
                    }
                }
                snapshot road_ID1_t0 : Road_ID1 {
                    attribute :>> angle = theta0;
                    attribute :>> surfaceFriction = sf0;
                }
            }
            snapshot context_t1 : VehicleRoadContext_ID1 {
                attribute :>> t = t1;
                snapshot vehicle_ID1_t1 : VehicleA_ID1 {
                    attribute :>> mass = m;
                    attribute :>> position = p1;
                    attribute :>> velocity = v1;
                    attribute :>> acceleration = a1;
                    exhibit vehicleStates.on;
                }
                snapshot road_ID1_t1 : Road_ID1 {
                    attribute :>> angle = theta1;
                    attribute :>> surfaceFriction = sf1;
                }
            }
            snapshot context_tn : VehicleRoadContext_ID1 {
                attribute :>> t = tn;
                snapshot vehicle_ID1_tn : VehicleA_ID1 {
                    attribute :>> mass = m;
                    attribute :>> position = pn;
                    attribute :>> velocity = vn;
                    attribute :>> acceleration = an;
                    exhibit vehicleStates.off;
                }
                snapshot road_ID1_tn : Road_ID1 {
                    attribute :>> angle = theta1;
                    attribute :>> surfaceFriction = sfn;
                }
            }
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 55) (line 2) (column 17) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 55) (line 2) (column 17) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 69) (line 2) (column 31) (len 4)))))
    (reference r1 (scope relative) (span (offset 91) (line 3) (column 17) (len 14)) (segments (segment 0 (token "Time") (name "Time") (separator none) (span (offset 91) (line 3) (column 17) (len 4))) (segment 1 (token "DateTime") (name "DateTime") (separator colon-colon) (span (offset 97) (line 3) (column 23) (len 8)))))
    (reference r2 (scope relative) (span (offset 123) (line 4) (column 17) (len 3)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 123) (line 4) (column 17) (len 3)))))
    (reference r3 (scope relative) (span (offset 235) (line 8) (column 30) (len 8)) (segments (segment 0 (token "DateTime") (name "DateTime") (separator none) (span (offset 235) (line 8) (column 30) (len 8)))))
    (reference r4 (scope relative) (span (offset 442) (line 18) (column 18) (len 9)) (segments (segment 0 (token "TimeValue") (name "TimeValue") (separator none) (span (offset 442) (line 18) (column 18) (len 9)))))
    (reference r5 (scope relative) (span (offset 502) (line 22) (column 21) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 502) (line 22) (column 21) (len 9)))))
    (reference r6 (scope relative) (span (offset 537) (line 23) (column 25) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 537) (line 23) (column 25) (len 4)))))
    (reference r7 (scope relative) (span (offset 567) (line 24) (column 25) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 567) (line 24) (column 25) (len 4)))))
    (reference r8 (scope relative) (span (offset 601) (line 25) (column 29) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 601) (line 25) (column 29) (len 4)))))
    (reference r9 (scope relative) (span (offset 754) (line 35) (column 22) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 754) (line 35) (column 22) (len 4)))))
    (reference r10 (scope relative) (span (offset 791) (line 36) (column 32) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 791) (line 36) (column 32) (len 4)))))
    (reference r11 (scope relative) (span (offset 859) (line 41) (column 18) (len 18)) (segments (segment 0 (token "'Part Definitions'") (name "Part Definitions") (separator none) (span (offset 859) (line 41) (column 18) (len 18)))))
    (reference r12 (scope relative) (span (offset 1831) (line 84) (column 18) (len 24)) (segments (segment 0 (token "'Individual Definitions'") (name "Individual Definitions") (separator none) (span (offset 1831) (line 84) (column 18) (len 24)))))
    (reference r13 (scope relative) (span (offset 1877) (line 85) (column 18) (len 6)) (segments (segment 0 (token "Values") (name "Values") (separator none) (span (offset 1877) (line 85) (column 18) (len 6)))))
  )
  (root (package (name "6-Individual and Snapshots") (body brace (import (target (span (span (offset 55) (line 2) (column 17) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 91) (line 3) (column 17) (len 14))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 123) (line 4) (column 17) (len 6))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 126) (line 4) (column 20) (len 3))) (separator (span (offset 126) (line 4) (column 20) (len 2))) (marker (span (offset 128) (line 4) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "Part Definitions") (body brace (part-def (name "Temporal-Spatial Reference") (body brace (attribute-usage (declaration-name "referenceTime") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "referenceCoordinateSystem") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "VehicleRoadContext") (body brace (attribute-usage (declaration-name "t") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "VehicleA") (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "position") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "velocity") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "acceleration") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (exhibit (declaration "vehicleStates") (state none)))) (part-def (name "Road") (body brace (attribute-usage (declaration-name "angle") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "surfaceFriction") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))) (package (name "Individual Definitions") (body brace (import (target (span (span (offset 859) (line 41) (column 18) (len 21))) (all none) (ref r11) (shape (namespace (wildcard-suffix (span (span (offset 877) (line 41) (column 36) (len 3))) (separator (span (offset 877) (line 41) (column 36) (len 2))) (marker (span (offset 879) (line 41) (column 38) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (individual-def) (individual-def) (individual-def) (individual-def))) (package (name "Values") (body brace (attribute-def (name "t0") (multiplicity none)) (attribute-def (name "t1") (multiplicity none)) (attribute-def (name "tn") (multiplicity none)) (attribute-def (name "m") (multiplicity none)) (attribute-def (name "p0") (multiplicity none)) (attribute-def (name "p1") (multiplicity none)) (attribute-def (name "pn") (multiplicity none)) (attribute-def (name "v0") (multiplicity none)) (attribute-def (name "v1") (multiplicity none)) (attribute-def (name "vn") (multiplicity none)) (attribute-def (name "a0") (multiplicity none)) (attribute-def (name "a1") (multiplicity none)) (attribute-def (name "an") (multiplicity none)) (attribute-def (name "theta0") (multiplicity none)) (attribute-def (name "theta1") (multiplicity none)) (attribute-def (name "thetan") (multiplicity none)) (attribute-def (name "sf0") (multiplicity none)) (attribute-def (name "sf1") (multiplicity none)) (attribute-def (name "sfn") (multiplicity none)))) (package (name "Individuals and Snapshots") (body brace (import (target (span (span (offset 1831) (line 84) (column 18) (len 27))) (all none) (ref r12) (shape (namespace (wildcard-suffix (span (span (offset 1855) (line 84) (column 42) (len 3))) (separator (span (offset 1855) (line 84) (column 42) (len 2))) (marker (span (offset 1857) (line 84) (column 44) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 1877) (line 85) (column 18) (len 9))) (all none) (ref r13) (shape (namespace (wildcard-suffix (span (span (offset 1883) (line 85) (column 24) (len 3))) (separator (span (offset 1883) (line 85) (column 24) (len 2))) (marker (span (offset 1885) (line 85) (column 26) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (occurrence (portion none) (declaration "reference") (short-name none) (target none)))))))
)
~~~
