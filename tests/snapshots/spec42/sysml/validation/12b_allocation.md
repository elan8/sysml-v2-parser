# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (12-Dependency Relationships): 12b-Allocation"))
~~~
# SOURCE
~~~sysml
package '12b-Allocation' {
	private import LogicalModel::*;
	private import PhysicalModel::*;
	
	package LogicalModel {
		action providePower {
			action generateTorque;
		}
		
		part torqueGenerator {
			perform providePower.generateTorque;
		}
	}
	
	package PhysicalModel {
		part powerTrain {
			part engine {
				perform providePower.generateTorque;
			}
		}
	}
	
	allocate torqueGenerator to powerTrain {
		allocate torqueGenerator.generateTorque to powerTrain.engine.generateTorque;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "12b_allocation.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '12b-Allocation' {
    private import LogicalModel::*;
    private import PhysicalModel::*;
    package LogicalModel {
        action providePower {
            action generateTorque;
        }
        part torqueGenerator {
            perform providePower.generateTorque;
        }
    }
    package PhysicalModel {
        part powerTrain {
            part engine {
                perform providePower.generateTorque;
            }
        }
    }
    allocate torqueGenerator to powerTrain {
        allocate torqueGenerator.generateTorque to powerTrain.engine.generateTorque;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 43) (line 2) (column 17) (len 12)) (segments (segment 0 (token "LogicalModel") (name "LogicalModel") (separator none) (span (offset 43) (line 2) (column 17) (len 12)))))
    (reference r1 (scope relative) (span (offset 76) (line 3) (column 17) (len 13)) (segments (segment 0 (token "PhysicalModel") (name "PhysicalModel") (separator none) (span (offset 76) (line 3) (column 17) (len 13)))))
  )
  (root (package (name "12b-Allocation") (body brace (import (target (span (span (offset 43) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 55) (line 2) (column 29) (len 3))) (separator (span (offset 55) (line 2) (column 29) (len 2))) (marker (span (offset 57) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 76) (line 3) (column 17) (len 16))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 89) (line 3) (column 30) (len 3))) (separator (span (offset 89) (line 3) (column 30) (len 2))) (marker (span (offset 91) (line 3) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "LogicalModel") (body brace (action-usage (name "providePower") (short-name none) (body brace (action-usage (name "generateTorque") (short-name none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "torqueGenerator") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (perform))))) (package (name "PhysicalModel") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "powerTrain") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (perform))))))) (allocation-usage))))
)
~~~
