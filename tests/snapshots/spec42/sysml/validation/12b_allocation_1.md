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
            out torque :> ISQ::torque;
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
    (reference r6 (scope relative) (span (offset 664) (line 28) (column 26) (len 15)) (segments (segment 0 (token "TorqueGenerator") (name "TorqueGenerator") (separator none) (span (offset 664) (line 28) (column 26) (len 15)))))
    (reference r7 (scope relative) (span (offset 758) (line 32) (column 11) (len 16)) (segments (segment 0 (token "torqueGeneration") (name "torqueGeneration") (separator none) (span (offset 758) (line 32) (column 11) (len 16)))))
    (reference r8 (scope relative) (span (offset 778) (line 32) (column 31) (len 15)) (segments (segment 0 (token "torqueGenerator") (name "torqueGenerator") (separator none) (span (offset 778) (line 32) (column 31) (len 15)))))
    (reference r9 (scope relative) (span (offset 921) (line 39) (column 21) (len 10)) (segments (segment 0 (token "PowerTrain") (name "PowerTrain") (separator none) (span (offset 921) (line 39) (column 21) (len 10)))))
  )
  (root (package (name "12b-Allocation-1") (body brace (import (target (span (span (offset 45) (line 2) (column 17) (len 5))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 47) (line 2) (column 19) (len 3))) (separator (span (offset 47) (line 2) (column 19) (len 2))) (marker (span (offset 49) (line 2) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 68) (line 3) (column 17) (len 19))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 84) (line 3) (column 33) (len 3))) (separator (span (offset 84) (line 3) (column 33) (len 2))) (marker (span (offset 86) (line 3) (column 35) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 105) (line 4) (column 17) (len 15))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 117) (line 4) (column 29) (len 3))) (separator (span (offset 117) (line 4) (column 29) (len 2))) (marker (span (offset 119) (line 4) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 138) (line 5) (column 17) (len 16))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 151) (line 5) (column 30) (len 3))) (separator (span (offset 151) (line 5) (column 30) (len 2))) (marker (span (offset 153) (line 5) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "RequirementModel") (body brace (requirement-usage (name "torqueGeneration") (multiplicity none)))) (package (name "LogicalModel") (body brace (action-def (name "GenerateTorque") (specializes none) (body brace (in-out (direction out) (reference false) (declaration "torque") (subsets (relationship (kind subsets) (implied false) (targets (ref r4)))) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 400) (line 17) (column 31) (len 26))))) (part-def (name "LogicalElement") (body semicolon)) (part-def (name "TorqueGenerator") (body brace (perform (declaration "generateTorque") (action none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (subsets none) (redefines none) (body semicolon)))) (action-usage (name "providePower") (short-name none) (body brace (action-usage (name "generateTorque") (short-name none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "torqueGenerator") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (perform))) (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r7))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r8)) (body semicolon)))) (package (name "PhysicalModel") (body brace (part-def (name "PhysicalElement") (body semicolon)) (part-def (name "PowerTrain") (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "powerTrain") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (perform))))))) (allocation-def (name "LogicalToPhysical") (modifiers)) (allocation-usage))))
)
~~~
