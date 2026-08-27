# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 05 (Redefinition): Redefinition Example"))
~~~
# SOURCE
~~~sysml
package 'Redefinition Example' {

	part def Vehicle {
		part eng : Engine;
	}
	part def SmallVehicle :> Vehicle {
		part smallEng : SmallEngine redefines eng;
	}
	part def BigVehicle :> Vehicle {
		part bigEng : BigEngine :>> eng;
	}

	part def Engine {
		part cyl : Cylinder[4..6];
	}
	part def SmallEngine :> Engine {
		part redefines cyl[4];
	}
	part def BigEngine :> Engine {
		part redefines cyl[6];
	}

	part def Cylinder;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "05_redefinition_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Redefinition Example' {
    part def Vehicle {
        part eng : Engine;
    }
    part def SmallVehicle :> Vehicle {
        part smallEng : SmallEngine redefines eng;
    }
    part def BigVehicle :> Vehicle {
        part bigEng : BigEngine :>> eng;
    }
    part def Engine {
        part cyl : Cylinder[4..6];
    }
    part def SmallEngine :> Engine {
        part redefines cyl[4];
    }
    part def BigEngine :> Engine {
        part redefines cyl[6];
    }
    part def Cylinder;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 67) (line 4) (column 14) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 67) (line 4) (column 14) (len 6)))))
    (reference r1 (scope relative) (span (offset 132) (line 7) (column 19) (len 11)) (segments (segment 0 (token "SmallEngine") (name "SmallEngine") (separator none) (span (offset 132) (line 7) (column 19) (len 11)))))
    (reference r2 (scope relative) (span (offset 154) (line 7) (column 41) (len 3)) (segments (segment 0 (token "eng") (name "eng") (separator none) (span (offset 154) (line 7) (column 41) (len 3)))))
    (reference r3 (scope relative) (span (offset 212) (line 10) (column 17) (len 9)) (segments (segment 0 (token "BigEngine") (name "BigEngine") (separator none) (span (offset 212) (line 10) (column 17) (len 9)))))
    (reference r4 (scope relative) (span (offset 226) (line 10) (column 31) (len 3)) (segments (segment 0 (token "eng") (name "eng") (separator none) (span (offset 226) (line 10) (column 31) (len 3)))))
    (reference r5 (scope relative) (span (offset 267) (line 14) (column 14) (len 8)) (segments (segment 0 (token "Cylinder") (name "Cylinder") (separator none) (span (offset 267) (line 14) (column 14) (len 8)))))
    (reference r6 (scope relative) (span (offset 337) (line 17) (column 18) (len 3)) (segments (segment 0 (token "cyl") (name "cyl") (separator none) (span (offset 337) (line 17) (column 18) (len 3)))))
    (reference r7 (scope relative) (span (offset 397) (line 20) (column 18) (len 3)) (segments (segment 0 (token "cyl") (name "cyl") (separator none) (span (offset 397) (line 20) (column 18) (len 3)))))
  )
  (root (package (name "Redefinition Example") (body brace (part-def (name "Vehicle") (modifiers) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "eng") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "SmallVehicle") (modifiers) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "smallEng") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (value none) (body semicolon)))) (part-def (name "BigVehicle") (modifiers) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "bigEng") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (value none) (body semicolon)))) (part-def (name "Engine") (modifiers) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "cyl") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity (lower (expression (span (offset 276) (line 14) (column 23) (len 1)) (integer 4))) (upper (expression (span (offset 279) (line 14) (column 26) (len 1)) (integer 6)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "SmallEngine") (modifiers) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity (lower (expression (span (offset 341) (line 17) (column 22) (len 1)) (integer 4))) (upper (expression (span (offset 341) (line 17) (column 22) (len 1)) (integer 4)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r6)))) (value none) (body semicolon)))) (part-def (name "BigEngine") (modifiers) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity (lower (expression (span (offset 401) (line 20) (column 22) (len 1)) (integer 6))) (upper (expression (span (offset 401) (line 20) (column 22) (len 1)) (integer 6)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r7)))) (value none) (body semicolon)))) (part-def (name "Cylinder") (modifiers) (body semicolon)))))
)
~~~
