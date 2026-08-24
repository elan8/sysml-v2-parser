# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 09 (Connections): Connections Example"))
~~~
# SOURCE
~~~sysml
package 'Connections Example' {
	
	part def WheelHubAssembly;
	part def WheelAssembly;
	part def Tire;
	part def TireBead;
	part def Wheel;
	part def TireMountingRim;
	part def LugBoltMountingHole;
	part def Hub;
	part def LugBoltThreadableHole;
	part def LugBoltJoint;
	
	connection def PressureSeat {
		end [1] part bead : TireBead;
		end [1] part mountingRim : TireMountingRim;
	}
	
	part wheelHubAssembly : WheelHubAssembly {
		
		part wheel : WheelAssembly[1] {
			part t : Tire[1] {
				part bead : TireBead[2];			
			}
			part w: Wheel[1] {
				part rim : TireMountingRim[2];
				part mountingHoles : LugBoltMountingHole[5];
			}						
			connection : PressureSeat 
				connect bead references t.bead 
				to mountingRim references w.rim;		
		}
		
		part lugBoltJoints : LugBoltJoint[0..5];
		part hub : Hub[1] {
			part h : LugBoltThreadableHole[5];
		}
		connect [0..1] lugBoltJoints to [1] wheel.w.mountingHoles;
		connect [0..1] lugBoltJoints to [1] hub.h;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "09_connections_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Connections Example' {
    part def WheelHubAssembly;
    part def WheelAssembly;
    part def Tire;
    part def TireBead;
    part def Wheel;
    part def TireMountingRim;
    part def LugBoltMountingHole;
    part def Hub;
    part def LugBoltThreadableHole;
    part def LugBoltJoint;
    connection def PressureSeat {
        end [1] part bead : TireBead;
        end [1] part mountingRim : TireMountingRim;
    }
    part wheelHubAssembly : WheelHubAssembly {
        part wheel : WheelAssembly[1] {
            part t : Tire[1] {
                part bead : TireBead[2];
            }
            part w : Wheel[1] {
                part rim : TireMountingRim[2];
                part mountingHoles : LugBoltMountingHole[5];
            }
            connection  : PressureSeat connect bead references t.bead to mountingRim references w.rim;
        }
        part lugBoltJoints : LugBoltJoint[0..5];
        part hub : Hub[1] {
            part h : LugBoltThreadableHole[5];
        }
        connect [0..1] lugBoltJoints to [1] wheel.w.mountingHoles;
        connect [0..1] lugBoltJoints to [1] hub.h;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 325) (line 15) (column 23) (len 8)) (segments (segment 0 (token "TireBead") (name "TireBead") (separator none) (span (offset 325) (line 15) (column 23) (len 8)))))
    (reference r1 (scope relative) (span (offset 364) (line 16) (column 30) (len 15)) (segments (segment 0 (token "TireMountingRim") (name "TireMountingRim") (separator none) (span (offset 364) (line 16) (column 30) (len 15)))))
    (reference r2 (scope relative) (span (offset 411) (line 19) (column 26) (len 16)) (segments (segment 0 (token "WheelHubAssembly") (name "WheelHubAssembly") (separator none) (span (offset 411) (line 19) (column 26) (len 16)))))
    (reference r3 (scope relative) (span (offset 448) (line 21) (column 16) (len 13)) (segments (segment 0 (token "WheelAssembly") (name "WheelAssembly") (separator none) (span (offset 448) (line 21) (column 16) (len 13)))))
    (reference r4 (scope relative) (span (offset 479) (line 22) (column 13) (len 4)) (segments (segment 0 (token "Tire") (name "Tire") (separator none) (span (offset 479) (line 22) (column 13) (len 4)))))
    (reference r5 (scope relative) (span (offset 505) (line 23) (column 17) (len 8)) (segments (segment 0 (token "TireBead") (name "TireBead") (separator none) (span (offset 505) (line 23) (column 17) (len 8)))))
    (reference r6 (scope relative) (span (offset 537) (line 25) (column 12) (len 5)) (segments (segment 0 (token "Wheel") (name "Wheel") (separator none) (span (offset 537) (line 25) (column 12) (len 5)))))
    (reference r7 (scope relative) (span (offset 563) (line 26) (column 16) (len 15)) (segments (segment 0 (token "TireMountingRim") (name "TireMountingRim") (separator none) (span (offset 563) (line 26) (column 16) (len 15)))))
    (reference r8 (scope relative) (span (offset 608) (line 27) (column 26) (len 19)) (segments (segment 0 (token "LugBoltMountingHole") (name "LugBoltMountingHole") (separator none) (span (offset 608) (line 27) (column 26) (len 19)))))
    (reference r9 (scope relative) (span (offset 778) (line 34) (column 24) (len 12)) (segments (segment 0 (token "LugBoltJoint") (name "LugBoltJoint") (separator none) (span (offset 778) (line 34) (column 24) (len 12)))))
    (reference r10 (scope relative) (span (offset 811) (line 35) (column 14) (len 3)) (segments (segment 0 (token "Hub") (name "Hub") (separator none) (span (offset 811) (line 35) (column 14) (len 3)))))
    (reference r11 (scope relative) (span (offset 832) (line 36) (column 13) (len 21)) (segments (segment 0 (token "LugBoltThreadableHole") (name "LugBoltThreadableHole") (separator none) (span (offset 832) (line 36) (column 13) (len 21)))))
  )
  (root (package (name "Connections Example") (body brace (part-def (name "WheelHubAssembly") (modifiers) (body semicolon)) (part-def (name "WheelAssembly") (modifiers) (body semicolon)) (part-def (name "Tire") (modifiers) (body semicolon)) (part-def (name "TireBead") (modifiers) (body semicolon)) (part-def (name "Wheel") (modifiers) (body semicolon)) (part-def (name "TireMountingRim") (modifiers) (body semicolon)) (part-def (name "LugBoltMountingHole") (modifiers) (body semicolon)) (part-def (name "Hub") (modifiers) (body semicolon)) (part-def (name "LugBoltThreadableHole") (modifiers) (body semicolon)) (part-def (name "LugBoltJoint") (modifiers) (body semicolon)) (connection-def (name "PressureSeat") (modifiers) (role ordinary) (specializes none) (body brace (part-usage (then false) (prefix (end (cross ((direction none) (derived false) (variance none) (constant false) (reference false) (name none) (short-name none) (typing none) (multiplicity (lower (expression (span (offset 310) (line 15) (column 8) (len 1)) (integer 1))) (upper (expression (span (offset 310) (line 15) (column 8) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)))) (extensions)) (declaration-name "bead") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (end (cross ((direction none) (derived false) (variance none) (constant false) (reference false) (name none) (short-name none) (typing none) (multiplicity (lower (expression (span (offset 342) (line 16) (column 8) (len 1)) (integer 1))) (upper (expression (span (offset 342) (line 16) (column 8) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)))) (extensions)) (declaration-name "mountingRim") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "wheelHubAssembly") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "wheel") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity (lower (expression (span (offset 462) (line 21) (column 30) (len 1)) (integer 1))) (upper (expression (span (offset 462) (line 21) (column 30) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "t") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity (lower (expression (span (offset 484) (line 22) (column 18) (len 1)) (integer 1))) (upper (expression (span (offset 484) (line 22) (column 18) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "bead") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity (lower (expression (span (offset 514) (line 23) (column 26) (len 1)) (integer 2))) (upper (expression (span (offset 514) (line 23) (column 26) (len 1)) (integer 2)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "w") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity (lower (expression (span (offset 543) (line 25) (column 18) (len 1)) (integer 1))) (upper (expression (span (offset 543) (line 25) (column 18) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rim") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity (lower (expression (span (offset 579) (line 26) (column 32) (len 1)) (integer 2))) (upper (expression (span (offset 579) (line 26) (column 32) (len 1)) (integer 2)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "mountingHoles") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity (lower (expression (span (offset 628) (line 27) (column 46) (len 1)) (integer 5))) (upper (expression (span (offset 628) (line 27) (column 46) (len 1)) (integer 5)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (connection))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "lugBoltJoints") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity (lower (expression (span (offset 791) (line 34) (column 37) (len 1)) (integer 0))) (upper (expression (span (offset 794) (line 34) (column 40) (len 1)) (integer 5)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "hub") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity (lower (expression (span (offset 815) (line 35) (column 18) (len 1)) (integer 1))) (upper (expression (span (offset 815) (line 35) (column 18) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "h") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (multiplicity (lower (expression (span (offset 854) (line 36) (column 35) (len 1)) (integer 5))) (upper (expression (span (offset 854) (line 36) (column 35) (len 1)) (integer 5)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (connect) (connect))))))
)
~~~
