# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 36 (Variability): Variation Configuration"))
~~~
# SOURCE
~~~sysml
package 'Variation Configuration' {
	private import 'Variation Usages'::*;
	
	part vehicle4Cyl :> vehicleFamily {
		part redefines engine = engine::'4cylEngine';
		part redefines transmission = transmission::manualTransmission;
	}
	
	part vehicle6Cyl :> vehicleFamily {
		part redefines engine = engine::'6cylEngine';
		part redefines transmission = transmission::manualTransmission;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "36_variation_configuration.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Variation Configuration' {
    private import 'Variation Usages'::*;
    part vehicle4Cyl :> vehicleFamily {
        part :>> engine = engine::'4cylEngine';
        part :>> transmission = transmission::manualTransmission;
    }
    part vehicle6Cyl :> vehicleFamily {
        part :>> engine = engine::'6cylEngine';
        part :>> transmission = transmission::manualTransmission;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 52) (line 2) (column 17) (len 18)) (segments (segment 0 (token "'Variation Usages'") (name "Variation Usages") (separator none) (span (offset 52) (line 2) (column 17) (len 18)))))
    (reference r1 (scope relative) (span (offset 98) (line 4) (column 22) (len 13)) (segments (segment 0 (token "vehicleFamily") (name "vehicleFamily") (separator none) (span (offset 98) (line 4) (column 22) (len 13)))))
    (reference r2 (scope relative) (span (offset 131) (line 5) (column 18) (len 6)) (segments (segment 0 (token "engine") (name "engine") (separator none) (span (offset 131) (line 5) (column 18) (len 6)))))
    (reference r3 (scope relative) (span (offset 140) (line 5) (column 27) (len 20)) (segments (segment 0 (token "engine") (name "engine") (separator none) (span (offset 140) (line 5) (column 27) (len 6))) (segment 1 (token "'4cylEngine'") (name "4cylEngine") (separator colon-colon) (span (offset 148) (line 5) (column 35) (len 12)))))
    (reference r4 (scope relative) (span (offset 179) (line 6) (column 18) (len 12)) (segments (segment 0 (token "transmission") (name "transmission") (separator none) (span (offset 179) (line 6) (column 18) (len 12)))))
    (reference r5 (scope relative) (span (offset 194) (line 6) (column 33) (len 32)) (segments (segment 0 (token "transmission") (name "transmission") (separator none) (span (offset 194) (line 6) (column 33) (len 12))) (segment 1 (token "manualTransmission") (name "manualTransmission") (separator colon-colon) (span (offset 208) (line 6) (column 47) (len 18)))))
    (reference r6 (scope relative) (span (offset 254) (line 9) (column 22) (len 13)) (segments (segment 0 (token "vehicleFamily") (name "vehicleFamily") (separator none) (span (offset 254) (line 9) (column 22) (len 13)))))
    (reference r7 (scope relative) (span (offset 287) (line 10) (column 18) (len 6)) (segments (segment 0 (token "engine") (name "engine") (separator none) (span (offset 287) (line 10) (column 18) (len 6)))))
    (reference r8 (scope relative) (span (offset 296) (line 10) (column 27) (len 20)) (segments (segment 0 (token "engine") (name "engine") (separator none) (span (offset 296) (line 10) (column 27) (len 6))) (segment 1 (token "'6cylEngine'") (name "6cylEngine") (separator colon-colon) (span (offset 304) (line 10) (column 35) (len 12)))))
    (reference r9 (scope relative) (span (offset 335) (line 11) (column 18) (len 12)) (segments (segment 0 (token "transmission") (name "transmission") (separator none) (span (offset 335) (line 11) (column 18) (len 12)))))
    (reference r10 (scope relative) (span (offset 350) (line 11) (column 33) (len 32)) (segments (segment 0 (token "transmission") (name "transmission") (separator none) (span (offset 350) (line 11) (column 33) (len 12))) (segment 1 (token "manualTransmission") (name "manualTransmission") (separator colon-colon) (span (offset 364) (line 11) (column 47) (len 18)))))
  )
  (root (package (name "Variation Configuration") (body brace (import (target (span (span (offset 52) (line 2) (column 17) (len 21))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 70) (line 2) (column 35) (len 3))) (separator (span (offset 70) (line 2) (column 35) (len 2))) (marker (span (offset 72) (line 2) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle4Cyl") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r1))) (value none))) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 140) (line 5) (column 27) (len 20)) (ref r3))))) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 194) (line 6) (column 33) (len 32)) (ref r5))))) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle6Cyl") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r6))) (value none))) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r7)))) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 296) (line 10) (column 27) (len 20)) (ref r8))))) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r9)))) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 350) (line 11) (column 33) (len 32)) (ref r10))))) (body semicolon)))))))
)
~~~
