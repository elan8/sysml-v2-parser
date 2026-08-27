# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Vehicle): VehicleUsages"))
~~~
# SOURCE
~~~sysml
package VehicleUsages {
	doc
	/*
	 * Example usages of elements from the vehicle definitions model.
	 */

	private import SI::N;
	private import SI::m;
	private import ScalarFunctions::*;

	public import VehicleDefinitions::*;

	/* VALUES */	 
	T1 = 10.0 [N * m];
	T2 = 20.0 [N * m];
	
	/* PARTS */	
	part narrowRimWheel: Wheel {
		doc /* Narrow-rim wheel configuration with 4 to 5 lugbolts. */

		part lugbolt: Lugbolt[4..5];
	}
	
	part wideRimWheel: Wheel {
		doc /* Wide-rim wheel configuration with 4 to 6 lugbolts. */	

		part lugbolt: Lugbolt[4..6];
	}

	part vehicle_C1: Vehicle {
		doc /* Basic Vehicle configuration showing a part hierarchy. */

		part frontAxleAssembly: AxleAssembly {
			part frontWheel[2] subsets narrowRimWheel {
				part redefines lugbolt[4] {
					attribute redefines tighteningTorque = T1;
				}
			}
			part frontAxle: Axle;
		}		
		part rearAxleAssembly: AxleAssembly {
			part rearWheel[2] subsets wideRimWheel {
				part redefines lugbolt[6] {
					attribute redefines tighteningTorque = T2;
				}
			}
			part rearAxle: Axle;			
		}
	}
	
	part vehicle_C2 subsets vehicle_C1 {
		doc /* Specialized configuration with part-specific ports. */

		part redefines frontAxleAssembly {
			part leftFrontWheel subsets frontWheel = frontWheel#(1);
			part rightFrontWheel subsets frontWheel = frontWheel#(2);
			
			interface leftFrontMount: Mounting connect 
				frontAxle.leftMountingPoint to leftFrontWheel.hub;
				
			interface rightFrontMount: Mounting connect 
				frontAxle.rightMountingPoint to rightFrontWheel.hub;
		}
		
		part rearAxleAssembly redefines vehicle_C1::rearAxleAssembly {
			part leftRearWheel subsets rearWheel = rearWheel#(1);
			part rightRearWheel subsets rearWheel = rearWheel#(2);

			interface leftRearMount: Mounting connect 
				rearAxle.leftMountingPoint to leftRearWheel.hub;
				
			interface rightRearMount: Mounting connect 
				rearAxle.rightMountingPoint to rightRearWheel.hub;
		}		
	}
	
	part vehicle_C3 subsets vehicle_C2 {
		doc /* Further specialized configuration with a connection to a deeply-nested port. */

		
		part transmission: Transmission {
			port drive: ~DriveIF;
		}
		
		part redefines rearAxleAssembly {
			part redefines rearAxle {
				port drive: DriveIF;
			}
		}
		
		interface driveShaft connect 
			transDrive ::> transmission.drive to axleDrive ::> rearAxleAssembly.rearAxle.drive {
			flow transDrive.driveTorque to axleDrive.driveTorque;
		}		
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "vehicle_usages.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package VehicleUsages {
    doc
    /*
	 * Example usages of elements from the vehicle definitions model.
	 */
    private import SI::N;
    private import SI::m;
    private import ScalarFunctions::*;
    public import VehicleDefinitions::*;
    /* VALUES */
    T1 = 10.0[N * m];
    T2 = 20.0[N * m];
    /* PARTS */
    part narrowRimWheel : Wheel {
        doc
        /* Narrow-rim wheel configuration with 4 to 5 lugbolts. */
        part lugbolt : Lugbolt[4..5];
    }
    part wideRimWheel : Wheel {
        doc
        /* Wide-rim wheel configuration with 4 to 6 lugbolts. */
        part lugbolt : Lugbolt[4..6];
    }
    part vehicle_C1 : Vehicle {
        doc
        /* Basic Vehicle configuration showing a part hierarchy. */
        part frontAxleAssembly : AxleAssembly {
            part frontWheel[2] subsets narrowRimWheel {
                part redefines lugbolt[4] {
                    attribute redefines tighteningTorque = T1;
                }
            }
            part frontAxle : Axle;
        }
        part rearAxleAssembly : AxleAssembly {
            part rearWheel[2] subsets wideRimWheel {
                part redefines lugbolt[6] {
                    attribute redefines tighteningTorque = T2;
                }
            }
            part rearAxle : Axle;
        }
    }
    part vehicle_C2 subsets vehicle_C1 {
        doc
        /* Specialized configuration with part-specific ports. */
        part redefines frontAxleAssembly {
            part leftFrontWheel subsets frontWheel = frontWheel#(1);
            part rightFrontWheel subsets frontWheel = frontWheel#(2);
            interface leftFrontMount : Mounting connect frontAxle.leftMountingPoint to leftFrontWheel.hub;
            interface rightFrontMount : Mounting connect frontAxle.rightMountingPoint to rightFrontWheel.hub;
        }
        part rearAxleAssembly redefines vehicle_C1::rearAxleAssembly {
            part leftRearWheel subsets rearWheel = rearWheel#(1);
            part rightRearWheel subsets rearWheel = rearWheel#(2);
            interface leftRearMount : Mounting connect rearAxle.leftMountingPoint to leftRearWheel.hub;
            interface rightRearMount : Mounting connect rearAxle.rightMountingPoint to rightRearWheel.hub;
        }
    }
    part vehicle_C3 subsets vehicle_C2 {
        doc
        /* Further specialized configuration with a connection to a deeply-nested port. */
        part transmission : Transmission {
            port drive : ~DriveIF;
        }
        part redefines rearAxleAssembly {
            part redefines rearAxle {
                port drive : DriveIF;
            }
        }
        interface driveShaft connect transDrive ::> transmission.drive to axleDrive ::> rearAxleAssembly.rearAxle.drive {
            flow from transDrive.driveTorque to axleDrive.driveTorque;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 122) (line 7) (column 17) (len 5)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 122) (line 7) (column 17) (len 2))) (segment 1 (token "N") (name "N") (separator colon-colon) (span (offset 126) (line 7) (column 21) (len 1)))))
    (reference r1 (scope relative) (span (offset 145) (line 8) (column 17) (len 5)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 145) (line 8) (column 17) (len 2))) (segment 1 (token "m") (name "m") (separator colon-colon) (span (offset 149) (line 8) (column 21) (len 1)))))
    (reference r2 (scope relative) (span (offset 168) (line 9) (column 17) (len 15)) (segments (segment 0 (token "ScalarFunctions") (name "ScalarFunctions") (separator none) (span (offset 168) (line 9) (column 17) (len 15)))))
    (reference r3 (scope relative) (span (offset 204) (line 11) (column 16) (len 18)) (segments (segment 0 (token "VehicleDefinitions") (name "VehicleDefinitions") (separator none) (span (offset 204) (line 11) (column 16) (len 18)))))
    (reference r4 (scope relative) (span (offset 256) (line 14) (column 13) (len 1)) (segments (segment 0 (token "N") (name "N") (separator none) (span (offset 256) (line 14) (column 13) (len 1)))))
    (reference r5 (scope relative) (span (offset 260) (line 14) (column 17) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 260) (line 14) (column 17) (len 1)))))
    (reference r6 (scope relative) (span (offset 276) (line 15) (column 13) (len 1)) (segments (segment 0 (token "N") (name "N") (separator none) (span (offset 276) (line 15) (column 13) (len 1)))))
    (reference r7 (scope relative) (span (offset 280) (line 15) (column 17) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 280) (line 15) (column 17) (len 1)))))
    (reference r8 (scope relative) (span (offset 322) (line 18) (column 23) (len 5)) (segments (segment 0 (token "Wheel") (name "Wheel") (separator none) (span (offset 322) (line 18) (column 23) (len 5)))))
    (reference r9 (scope relative) (span (offset 412) (line 21) (column 17) (len 7)) (segments (segment 0 (token "Lugbolt") (name "Lugbolt") (separator none) (span (offset 412) (line 21) (column 17) (len 7)))))
    (reference r10 (scope relative) (span (offset 452) (line 24) (column 21) (len 5)) (segments (segment 0 (token "Wheel") (name "Wheel") (separator none) (span (offset 452) (line 24) (column 21) (len 5)))))
    (reference r11 (scope relative) (span (offset 541) (line 27) (column 17) (len 7)) (segments (segment 0 (token "Lugbolt") (name "Lugbolt") (separator none) (span (offset 541) (line 27) (column 17) (len 7)))))
    (reference r12 (scope relative) (span (offset 578) (line 30) (column 19) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 578) (line 30) (column 19) (len 7)))))
    (reference r13 (scope relative) (span (offset 681) (line 33) (column 27) (len 12)) (segments (segment 0 (token "AxleAssembly") (name "AxleAssembly") (separator none) (span (offset 681) (line 33) (column 27) (len 12)))))
    (reference r14 (scope relative) (span (offset 726) (line 34) (column 31) (len 14)) (segments (segment 0 (token "narrowRimWheel") (name "narrowRimWheel") (separator none) (span (offset 726) (line 34) (column 31) (len 14)))))
    (reference r15 (scope relative) (span (offset 762) (line 35) (column 20) (len 7)) (segments (segment 0 (token "lugbolt") (name "lugbolt") (separator none) (span (offset 762) (line 35) (column 20) (len 7)))))
    (reference r16 (scope relative) (span (offset 800) (line 36) (column 26) (len 16)) (segments (segment 0 (token "tighteningTorque") (name "tighteningTorque") (separator none) (span (offset 800) (line 36) (column 26) (len 16)))))
    (reference r17 (scope relative) (span (offset 819) (line 36) (column 45) (len 2)) (segments (segment 0 (token "T1") (name "T1") (separator none) (span (offset 819) (line 36) (column 45) (len 2)))))
    (reference r18 (scope relative) (span (offset 853) (line 39) (column 20) (len 4)) (segments (segment 0 (token "Axle") (name "Axle") (separator none) (span (offset 853) (line 39) (column 20) (len 4)))))
    (reference r19 (scope relative) (span (offset 890) (line 41) (column 26) (len 12)) (segments (segment 0 (token "AxleAssembly") (name "AxleAssembly") (separator none) (span (offset 890) (line 41) (column 26) (len 12)))))
    (reference r20 (scope relative) (span (offset 934) (line 42) (column 30) (len 12)) (segments (segment 0 (token "wideRimWheel") (name "wideRimWheel") (separator none) (span (offset 934) (line 42) (column 30) (len 12)))))
    (reference r21 (scope relative) (span (offset 968) (line 43) (column 20) (len 7)) (segments (segment 0 (token "lugbolt") (name "lugbolt") (separator none) (span (offset 968) (line 43) (column 20) (len 7)))))
    (reference r22 (scope relative) (span (offset 1006) (line 44) (column 26) (len 16)) (segments (segment 0 (token "tighteningTorque") (name "tighteningTorque") (separator none) (span (offset 1006) (line 44) (column 26) (len 16)))))
    (reference r23 (scope relative) (span (offset 1025) (line 44) (column 45) (len 2)) (segments (segment 0 (token "T2") (name "T2") (separator none) (span (offset 1025) (line 44) (column 45) (len 2)))))
    (reference r24 (scope relative) (span (offset 1058) (line 47) (column 19) (len 4)) (segments (segment 0 (token "Axle") (name "Axle") (separator none) (span (offset 1058) (line 47) (column 19) (len 4)))))
    (reference r25 (scope relative) (span (offset 1101) (line 51) (column 26) (len 10)) (segments (segment 0 (token "vehicle_C1") (name "vehicle_C1") (separator none) (span (offset 1101) (line 51) (column 26) (len 10)))))
    (reference r26 (scope relative) (span (offset 1196) (line 54) (column 18) (len 17)) (segments (segment 0 (token "frontAxleAssembly") (name "frontAxleAssembly") (separator none) (span (offset 1196) (line 54) (column 18) (len 17)))))
    (reference r27 (scope relative) (span (offset 1247) (line 55) (column 32) (len 10)) (segments (segment 0 (token "frontWheel") (name "frontWheel") (separator none) (span (offset 1247) (line 55) (column 32) (len 10)))))
    (reference r28 (scope relative) (span (offset 1260) (line 55) (column 45) (len 10)) (segments (segment 0 (token "frontWheel") (name "frontWheel") (separator none) (span (offset 1260) (line 55) (column 45) (len 10)))))
    (reference r29 (scope relative) (span (offset 1308) (line 56) (column 33) (len 10)) (segments (segment 0 (token "frontWheel") (name "frontWheel") (separator none) (span (offset 1308) (line 56) (column 33) (len 10)))))
    (reference r30 (scope relative) (span (offset 1321) (line 56) (column 46) (len 10)) (segments (segment 0 (token "frontWheel") (name "frontWheel") (separator none) (span (offset 1321) (line 56) (column 46) (len 10)))))
    (reference r31 (scope relative) (span (offset 1392) (line 59) (column 5) (len 27)) (segments (segment 0 (token "frontAxle") (name "frontAxle") (separator none) (span (offset 1392) (line 59) (column 5) (len 9))) (segment 1 (token "leftMountingPoint") (name "leftMountingPoint") (separator dot) (span (offset 1402) (line 59) (column 15) (len 17)))))
    (reference r32 (scope relative) (span (offset 1423) (line 59) (column 36) (len 18)) (segments (segment 0 (token "leftFrontWheel") (name "leftFrontWheel") (separator none) (span (offset 1423) (line 59) (column 36) (len 14))) (segment 1 (token "hub") (name "hub") (separator dot) (span (offset 1438) (line 59) (column 51) (len 3)))))
    (reference r33 (scope relative) (span (offset 1500) (line 62) (column 5) (len 28)) (segments (segment 0 (token "frontAxle") (name "frontAxle") (separator none) (span (offset 1500) (line 62) (column 5) (len 9))) (segment 1 (token "rightMountingPoint") (name "rightMountingPoint") (separator dot) (span (offset 1510) (line 62) (column 15) (len 18)))))
    (reference r34 (scope relative) (span (offset 1532) (line 62) (column 37) (len 19)) (segments (segment 0 (token "rightFrontWheel") (name "rightFrontWheel") (separator none) (span (offset 1532) (line 62) (column 37) (len 15))) (segment 1 (token "hub") (name "hub") (separator dot) (span (offset 1548) (line 62) (column 53) (len 3)))))
    (reference r35 (scope relative) (span (offset 1594) (line 65) (column 35) (len 28)) (segments (segment 0 (token "vehicle_C1") (name "vehicle_C1") (separator none) (span (offset 1594) (line 65) (column 35) (len 10))) (segment 1 (token "rearAxleAssembly") (name "rearAxleAssembly") (separator colon-colon) (span (offset 1606) (line 65) (column 47) (len 16)))))
    (reference r36 (scope relative) (span (offset 1655) (line 66) (column 31) (len 9)) (segments (segment 0 (token "rearWheel") (name "rearWheel") (separator none) (span (offset 1655) (line 66) (column 31) (len 9)))))
    (reference r37 (scope relative) (span (offset 1667) (line 66) (column 43) (len 9)) (segments (segment 0 (token "rearWheel") (name "rearWheel") (separator none) (span (offset 1667) (line 66) (column 43) (len 9)))))
    (reference r38 (scope relative) (span (offset 1713) (line 67) (column 32) (len 9)) (segments (segment 0 (token "rearWheel") (name "rearWheel") (separator none) (span (offset 1713) (line 67) (column 32) (len 9)))))
    (reference r39 (scope relative) (span (offset 1725) (line 67) (column 44) (len 9)) (segments (segment 0 (token "rearWheel") (name "rearWheel") (separator none) (span (offset 1725) (line 67) (column 44) (len 9)))))
    (reference r40 (scope relative) (span (offset 1791) (line 70) (column 5) (len 26)) (segments (segment 0 (token "rearAxle") (name "rearAxle") (separator none) (span (offset 1791) (line 70) (column 5) (len 8))) (segment 1 (token "leftMountingPoint") (name "leftMountingPoint") (separator dot) (span (offset 1800) (line 70) (column 14) (len 17)))))
    (reference r41 (scope relative) (span (offset 1821) (line 70) (column 35) (len 17)) (segments (segment 0 (token "leftRearWheel") (name "leftRearWheel") (separator none) (span (offset 1821) (line 70) (column 35) (len 13))) (segment 1 (token "hub") (name "hub") (separator dot) (span (offset 1835) (line 70) (column 49) (len 3)))))
    (reference r42 (scope relative) (span (offset 1896) (line 73) (column 5) (len 27)) (segments (segment 0 (token "rearAxle") (name "rearAxle") (separator none) (span (offset 1896) (line 73) (column 5) (len 8))) (segment 1 (token "rightMountingPoint") (name "rightMountingPoint") (separator dot) (span (offset 1905) (line 73) (column 14) (len 18)))))
    (reference r43 (scope relative) (span (offset 1927) (line 73) (column 36) (len 18)) (segments (segment 0 (token "rightRearWheel") (name "rightRearWheel") (separator none) (span (offset 1927) (line 73) (column 36) (len 14))) (segment 1 (token "hub") (name "hub") (separator dot) (span (offset 1942) (line 73) (column 51) (len 3)))))
    (reference r44 (scope relative) (span (offset 1983) (line 77) (column 26) (len 10)) (segments (segment 0 (token "vehicle_C2") (name "vehicle_C2") (separator none) (span (offset 1983) (line 77) (column 26) (len 10)))))
    (reference r45 (scope relative) (span (offset 2110) (line 81) (column 22) (len 12)) (segments (segment 0 (token "Transmission") (name "Transmission") (separator none) (span (offset 2110) (line 81) (column 22) (len 12)))))
    (reference r46 (scope relative) (span (offset 2141) (line 82) (column 17) (len 7)) (segments (segment 0 (token "DriveIF") (name "DriveIF") (separator none) (span (offset 2141) (line 82) (column 17) (len 7)))))
    (reference r47 (scope relative) (span (offset 2174) (line 85) (column 18) (len 16)) (segments (segment 0 (token "rearAxleAssembly") (name "rearAxleAssembly") (separator none) (span (offset 2174) (line 85) (column 18) (len 16)))))
    (reference r48 (scope relative) (span (offset 2211) (line 86) (column 19) (len 8)) (segments (segment 0 (token "rearAxle") (name "rearAxle") (separator none) (span (offset 2211) (line 86) (column 19) (len 8)))))
    (reference r49 (scope relative) (span (offset 2238) (line 87) (column 17) (len 7)) (segments (segment 0 (token "DriveIF") (name "DriveIF") (separator none) (span (offset 2238) (line 87) (column 17) (len 7)))))
    (reference r50 (scope relative) (span (offset 2309) (line 92) (column 19) (len 18)) (segments (segment 0 (token "transmission") (name "transmission") (separator none) (span (offset 2309) (line 92) (column 19) (len 12))) (segment 1 (token "drive") (name "drive") (separator dot) (span (offset 2322) (line 92) (column 32) (len 5)))))
    (reference r51 (scope relative) (span (offset 2345) (line 92) (column 55) (len 31)) (segments (segment 0 (token "rearAxleAssembly") (name "rearAxleAssembly") (separator none) (span (offset 2345) (line 92) (column 55) (len 16))) (segment 1 (token "rearAxle") (name "rearAxle") (separator dot) (span (offset 2362) (line 92) (column 72) (len 8))) (segment 2 (token "drive") (name "drive") (separator dot) (span (offset 2371) (line 92) (column 81) (len 5)))))
    (reference r52 (scope relative) (span (offset 2387) (line 93) (column 9) (len 22)) (segments (segment 0 (token "transDrive") (name "transDrive") (separator none) (span (offset 2387) (line 93) (column 9) (len 10))) (segment 1 (token "driveTorque") (name "driveTorque") (separator dot) (span (offset 2398) (line 93) (column 20) (len 11)))))
    (reference r53 (scope relative) (span (offset 2413) (line 93) (column 35) (len 21)) (segments (segment 0 (token "axleDrive") (name "axleDrive") (separator none) (span (offset 2413) (line 93) (column 35) (len 9))) (segment 1 (token "driveTorque") (name "driveTorque") (separator dot) (span (offset 2423) (line 93) (column 45) (len 11)))))
  )
  (root (package (name "VehicleUsages") (body brace (doc (name none) (locale none) (body (span (offset 32) (line 3) (column 4) (len 70)) (normalized "Example usages of elements from the vehicle definitions model.\n"))) (import (target (span (span (offset 122) (line 7) (column 17) (len 5))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 145) (line 8) (column 17) (len 5))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 168) (line 9) (column 17) (len 18))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 183) (line 9) (column 32) (len 3))) (separator (span (offset 183) (line 9) (column 32) (len 2))) (marker (span (offset 185) (line 9) (column 34) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 204) (line 11) (column 16) (len 21))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 222) (line 11) (column 34) (len 3))) (separator (span (offset 222) (line 11) (column 34) (len 2))) (marker (span (offset 224) (line 11) (column 36) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 231) (line 13) (column 4) (len 8)) (normalized "VALUES "))) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "T1") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 250) (line 14) (column 7) (len 12)) (bracket (base (expression (span (offset 250) (line 14) (column 7) (len 4)) (real "10.0"))) (operands (sequence-list (element first (expression (span (offset 256) (line 14) (column 13) (len 5)) (binary (operator "*") (left (expression (span (offset 256) (line 14) (column 13) (len 1)) (ref r4))) (right (expression (span (offset 260) (line 14) (column 17) (len 1)) (ref r5))))))))))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "T2") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 270) (line 15) (column 7) (len 12)) (bracket (base (expression (span (offset 270) (line 15) (column 7) (len 4)) (real "20.0"))) (operands (sequence-list (element first (expression (span (offset 276) (line 15) (column 13) (len 5)) (binary (operator "*") (left (expression (span (offset 276) (line 15) (column 13) (len 1)) (ref r6))) (right (expression (span (offset 280) (line 15) (column 17) (len 1)) (ref r7))))))))))))) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 289) (line 17) (column 4) (len 7)) (normalized "PARTS "))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "narrowRimWheel") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (doc (name none) (locale none) (body (span (offset 338) (line 19) (column 9) (len 54)) (normalized "Narrow-rim wheel configuration with 4 to 5 lugbolts. "))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "lugbolt") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity (lower (expression (span (offset 420) (line 21) (column 25) (len 1)) (integer 4))) (upper (expression (span (offset 423) (line 21) (column 28) (len 1)) (integer 5)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "wideRimWheel") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (doc (name none) (locale none) (body (span (offset 468) (line 25) (column 9) (len 52)) (normalized "Wide-rim wheel configuration with 4 to 6 lugbolts. "))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "lugbolt") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (multiplicity (lower (expression (span (offset 549) (line 27) (column 25) (len 1)) (integer 4))) (upper (expression (span (offset 552) (line 27) (column 28) (len 1)) (integer 6)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle_C1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (doc (name none) (locale none) (body (span (offset 596) (line 31) (column 9) (len 55)) (normalized "Basic Vehicle configuration showing a part hierarchy. "))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "frontAxleAssembly") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "frontWheel") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 715) (line 34) (column 20) (len 1)) (integer 2))) (upper (expression (span (offset 715) (line 34) (column 20) (len 1)) (integer 2)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r14))) (value none))) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity (lower (expression (span (offset 770) (line 35) (column 28) (len 1)) (integer 4))) (upper (expression (span (offset 770) (line 35) (column 28) (len 1)) (integer 4)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r15)))) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r16)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 819) (line 36) (column 45) (len 2)) (ref r17))))) (body semicolon)))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "frontAxle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r18)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearAxleAssembly") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r19)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearWheel") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 923) (line 42) (column 19) (len 1)) (integer 2))) (upper (expression (span (offset 923) (line 42) (column 19) (len 1)) (integer 2)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r20))) (value none))) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity (lower (expression (span (offset 976) (line 43) (column 28) (len 1)) (integer 6))) (upper (expression (span (offset 976) (line 43) (column 28) (len 1)) (integer 6)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r21)))) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r22)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1025) (line 44) (column 45) (len 2)) (ref r23))))) (body semicolon)))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearAxle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r24)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle_C2") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r25))) (value none))) (redefines none) (value none) (body brace (doc (name none) (locale none) (body (span (offset 1122) (line 52) (column 9) (len 53)) (normalized "Specialized configuration with part-specific ports. "))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r26)))) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "leftFrontWheel") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r27))) (value (expression (span (offset 1260) (line 55) (column 45) (len 14)) (index (base (expression (span (offset 1260) (line 55) (column 45) (len 10)) (ref r28))) (operands (sequence-list (element first (expression (span (offset 1272) (line 55) (column 57) (len 1)) (integer 1)))))))))) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rightFrontWheel") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r29))) (value (expression (span (offset 1321) (line 56) (column 46) (len 14)) (index (base (expression (span (offset 1321) (line 56) (column 46) (len 10)) (ref r30))) (operands (sequence-list (element first (expression (span (offset 1333) (line 56) (column 58) (len 1)) (integer 2)))))))))) (redefines none) (value none) (body semicolon)) (interface-usage (form typed-connect) (part (binary (from (interface-end (multiplicity none) (target (ref r31)))) (to (interface-end (multiplicity none) (target (ref r32)))))) (body semicolon)) (interface-usage (form typed-connect) (part (binary (from (interface-end (multiplicity none) (target (ref r33)))) (to (interface-end (multiplicity none) (target (ref r34)))))) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearAxleAssembly") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r35)))) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "leftRearWheel") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r36))) (value (expression (span (offset 1667) (line 66) (column 43) (len 13)) (index (base (expression (span (offset 1667) (line 66) (column 43) (len 9)) (ref r37))) (operands (sequence-list (element first (expression (span (offset 1678) (line 66) (column 54) (len 1)) (integer 1)))))))))) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rightRearWheel") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r38))) (value (expression (span (offset 1725) (line 67) (column 44) (len 13)) (index (base (expression (span (offset 1725) (line 67) (column 44) (len 9)) (ref r39))) (operands (sequence-list (element first (expression (span (offset 1736) (line 67) (column 55) (len 1)) (integer 2)))))))))) (redefines none) (value none) (body semicolon)) (interface-usage (form typed-connect) (part (binary (from (interface-end (multiplicity none) (target (ref r40)))) (to (interface-end (multiplicity none) (target (ref r41)))))) (body semicolon)) (interface-usage (form typed-connect) (part (binary (from (interface-end (multiplicity none) (target (ref r42)))) (to (interface-end (multiplicity none) (target (ref r43)))))) (body semicolon)))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle_C3") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r44))) (value none))) (redefines none) (value none) (body brace (doc (name none) (locale none) (body (span (offset 2004) (line 78) (column 9) (len 78)) (normalized "Further specialized configuration with a connection to a deeply-nested port. "))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "transmission") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r45)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "drive") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r46)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r47)))) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r48)))) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "drive") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r49)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))) (interface-usage (form typed-connect) (part (binary (from (interface-end (multiplicity none) (target (named (name "transDrive") (references symbol) (target (ref r50)))))) (to (interface-end (multiplicity none) (target (named (name "axleDrive") (references symbol) (target (ref r51)))))))) (body brace (flow-usage (kind flow) (visibility none) (declaration (endpoint-only (from (connector-end (multiplicity none) (target (ref r52)) (references none))) (to (connector-end (multiplicity none) (target (ref r53)) (references none))))) (body (body semicolon))))))))))
)
~~~
