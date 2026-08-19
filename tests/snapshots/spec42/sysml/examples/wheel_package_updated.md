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
    (reference r1 (scope relative) (span (offset 1141) (line 70) (column 25) (len 16)) (segments (segment 0 (token "WheelHubAssembly") (name "WheelHubAssembly") (separator none) (span (offset 1141) (line 70) (column 25) (len 16)))))
    (reference r2 (scope relative) (span (offset 1174) (line 71) (column 15) (len 13)) (segments (segment 0 (token "WheelAssembly") (name "WheelAssembly") (separator none) (span (offset 1174) (line 71) (column 15) (len 13)))))
    (reference r3 (scope relative) (span (offset 1204) (line 72) (column 12) (len 4)) (segments (segment 0 (token "Tire") (name "Tire") (separator none) (span (offset 1204) (line 72) (column 12) (len 4)))))
    (reference r4 (scope relative) (span (offset 1230) (line 73) (column 17) (len 8)) (segments (segment 0 (token "TireBead") (name "TireBead") (separator none) (span (offset 1230) (line 73) (column 17) (len 8)))))
    (reference r5 (scope relative) (span (offset 1262) (line 75) (column 12) (len 5)) (segments (segment 0 (token "Wheel") (name "Wheel") (separator none) (span (offset 1262) (line 75) (column 12) (len 5)))))
    (reference r6 (scope relative) (span (offset 1288) (line 76) (column 16) (len 15)) (segments (segment 0 (token "TireMountingRim") (name "TireMountingRim") (separator none) (span (offset 1288) (line 76) (column 16) (len 15)))))
    (reference r7 (scope relative) (span (offset 1321) (line 77) (column 14) (len 14)) (segments (segment 0 (token "InflationValve") (name "InflationValve") (separator none) (span (offset 1321) (line 77) (column 14) (len 14)))))
    (reference r8 (scope relative) (span (offset 1358) (line 78) (column 19) (len 13)) (segments (segment 0 (token "BalanceWeight") (name "BalanceWeight") (separator none) (span (offset 1358) (line 78) (column 19) (len 13)))))
    (reference r9 (scope relative) (span (offset 1404) (line 79) (column 26) (len 19)) (segments (segment 0 (token "LugBoltMountingHole") (name "LugBoltMountingHole") (separator none) (span (offset 1404) (line 79) (column 26) (len 19)))))
    (reference r10 (scope relative) (span (offset 1521) (line 83) (column 23) (len 12)) (segments (segment 0 (token "LugBoltJoint") (name "LugBoltJoint") (separator none) (span (offset 1521) (line 83) (column 23) (len 12)))))
    (reference r11 (scope relative) (span (offset 1565) (line 84) (column 22) (len 19)) (segments (segment 0 (token "LugBoltMountingHole") (name "LugBoltMountingHole") (separator none) (span (offset 1565) (line 84) (column 22) (len 19)))))
    (reference r12 (scope relative) (span (offset 1596) (line 84) (column 53) (len 21)) (segments (segment 0 (token "wheel") (name "wheel") (separator none) (span (offset 1596) (line 84) (column 53) (len 5))) (segment 1 (token "w") (name "w") (separator dot) (span (offset 1602) (line 84) (column 59) (len 1))) (segment 2 (token "mountingHoles") (name "mountingHoles") (separator dot) (span (offset 1604) (line 84) (column 61) (len 13)))))
    (reference r13 (scope relative) (span (offset 1640) (line 85) (column 22) (len 21)) (segments (segment 0 (token "LugBoltThreadableHole") (name "LugBoltThreadableHole") (separator none) (span (offset 1640) (line 85) (column 22) (len 21)))))
    (reference r14 (scope relative) (span (offset 1673) (line 85) (column 55) (len 5)) (segments (segment 0 (token "hub") (name "hub") (separator none) (span (offset 1673) (line 85) (column 55) (len 3))) (segment 1 (token "h") (name "h") (separator dot) (span (offset 1677) (line 85) (column 59) (len 1)))))
    (reference r15 (scope relative) (span (offset 1696) (line 87) (column 13) (len 3)) (segments (segment 0 (token "Hub") (name "Hub") (separator none) (span (offset 1696) (line 87) (column 13) (len 3)))))
    (reference r16 (scope relative) (span (offset 1716) (line 88) (column 12) (len 21)) (segments (segment 0 (token "LugBoltThreadableHole") (name "LugBoltThreadableHole") (separator none) (span (offset 1716) (line 88) (column 12) (len 21)))))
  )
  (root (package (name "Wheel Package - Updated") (body brace (doc (name none) (locale none) (body (span (offset 44) (line 3) (column 4) (len 76)) (normalized "Example from the SysML 1.6 spec, subclause 8.4.1 Wheel Hub Assembly.\n"))) (import (target (span (span (offset 140) (line 7) (column 17) (len 6))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 143) (line 7) (column 20) (len 3))) (separator (span (offset 143) (line 7) (column 20) (len 2))) (marker (span (offset 145) (line 7) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (default-reference-usage) (part-def (name "WheelHubAssembly") (body semicolon)) (part-def (name "WheelAssembly") (body brace (default-reference-usage))) (part-def (name "Tire") (body brace (default-reference-usage) (action-usage))) (part-def (name "TireBead") (body semicolon)) (connection-def (name "PressureSeat") (modifiers) (role ordinary) (specializes none) (body brace (malformed (code "recovered_connection_def_body_element") (found "end : TireBead[1];") (span (offset 471) (line 28) (column 3) (len 21))) (malformed (code "recovered_connection_def_body_element") (found "end : TireMountingRim[1];") (span (offset 492) (line 29) (column 3) (len 27))))) (part-def (name "Wheel") (body brace (default-reference-usage) (default-reference-usage))) (connection-def (name "BandMount") (modifiers) (role ordinary) (specializes none) (body brace (malformed (code "recovered_connection_def_body_element") (found "end : Wheel[1];") (span (offset 619) (line 38) (column 3) (len 18))) (malformed (code "recovered_connection_def_body_element") (found "end : WirelessTirePressureMonitor[1];") (span (offset 637) (line 39) (column 3) (len 39))))) (part-def (name "WirelessTirePressureMonitor") (body brace (action-usage))) (part-def (name "TireMountingRim") (body semicolon)) (part-def (name "InflationValve") (body semicolon)) (part-def (name "BalanceWeight") (body semicolon)) (part-def (name "LugBoltMountingHole") (body brace (default-reference-usage))) (part-def (name "LugBoltJoint") (body brace (default-reference-usage) (default-reference-usage))) (part-def (name "Hub") (body semicolon)) (part-def (name "LugBoltThreadableHole") (body brace (default-reference-usage) (default-reference-usage))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "wheelHubAssembly") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "wheel") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity (lower (expression (span (offset 1188) (line 71) (column 29) (len 1)) (integer 1))) (upper (expression (span (offset 1188) (line 71) (column 29) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "t") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity (lower (expression (span (offset 1209) (line 72) (column 17) (len 1)) (integer 1))) (upper (expression (span (offset 1209) (line 72) (column 17) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "bead") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity (lower (expression (span (offset 1239) (line 73) (column 26) (len 1)) (integer 2))) (upper (expression (span (offset 1239) (line 73) (column 26) (len 1)) (integer 2)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "w") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity (lower (expression (span (offset 1268) (line 75) (column 18) (len 1)) (integer 1))) (upper (expression (span (offset 1268) (line 75) (column 18) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rim") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity (lower (expression (span (offset 1304) (line 76) (column 32) (len 1)) (integer 2))) (upper (expression (span (offset 1304) (line 76) (column 32) (len 1)) (integer 2)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "v") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity (lower (expression (span (offset 1336) (line 77) (column 29) (len 1)) (integer 1))) (upper (expression (span (offset 1336) (line 77) (column 29) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "weight") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity (lower (expression (span (offset 1372) (line 78) (column 33) (len 1)) (integer 0))) (upper (expression (span (offset 1375) (line 78) (column 36) (len 1)) (integer 6)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "mountingHoles") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity (lower (expression (span (offset 1424) (line 79) (column 46) (len 1)) (integer 5))) (upper (expression (span (offset 1424) (line 79) (column 46) (len 1)) (integer 5)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (connection))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "lugBoltJoints") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity (lower (expression (span (offset 1534) (line 83) (column 36) (len 1)) (integer 5))) (upper (expression (span (offset 1534) (line 83) (column 36) (len 1)) (integer 5)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (ref (name "mountingHole") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (redefines none) (subsets (relationship (kind subsets) (implied false) (targets (ref r12)))) (body semicolon)) (ref (name "threadedHole") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (redefines none) (subsets (relationship (kind subsets) (implied false) (targets (ref r14)))) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "hub") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (multiplicity (lower (expression (span (offset 1700) (line 87) (column 17) (len 1)) (integer 1))) (upper (expression (span (offset 1700) (line 87) (column 17) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "h") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (multiplicity (lower (expression (span (offset 1738) (line 88) (column 34) (len 1)) (integer 5))) (upper (expression (span (offset 1738) (line 88) (column 34) (len 1)) (integer 5)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))))))))
)
~~~
