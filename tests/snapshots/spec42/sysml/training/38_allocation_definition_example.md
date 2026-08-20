# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 38 (Allocation): Allocation Definition Example"))
~~~
# SOURCE
~~~sysml
package 'Allocation Definition Example' {
	package LogicalModel {
		action def ProvidePower;
		action def GenerateTorque;
		
		part def LogicalElement;
		part def TorqueGenerator :> LogicalElement;
		
		action providePower : ProvidePower {
			action generateTorque : GenerateTorque;
		}
		
		part torqueGenerator : TorqueGenerator {
			perform providePower.generateTorque;
		}
		
	}
	
	package PhysicalModel {
		private import LogicalModel::*;
		
		part def PhysicalElement;
		part def PowerTrain :> PhysicalElement;
		
		part powerTrain : PowerTrain {
			part engine {
				perform providePower.generateTorque;
			}
		}
	
		allocation def LogicalToPhysical {
			end logical : LogicalElement;
			end physical : PhysicalElement;
		}
		
		allocation torqueGenAlloc : LogicalToPhysical allocate torqueGenerator to powerTrain;
	}	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "38_allocation_definition_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Allocation Definition Example' {
    package LogicalModel {
        action def ProvidePower;
        action def GenerateTorque;
        part def LogicalElement;
        part def TorqueGenerator :> LogicalElement;
        action providePower : ProvidePower {
            action generateTorque : GenerateTorque;
        }
        part torqueGenerator : TorqueGenerator {
            perform providePower.generateTorque;
        }
    }
    package PhysicalModel {
        private import LogicalModel::*;
        part def PhysicalElement;
        part def PowerTrain :> PhysicalElement;
        part powerTrain : PowerTrain {
            part engine {
                perform providePower.generateTorque;
            }
        }
        allocation def LogicalToPhysical {
            end logical : LogicalElement;
            end physical : PhysicalElement;
        }
        allocation torqueGenAlloc : LogicalToPhysical allocate torqueGenerator to powerTrain;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 315) (line 13) (column 26) (len 15)) (segments (segment 0 (token "TorqueGenerator") (name "TorqueGenerator") (separator none) (span (offset 315) (line 13) (column 26) (len 15)))))
    (reference r1 (scope relative) (span (offset 344) (line 14) (column 12) (len 27)) (segments (segment 0 (token "providePower") (name "providePower") (separator none) (span (offset 344) (line 14) (column 12) (len 12))) (segment 1 (token "generateTorque") (name "generateTorque") (separator dot) (span (offset 357) (line 14) (column 25) (len 14)))))
    (reference r2 (scope relative) (span (offset 427) (line 20) (column 18) (len 12)) (segments (segment 0 (token "LogicalModel") (name "LogicalModel") (separator none) (span (offset 427) (line 20) (column 18) (len 12)))))
    (reference r3 (scope relative) (span (offset 540) (line 25) (column 21) (len 10)) (segments (segment 0 (token "PowerTrain") (name "PowerTrain") (separator none) (span (offset 540) (line 25) (column 21) (len 10)))))
    (reference r4 (scope relative) (span (offset 582) (line 27) (column 13) (len 27)) (segments (segment 0 (token "providePower") (name "providePower") (separator none) (span (offset 582) (line 27) (column 13) (len 12))) (segment 1 (token "generateTorque") (name "generateTorque") (separator dot) (span (offset 595) (line 27) (column 26) (len 14)))))
  )
  (root (package (name "Allocation Definition Example") (body brace (package (name "LogicalModel") (body brace (action-def (name "ProvidePower") (modifiers) (specializes none) (body semicolon)) (action-def (name "GenerateTorque") (modifiers) (specializes none) (body semicolon)) (part-def (name "LogicalElement") (modifiers) (body semicolon)) (part-def (name "TorqueGenerator") (modifiers) (body semicolon)) (action-usage (name "providePower") (short-name none) (body brace (action-usage (name "generateTorque") (short-name none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "torqueGenerator") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (perform (declaration "") (action (ref r1)) (typing none) (subsets none) (redefines none) (body semicolon)))))) (package (name "PhysicalModel") (body brace (import (target (span (span (offset 427) (line 20) (column 18) (len 15))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 439) (line 20) (column 30) (len 3))) (separator (span (offset 439) (line 20) (column 30) (len 2))) (marker (span (offset 441) (line 20) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "PhysicalElement") (modifiers) (body semicolon)) (part-def (name "PowerTrain") (modifiers) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "powerTrain") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (perform (declaration "") (action (ref r4)) (typing none) (subsets none) (redefines none) (body semicolon)))))) (allocation-def (name "LogicalToPhysical") (modifiers)) (allocation-usage))))))
)
~~~
