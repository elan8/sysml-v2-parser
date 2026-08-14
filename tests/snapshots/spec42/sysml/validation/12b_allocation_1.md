# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (12-Dependency Relationships): 12b-Allocation-1"))
~~~
# SOURCE
~~~sysml
package '12b-Allocation-1' {
	private import SI::*;
	private import RequirementModel::*;
	private import LogicalModel::*;
	private import PhysicalModel::*;
	
	package RequirementModel {
		requirement torqueGeneration {
			subject generator: TorqueGenerator;
			require constraint { 
				 generator.generateTorque.torque > 0.0 [N*m]
			}
		}
	}
	
	package LogicalModel {
		action def GenerateTorque { out torque :> ISQ::torque; }
		
		part def LogicalElement;
		part def TorqueGenerator :> LogicalElement {
			perform action generateTorque : GenerateTorque;
		}	
		
		action providePower {
			action generateTorque : GenerateTorque;
		}
		
		part torqueGenerator : TorqueGenerator {
			perform providePower.generateTorque :>> generateTorque;
		}
		
		satisfy torqueGeneration by torqueGenerator;			
	}
	
	package PhysicalModel {
		part def PhysicalElement;
		part def PowerTrain :> PhysicalElement;
		
		part powerTrain : PowerTrain {
			part engine {
				perform providePower.generateTorque;
			}
		}
	}
	
	allocation def LogicalToPhysical {
		end logical : LogicalElement;
		end physical : PhysicalElement;
	}
	
	allocation torqueGenAlloc : LogicalToPhysical 
		allocate logical ::> torqueGenerator to physical ::> powerTrain {
			
		allocate torqueGenerator.generateTorque to powerTrain.engine.generateTorque;		
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "12b_allocation_1.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '12b-Allocation-1' {
    private import SI::*;
    private import RequirementModel::*;
    private import LogicalModel::*;
    private import PhysicalModel::*;
    package RequirementModel {
        requirement torqueGeneration {
            subject generator : TorqueGenerator;
            require constraint {
                generator.generateTorque.torque > 0.0 ['N*m'];
            }
        }
    }
    package LogicalModel {
        action def GenerateTorque {
            out torque : ISQ::torque;
        }
        part def LogicalElement;
        part def TorqueGenerator :> LogicalElement {
            perform action generateTorque : GenerateTorque;
        }
        action providePower {
            action generateTorque : GenerateTorque;
        }
        part torqueGenerator : TorqueGenerator {
            perform providePower.generateTorque :>> generateTorque;
        }
        satisfy torqueGeneration by torqueGenerator;
    }
    package PhysicalModel {
        part def PhysicalElement;
        part def PowerTrain :> PhysicalElement;
        part powerTrain : PowerTrain {
            part engine {
                perform providePower.generateTorque;
            }
        }
    }
    allocation def LogicalToPhysical {
        end logical : LogicalElement;
        end physical : PhysicalElement;
    }
    allocation torqueGenAlloc : LogicalToPhysical allocate logical references torqueGenerator to physical references powerTrain {
        allocate torqueGenerator.generateTorque to powerTrain.engine.generateTorque;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 45) (line 2) (column 17) (len 2)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 45) (line 2) (column 17) (len 2)))))
    (reference r1 (scope relative) (span (offset 68) (line 3) (column 17) (len 16)) (segments (segment 0 (token "RequirementModel") (name "RequirementModel") (separator none) (span (offset 68) (line 3) (column 17) (len 16)))))
    (reference r2 (scope relative) (span (offset 105) (line 4) (column 17) (len 12)) (segments (segment 0 (token "LogicalModel") (name "LogicalModel") (separator none) (span (offset 105) (line 4) (column 17) (len 12)))))
    (reference r3 (scope relative) (span (offset 138) (line 5) (column 17) (len 13)) (segments (segment 0 (token "PhysicalModel") (name "PhysicalModel") (separator none) (span (offset 138) (line 5) (column 17) (len 13)))))
    (reference r4 (scope relative) (span (offset 414) (line 17) (column 45) (len 11)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 414) (line 17) (column 45) (len 3))) (segment 1 (token "torque") (name "torque") (separator colon-colon) (span (offset 419) (line 17) (column 50) (len 6)))))
    (reference r5 (scope relative) (span (offset 541) (line 21) (column 36) (len 14)) (segments (segment 0 (token "GenerateTorque") (name "GenerateTorque") (separator none) (span (offset 541) (line 21) (column 36) (len 14)))))
  )
  (root (package (name "12b-Allocation-1") (body (import (target (span (span (offset 45) (line 2) (column 17) (len 5))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 47) (line 2) (column 19) (len 3))) (separator (span (offset 47) (line 2) (column 19) (len 2))) (marker (span (offset 49) (line 2) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 68) (line 3) (column 17) (len 19))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 84) (line 3) (column 33) (len 3))) (separator (span (offset 84) (line 3) (column 33) (len 2))) (marker (span (offset 86) (line 3) (column 35) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 105) (line 4) (column 17) (len 15))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 117) (line 4) (column 29) (len 3))) (separator (span (offset 117) (line 4) (column 29) (len 2))) (marker (span (offset 119) (line 4) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 138) (line 5) (column 17) (len 16))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 151) (line 5) (column 30) (len 3))) (separator (span (offset 151) (line 5) (column 30) (len 2))) (marker (span (offset 153) (line 5) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "RequirementModel") (body (requirement-usage))) (package (name "LogicalModel") (body (action-def (name "GenerateTorque") (specializes none) (body (in-out (direction out) (reference false) (declaration "torque") (type (ref r4)) (multiplicity none) (ordered false) (nonunique false) (redefines none) (value none) (span (offset 400) (line 17) (column 31) (len 26))))) (part-def (name "LogicalElement") (body semicolon)) (part-def (name "TorqueGenerator") (body (perform (declaration "generateTorque") (action none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (subsets none) (redefines none) (body semicolon)))) (action-usage) (part-usage) (satisfy))) (package (name "PhysicalModel") (body (part-def (name "PhysicalElement") (body semicolon)) (part-def (name "PowerTrain") (body semicolon)) (part-usage))) (allocation-def) (allocation-usage))))
)
~~~
