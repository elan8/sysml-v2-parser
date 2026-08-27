# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (03-Function-based Behavior): 3a-Function-based Behavior-2"))
~~~
# SOURCE
~~~sysml
package '3a-Function-based Behavior-2' {
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
	
		action 'provide power': 'Provide Power'{
			in fuelCmd: FuelCmd;
			out wheelTorque1: Torque; 
			out wheelTorque2: Torque;

			// ITEM FLOW PART
			
			action 'generate torque': 'Generate Torque'{
				/*
				 * The binding connector shorthand can be used on action parameters.
				 */
				in fuelCmd = 'provide power'::fuelCmd;
			}
			
			flow 'generate torque'.engineTorque 
			    to 'amplify torque'.engineTorque;
			
			action 'amplify torque': 'Amplify Torque';
			
			flow 'amplify torque'.transmissionTorque 
			    to 'transfer torque'.transmissionTorque;
			
			action 'transfer torque': 'Transfer Torque';
			
			flow 'transfer torque'.driveshaftTorque 
			    to 'distribute torque'.driveShaftTorque;
			
			action 'distribute torque': 'Distribute Torque';
			
			// CONTROL FLOW PART

			/*
			 * The following uses a shorthand for a sequence of successions.
			 * The source of the first first is given by "first start",
			 * and the target of each succeeding first is indicated by
			 * using the "then" keyword.
			 */
			first start;
			then merge continue;	
			then action engineStarted accept engineStart: EngineStart;			
			then action engineStopped accept engineOff: EngineOff;	
			then continue;
			
			/* Enable torque generation. */
			first engineStarted then 'generate torque';
			first engineStarted then 'amplify torque';
			first engineStarted then 'transfer torque';
			first engineStarted then 'distribute torque';
			
			/* Disable torque generation. */
			first 'generate torque' then engineStopped;		
			first 'amplify torque' then engineStopped;		
			first 'transfer torque' then engineStopped;		
			first 'distribute torque' then engineStopped;		
		}
	
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "3a_function_based_behavior_2.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '3a-Function-based Behavior-2' {
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
            action 'generate torque' : 'Generate Torque' {
                /*
				 * The binding connector shorthand can be used on action parameters.
				 */
                in fuelCmd = 'provide power'::fuelCmd;
            }
            flow from 'generate torque'.engineTorque to 'amplify torque'.engineTorque;
            action 'amplify torque' : 'Amplify Torque';
            flow from 'amplify torque'.transmissionTorque to 'transfer torque'.transmissionTorque;
            action 'transfer torque' : 'Transfer Torque';
            flow from 'transfer torque'.driveshaftTorque to 'distribute torque'.driveShaftTorque;
            action 'distribute torque' : 'Distribute Torque';
            /*
			 * The following uses a shorthand for a sequence of successions.
			 * The source of the first first is given by "first start",
			 * and the target of each succeeding first is indicated by
			 * using the "then" keyword.
			 */
            first start;
            then merge continue;
            then action engineStarted accept engineStart : EngineStart;
            then action engineStopped accept engineOff : EngineOff;
            then continue;
            /* Enable torque generation. */
            first engineStarted then 'generate torque';
            first engineStarted then 'amplify torque';
            first engineStarted then 'transfer torque';
            first engineStarted then 'distribute torque';
            /* Disable torque generation. */
            first 'generate torque' then engineStopped;
            first 'amplify torque' then engineStopped;
            first 'transfer torque' then engineStopped;
            first 'distribute torque' then engineStopped;
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
    (reference r16 (scope relative) (span (offset 1010) (line 35) (column 30) (len 17)) (segments (segment 0 (token "'Generate Torque'") (name "Generate Torque") (separator none) (span (offset 1010) (line 35) (column 30) (len 17)))))
    (reference r17 (scope relative) (span (offset 1177) (line 42) (column 9) (len 30)) (segments (segment 0 (token "'generate torque'") (name "generate torque") (separator none) (span (offset 1177) (line 42) (column 9) (len 17))) (segment 1 (token "engineTorque") (name "engineTorque") (separator dot) (span (offset 1195) (line 42) (column 27) (len 12)))))
    (reference r18 (scope relative) (span (offset 1219) (line 43) (column 11) (len 29)) (segments (segment 0 (token "'amplify torque'") (name "amplify torque") (separator none) (span (offset 1219) (line 43) (column 11) (len 16))) (segment 1 (token "engineTorque") (name "engineTorque") (separator dot) (span (offset 1236) (line 43) (column 28) (len 12)))))
    (reference r19 (scope relative) (span (offset 1282) (line 45) (column 29) (len 16)) (segments (segment 0 (token "'Amplify Torque'") (name "Amplify Torque") (separator none) (span (offset 1282) (line 45) (column 29) (len 16)))))
    (reference r20 (scope relative) (span (offset 1312) (line 47) (column 9) (len 35)) (segments (segment 0 (token "'amplify torque'") (name "amplify torque") (separator none) (span (offset 1312) (line 47) (column 9) (len 16))) (segment 1 (token "transmissionTorque") (name "transmissionTorque") (separator dot) (span (offset 1329) (line 47) (column 26) (len 18)))))
    (reference r21 (scope relative) (span (offset 1359) (line 48) (column 11) (len 36)) (segments (segment 0 (token "'transfer torque'") (name "transfer torque") (separator none) (span (offset 1359) (line 48) (column 11) (len 17))) (segment 1 (token "transmissionTorque") (name "transmissionTorque") (separator dot) (span (offset 1377) (line 48) (column 29) (len 18)))))
    (reference r22 (scope relative) (span (offset 1430) (line 50) (column 30) (len 17)) (segments (segment 0 (token "'Transfer Torque'") (name "Transfer Torque") (separator none) (span (offset 1430) (line 50) (column 30) (len 17)))))
    (reference r23 (scope relative) (span (offset 1461) (line 52) (column 9) (len 34)) (segments (segment 0 (token "'transfer torque'") (name "transfer torque") (separator none) (span (offset 1461) (line 52) (column 9) (len 17))) (segment 1 (token "driveshaftTorque") (name "driveshaftTorque") (separator dot) (span (offset 1479) (line 52) (column 27) (len 16)))))
    (reference r24 (scope relative) (span (offset 1507) (line 53) (column 11) (len 36)) (segments (segment 0 (token "'distribute torque'") (name "distribute torque") (separator none) (span (offset 1507) (line 53) (column 11) (len 19))) (segment 1 (token "driveShaftTorque") (name "driveShaftTorque") (separator dot) (span (offset 1527) (line 53) (column 31) (len 16)))))
    (reference r25 (scope relative) (span (offset 1580) (line 55) (column 32) (len 19)) (segments (segment 0 (token "'Distribute Torque'") (name "Distribute Torque") (separator none) (span (offset 1580) (line 55) (column 32) (len 19)))))
    (reference r26 (scope relative) (span (offset 1898) (line 66) (column 15) (len 8)) (segments (segment 0 (token "continue") (name "continue") (separator none) (span (offset 1898) (line 66) (column 15) (len 8)))))
    (reference r27 (scope relative) (span (offset 1958) (line 67) (column 50) (len 11)) (segments (segment 0 (token "EngineStart") (name "EngineStart") (separator none) (span (offset 1958) (line 67) (column 50) (len 11)))))
    (reference r28 (scope relative) (span (offset 2021) (line 68) (column 48) (len 9)) (segments (segment 0 (token "EngineOff") (name "EngineOff") (separator none) (span (offset 2021) (line 68) (column 48) (len 9)))))
  )
  (root (package (name "3a-Function-based Behavior-2") (body brace (import (target (span (span (offset 56) (line 2) (column 16) (len 14))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 67) (line 2) (column 27) (len 3))) (separator (span (offset 67) (line 2) (column 27) (len 2))) (marker (span (offset 69) (line 2) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 87) (line 3) (column 16) (len 9))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 93) (line 3) (column 22) (len 3))) (separator (span (offset 93) (line 3) (column 22) (len 2))) (marker (span (offset 95) (line 3) (column 24) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "Definitions") (body brace (alias (name "Torque") (target (ref r2)) (body semicolon)) (attribute-def (declaration-name "FuelCmd") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "EngineStart") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "EngineOff") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (action-def (name "Generate Torque") (modifiers) (specializes none) (body brace (in-out (direction in) (kind none) (reference false) (declaration "fuelCmd") (subsets none) (type (ref r3)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 339) (line 17) (column 34) (len 20))) (in-out (direction out) (kind none) (reference false) (declaration "engineTorque") (subsets none) (type (ref r4)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 360) (line 17) (column 55) (len 25))))) (action-def (name "Amplify Torque") (modifiers) (specializes none) (body brace (in-out (direction in) (kind none) (reference false) (declaration "engineTorque") (subsets none) (type (ref r5)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 420) (line 18) (column 33) (len 24))) (in-out (direction out) (kind none) (reference false) (declaration "transmissionTorque") (subsets none) (type (ref r6)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 445) (line 18) (column 58) (len 31))))) (action-def (name "Transfer Torque") (modifiers) (specializes none) (body brace (in-out (direction in) (kind none) (reference false) (declaration "transmissionTorque") (subsets none) (type (ref r7)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 512) (line 19) (column 34) (len 30))) (in-out (direction out) (kind none) (reference false) (declaration "driveshaftTorque") (subsets none) (type (ref r8)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 543) (line 19) (column 65) (len 29))))) (action-def (name "Distribute Torque") (modifiers) (specializes none) (body brace (in-out (direction in) (kind none) (reference false) (declaration "driveShaftTorque") (subsets none) (type (ref r9)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 610) (line 20) (column 36) (len 28))) (in-out (direction out) (kind none) (reference false) (declaration "wheelTorque1") (subsets none) (type (ref r10)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 639) (line 20) (column 65) (len 25))) (in-out (direction out) (kind none) (reference false) (declaration "wheelTorque2") (subsets none) (type (ref r11)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 665) (line 20) (column 91) (len 25))))) (action-def (name "Provide Power") (modifiers) (specializes none) (body brace (in-out (direction in) (kind none) (reference false) (declaration "fuelCmd") (subsets none) (type (ref r12)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 727) (line 22) (column 32) (len 20))) (in-out (direction out) (kind none) (reference false) (declaration "wheelTorque1") (subsets none) (type (ref r13)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 748) (line 22) (column 53) (len 25))) (in-out (direction out) (kind none) (reference false) (declaration "wheelTorque2") (subsets none) (type (ref r14)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 774) (line 22) (column 79) (len 25))))))) (package (name "Usages") (body brace (action-usage (keyword action) (name "provide power") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (in-out-declaration) (in-out-declaration) (in-out-declaration) (action-usage (keyword action) (name "generate torque") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1035) (line 36) (column 7) (len 79)) (normalized "The binding connector shorthand can be used on action parameters.\n"))) (in-out-declaration))) (flow-usage (kind flow) (visibility none) (declaration (endpoint-only (from (connector-end (multiplicity none) (target (ref r17)) (references none))) (to (connector-end (multiplicity none) (target (ref r18)) (references none))))) (body (body semicolon))) (action-usage (keyword action) (name "amplify torque") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r19)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (flow-usage (kind flow) (visibility none) (declaration (endpoint-only (from (connector-end (multiplicity none) (target (ref r20)) (references none))) (to (connector-end (multiplicity none) (target (ref r21)) (references none))))) (body (body semicolon))) (action-usage (keyword action) (name "transfer torque") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r22)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (flow-usage (kind flow) (visibility none) (declaration (endpoint-only (from (connector-end (multiplicity none) (target (ref r23)) (references none))) (to (connector-end (multiplicity none) (target (ref r24)) (references none))))) (body (body semicolon))) (action-usage (keyword action) (name "distribute torque") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r25)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1635) (line 59) (column 6) (len 230)) (normalized "The following uses a shorthand for a sequence of successions.\nThe source of the first first is given by \"first start\",\nand the target of each succeeding first is indicated by\nusing the \"then\" keyword.\n"))) (first) (then-control (merge (declaration (named (expression (span (offset 1898) (line 66) (column 15) (len 8)) (ref r26)))) (body semicolon (span (span (offset 1906) (line 66) (column 23) (len 1)))))) (then-action (action-usage (keyword action) (name "engineStarted") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (accept (payload (name "engineStart") (type (ref r27)) (via none))) (body semicolon))) (then-action (action-usage (keyword action) (name "engineStopped") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (accept (payload (name "engineOff") (type (ref r28)) (via none))) (body semicolon))) (then-action) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 2060) (line 71) (column 6) (len 27)) (normalized "Enable torque generation. "))) (first) (first) (first) (first) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 2288) (line 77) (column 6) (len 28)) (normalized "Disable torque generation. "))) (first) (first) (first) (first))))))))
)
~~~
