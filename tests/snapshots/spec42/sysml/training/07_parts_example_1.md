# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 07 (Parts): Parts Example-1"))
~~~
# SOURCE
~~~sysml
package 'Parts Example-1' {
	
	// Definitions
	
	part def Vehicle {
		part eng : Engine;
	}
	
	part def Engine {
		part cyl : Cylinder[4..6];
	}
	
	part def Cylinder;
	
	// Usages
	
	part smallVehicle : Vehicle {
		part redefines eng {
			part redefines cyl[4];
		}
	}
	
	part bigVehicle : Vehicle {
		part redefines eng {
			part redefines cyl[6];
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "07_parts_example_1.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Parts Example-1' {
    part def Vehicle {
        part eng : Engine;
    }
    part def Engine {
        part cyl : Cylinder[4..6];
    }
    part def Cylinder;
    part smallVehicle : Vehicle {
        part :>> eng {
            part :>> cyl[4];
        }
    }
    part bigVehicle : Vehicle {
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
    (reference r0 (scope relative) (span (offset 81) (line 6) (column 14) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 81) (line 6) (column 14) (len 6)))))
    (reference r1 (scope relative) (span (offset 126) (line 10) (column 14) (len 8)) (segments (segment 0 (token "Cylinder") (name "Cylinder") (separator none) (span (offset 126) (line 10) (column 14) (len 8)))))
    (reference r2 (scope relative) (span (offset 203) (line 17) (column 22) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 203) (line 17) (column 22) (len 7)))))
    (reference r3 (scope relative) (span (offset 230) (line 18) (column 18) (len 3)) (segments (segment 0 (token "eng") (name "eng") (separator none) (span (offset 230) (line 18) (column 18) (len 3)))))
    (reference r4 (scope relative) (span (offset 254) (line 19) (column 19) (len 3)) (segments (segment 0 (token "cyl") (name "cyl") (separator none) (span (offset 254) (line 19) (column 19) (len 3)))))
    (reference r5 (scope relative) (span (offset 290) (line 23) (column 20) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 290) (line 23) (column 20) (len 7)))))
    (reference r6 (scope relative) (span (offset 317) (line 24) (column 18) (len 3)) (segments (segment 0 (token "eng") (name "eng") (separator none) (span (offset 317) (line 24) (column 18) (len 3)))))
    (reference r7 (scope relative) (span (offset 341) (line 25) (column 19) (len 3)) (segments (segment 0 (token "cyl") (name "cyl") (separator none) (span (offset 341) (line 25) (column 19) (len 3)))))
  )
  (root (package (name "Parts Example-1") (body brace (part-def (name "Vehicle") (modifiers) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "eng") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "Engine") (modifiers) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "cyl") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity (lower (expression (span (offset 135) (line 10) (column 23) (len 1)) (integer 4))) (upper (expression (span (offset 138) (line 10) (column 26) (len 1)) (integer 6)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "Cylinder") (modifiers) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "smallVehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r3)))) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity (lower (expression (span (offset 258) (line 19) (column 23) (len 1)) (integer 4))) (upper (expression (span (offset 258) (line 19) (column 23) (len 1)) (integer 4)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (value none) (body semicolon)))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "bigVehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r6)))) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity (lower (expression (span (offset 345) (line 25) (column 23) (len 1)) (integer 6))) (upper (expression (span (offset 345) (line 25) (column 23) (len 1)) (integer 6)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r7)))) (value none) (body semicolon)))))))))
)
~~~
