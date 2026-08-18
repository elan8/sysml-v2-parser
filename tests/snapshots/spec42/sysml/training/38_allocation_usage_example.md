# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 38 (Allocation): Allocation Usage Example"))
~~~
# SOURCE
~~~sysml
package 'Allocation Usage Example' {
	package LogicalModel {
		action def ProvidePower;
		action def GenerateTorque;
		
		part def TorqueGenerator;
		
		action providePower : ProvidePower {
			action generateTorque : GenerateTorque;
		}
		
		part torqueGenerator : TorqueGenerator {
			perform providePower.generateTorque;
		}
	}
	
	package PhysicalModel {
		private import LogicalModel::*;
	
		part def PowerTrain;
		part def Engine;
		
		part powerTrain : PowerTrain {
			part engine : Engine {
				perform providePower.generateTorque;
			}
		}
		
		allocate torqueGenerator to powerTrain {
			allocate torqueGenerator.generateTorque to powerTrain.engine.generateTorque;
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "38_allocation_usage_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Allocation Usage Example' {
    package LogicalModel {
        action def ProvidePower;
        action def GenerateTorque;
        part def TorqueGenerator;
        action providePower : ProvidePower {
            action generateTorque : GenerateTorque;
        }
        part torqueGenerator : TorqueGenerator {
            perform providePower.generateTorque;
        }
    }
    package PhysicalModel {
        private import LogicalModel::*;
        part def PowerTrain;
        part def Engine;
        part powerTrain : PowerTrain {
            part engine : Engine {
                perform providePower.generateTorque;
            }
        }
        allocate torqueGenerator to powerTrain {
            allocate torqueGenerator.generateTorque to powerTrain.engine.generateTorque;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 265) (line 12) (column 26) (len 15)) (segments (segment 0 (token "TorqueGenerator") (name "TorqueGenerator") (separator none) (span (offset 265) (line 12) (column 26) (len 15)))))
    (reference r1 (scope relative) (span (offset 374) (line 18) (column 18) (len 12)) (segments (segment 0 (token "LogicalModel") (name "LogicalModel") (separator none) (span (offset 374) (line 18) (column 18) (len 12)))))
    (reference r2 (scope relative) (span (offset 458) (line 23) (column 21) (len 10)) (segments (segment 0 (token "PowerTrain") (name "PowerTrain") (separator none) (span (offset 458) (line 23) (column 21) (len 10)))))
    (reference r3 (scope relative) (span (offset 488) (line 24) (column 18) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 488) (line 24) (column 18) (len 6)))))
  )
  (root (package (name "Allocation Usage Example") (body brace (package (name "LogicalModel") (body brace (action-def (name "ProvidePower") (specializes none) (body semicolon)) (action-def (name "GenerateTorque") (specializes none) (body semicolon)) (part-def (name "TorqueGenerator") (body semicolon)) (action-usage (name "providePower") (short-name none) (body brace (action-usage (name "generateTorque") (short-name none) (body semicolon)))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "torqueGenerator") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (perform))))) (package (name "PhysicalModel") (body brace (import (target (span (span (offset 374) (line 18) (column 18) (len 15))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 386) (line 18) (column 30) (len 3))) (separator (span (offset 386) (line 18) (column 30) (len 2))) (marker (span (offset 388) (line 18) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "PowerTrain") (body semicolon)) (part-def (name "Engine") (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "powerTrain") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (perform))))) (allocation-usage))))))
)
~~~
