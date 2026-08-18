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
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 245) (line 14) (column 2) (len 56)) (message "unrecognized declaration `T1` in package body"))
      (diagnostic (code "recovered_part_usage_body_element") (severity error) (category parseerror) (span (offset 2261) (line 91) (column 3) (len 182)) (message "unexpected token in part usage body"))
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
    T1 = 10.0 [N * m];
	T2 = 20.0 [N * m];
	
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
            part frontWheel[2] :> narrowRimWheel {
                part :>> lugbolt[4] {
                    attribute :>> tighteningTorque = T1;
                }
            }
            part frontAxle : Axle;
        }
        part rearAxleAssembly : AxleAssembly {
            part rearWheel[2] :> wideRimWheel {
                part :>> lugbolt[6] {
                    attribute :>> tighteningTorque = T2;
                }
            }
            part rearAxle : Axle;
        }
    }
    part vehicle_C2 :> vehicle_C1 {
        doc
        /* Specialized configuration with part-specific ports. */
        part :>> frontAxleAssembly {
            part leftFrontWheel :> frontWheel = frontWheel#(1);
            part rightFrontWheel :> frontWheel = frontWheel#(2);
            interface leftFrontMount : Mounting connect frontAxle.leftMountingPoint to leftFrontWheel.hub;
            interface rightFrontMount : Mounting connect frontAxle.rightMountingPoint to rightFrontWheel.hub;
        }
        part rearAxleAssembly :>> vehicle_C1::rearAxleAssembly {
            part leftRearWheel :> rearWheel = rearWheel#(1);
            part rightRearWheel :> rearWheel = rearWheel#(2);
            interface leftRearMount : Mounting connect rearAxle.leftMountingPoint to leftRearWheel.hub;
            interface rightRearMount : Mounting connect rearAxle.rightMountingPoint to rightRearWheel.hub;
        }
    }
    part vehicle_C3 :> vehicle_C2 {
        doc
        /* Further specialized configuration with a connection to a deeply-nested port. */
        part transmission : Transmission {
            port drive : ~DriveIF;
        }
        part :>> rearAxleAssembly {
            part :>> rearAxle {
                port drive : DriveIF;
            }
        }
        interface driveShaft connect 
			transDrive ::> transmission.drive to axleDrive ::> rearAxleAssembly.rearAxle.drive {
			flow transDrive.driveTorque to axleDrive.driveTorque;
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
    (reference r4 (scope relative) (span (offset 322) (line 18) (column 23) (len 5)) (segments (segment 0 (token "Wheel") (name "Wheel") (separator none) (span (offset 322) (line 18) (column 23) (len 5)))))
    (reference r5 (scope relative) (span (offset 412) (line 21) (column 17) (len 7)) (segments (segment 0 (token "Lugbolt") (name "Lugbolt") (separator none) (span (offset 412) (line 21) (column 17) (len 7)))))
    (reference r6 (scope relative) (span (offset 452) (line 24) (column 21) (len 5)) (segments (segment 0 (token "Wheel") (name "Wheel") (separator none) (span (offset 452) (line 24) (column 21) (len 5)))))
    (reference r7 (scope relative) (span (offset 541) (line 27) (column 17) (len 7)) (segments (segment 0 (token "Lugbolt") (name "Lugbolt") (separator none) (span (offset 541) (line 27) (column 17) (len 7)))))
    (reference r8 (scope relative) (span (offset 578) (line 30) (column 19) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 578) (line 30) (column 19) (len 7)))))
    (reference r9 (scope relative) (span (offset 681) (line 33) (column 27) (len 12)) (segments (segment 0 (token "AxleAssembly") (name "AxleAssembly") (separator none) (span (offset 681) (line 33) (column 27) (len 12)))))
    (reference r10 (scope relative) (span (offset 726) (line 34) (column 31) (len 14)) (segments (segment 0 (token "narrowRimWheel") (name "narrowRimWheel") (separator none) (span (offset 726) (line 34) (column 31) (len 14)))))
    (reference r11 (scope relative) (span (offset 762) (line 35) (column 20) (len 7)) (segments (segment 0 (token "lugbolt") (name "lugbolt") (separator none) (span (offset 762) (line 35) (column 20) (len 7)))))
    (reference r12 (scope relative) (span (offset 800) (line 36) (column 26) (len 16)) (segments (segment 0 (token "tighteningTorque") (name "tighteningTorque") (separator none) (span (offset 800) (line 36) (column 26) (len 16)))))
    (reference r13 (scope relative) (span (offset 819) (line 36) (column 45) (len 2)) (segments (segment 0 (token "T1") (name "T1") (separator none) (span (offset 819) (line 36) (column 45) (len 2)))))
    (reference r14 (scope relative) (span (offset 853) (line 39) (column 20) (len 4)) (segments (segment 0 (token "Axle") (name "Axle") (separator none) (span (offset 853) (line 39) (column 20) (len 4)))))
    (reference r15 (scope relative) (span (offset 890) (line 41) (column 26) (len 12)) (segments (segment 0 (token "AxleAssembly") (name "AxleAssembly") (separator none) (span (offset 890) (line 41) (column 26) (len 12)))))
    (reference r16 (scope relative) (span (offset 934) (line 42) (column 30) (len 12)) (segments (segment 0 (token "wideRimWheel") (name "wideRimWheel") (separator none) (span (offset 934) (line 42) (column 30) (len 12)))))
    (reference r17 (scope relative) (span (offset 968) (line 43) (column 20) (len 7)) (segments (segment 0 (token "lugbolt") (name "lugbolt") (separator none) (span (offset 968) (line 43) (column 20) (len 7)))))
    (reference r18 (scope relative) (span (offset 1006) (line 44) (column 26) (len 16)) (segments (segment 0 (token "tighteningTorque") (name "tighteningTorque") (separator none) (span (offset 1006) (line 44) (column 26) (len 16)))))
    (reference r19 (scope relative) (span (offset 1025) (line 44) (column 45) (len 2)) (segments (segment 0 (token "T2") (name "T2") (separator none) (span (offset 1025) (line 44) (column 45) (len 2)))))
    (reference r20 (scope relative) (span (offset 1058) (line 47) (column 19) (len 4)) (segments (segment 0 (token "Axle") (name "Axle") (separator none) (span (offset 1058) (line 47) (column 19) (len 4)))))
    (reference r21 (scope relative) (span (offset 1101) (line 51) (column 26) (len 10)) (segments (segment 0 (token "vehicle_C1") (name "vehicle_C1") (separator none) (span (offset 1101) (line 51) (column 26) (len 10)))))
    (reference r22 (scope relative) (span (offset 1196) (line 54) (column 18) (len 17)) (segments (segment 0 (token "frontAxleAssembly") (name "frontAxleAssembly") (separator none) (span (offset 1196) (line 54) (column 18) (len 17)))))
    (reference r23 (scope relative) (span (offset 1247) (line 55) (column 32) (len 10)) (segments (segment 0 (token "frontWheel") (name "frontWheel") (separator none) (span (offset 1247) (line 55) (column 32) (len 10)))))
    (reference r24 (scope relative) (span (offset 1260) (line 55) (column 45) (len 10)) (segments (segment 0 (token "frontWheel") (name "frontWheel") (separator none) (span (offset 1260) (line 55) (column 45) (len 10)))))
    (reference r25 (scope relative) (span (offset 1308) (line 56) (column 33) (len 10)) (segments (segment 0 (token "frontWheel") (name "frontWheel") (separator none) (span (offset 1308) (line 56) (column 33) (len 10)))))
    (reference r26 (scope relative) (span (offset 1321) (line 56) (column 46) (len 10)) (segments (segment 0 (token "frontWheel") (name "frontWheel") (separator none) (span (offset 1321) (line 56) (column 46) (len 10)))))
    (reference r27 (scope relative) (span (offset 1594) (line 65) (column 35) (len 28)) (segments (segment 0 (token "vehicle_C1") (name "vehicle_C1") (separator none) (span (offset 1594) (line 65) (column 35) (len 10))) (segment 1 (token "rearAxleAssembly") (name "rearAxleAssembly") (separator colon-colon) (span (offset 1606) (line 65) (column 47) (len 16)))))
    (reference r28 (scope relative) (span (offset 1655) (line 66) (column 31) (len 9)) (segments (segment 0 (token "rearWheel") (name "rearWheel") (separator none) (span (offset 1655) (line 66) (column 31) (len 9)))))
    (reference r29 (scope relative) (span (offset 1667) (line 66) (column 43) (len 9)) (segments (segment 0 (token "rearWheel") (name "rearWheel") (separator none) (span (offset 1667) (line 66) (column 43) (len 9)))))
    (reference r30 (scope relative) (span (offset 1713) (line 67) (column 32) (len 9)) (segments (segment 0 (token "rearWheel") (name "rearWheel") (separator none) (span (offset 1713) (line 67) (column 32) (len 9)))))
    (reference r31 (scope relative) (span (offset 1725) (line 67) (column 44) (len 9)) (segments (segment 0 (token "rearWheel") (name "rearWheel") (separator none) (span (offset 1725) (line 67) (column 44) (len 9)))))
    (reference r32 (scope relative) (span (offset 1983) (line 77) (column 26) (len 10)) (segments (segment 0 (token "vehicle_C2") (name "vehicle_C2") (separator none) (span (offset 1983) (line 77) (column 26) (len 10)))))
    (reference r33 (scope relative) (span (offset 2110) (line 81) (column 22) (len 12)) (segments (segment 0 (token "Transmission") (name "Transmission") (separator none) (span (offset 2110) (line 81) (column 22) (len 12)))))
    (reference r34 (scope relative) (span (offset 2174) (line 85) (column 18) (len 16)) (segments (segment 0 (token "rearAxleAssembly") (name "rearAxleAssembly") (separator none) (span (offset 2174) (line 85) (column 18) (len 16)))))
    (reference r35 (scope relative) (span (offset 2211) (line 86) (column 19) (len 8)) (segments (segment 0 (token "rearAxle") (name "rearAxle") (separator none) (span (offset 2211) (line 86) (column 19) (len 8)))))
  )
  (root (package (name "VehicleUsages") (body brace (doc) (import (target (span (span (offset 122) (line 7) (column 17) (len 5))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 145) (line 8) (column 17) (len 5))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 168) (line 9) (column 17) (len 18))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 183) (line 9) (column 32) (len 3))) (separator (span (offset 183) (line 9) (column 32) (len 2))) (marker (span (offset 185) (line 9) (column 34) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 204) (line 11) (column 16) (len 21))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 222) (line 11) (column 34) (len 3))) (separator (span (offset 222) (line 11) (column 34) (len 2))) (marker (span (offset 224) (line 11) (column 36) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (malformed (code "unrecognized_declaration_in_scope") (found "T1 = 10.0 [N * m];") (span (offset 245) (line 14) (column 2) (len 56))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "narrowRimWheel") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (doc) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "lugbolt") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity (lower (expression (span (offset 420) (line 21) (column 25) (len 1)) (integer 4))) (upper (expression (span (offset 423) (line 21) (column 28) (len 1)) (integer 5)))) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "wideRimWheel") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (doc) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "lugbolt") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity (lower (expression (span (offset 549) (line 27) (column 25) (len 1)) (integer 4))) (upper (expression (span (offset 552) (line 27) (column 28) (len 1)) (integer 6)))) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle_C1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (doc) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "frontAxleAssembly") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "frontWheel") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 715) (line 34) (column 20) (len 1)) (integer 2))) (upper (expression (span (offset 715) (line 34) (column 20) (len 1)) (integer 2)))) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r10))) (value none))) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity (lower (expression (span (offset 770) (line 35) (column 28) (len 1)) (integer 4))) (upper (expression (span (offset 770) (line 35) (column 28) (len 1)) (integer 4)))) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r11)))) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r12)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 819) (line 36) (column 45) (len 2)) (ref r13))))) (body semicolon)))))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "frontAxle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r14)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearAxleAssembly") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearWheel") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 923) (line 42) (column 19) (len 1)) (integer 2))) (upper (expression (span (offset 923) (line 42) (column 19) (len 1)) (integer 2)))) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r16))) (value none))) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity (lower (expression (span (offset 976) (line 43) (column 28) (len 1)) (integer 6))) (upper (expression (span (offset 976) (line 43) (column 28) (len 1)) (integer 6)))) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r17)))) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r18)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1025) (line 44) (column 45) (len 2)) (ref r19))))) (body semicolon)))))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearAxle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r20)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle_C2") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r21))) (value none))) (redefines none) (value none) (body brace (doc) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r22)))) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "leftFrontWheel") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r23))) (value (expression (span (offset 1260) (line 55) (column 45) (len 14)) (index (base (expression (span (offset 1260) (line 55) (column 45) (len 10)) (ref r24))) (index (expression (span (offset 1272) (line 55) (column 57) (len 1)) (integer 1)))))))) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rightFrontWheel") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r25))) (value (expression (span (offset 1321) (line 56) (column 46) (len 14)) (index (base (expression (span (offset 1321) (line 56) (column 46) (len 10)) (ref r26))) (index (expression (span (offset 1333) (line 56) (column 58) (len 1)) (integer 2)))))))) (redefines none) (value none) (body semicolon)) (interface-usage) (interface-usage))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearAxleAssembly") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r27)))) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "leftRearWheel") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r28))) (value (expression (span (offset 1667) (line 66) (column 43) (len 13)) (index (base (expression (span (offset 1667) (line 66) (column 43) (len 9)) (ref r29))) (index (expression (span (offset 1678) (line 66) (column 54) (len 1)) (integer 1)))))))) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rightRearWheel") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r30))) (value (expression (span (offset 1725) (line 67) (column 44) (len 13)) (index (base (expression (span (offset 1725) (line 67) (column 44) (len 9)) (ref r31))) (index (expression (span (offset 1736) (line 67) (column 55) (len 1)) (integer 2)))))))) (redefines none) (value none) (body semicolon)) (interface-usage) (interface-usage))))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle_C3") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r32))) (value none))) (redefines none) (value none) (body brace (doc) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "transmission") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r33)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (port-usage))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r34)))) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r35)))) (value none) (body brace (port-usage))))) (malformed (code "recovered_part_usage_body_element") (found "interface driveShaft connect") (span (offset 2261) (line 91) (column 3) (len 182))))))))
)
~~~
