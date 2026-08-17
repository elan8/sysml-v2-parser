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
      (diagnostic (code "missing_semicolon") (severity error) (category parseerror) (span (offset 646) (line 29) (column 4) (len 31)) (message "missing semicolon before next declaration"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 646) (line 29) (column 4) (len 31)) (message "suppressed 1 cascading missing_semicolon diagnostic after earlier recovery errors"))
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
        end bead : TireBead[1];
        end mountingRim : TireMountingRim[1];
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
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 325) (line 15) (column 23) (len 8)) (segments (segment 0 (token "TireBead") (name "TireBead") (separator none) (span (offset 325) (line 15) (column 23) (len 8)))))
    (reference r1 (scope relative) (span (offset 364) (line 16) (column 30) (len 15)) (segments (segment 0 (token "TireMountingRim") (name "TireMountingRim") (separator none) (span (offset 364) (line 16) (column 30) (len 15)))))
    (reference r2 (scope relative) (span (offset 411) (line 19) (column 26) (len 16)) (segments (segment 0 (token "WheelHubAssembly") (name "WheelHubAssembly") (separator none) (span (offset 411) (line 19) (column 26) (len 16)))))
  )
  (root (package (name "Connections Example") (body brace (part-def (name "WheelHubAssembly") (body semicolon)) (part-def (name "WheelAssembly") (body semicolon)) (part-def (name "Tire") (body semicolon)) (part-def (name "TireBead") (body semicolon)) (part-def (name "Wheel") (body semicolon)) (part-def (name "TireMountingRim") (body semicolon)) (part-def (name "LugBoltMountingHole") (body semicolon)) (part-def (name "Hub") (body semicolon)) (part-def (name "LugBoltThreadableHole") (body semicolon)) (part-def (name "LugBoltJoint") (body semicolon)) (connection-def (name "PressureSeat") (role ordinary) (specializes none) (body brace (end (identity (declaration (name "bead") (span (offset 318) (line 15) (column 16) (len 4)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (references none) (redefines none) (crosses none)) (end (identity (declaration (name "mountingRim") (span (offset 350) (line 16) (column 16) (len 11)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (references none) (redefines none) (crosses none)))) (part-usage (declaration-name "wheelHubAssembly") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (body brace (part-usage) (part-usage) (part-usage) (connect) (connect))))))
)
~~~
