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
    individual  : Vehicle_1 {
        timeslice aliceDriving {
            ref individual item : Alice :>> driver;
            snapshot  :>> start {
                attribute  :>> mass = 2000.0;
            }
            snapshot  :>> done {
                attribute  :>> mass = 1500.0;
            }
        }
        then timeslice bobDriving {
            ref individual item : Bob :>> driver;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 56) (line 2) (column 17) (len 35)) (segments (segment 0 (token "'Individuals and Snapshots Example'") (name "Individuals and Snapshots Example") (separator none) (span (offset 56) (line 2) (column 17) (len 35)))))
  )
  (root (package (name "Individuals and Time Slices") (body (import (target (span (span (offset 56) (line 2) (column 17) (len 38))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 91) (line 2) (column 52) (len 3))) (separator (span (offset 91) (line 2) (column 52) (len 2))) (marker (span (offset 93) (line 2) (column 54) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (item-def) (item-def) (occurrence (declaration "") (target none)))))
)
~~~
