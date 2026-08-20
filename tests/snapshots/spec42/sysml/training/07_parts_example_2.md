# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 07 (Parts): Parts Example-2"))
~~~
# SOURCE
~~~sysml
package 'Parts Example-2' {
	
	// Definitions
	
	part def Vehicle;	
	part def Engine;	
	part def Cylinder;
	
	// Usages
	
	part vehicle : Vehicle {
		part eng : Engine {
			part cyl : Cylinder[4..6];
		}
	}
	
	part smallVehicle :> vehicle {
		part redefines eng {
			part redefines cyl[4];
		}
	}
	
	part bigVehicle :> vehicle {
		part redefines eng {
			part redefines cyl[6];
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "07_parts_example_2.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Parts Example-2' {
    part def Vehicle;
    part def Engine;
    part def Cylinder;
    part vehicle : Vehicle {
        part eng : Engine {
            part cyl : Cylinder[4..6];
        }
    }
    part smallVehicle :> vehicle {
        part :>> eng {
            part :>> cyl[4];
        }
    }
    part bigVehicle :> vehicle {
        part :>> eng {
            part :>> cyl[6];
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 138) (line 11) (column 17) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 138) (line 11) (column 17) (len 7)))))
    (reference r1 (scope relative) (span (offset 161) (line 12) (column 14) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 161) (line 12) (column 14) (len 6)))))
    (reference r2 (scope relative) (span (offset 184) (line 13) (column 15) (len 8)) (segments (segment 0 (token "Cylinder") (name "Cylinder") (separator none) (span (offset 184) (line 13) (column 15) (len 8)))))
    (reference r3 (scope relative) (span (offset 231) (line 17) (column 23) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 231) (line 17) (column 23) (len 7)))))
    (reference r4 (scope relative) (span (offset 258) (line 18) (column 18) (len 3)) (segments (segment 0 (token "eng") (name "eng") (separator none) (span (offset 258) (line 18) (column 18) (len 3)))))
    (reference r5 (scope relative) (span (offset 282) (line 19) (column 19) (len 3)) (segments (segment 0 (token "cyl") (name "cyl") (separator none) (span (offset 282) (line 19) (column 19) (len 3)))))
    (reference r6 (scope relative) (span (offset 319) (line 23) (column 21) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 319) (line 23) (column 21) (len 7)))))
    (reference r7 (scope relative) (span (offset 346) (line 24) (column 18) (len 3)) (segments (segment 0 (token "eng") (name "eng") (separator none) (span (offset 346) (line 24) (column 18) (len 3)))))
    (reference r8 (scope relative) (span (offset 370) (line 25) (column 19) (len 3)) (segments (segment 0 (token "cyl") (name "cyl") (separator none) (span (offset 370) (line 25) (column 19) (len 3)))))
  )
  (root (package (name "Parts Example-2") (body brace (part-def (name "Vehicle") (modifiers) (body semicolon)) (part-def (name "Engine") (modifiers) (body semicolon)) (part-def (name "Cylinder") (modifiers) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "eng") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "cyl") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity (lower (expression (span (offset 193) (line 13) (column 24) (len 1)) (integer 4))) (upper (expression (span (offset 196) (line 13) (column 27) (len 1)) (integer 6)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "smallVehicle") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r3))) (value none))) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity (lower (expression (span (offset 286) (line 19) (column 23) (len 1)) (integer 4))) (upper (expression (span (offset 286) (line 19) (column 23) (len 1)) (integer 4)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r5)))) (value none) (body semicolon)))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "bigVehicle") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r6))) (value none))) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r7)))) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity (lower (expression (span (offset 374) (line 25) (column 23) (len 1)) (integer 6))) (upper (expression (span (offset 374) (line 25) (column 23) (len 1)) (integer 6)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r8)))) (value none) (body semicolon)))))))))
)
~~~
