# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 28 (Individuals): Individuals and Time Slices"))
~~~
# SOURCE
~~~sysml
package 'Individuals and Time Slices' {
	private import 'Individuals and Snapshots Example'::*;
	
	individual item def Alice :> Person;
	individual item def Bob :> Person;
	
	individual : Vehicle_1 {
		
		timeslice aliceDriving {
			ref individual item :>> driver : Alice;

			snapshot :>> start {
				:>> mass = 2000.0;
			}
			
			snapshot :>> done {
				:>> mass = 1500.0;
			}			
		}
		
		then timeslice bobDriving {
			ref individual item :>> driver : Bob;
		}
		
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "28_individuals_and_time_slices.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Individuals and Time Slices' {
    private import 'Individuals and Snapshots Example'::*;
    individual item def Alice :> Person;
    individual item def Bob :> Person;
    individual : Vehicle_1 {
        timeslice aliceDriving {
            ref individual item :>> driver : Alice;
            snapshot :>> start {
                attribute :>> mass = 2000.0;
            }
            snapshot :>> done {
                attribute :>> mass = 1500.0;
            }
        }
        then timeslice bobDriving {
            ref individual item :>> driver : Bob;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 56) (line 2) (column 17) (len 35)) (segments (segment 0 (token "'Individuals and Snapshots Example'") (name "Individuals and Snapshots Example") (separator none) (span (offset 56) (line 2) (column 17) (len 35)))))
    (reference r1 (scope relative) (span (offset 128) (line 4) (column 31) (len 6)) (segments (segment 0 (token "Person") (name "Person") (separator none) (span (offset 128) (line 4) (column 31) (len 6)))))
    (reference r2 (scope relative) (span (offset 164) (line 5) (column 29) (len 6)) (segments (segment 0 (token "Person") (name "Person") (separator none) (span (offset 164) (line 5) (column 29) (len 6)))))
    (reference r3 (scope relative) (span (offset 266) (line 10) (column 37) (len 5)) (segments (segment 0 (token "Alice") (name "Alice") (separator none) (span (offset 266) (line 10) (column 37) (len 5)))))
    (reference r4 (scope relative) (span (offset 257) (line 10) (column 28) (len 6)) (segments (segment 0 (token "driver") (name "driver") (separator none) (span (offset 257) (line 10) (column 28) (len 6)))))
    (reference r5 (scope relative) (span (offset 306) (line 13) (column 9) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 306) (line 13) (column 9) (len 4)))))
    (reference r6 (scope relative) (span (offset 361) (line 17) (column 9) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 361) (line 17) (column 9) (len 4)))))
    (reference r7 (scope relative) (span (offset 457) (line 22) (column 37) (len 3)) (segments (segment 0 (token "Bob") (name "Bob") (separator none) (span (offset 457) (line 22) (column 37) (len 3)))))
    (reference r8 (scope relative) (span (offset 448) (line 22) (column 28) (len 6)) (segments (segment 0 (token "driver") (name "driver") (separator none) (span (offset 448) (line 22) (column 28) (len 6)))))
  )
  (root (package (name "Individuals and Time Slices") (body brace (import (target (span (span (offset 56) (line 2) (column 17) (len 38))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 91) (line 2) (column 52) (len 3))) (separator (span (offset 91) (line 2) (column 52) (len 2))) (marker (span (offset 93) (line 2) (column 54) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (item-def (name "Alice") (individual true) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r1)))) (body semicolon)) (item-def (name "Bob") (individual true) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r2)))) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion none) (extensions)) (declaration "") (short-name none) (target none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion timeslice) (extensions)) (declaration "aliceDriving") (short-name none) (target none) (body brace (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual true) (portion none) (extensions)) (declaration "") (short-name none) (type (ref r3)) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (value none) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration "") (short-name none) (target none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r5)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 313) (line 13) (column 16) (len 6)) (real "2000.0"))))) (body semicolon)))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration "") (short-name none) (target none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r6)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 368) (line 17) (column 16) (len 6)) (real "1500.0"))))) (body semicolon)))))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion timeslice) (extensions)) (declaration "bobDriving") (short-name none) (target none) (body brace (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual true) (portion none) (extensions)) (declaration "") (short-name none) (type (ref r7)) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r8)))) (value none) (body semicolon)))))))))
)
~~~
