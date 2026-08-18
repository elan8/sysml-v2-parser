# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 04 (Subsetting): Subsetting Example"))
~~~
# SOURCE
~~~sysml
package 'Subsetting Example' {
	
	part def Vehicle {
		part parts : VehiclePart[*];
		
		part eng : Engine subsets parts;
		part trans : Transmission subsets parts;
		part wheels : Wheel[4] :> parts;
	}
	
	abstract part def VehiclePart;
	part def Engine :> VehiclePart;
	part def Transmission :> VehiclePart;
	part def Wheel :> VehiclePart;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "04_subsetting_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Subsetting Example' {
    part def Vehicle {
        part parts : VehiclePart[*];
        part eng : Engine :> parts;
        part trans : Transmission :> parts;
        part wheels : Wheel[4] :> parts;
    }
    abstract part def VehiclePart;
    part def Engine :> VehiclePart;
    part def Transmission :> VehiclePart;
    part def Wheel :> VehiclePart;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 68) (line 4) (column 16) (len 11)) (segments (segment 0 (token "VehiclePart") (name "VehiclePart") (separator none) (span (offset 68) (line 4) (column 16) (len 11)))))
    (reference r1 (scope relative) (span (offset 100) (line 6) (column 14) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 100) (line 6) (column 14) (len 6)))))
    (reference r2 (scope relative) (span (offset 115) (line 6) (column 29) (len 5)) (segments (segment 0 (token "parts") (name "parts") (separator none) (span (offset 115) (line 6) (column 29) (len 5)))))
    (reference r3 (scope relative) (span (offset 137) (line 7) (column 16) (len 12)) (segments (segment 0 (token "Transmission") (name "Transmission") (separator none) (span (offset 137) (line 7) (column 16) (len 12)))))
    (reference r4 (scope relative) (span (offset 158) (line 7) (column 37) (len 5)) (segments (segment 0 (token "parts") (name "parts") (separator none) (span (offset 158) (line 7) (column 37) (len 5)))))
    (reference r5 (scope relative) (span (offset 181) (line 8) (column 17) (len 5)) (segments (segment 0 (token "Wheel") (name "Wheel") (separator none) (span (offset 181) (line 8) (column 17) (len 5)))))
    (reference r6 (scope relative) (span (offset 193) (line 8) (column 29) (len 5)) (segments (segment 0 (token "parts") (name "parts") (separator none) (span (offset 193) (line 8) (column 29) (len 5)))))
  )
  (root (package (name "Subsetting Example") (body brace (part-def (name "Vehicle") (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "parts") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "eng") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r2))) (value none))) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "trans") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r4))) (value none))) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "wheels") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity (lower (expression (span (offset 187) (line 8) (column 23) (len 1)) (integer 4))) (upper (expression (span (offset 187) (line 8) (column 23) (len 1)) (integer 4)))) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r6))) (value none))) (redefines none) (value none) (body semicolon)))) (part-def (name "VehiclePart") (body semicolon)) (part-def (name "Engine") (body semicolon)) (part-def (name "Transmission") (body semicolon)) (part-def (name "Wheel") (body semicolon)))))
)
~~~
