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
        part :>> chs {
            part :>> rb : LightRollBar[0..1];
            part :>> w {
                part :>> lb;
            }
        }
        part :>> eng {
            part :>> cyl[4];
        }
        ref lugBolts[24] = chs.w.lb;
    }
    part 'vehicle model 2' :> vehicle {
        part :>> chs {
            part :>> rb[0];
            part :>> w {
                part :>> lb[6..7];
            }
        }
        part :>> eng {
            part :>> cyl[6..8];
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 448) (line 28) (column 17) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 448) (line 28) (column 17) (len 7)))))
    (reference r1 (scope relative) (span (offset 471) (line 29) (column 14) (len 18)) (segments (segment 0 (token "'Chassis Assembly'") (name "Chassis Assembly") (separator none) (span (offset 471) (line 29) (column 14) (len 18)))))
    (reference r2 (scope relative) (span (offset 508) (line 30) (column 14) (len 7)) (segments (segment 0 (token "RollBar") (name "RollBar") (separator none) (span (offset 508) (line 30) (column 14) (len 7)))))
    (reference r3 (scope relative) (span (offset 535) (line 31) (column 13) (len 5)) (segments (segment 0 (token "Wheel") (name "Wheel") (separator none) (span (offset 535) (line 31) (column 13) (len 5)))))
    (reference r4 (scope relative) (span (offset 560) (line 32) (column 15) (len 7)) (segments (segment 0 (token "LugBolt") (name "LugBolt") (separator none) (span (offset 560) (line 32) (column 15) (len 7)))))
    (reference r5 (scope relative) (span (offset 597) (line 35) (column 13) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 597) (line 35) (column 13) (len 6)))))
    (reference r6 (scope relative) (span (offset 623) (line 36) (column 15) (len 8)) (segments (segment 0 (token "Cylinder") (name "Cylinder") (separator none) (span (offset 623) (line 36) (column 15) (len 8)))))
    (reference r7 (scope relative) (span (offset 677) (line 41) (column 28) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 677) (line 41) (column 28) (len 7)))))
    (reference r8 (scope relative) (span (offset 704) (line 42) (column 18) (len 3)) (segments (segment 0 (token "chs") (name "chs") (separator none) (span (offset 704) (line 42) (column 18) (len 3)))))
    (reference r9 (scope relative) (span (offset 733) (line 43) (column 24) (len 12)) (segments (segment 0 (token "LightRollBar") (name "LightRollBar") (separator none) (span (offset 733) (line 43) (column 24) (len 12)))))
    (reference r10 (scope relative) (span (offset 728) (line 43) (column 19) (len 2)) (segments (segment 0 (token "rb") (name "rb") (separator none) (span (offset 728) (line 43) (column 19) (len 2)))))
    (reference r11 (scope relative) (span (offset 771) (line 44) (column 19) (len 1)) (segments (segment 0 (token "w") (name "w") (separator none) (span (offset 771) (line 44) (column 19) (len 1)))))
    (reference r12 (scope relative) (span (offset 794) (line 45) (column 20) (len 2)) (segments (segment 0 (token "lb") (name "lb") (separator none) (span (offset 794) (line 45) (column 20) (len 2)))))
    (reference r13 (scope relative) (span (offset 824) (line 48) (column 18) (len 3)) (segments (segment 0 (token "eng") (name "eng") (separator none) (span (offset 824) (line 48) (column 18) (len 3)))))
    (reference r14 (scope relative) (span (offset 848) (line 49) (column 19) (len 3)) (segments (segment 0 (token "cyl") (name "cyl") (separator none) (span (offset 848) (line 49) (column 19) (len 3)))))
    (reference r15 (scope relative) (span (offset 968) (line 56) (column 28) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 968) (line 56) (column 28) (len 7)))))
    (reference r16 (scope relative) (span (offset 995) (line 57) (column 18) (len 3)) (segments (segment 0 (token "chs") (name "chs") (separator none) (span (offset 995) (line 57) (column 18) (len 3)))))
    (reference r17 (scope relative) (span (offset 1019) (line 58) (column 19) (len 2)) (segments (segment 0 (token "rb") (name "rb") (separator none) (span (offset 1019) (line 58) (column 19) (len 2)))))
    (reference r18 (scope relative) (span (offset 1044) (line 59) (column 19) (len 1)) (segments (segment 0 (token "w") (name "w") (separator none) (span (offset 1044) (line 59) (column 19) (len 1)))))
    (reference r19 (scope relative) (span (offset 1115) (line 61) (column 20) (len 2)) (segments (segment 0 (token "lb") (name "lb") (separator none) (span (offset 1115) (line 61) (column 20) (len 2)))))
    (reference r20 (scope relative) (span (offset 1151) (line 64) (column 18) (len 3)) (segments (segment 0 (token "eng") (name "eng") (separator none) (span (offset 1151) (line 64) (column 18) (len 3)))))
    (reference r21 (scope relative) (span (offset 1175) (line 65) (column 19) (len 3)) (segments (segment 0 (token "cyl") (name "cyl") (separator none) (span (offset 1175) (line 65) (column 19) (len 3)))))
  )
  (root (package (name "Vehicle Decomposition - Updated") (body brace (doc (name none) (locale none) (body (span (offset 52) (line 3) (column 4) (len 124)) (normalized "Example from the SysML 1.6 spec, subclause 8.4.5 Constraining Decomposition,\nupdated for usage-focused approach.\n"))) (part-def (name "Vehicle") (modifiers) (body semicolon)) (part-def (name "Chassis Assembly") (modifiers) (body semicolon)) (part-def (name "Wheel") (modifiers) (body semicolon)) (part-def (name "LugBolt") (modifiers) (body semicolon)) (part-def (name "RollBar") (modifiers) (body semicolon)) (part-def (name "HeavyRollBar") (modifiers) (body semicolon)) (part-def (name "LightRollBar") (modifiers) (body semicolon)) (part-def (name "Engine") (modifiers) (body semicolon)) (part-def (name "Cylinder") (modifiers) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "chs") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity (lower (expression (span (offset 490) (line 29) (column 33) (len 1)) (integer 1))) (upper (expression (span (offset 490) (line 29) (column 33) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rb") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity (lower (expression (span (offset 516) (line 30) (column 22) (len 1)) (integer 0))) (upper (expression (span (offset 519) (line 30) (column 25) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "w") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity (lower (expression (span (offset 541) (line 31) (column 19) (len 1)) (integer 4))) (upper (expression (span (offset 541) (line 31) (column 19) (len 1)) (integer 4)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "lb") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity (lower (expression (span (offset 568) (line 32) (column 23) (len 1)) (integer 6))) (upper (expression (span (offset 571) (line 32) (column 26) (len 2)) (integer 10)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "eng") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity (lower (expression (span (offset 604) (line 35) (column 20) (len 1)) (integer 1))) (upper (expression (span (offset 604) (line 35) (column 20) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "cyl") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity (lower (expression (span (offset 632) (line 36) (column 24) (len 1)) (integer 4))) (upper (expression (span (offset 635) (line 36) (column 27) (len 1)) (integer 8)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle model 1") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r7))) (value none))) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r8)))) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity (lower (expression (span (offset 746) (line 43) (column 37) (len 1)) (integer 0))) (upper (expression (span (offset 749) (line 43) (column 40) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r10)))) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r11)))) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r12)))) (value none) (body semicolon)))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r13)))) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity (lower (expression (span (offset 852) (line 49) (column 23) (len 1)) (integer 4))) (upper (expression (span (offset 852) (line 49) (column 23) (len 1)) (integer 4)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r14)))) (value none) (body semicolon)))) (ref (name "lugBolts") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing none) (redefines none) (subsets none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle model 2") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r15))) (value none))) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r16)))) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity (lower (expression (span (offset 1022) (line 58) (column 22) (len 1)) (integer 0))) (upper (expression (span (offset 1022) (line 58) (column 22) (len 1)) (integer 0)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r17)))) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r18)))) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity (lower (expression (span (offset 1118) (line 61) (column 23) (len 1)) (integer 6))) (upper (expression (span (offset 1121) (line 61) (column 26) (len 1)) (integer 7)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r19)))) (value none) (body semicolon)))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r20)))) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity (lower (expression (span (offset 1179) (line 65) (column 23) (len 1)) (integer 6))) (upper (expression (span (offset 1182) (line 65) (column 26) (len 1)) (integer 8)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r21)))) (value none) (body semicolon)))))))))
)
~~~
