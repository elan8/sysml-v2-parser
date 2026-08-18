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
    (reference r12 (scope relative) (span (offset 1298) (line 56) (column 18) (len 9)) (segments (segment 0 (token "TimeValue") (name "TimeValue") (separator none) (span (offset 1298) (line 56) (column 18) (len 9)))))
    (reference r13 (scope relative) (span (offset 1326) (line 57) (column 18) (len 9)) (segments (segment 0 (token "TimeValue") (name "TimeValue") (separator none) (span (offset 1326) (line 57) (column 18) (len 9)))))
    (reference r14 (scope relative) (span (offset 1354) (line 58) (column 18) (len 9)) (segments (segment 0 (token "TimeValue") (name "TimeValue") (separator none) (span (offset 1354) (line 58) (column 18) (len 9)))))
    (reference r15 (scope relative) (span (offset 1384) (line 60) (column 17) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 1384) (line 60) (column 17) (len 9)))))
    (reference r16 (scope relative) (span (offset 1415) (line 62) (column 18) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 1415) (line 62) (column 18) (len 4)))))
    (reference r17 (scope relative) (span (offset 1438) (line 63) (column 18) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 1438) (line 63) (column 18) (len 4)))))
    (reference r18 (scope relative) (span (offset 1461) (line 64) (column 18) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 1461) (line 64) (column 18) (len 4)))))
    (reference r19 (scope relative) (span (offset 1487) (line 66) (column 18) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 1487) (line 66) (column 18) (len 4)))))
    (reference r20 (scope relative) (span (offset 1510) (line 67) (column 18) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 1510) (line 67) (column 18) (len 4)))))
    (reference r21 (scope relative) (span (offset 1533) (line 68) (column 18) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 1533) (line 68) (column 18) (len 4)))))
    (reference r22 (scope relative) (span (offset 1559) (line 70) (column 18) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 1559) (line 70) (column 18) (len 4)))))
    (reference r23 (scope relative) (span (offset 1582) (line 71) (column 18) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 1582) (line 71) (column 18) (len 4)))))
    (reference r24 (scope relative) (span (offset 1605) (line 72) (column 18) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 1605) (line 72) (column 18) (len 4)))))
    (reference r25 (scope relative) (span (offset 1635) (line 74) (column 22) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 1635) (line 74) (column 22) (len 4)))))
    (reference r26 (scope relative) (span (offset 1662) (line 75) (column 22) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 1662) (line 75) (column 22) (len 4)))))
    (reference r27 (scope relative) (span (offset 1689) (line 76) (column 22) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 1689) (line 76) (column 22) (len 4)))))
    (reference r28 (scope relative) (span (offset 1716) (line 78) (column 19) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 1716) (line 78) (column 19) (len 4)))))
    (reference r29 (scope relative) (span (offset 1740) (line 79) (column 19) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 1740) (line 79) (column 19) (len 4)))))
    (reference r30 (scope relative) (span (offset 1764) (line 80) (column 19) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 1764) (line 80) (column 19) (len 4)))))
    (reference r31 (scope relative) (span (offset 1831) (line 84) (column 18) (len 24)) (segments (segment 0 (token "'Individual Definitions'") (name "Individual Definitions") (separator none) (span (offset 1831) (line 84) (column 18) (len 24)))))
    (reference r32 (scope relative) (span (offset 1877) (line 85) (column 18) (len 6)) (segments (segment 0 (token "Values") (name "Values") (separator none) (span (offset 1877) (line 85) (column 18) (len 6)))))
    (reference r33 (scope relative) (span (offset 2184) (line 95) (column 9) (len 1)) (segments (segment 0 (token "t") (name "t") (separator none) (span (offset 2184) (line 95) (column 9) (len 1)))))
    (reference r34 (scope relative) (span (offset 2188) (line 95) (column 13) (len 2)) (segments (segment 0 (token "t0") (name "t0") (separator none) (span (offset 2188) (line 95) (column 13) (len 2)))))
    (reference r35 (scope relative) (span (offset 2491) (line 108) (column 10) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 2491) (line 108) (column 10) (len 4)))))
    (reference r36 (scope relative) (span (offset 2498) (line 108) (column 17) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 2498) (line 108) (column 17) (len 1)))))
    (reference r37 (scope relative) (span (offset 2510) (line 109) (column 10) (len 8)) (segments (segment 0 (token "position") (name "position") (separator none) (span (offset 2510) (line 109) (column 10) (len 8)))))
    (reference r38 (scope relative) (span (offset 2521) (line 109) (column 21) (len 2)) (segments (segment 0 (token "p0") (name "p0") (separator none) (span (offset 2521) (line 109) (column 21) (len 2)))))
    (reference r39 (scope relative) (span (offset 2534) (line 110) (column 10) (len 8)) (segments (segment 0 (token "velocity") (name "velocity") (separator none) (span (offset 2534) (line 110) (column 10) (len 8)))))
    (reference r40 (scope relative) (span (offset 2545) (line 110) (column 21) (len 2)) (segments (segment 0 (token "v0") (name "v0") (separator none) (span (offset 2545) (line 110) (column 21) (len 2)))))
    (reference r41 (scope relative) (span (offset 2558) (line 111) (column 10) (len 12)) (segments (segment 0 (token "acceleration") (name "acceleration") (separator none) (span (offset 2558) (line 111) (column 10) (len 12)))))
    (reference r42 (scope relative) (span (offset 2573) (line 111) (column 25) (len 2)) (segments (segment 0 (token "a0") (name "a0") (separator none) (span (offset 2573) (line 111) (column 25) (len 2)))))
    (reference r43 (scope relative) (span (offset 2869) (line 123) (column 10) (len 5)) (segments (segment 0 (token "angle") (name "angle") (separator none) (span (offset 2869) (line 123) (column 10) (len 5)))))
    (reference r44 (scope relative) (span (offset 2877) (line 123) (column 18) (len 6)) (segments (segment 0 (token "theta0") (name "theta0") (separator none) (span (offset 2877) (line 123) (column 18) (len 6)))))
    (reference r45 (scope relative) (span (offset 2894) (line 124) (column 10) (len 15)) (segments (segment 0 (token "surfaceFriction") (name "surfaceFriction") (separator none) (span (offset 2894) (line 124) (column 10) (len 15)))))
    (reference r46 (scope relative) (span (offset 2912) (line 124) (column 28) (len 3)) (segments (segment 0 (token "sf0") (name "sf0") (separator none) (span (offset 2912) (line 124) (column 28) (len 3)))))
    (reference r47 (scope relative) (span (offset 2990) (line 129) (column 9) (len 1)) (segments (segment 0 (token "t") (name "t") (separator none) (span (offset 2990) (line 129) (column 9) (len 1)))))
    (reference r48 (scope relative) (span (offset 2994) (line 129) (column 13) (len 2)) (segments (segment 0 (token "t1") (name "t1") (separator none) (span (offset 2994) (line 129) (column 13) (len 2)))))
    (reference r49 (scope relative) (span (offset 3057) (line 132) (column 10) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 3057) (line 132) (column 10) (len 4)))))
    (reference r50 (scope relative) (span (offset 3064) (line 132) (column 17) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 3064) (line 132) (column 17) (len 1)))))
    (reference r51 (scope relative) (span (offset 3076) (line 133) (column 10) (len 8)) (segments (segment 0 (token "position") (name "position") (separator none) (span (offset 3076) (line 133) (column 10) (len 8)))))
    (reference r52 (scope relative) (span (offset 3087) (line 133) (column 21) (len 2)) (segments (segment 0 (token "p1") (name "p1") (separator none) (span (offset 3087) (line 133) (column 21) (len 2)))))
    (reference r53 (scope relative) (span (offset 3100) (line 134) (column 10) (len 8)) (segments (segment 0 (token "velocity") (name "velocity") (separator none) (span (offset 3100) (line 134) (column 10) (len 8)))))
    (reference r54 (scope relative) (span (offset 3111) (line 134) (column 21) (len 2)) (segments (segment 0 (token "v1") (name "v1") (separator none) (span (offset 3111) (line 134) (column 21) (len 2)))))
    (reference r55 (scope relative) (span (offset 3124) (line 135) (column 10) (len 12)) (segments (segment 0 (token "acceleration") (name "acceleration") (separator none) (span (offset 3124) (line 135) (column 10) (len 12)))))
    (reference r56 (scope relative) (span (offset 3139) (line 135) (column 25) (len 2)) (segments (segment 0 (token "a1") (name "a1") (separator none) (span (offset 3139) (line 135) (column 25) (len 2)))))
    (reference r57 (scope relative) (span (offset 3238) (line 141) (column 10) (len 5)) (segments (segment 0 (token "angle") (name "angle") (separator none) (span (offset 3238) (line 141) (column 10) (len 5)))))
    (reference r58 (scope relative) (span (offset 3246) (line 141) (column 18) (len 6)) (segments (segment 0 (token "theta1") (name "theta1") (separator none) (span (offset 3246) (line 141) (column 18) (len 6)))))
    (reference r59 (scope relative) (span (offset 3263) (line 142) (column 10) (len 15)) (segments (segment 0 (token "surfaceFriction") (name "surfaceFriction") (separator none) (span (offset 3263) (line 142) (column 10) (len 15)))))
    (reference r60 (scope relative) (span (offset 3281) (line 142) (column 28) (len 3)) (segments (segment 0 (token "sf1") (name "sf1") (separator none) (span (offset 3281) (line 142) (column 28) (len 3)))))
    (reference r61 (scope relative) (span (offset 3373) (line 149) (column 9) (len 1)) (segments (segment 0 (token "t") (name "t") (separator none) (span (offset 3373) (line 149) (column 9) (len 1)))))
    (reference r62 (scope relative) (span (offset 3377) (line 149) (column 13) (len 2)) (segments (segment 0 (token "tn") (name "tn") (separator none) (span (offset 3377) (line 149) (column 13) (len 2)))))
    (reference r63 (scope relative) (span (offset 3440) (line 152) (column 10) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 3440) (line 152) (column 10) (len 4)))))
    (reference r64 (scope relative) (span (offset 3447) (line 152) (column 17) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 3447) (line 152) (column 17) (len 1)))))
    (reference r65 (scope relative) (span (offset 3459) (line 153) (column 10) (len 8)) (segments (segment 0 (token "position") (name "position") (separator none) (span (offset 3459) (line 153) (column 10) (len 8)))))
    (reference r66 (scope relative) (span (offset 3470) (line 153) (column 21) (len 2)) (segments (segment 0 (token "pn") (name "pn") (separator none) (span (offset 3470) (line 153) (column 21) (len 2)))))
    (reference r67 (scope relative) (span (offset 3483) (line 154) (column 10) (len 8)) (segments (segment 0 (token "velocity") (name "velocity") (separator none) (span (offset 3483) (line 154) (column 10) (len 8)))))
    (reference r68 (scope relative) (span (offset 3494) (line 154) (column 21) (len 2)) (segments (segment 0 (token "vn") (name "vn") (separator none) (span (offset 3494) (line 154) (column 21) (len 2)))))
    (reference r69 (scope relative) (span (offset 3507) (line 155) (column 10) (len 12)) (segments (segment 0 (token "acceleration") (name "acceleration") (separator none) (span (offset 3507) (line 155) (column 10) (len 12)))))
    (reference r70 (scope relative) (span (offset 3522) (line 155) (column 25) (len 2)) (segments (segment 0 (token "an") (name "an") (separator none) (span (offset 3522) (line 155) (column 25) (len 2)))))
    (reference r71 (scope relative) (span (offset 3622) (line 161) (column 10) (len 5)) (segments (segment 0 (token "angle") (name "angle") (separator none) (span (offset 3622) (line 161) (column 10) (len 5)))))
    (reference r72 (scope relative) (span (offset 3630) (line 161) (column 18) (len 6)) (segments (segment 0 (token "theta1") (name "theta1") (separator none) (span (offset 3630) (line 161) (column 18) (len 6)))))
    (reference r73 (scope relative) (span (offset 3647) (line 162) (column 10) (len 15)) (segments (segment 0 (token "surfaceFriction") (name "surfaceFriction") (separator none) (span (offset 3647) (line 162) (column 10) (len 15)))))
    (reference r74 (scope relative) (span (offset 3665) (line 162) (column 28) (len 3)) (segments (segment 0 (token "sfn") (name "sfn") (separator none) (span (offset 3665) (line 162) (column 28) (len 3)))))
  )
  (root (package (name "6-Individual and Snapshots") (body brace (import (target (span (span (offset 55) (line 2) (column 17) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 91) (line 3) (column 17) (len 14))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 123) (line 4) (column 17) (len 6))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 126) (line 4) (column 20) (len 3))) (separator (span (offset 126) (line 4) (column 20) (len 2))) (marker (span (offset 128) (line 4) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "Part Definitions") (body brace (part-def (name "Temporal-Spatial Reference") (body brace (attribute-usage (declaration-name "referenceTime") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "referenceCoordinateSystem") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "VehicleRoadContext") (body brace (attribute-usage (declaration-name "t") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "VehicleA") (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "position") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "velocity") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "acceleration") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (exhibit (declaration "vehicleStates") (state none)))) (part-def (name "Road") (body brace (attribute-usage (declaration-name "angle") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "surfaceFriction") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))) (package (name "Individual Definitions") (body brace (import (target (span (span (offset 859) (line 41) (column 18) (len 21))) (all none) (ref r11) (shape (namespace (wildcard-suffix (span (span (offset 877) (line 41) (column 36) (len 3))) (separator (span (offset 877) (line 41) (column 36) (len 2))) (marker (span (offset 879) (line 41) (column 38) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (individual-def) (individual-def) (individual-def) (individual-def))) (package (name "Values") (body brace (attribute-def (declaration-name "t0") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "t1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "tn") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r14)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "m") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "p0") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "p1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r17)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "pn") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r18)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "v0") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r19)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "v1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r20)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "vn") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r21)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "a0") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r22)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "a1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r23)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "an") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r24)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "theta0") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r25)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "theta1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r26)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "thetan") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r27)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "sf0") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r28)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "sf1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r29)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "sfn") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r30)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)))) (package (name "Individuals and Snapshots") (body brace (import (target (span (span (offset 1831) (line 84) (column 18) (len 27))) (all none) (ref r31) (shape (namespace (wildcard-suffix (span (span (offset 1855) (line 84) (column 42) (len 3))) (separator (span (offset 1855) (line 84) (column 42) (len 2))) (marker (span (offset 1857) (line 84) (column 44) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 1877) (line 85) (column 18) (len 9))) (all none) (ref r32) (shape (namespace (wildcard-suffix (span (span (offset 1883) (line 85) (column 24) (len 3))) (separator (span (offset 1883) (line 85) (column 24) (len 2))) (marker (span (offset 1885) (line 85) (column 26) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion none) (extensions)) (declaration "reference") (short-name none) (target none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration "context_t0") (short-name none) (target none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r33)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2188) (line 95) (column 13) (len 2)) (ref r34))))) (body brace)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration "vehicle_ID1_t0") (short-name none) (target none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r35)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2498) (line 108) (column 17) (len 1)) (ref r36))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r37)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2521) (line 109) (column 21) (len 2)) (ref r38))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r39)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2545) (line 110) (column 21) (len 2)) (ref r40))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r41)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2573) (line 111) (column 25) (len 2)) (ref r42))))) (body semicolon)) (state-usage))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration "road_ID1_t0") (short-name none) (target none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r43)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2877) (line 123) (column 18) (len 6)) (ref r44))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r45)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2912) (line 124) (column 28) (len 3)) (ref r46))))) (body semicolon)))))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration "context_t1") (short-name none) (target none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r47)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2994) (line 129) (column 13) (len 2)) (ref r48))))) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration "vehicle_ID1_t1") (short-name none) (target none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r49)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3064) (line 132) (column 17) (len 1)) (ref r50))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r51)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3087) (line 133) (column 21) (len 2)) (ref r52))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r53)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3111) (line 134) (column 21) (len 2)) (ref r54))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r55)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3139) (line 135) (column 25) (len 2)) (ref r56))))) (body semicolon)) (state-usage))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration "road_ID1_t1") (short-name none) (target none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r57)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3246) (line 141) (column 18) (len 6)) (ref r58))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r59)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3281) (line 142) (column 28) (len 3)) (ref r60))))) (body semicolon)))))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration "context_tn") (short-name none) (target none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r61)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3377) (line 149) (column 13) (len 2)) (ref r62))))) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration "vehicle_ID1_tn") (short-name none) (target none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r63)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3447) (line 152) (column 17) (len 1)) (ref r64))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r65)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3470) (line 153) (column 21) (len 2)) (ref r66))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r67)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3494) (line 154) (column 21) (len 2)) (ref r68))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r69)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3522) (line 155) (column 25) (len 2)) (ref r70))))) (body semicolon)) (state-usage))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration "road_ID1_tn") (short-name none) (target none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r71)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3630) (line 161) (column 18) (len 6)) (ref r72))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r73)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3665) (line 162) (column 28) (len 3)) (ref r74))))) (body semicolon)))))))))))))
)
~~~
