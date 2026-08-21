# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (03-Function-based Behavior): 3a-Function-based Behavior-3"))
~~~
# SOURCE
~~~sysml
package '3a-Function-based Behavior-5' {
	public import Definitions::*;
	public import Usages::*;

	package Definitions {
		alias Torque for ISQ::TorqueValue;
		
		// ATTRIBUTE DEFINITIONS
		
		attribute def FuelCmd;
		
		attribute def EngineStart;
		attribute def EngineOff;
		
		// ACTION DEFINITIONS
		
		action def 'Generate Torque' { in fuelCmd: FuelCmd; out engineTorque: Torque; }
		action def 'Amplify Torque' { in engineTorque: Torque; out transmissionTorque: Torque; }
		action def 'Transfer Torque' { in transmissionTorque: Torque; out driveshaftTorque: Torque; }
		action def 'Distribute Torque' { in driveShaftTorque: Torque; out wheelTorque1: Torque; out wheelTorque2: Torque; }
		
		action def 'Provide Power' { in fuelCmd: FuelCmd; out wheelTorque1: Torque; out wheelTorque2: Torque; }
	
	}
	
	package Usages {
	
		action 'provide power': 'Provide Power' {
			// PARAMETERS
			
			in fuelCmd: FuelCmd; 
			out wheelTorque1: Torque; 
			out wheelTorque2: Torque;
		
			loop {
				accept engineStart : EngineStart;
				then action {
					action 'generate torque': 'Generate Torque' {
						in fuelCmd = 'provide power'::fuelCmd;
						out engineTorque: Torque;
					}
					
					flow 'generate torque'.engineTorque 
					    to 'amplify torque'.engineTorque;
					
					action 'amplify torque': 'Amplify Torque' {
						in engineTorque: Torque;
						out transmissionTorque: Torque;
					}
					
					flow 'amplify torque'.transmissionTorque 
					    to 'transfer torque'.transmissionTorque;
					
					action 'transfer torque': 'Transfer Torque' {
						in transmissionTorque: Torque; 
						out driveshaftTorque: Torque;
					}
					
					flow 'transfer torque'.driveshaftTorque 
					    to 'distribute torque'.driveshaftTorque;
					
					action 'distribute torque': 'Distribute Torque' {
						in driveshaftTorque: Torque;
						out wheelTorque1: Torque;
						out wheelTorque2: Torque;
					}
				}
				then action accept engineOff : EngineOff;
			}	
		}
	
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "3a_function_based_behavior_3.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '3a-Function-based Behavior-5' {
    public import Definitions::*;
    public import Usages::*;
    package Definitions {
        alias Torque for ISQ::TorqueValue;
        attribute def FuelCmd;
        attribute def EngineStart;
        attribute def EngineOff;
        action def 'Generate Torque' {
            in fuelCmd : FuelCmd;
            out engineTorque : Torque;
        }
        action def 'Amplify Torque' {
            in engineTorque : Torque;
            out transmissionTorque : Torque;
        }
        action def 'Transfer Torque' {
            in transmissionTorque : Torque;
            out driveshaftTorque : Torque;
        }
        action def 'Distribute Torque' {
            in driveShaftTorque : Torque;
            out wheelTorque1 : Torque;
            out wheelTorque2 : Torque;
        }
        action def 'Provide Power' {
            in fuelCmd : FuelCmd;
            out wheelTorque1 : Torque;
            out wheelTorque2 : Torque;
        }
    }
    package Usages {
        action 'provide power' : 'Provide Power' {
            in fuelCmd : FuelCmd;
            out wheelTorque1 : Torque;
            out wheelTorque2 : Torque;
            loop  {
                accept engineStart : EngineStart;
                then action {
                    action 'generate torque' : 'Generate Torque' {
                        in fuelCmd = 'provide power'::fuelCmd;
                        out engineTorque : Torque;
                    }
                    flow from 'generate torque'.engineTorque to 'amplify torque'.engineTorque;
                    action 'amplify torque' : 'Amplify Torque' {
                        in engineTorque : Torque;
                        out transmissionTorque : Torque;
                    }
                    flow from 'amplify torque'.transmissionTorque to 'transfer torque'.transmissionTorque;
                    action 'transfer torque' : 'Transfer Torque' {
                        in transmissionTorque : Torque;
                        out driveshaftTorque : Torque;
                    }
                    flow from 'transfer torque'.driveshaftTorque to 'distribute torque'.driveshaftTorque;
                    action 'distribute torque' : 'Distribute Torque' {
                        in driveshaftTorque : Torque;
                        out wheelTorque1 : Torque;
                        out wheelTorque2 : Torque;
                    }
                }
                then action accept engineOff : EngineOff;
            }
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 56) (line 2) (column 16) (len 11)) (segments (segment 0 (token "Definitions") (name "Definitions") (separator none) (span (offset 56) (line 2) (column 16) (len 11)))))
    (reference r1 (scope relative) (span (offset 87) (line 3) (column 16) (len 6)) (segments (segment 0 (token "Usages") (name "Usages") (separator none) (span (offset 87) (line 3) (column 16) (len 6)))))
    (reference r2 (scope relative) (span (offset 141) (line 6) (column 20) (len 16)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 141) (line 6) (column 20) (len 3))) (segment 1 (token "TorqueValue") (name "TorqueValue") (separator colon-colon) (span (offset 146) (line 6) (column 25) (len 11)))))
    (reference r3 (scope relative) (span (offset 351) (line 17) (column 46) (len 7)) (segments (segment 0 (token "FuelCmd") (name "FuelCmd") (separator none) (span (offset 351) (line 17) (column 46) (len 7)))))
    (reference r4 (scope relative) (span (offset 378) (line 17) (column 73) (len 6)) (segments (segment 0 (token "Torque") (name "Torque") (separator none) (span (offset 378) (line 17) (column 73) (len 6)))))
    (reference r5 (scope relative) (span (offset 437) (line 18) (column 50) (len 6)) (segments (segment 0 (token "Torque") (name "Torque") (separator none) (span (offset 437) (line 18) (column 50) (len 6)))))
    (reference r6 (scope relative) (span (offset 469) (line 18) (column 82) (len 6)) (segments (segment 0 (token "Torque") (name "Torque") (separator none) (span (offset 469) (line 18) (column 82) (len 6)))))
    (reference r7 (scope relative) (span (offset 535) (line 19) (column 57) (len 6)) (segments (segment 0 (token "Torque") (name "Torque") (separator none) (span (offset 535) (line 19) (column 57) (len 6)))))
    (reference r8 (scope relative) (span (offset 565) (line 19) (column 87) (len 6)) (segments (segment 0 (token "Torque") (name "Torque") (separator none) (span (offset 565) (line 19) (column 87) (len 6)))))
    (reference r9 (scope relative) (span (offset 631) (line 20) (column 57) (len 6)) (segments (segment 0 (token "Torque") (name "Torque") (separator none) (span (offset 631) (line 20) (column 57) (len 6)))))
    (reference r10 (scope relative) (span (offset 657) (line 20) (column 83) (len 6)) (segments (segment 0 (token "Torque") (name "Torque") (separator none) (span (offset 657) (line 20) (column 83) (len 6)))))
    (reference r11 (scope relative) (span (offset 683) (line 20) (column 109) (len 6)) (segments (segment 0 (token "Torque") (name "Torque") (separator none) (span (offset 683) (line 20) (column 109) (len 6)))))
    (reference r12 (scope relative) (span (offset 739) (line 22) (column 44) (len 7)) (segments (segment 0 (token "FuelCmd") (name "FuelCmd") (separator none) (span (offset 739) (line 22) (column 44) (len 7)))))
    (reference r13 (scope relative) (span (offset 766) (line 22) (column 71) (len 6)) (segments (segment 0 (token "Torque") (name "Torque") (separator none) (span (offset 766) (line 22) (column 71) (len 6)))))
    (reference r14 (scope relative) (span (offset 792) (line 22) (column 97) (len 6)) (segments (segment 0 (token "Torque") (name "Torque") (separator none) (span (offset 792) (line 22) (column 97) (len 6)))))
    (reference r15 (scope relative) (span (offset 855) (line 28) (column 27) (len 15)) (segments (segment 0 (token "'Provide Power'") (name "Provide Power") (separator none) (span (offset 855) (line 28) (column 27) (len 15)))))
  )
  (root (package (name "3a-Function-based Behavior-5") (body brace (import (target (span (span (offset 56) (line 2) (column 16) (len 14))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 67) (line 2) (column 27) (len 3))) (separator (span (offset 67) (line 2) (column 27) (len 2))) (marker (span (offset 69) (line 2) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 87) (line 3) (column 16) (len 9))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 93) (line 3) (column 22) (len 3))) (separator (span (offset 93) (line 3) (column 22) (len 2))) (marker (span (offset 95) (line 3) (column 24) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "Definitions") (body brace (alias (name "Torque") (target (ref r2)) (body semicolon)) (attribute-def (declaration-name "FuelCmd") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "EngineStart") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "EngineOff") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (action-def (name "Generate Torque") (modifiers) (specializes none) (body brace (in-out (direction in) (reference false) (declaration "fuelCmd") (subsets none) (type (ref r3)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 339) (line 17) (column 34) (len 20))) (in-out (direction out) (reference false) (declaration "engineTorque") (subsets none) (type (ref r4)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 360) (line 17) (column 55) (len 25))))) (action-def (name "Amplify Torque") (modifiers) (specializes none) (body brace (in-out (direction in) (reference false) (declaration "engineTorque") (subsets none) (type (ref r5)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 420) (line 18) (column 33) (len 24))) (in-out (direction out) (reference false) (declaration "transmissionTorque") (subsets none) (type (ref r6)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 445) (line 18) (column 58) (len 31))))) (action-def (name "Transfer Torque") (modifiers) (specializes none) (body brace (in-out (direction in) (reference false) (declaration "transmissionTorque") (subsets none) (type (ref r7)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 512) (line 19) (column 34) (len 30))) (in-out (direction out) (reference false) (declaration "driveshaftTorque") (subsets none) (type (ref r8)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 543) (line 19) (column 65) (len 29))))) (action-def (name "Distribute Torque") (modifiers) (specializes none) (body brace (in-out (direction in) (reference false) (declaration "driveShaftTorque") (subsets none) (type (ref r9)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 610) (line 20) (column 36) (len 28))) (in-out (direction out) (reference false) (declaration "wheelTorque1") (subsets none) (type (ref r10)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 639) (line 20) (column 65) (len 25))) (in-out (direction out) (reference false) (declaration "wheelTorque2") (subsets none) (type (ref r11)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 665) (line 20) (column 91) (len 25))))) (action-def (name "Provide Power") (modifiers) (specializes none) (body brace (in-out (direction in) (reference false) (declaration "fuelCmd") (subsets none) (type (ref r12)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 727) (line 22) (column 32) (len 20))) (in-out (direction out) (reference false) (declaration "wheelTorque1") (subsets none) (type (ref r13)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 748) (line 22) (column 53) (len 25))) (in-out (direction out) (reference false) (declaration "wheelTorque2") (subsets none) (type (ref r14)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 774) (line 22) (column 79) (len 25))))))) (package (name "Usages") (body brace (action-usage (name "provide power") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (in-out-declaration) (in-out-declaration) (in-out-declaration) (loop))))))))
)
~~~
