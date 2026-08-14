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
                part  :>> lugbolt[4] {
                    attribute :>> tighteningTorque = T1;
                }
            }
            part frontAxle : Axle;
        }
        part rearAxleAssembly : AxleAssembly {
            part rearWheel[2] :> wideRimWheel {
                part  :>> lugbolt[6] {
                    attribute :>> tighteningTorque = T2;
                }
            }
            part rearAxle : Axle;
        }
    }
    part vehicle_C2 :> vehicle_C1 {
        doc
        /* Specialized configuration with part-specific ports. */
        part  :>> frontAxleAssembly {
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
        part  :>> rearAxleAssembly {
            part  :>> rearAxle {
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
  )
  (root (package (name "VehicleUsages") (body (doc) (import (target (span (span (offset 122) (line 7) (column 17) (len 5))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 145) (line 8) (column 17) (len 5))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 168) (line 9) (column 17) (len 18))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 183) (line 9) (column 32) (len 3))) (separator (span (offset 183) (line 9) (column 32) (len 2))) (marker (span (offset 185) (line 9) (column 34) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 204) (line 11) (column 16) (len 21))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 222) (line 11) (column 34) (len 3))) (separator (span (offset 222) (line 11) (column 34) (len 2))) (marker (span (offset 224) (line 11) (column 36) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (malformed (code "unrecognized_declaration_in_scope") (found "T1 = 10.0 [N * m];") (span (offset 245) (line 14) (column 2) (len 56))) (part-usage) (part-usage) (part-usage) (part-usage) (part-usage))))
)
~~~
