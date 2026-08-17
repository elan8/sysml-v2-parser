# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (v1 Spec): Vehicle Decomposition - Updated"))
~~~
# SOURCE
~~~sysml
package 'Vehicle Decomposition - Updated' {
	doc
	/*
	 * Example from the SysML 1.6 spec, subclause 8.4.5 Constraining Decomposition,
	 * updated for usage-focused approach.
	 */
	
	// Blocks
	
	part def Vehicle;
	
	part def 'Chassis Assembly';
	
	part def Wheel;
	
	part def LugBolt;
	
	part def RollBar;
	part def HeavyRollBar :> RollBar;
	part def LightRollBar :> RollBar;
	
	part def Engine;
	
	part def Cylinder;
	
	// Parts
	
	part vehicle : Vehicle {
		part chs : 'Chassis Assembly'[1] {
			part rb : RollBar[0..1];
			part w : Wheel[4] {
				part lb : LugBolt[6..10];
			}
		}
		part eng: Engine[1] {
			part cyl : Cylinder[4..8];
		}
	}
	
	
	part 'vehicle model 1' :> vehicle {
		part redefines chs {
			part redefines rb : LightRollBar[0..1];
			part redefines w {
				part redefines lb;
			}
		}
		part redefines eng {
			part redefines cyl[4];
		}
		
		// Constrains total number of lugbolts.
		ref lugBolts[24] = chs.w.lb;
	}
	
	part 'vehicle model 2' :> vehicle {
		part redefines chs {
			part redefines rb[0];
			part redefines w {
				// Constrains number of lugbolts per wheel.
				part redefines lb[6..7];
			}
		}
		part redefines eng {
			part redefines cyl[6..8];
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "vehicle_decomposition_updated.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Vehicle Decomposition - Updated' {
    doc
    /*
	 * Example from the SysML 1.6 spec, subclause 8.4.5 Constraining Decomposition,
	 * updated for usage-focused approach.
	 */
    part def Vehicle;
    part def 'Chassis Assembly';
    part def Wheel;
    part def LugBolt;
    part def RollBar;
    part def HeavyRollBar :> RollBar;
    part def LightRollBar :> RollBar;
    part def Engine;
    part def Cylinder;
    part vehicle : Vehicle {
        part chs : 'Chassis Assembly'[1] {
            part rb : RollBar[0..1];
            part w : Wheel[4] {
                part lb : LugBolt[6..10];
            }
        }
        part eng : Engine[1] {
            part cyl : Cylinder[4..8];
        }
    }
    part 'vehicle model 1' :> vehicle {
        part  :>> chs {
            part  :>> rb : LightRollBar[0..1];
            part  :>> w {
                part  :>> lb;
            }
        }
        part  :>> eng {
            part  :>> cyl[4];
        }
        ref lugBolts[24] = chs.w.lb;
    }
    part 'vehicle model 2' :> vehicle {
        part  :>> chs {
            part  :>> rb[0];
            part  :>> w {
                part  :>> lb[6..7];
            }
        }
        part  :>> eng {
            part  :>> cyl[6..8];
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 448) (line 28) (column 17) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 448) (line 28) (column 17) (len 7)))))
  )
  (root (package (name "Vehicle Decomposition - Updated") (body brace (doc) (part-def (name "Vehicle") (body semicolon)) (part-def (name "Chassis Assembly") (body semicolon)) (part-def (name "Wheel") (body semicolon)) (part-def (name "LugBolt") (body semicolon)) (part-def (name "RollBar") (body semicolon)) (part-def (name "HeavyRollBar") (body semicolon)) (part-def (name "LightRollBar") (body semicolon)) (part-def (name "Engine") (body semicolon)) (part-def (name "Cylinder") (body semicolon)) (part-usage (declaration-name "vehicle") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity-modifiers (ordered false) (nonunique false)) (body brace (part-usage) (part-usage))) (part-usage (declaration-name "vehicle model 1") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body brace (part-usage) (part-usage) (ref (name "lugBolts") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing none) (redefines none) (subsets none) (body semicolon)))) (part-usage (declaration-name "vehicle model 2") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body brace (part-usage) (part-usage))))))
)
~~~
