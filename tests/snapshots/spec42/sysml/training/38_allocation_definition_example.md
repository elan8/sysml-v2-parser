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
    (reference r0 (scope relative) (span (offset 427) (line 20) (column 18) (len 12)) (segments (segment 0 (token "LogicalModel") (name "LogicalModel") (separator none) (span (offset 427) (line 20) (column 18) (len 12)))))
  )
  (root (package (name "Allocation Definition Example") (body (package (name "LogicalModel") (body (action-def (name "ProvidePower") (specializes none) (body semicolon)) (action-def (name "GenerateTorque") (specializes none) (body semicolon)) (part-def (name "LogicalElement") (body semicolon)) (part-def (name "TorqueGenerator") (body semicolon)) (action-usage) (part-usage))) (package (name "PhysicalModel") (body (import (target (span (span (offset 427) (line 20) (column 18) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 439) (line 20) (column 30) (len 3))) (separator (span (offset 439) (line 20) (column 30) (len 2))) (marker (span (offset 441) (line 20) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "PhysicalElement") (body semicolon)) (part-def (name "PowerTrain") (body semicolon)) (part-usage) (allocation-def) (allocation-usage))))))
)
~~~
