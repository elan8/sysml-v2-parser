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
                generator.generateTorque.torque > 0.0[N * m]
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
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 45) (line 2) (column 17) (len 2)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 45) (line 2) (column 17) (len 2)))))
    (reference r1 (scope relative) (span (offset 68) (line 3) (column 17) (len 16)) (segments (segment 0 (token "RequirementModel") (name "RequirementModel") (separator none) (span (offset 68) (line 3) (column 17) (len 16)))))
    (reference r2 (scope relative) (span (offset 105) (line 4) (column 17) (len 12)) (segments (segment 0 (token "LogicalModel") (name "LogicalModel") (separator none) (span (offset 105) (line 4) (column 17) (len 12)))))
    (reference r3 (scope relative) (span (offset 138) (line 5) (column 17) (len 13)) (segments (segment 0 (token "PhysicalModel") (name "PhysicalModel") (separator none) (span (offset 138) (line 5) (column 17) (len 13)))))
    (reference r4 (scope relative) (span (offset 241) (line 9) (column 23) (len 15)) (segments (segment 0 (token "TorqueGenerator") (name "TorqueGenerator") (separator none) (span (offset 241) (line 9) (column 23) (len 15)))))
    (reference r5 (scope relative) (span (offset 288) (line 11) (column 6) (len 9)) (segments (segment 0 (token "generator") (name "generator") (separator none) (span (offset 288) (line 11) (column 6) (len 9)))))
    (reference r6 (scope relative) (span (offset 298) (line 11) (column 16) (len 14)) (segments (segment 0 (token "generateTorque") (name "generateTorque") (separator none) (span (offset 298) (line 11) (column 16) (len 14)))))
    (reference r7 (scope relative) (span (offset 313) (line 11) (column 31) (len 6)) (segments (segment 0 (token "torque") (name "torque") (separator none) (span (offset 313) (line 11) (column 31) (len 6)))))
    (reference r8 (scope relative) (span (offset 327) (line 11) (column 45) (len 1)) (segments (segment 0 (token "N") (name "N") (separator none) (span (offset 327) (line 11) (column 45) (len 1)))))
    (reference r9 (scope relative) (span (offset 329) (line 11) (column 47) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 329) (line 11) (column 47) (len 1)))))
    (reference r10 (scope relative) (span (offset 414) (line 17) (column 45) (len 11)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 414) (line 17) (column 45) (len 3))) (segment 1 (token "torque") (name "torque") (separator colon-colon) (span (offset 419) (line 17) (column 50) (len 6)))))
    (reference r11 (scope relative) (span (offset 541) (line 21) (column 36) (len 14)) (segments (segment 0 (token "GenerateTorque") (name "GenerateTorque") (separator none) (span (offset 541) (line 21) (column 36) (len 14)))))
    (reference r12 (scope relative) (span (offset 616) (line 25) (column 28) (len 14)) (segments (segment 0 (token "GenerateTorque") (name "GenerateTorque") (separator none) (span (offset 616) (line 25) (column 28) (len 14)))))
    (reference r13 (scope relative) (span (offset 664) (line 28) (column 26) (len 15)) (segments (segment 0 (token "TorqueGenerator") (name "TorqueGenerator") (separator none) (span (offset 664) (line 28) (column 26) (len 15)))))
    (reference r14 (scope relative) (span (offset 693) (line 29) (column 12) (len 27)) (segments (segment 0 (token "providePower") (name "providePower") (separator none) (span (offset 693) (line 29) (column 12) (len 12))) (segment 1 (token "generateTorque") (name "generateTorque") (separator dot) (span (offset 706) (line 29) (column 25) (len 14)))))
    (reference r15 (scope relative) (span (offset 725) (line 29) (column 44) (len 14)) (segments (segment 0 (token "generateTorque") (name "generateTorque") (separator none) (span (offset 725) (line 29) (column 44) (len 14)))))
    (reference r16 (scope relative) (span (offset 758) (line 32) (column 11) (len 16)) (segments (segment 0 (token "torqueGeneration") (name "torqueGeneration") (separator none) (span (offset 758) (line 32) (column 11) (len 16)))))
    (reference r17 (scope relative) (span (offset 778) (line 32) (column 31) (len 15)) (segments (segment 0 (token "torqueGenerator") (name "torqueGenerator") (separator none) (span (offset 778) (line 32) (column 31) (len 15)))))
    (reference r18 (scope relative) (span (offset 921) (line 39) (column 21) (len 10)) (segments (segment 0 (token "PowerTrain") (name "PowerTrain") (separator none) (span (offset 921) (line 39) (column 21) (len 10)))))
    (reference r19 (scope relative) (span (offset 963) (line 41) (column 13) (len 27)) (segments (segment 0 (token "providePower") (name "providePower") (separator none) (span (offset 963) (line 41) (column 13) (len 12))) (segment 1 (token "generateTorque") (name "generateTorque") (separator dot) (span (offset 976) (line 41) (column 26) (len 14)))))
    (reference r20 (scope relative) (span (offset 1058) (line 47) (column 17) (len 14)) (segments (segment 0 (token "LogicalElement") (name "LogicalElement") (separator none) (span (offset 1058) (line 47) (column 17) (len 14)))))
    (reference r21 (scope relative) (span (offset 1091) (line 48) (column 18) (len 15)) (segments (segment 0 (token "PhysicalElement") (name "PhysicalElement") (separator none) (span (offset 1091) (line 48) (column 18) (len 15)))))
    (reference r22 (scope relative) (span (offset 1142) (line 51) (column 30) (len 17)) (segments (segment 0 (token "LogicalToPhysical") (name "LogicalToPhysical") (separator none) (span (offset 1142) (line 51) (column 30) (len 17)))))
    (reference r23 (scope relative) (span (offset 1172) (line 52) (column 12) (len 7)) (segments (segment 0 (token "logical") (name "logical") (separator none) (span (offset 1172) (line 52) (column 12) (len 7)))))
    (reference r24 (scope relative) (span (offset 1184) (line 52) (column 24) (len 15)) (segments (segment 0 (token "torqueGenerator") (name "torqueGenerator") (separator none) (span (offset 1184) (line 52) (column 24) (len 15)))))
    (reference r25 (scope relative) (span (offset 1203) (line 52) (column 43) (len 8)) (segments (segment 0 (token "physical") (name "physical") (separator none) (span (offset 1203) (line 52) (column 43) (len 8)))))
    (reference r26 (scope relative) (span (offset 1216) (line 52) (column 56) (len 10)) (segments (segment 0 (token "powerTrain") (name "powerTrain") (separator none) (span (offset 1216) (line 52) (column 56) (len 10)))))
  )
  (root (package (name "12b-Allocation-1") (body brace (import (visibility private) (target (span (span (offset 45) (line 2) (column 17) (len 5))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 47) (line 2) (column 19) (len 3))) (separator (span (offset 47) (line 2) (column 19) (len 2))) (marker (span (offset 49) (line 2) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none)))) (body semicolon)) (import (visibility private) (target (span (span (offset 68) (line 3) (column 17) (len 19))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 84) (line 3) (column 33) (len 3))) (separator (span (offset 84) (line 3) (column 33) (len 2))) (marker (span (offset 86) (line 3) (column 35) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none)))) (body semicolon)) (import (visibility private) (target (span (span (offset 105) (line 4) (column 17) (len 15))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 117) (line 4) (column 29) (len 3))) (separator (span (offset 117) (line 4) (column 29) (len 2))) (marker (span (offset 119) (line 4) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none)))) (body semicolon)) (import (visibility private) (target (span (span (offset 138) (line 5) (column 17) (len 16))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 151) (line 5) (column 30) (len 3))) (separator (span (offset 151) (line 5) (column 30) (len 2))) (marker (span (offset 153) (line 5) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none)))) (body semicolon)) (package (name "RequirementModel") (body brace (requirement-usage (name "torqueGeneration") (short-name none) (typing none) (abstract false) (variation false) none (subsets none) (references none) (body brace (subject (name "generator") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (require-constraint (kind require) (constraint-keyword true) (name none) (target none) (specializations) (value none) (body brace (expression (span (offset 288) (line 11) (column 6) (len 43)) (binary (operator ">") (left (expression (span (offset 288) (line 11) (column 6) (len 31)) (member-access (base (expression (span (offset 288) (line 11) (column 6) (len 24)) (member-access (base (expression (span (offset 288) (line 11) (column 6) (len 9)) (ref r5))) (separator dot) (member (ref r6))))) (separator dot) (member (ref r7))))) (right (expression (span (offset 322) (line 11) (column 40) (len 9)) (bracket (base (expression (span (offset 322) (line 11) (column 40) (len 3)) (real "0.0"))) (operands (sequence-list (element first (expression (span (offset 327) (line 11) (column 45) (len 3)) (binary (operator "*") (left (expression (span (offset 327) (line 11) (column 45) (len 1)) (ref r8))) (right (expression (span (offset 329) (line 11) (column 47) (len 1)) (ref r9))))))))))))))))))) (package (name "LogicalModel") (body brace (action-def (name "GenerateTorque") (modifiers) (specializes none) (body brace (in-out (direction out) (kind none) (reference false) (declaration "torque") (short-name none) (subsets (relationship (kind subsets) (implied false) (targets (ref r10)))) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 400) (line 17) (column 31) (len 26))))) (part-def (name "LogicalElement") (modifiers) (body semicolon)) (part-def (name "TorqueGenerator") (modifiers) (body brace (perform (target (action (name "generateTorque") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (value none) (body semicolon)))) (action-usage (keyword action) (name "providePower") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (action-usage (keyword action) (name "generateTorque") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "torqueGenerator") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (perform (target (reference (action (ref r14)) (specialization (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r15)))) (references none) (crosses none) (intersects none)))) (value none) (body semicolon)))) (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r16))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r17)) (body semicolon)))) (package (name "PhysicalModel") (body brace (part-def (name "PhysicalElement") (modifiers) (body semicolon)) (part-def (name "PowerTrain") (modifiers) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "powerTrain") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r18)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (perform (target (reference (action (ref r19)) (specialization none))) (value none) (body semicolon)))))))) (allocation-def (name "LogicalToPhysical") (modifiers) (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions) (short-name none) (identity (declaration (name "logical") (span (offset 1048) (line 47) (column 7) (len 7)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r20)))) (references none) (multiplicity none) (redefines none) (crosses none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions) (short-name none) (identity (declaration (name "physical") (span (offset 1080) (line 48) (column 7) (len 8)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r21)))) (references none) (multiplicity none) (redefines none) (crosses none)))) (allocation-usage (name "torqueGenAlloc") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r22)))) (ends (connector-end (multiplicity none) (target (ref r23)) (references (ref r24))) (connector-end (multiplicity none) (target (ref r25)) (references (ref r26)))) (body brace (allocate))))))
)
~~~
