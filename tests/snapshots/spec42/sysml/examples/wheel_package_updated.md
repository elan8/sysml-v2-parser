# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (v1 Spec): Wheel Package - Updated"))
~~~
# SOURCE
~~~sysml
package 'Wheel Package - Updated' {
	doc
	/*
	 * Example from the SysML 1.6 spec, subclause 8.4.1 Wheel Hub Assembly.
	 */

	private import ISQ::*;
	
	// Quantities
	
	pressure = force / length^2; 
	
	// Blocks
	
	part def WheelHubAssembly;
	part def WheelAssembly {
		inflationPressure :> pressure;
	}
	
	part def Tire {
		tireSpecification : ScalarValues::String;		
		action mountTire; // Should be operation
	}
	
	part def TireBead;
	
	connection def PressureSeat {
		end : TireBead[1];
		end : TireMountingRim[1];
	}
	
	part def Wheel {
		diameter :> length;
		width :> length;		
	}
	
	connection def BandMount {
		end : Wheel[1];
		end : WirelessTirePressureMonitor[1];
	}
	
	part def WirelessTirePressureMonitor {
		action transmitPressure; // Should be operation
	}
	
	part def TireMountingRim;
	
	part def InflationValve;
	
	part def BalanceWeight;
	
	part def LugBoltMountingHole {
		lugBoltSize :> length;
	}
	
	part def LugBoltJoint {
		torque :> ISQ::torque;
		boltTension :> force;
	}
	
	part def Hub;
	
	part def LugBoltThreadableHole {
		lugBoltSize :> length;
		threadSize :> length;
	}
	
	// Parts
	
	part wheelHubAssembly: WheelHubAssembly {
		part wheel: WheelAssembly[1] {
			part t: Tire[1] {
				part bead : TireBead[2];			
			}
			part w: Wheel[1] {
				part rim : TireMountingRim[2];
				part v : InflationValve[1];
				part weight : BalanceWeight[0..6];
				part mountingHoles : LugBoltMountingHole[5];
			}						
			connection : PressureSeat connect t.bead to w.rim;		
		}
		part lugBoltJoints: LugBoltJoint[5] {					
			ref mountingHole: LugBoltMountingHole[1] subsets wheel.w.mountingHoles;
			ref threadedHole: LugBoltThreadableHole[1] subsets hub.h;
		}
		part hub: Hub[1] {
			part h: LugBoltThreadableHole[5];
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "wheel_package_updated.md"
    (diagnostics
      (diagnostic (code "recovered_connection_def_body_element") (severity error) (category parseerror) (span (offset 471) (line 28) (column 3) (len 21)) (message "unexpected token in connection definition body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 471) (line 28) (column 3) (len 21)) (message "suppressed 3 cascading recovered diagnostics after earlier recovery errors"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Wheel Package - Updated' {
    doc
    /*
	 * Example from the SysML 1.6 spec, subclause 8.4.1 Wheel Hub Assembly.
	 */
    private import ISQ::*;
    pressure = force / length ^ 2;
    part def WheelHubAssembly;
    part def WheelAssembly {
        inflationPressure :> pressure;
    }
    part def Tire {
        tireSpecification : ScalarValues::String;
        action mountTire;
    }
    part def TireBead;
    connection def PressureSeat {
        end : TireBead[1];
        end : TireMountingRim[1];
    }
    part def Wheel {
        diameter :> length;
        width :> length;
    }
    connection def BandMount {
        end : Wheel[1];
        end : WirelessTirePressureMonitor[1];
    }
    part def WirelessTirePressureMonitor {
        action transmitPressure;
    }
    part def TireMountingRim;
    part def InflationValve;
    part def BalanceWeight;
    part def LugBoltMountingHole {
        lugBoltSize :> length;
    }
    part def LugBoltJoint {
        torque :> ISQ::torque;
        boltTension :> force;
    }
    part def Hub;
    part def LugBoltThreadableHole {
        lugBoltSize :> length;
        threadSize :> length;
    }
    part wheelHubAssembly : WheelHubAssembly {
        part wheel : WheelAssembly[1] {
            part t : Tire[1] {
                part bead : TireBead[2];
            }
            part w : Wheel[1] {
                part rim : TireMountingRim[2];
                part v : InflationValve[1];
                part weight : BalanceWeight[0..6];
                part mountingHoles : LugBoltMountingHole[5];
            }
            connection  : PressureSeat connect t.bead to w.rim;
        }
        part lugBoltJoints : LugBoltJoint[5] {
            ref mountingHole : LugBoltMountingHole[1] :> wheel.w.mountingHoles;
            ref threadedHole : LugBoltThreadableHole[1] :> hub.h;
        }
        part hub : Hub[1] {
            part h : LugBoltThreadableHole[5];
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 140) (line 7) (column 17) (len 3)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 140) (line 7) (column 17) (len 3)))))
  )
  (root (package (name "Wheel Package - Updated") (body brace (doc) (import (target (span (span (offset 140) (line 7) (column 17) (len 6))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 143) (line 7) (column 20) (len 3))) (separator (span (offset 143) (line 7) (column 20) (len 2))) (marker (span (offset 145) (line 7) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (default-reference-usage) (part-def (name "WheelHubAssembly") (body semicolon)) (part-def (name "WheelAssembly") (body brace (default-reference-usage))) (part-def (name "Tire") (body brace (default-reference-usage) (action-usage))) (part-def (name "TireBead") (body semicolon)) (connection-def (name "PressureSeat") (role ordinary) (specializes none) (body brace (malformed (code "recovered_connection_def_body_element") (found "end : TireBead[1];") (span (offset 471) (line 28) (column 3) (len 21))) (malformed (code "recovered_connection_def_body_element") (found "end : TireMountingRim[1];") (span (offset 492) (line 29) (column 3) (len 27))))) (part-def (name "Wheel") (body brace (default-reference-usage) (default-reference-usage))) (connection-def (name "BandMount") (role ordinary) (specializes none) (body brace (malformed (code "recovered_connection_def_body_element") (found "end : Wheel[1];") (span (offset 619) (line 38) (column 3) (len 18))) (malformed (code "recovered_connection_def_body_element") (found "end : WirelessTirePressureMonitor[1];") (span (offset 637) (line 39) (column 3) (len 39))))) (part-def (name "WirelessTirePressureMonitor") (body brace (action-usage))) (part-def (name "TireMountingRim") (body semicolon)) (part-def (name "InflationValve") (body semicolon)) (part-def (name "BalanceWeight") (body semicolon)) (part-def (name "LugBoltMountingHole") (body brace (default-reference-usage))) (part-def (name "LugBoltJoint") (body brace (default-reference-usage) (default-reference-usage))) (part-def (name "Hub") (body semicolon)) (part-def (name "LugBoltThreadableHole") (body brace (default-reference-usage) (default-reference-usage))) (part-usage))))
)
~~~
