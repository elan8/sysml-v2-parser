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
                in fuelCmd = 'provide power'::fuelCmd;
            }
            flow  'generate torque'.engineTorque to 'amplify torque'.engineTorque;
            action 'amplify torque' : 'Amplify Torque';
            flow  'amplify torque'.transmissionTorque to 'transfer torque'.transmissionTorque;
            action 'transfer torque' : 'Transfer Torque';
            flow  'transfer torque'.driveshaftTorque to 'distribute torque'.driveShaftTorque;
            action 'distribute torque' : 'Distribute Torque';
            first start;
            then merge continue;
            then action engineStarted accept engineStart : EngineStart;
            then action engineStopped accept engineOff : EngineOff;
            then continue;
            first engineStarted then 'generate torque';
            first engineStarted then 'amplify torque';
            first engineStarted then 'transfer torque';
            first engineStarted then 'distribute torque';
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
  )
  (root (package (name "3a-Function-based Behavior-2") (body (import (target (span (span (offset 56) (line 2) (column 16) (len 14))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 67) (line 2) (column 27) (len 3))) (separator (span (offset 67) (line 2) (column 27) (len 2))) (marker (span (offset 69) (line 2) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 87) (line 3) (column 16) (len 9))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 93) (line 3) (column 22) (len 3))) (separator (span (offset 93) (line 3) (column 22) (len 2))) (marker (span (offset 95) (line 3) (column 24) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "Definitions") (body (alias (name "Torque") (target (ref r2)) (body semicolon)) (attribute-def) (attribute-def) (attribute-def) (action-def (name "Generate Torque") (specializes none) (body (in-out (direction in) (reference false) (declaration "fuelCmd") (type (ref r3)) (multiplicity none) (ordered false) (nonunique false) (redefines none) (value none) (span (offset 339) (line 17) (column 34) (len 20))) (in-out (direction out) (reference false) (declaration "engineTorque") (type (ref r4)) (multiplicity none) (ordered false) (nonunique false) (redefines none) (value none) (span (offset 360) (line 17) (column 55) (len 25))))) (action-def (name "Amplify Torque") (specializes none) (body (in-out (direction in) (reference false) (declaration "engineTorque") (type (ref r5)) (multiplicity none) (ordered false) (nonunique false) (redefines none) (value none) (span (offset 420) (line 18) (column 33) (len 24))) (in-out (direction out) (reference false) (declaration "transmissionTorque") (type (ref r6)) (multiplicity none) (ordered false) (nonunique false) (redefines none) (value none) (span (offset 445) (line 18) (column 58) (len 31))))) (action-def (name "Transfer Torque") (specializes none) (body (in-out (direction in) (reference false) (declaration "transmissionTorque") (type (ref r7)) (multiplicity none) (ordered false) (nonunique false) (redefines none) (value none) (span (offset 512) (line 19) (column 34) (len 30))) (in-out (direction out) (reference false) (declaration "driveshaftTorque") (type (ref r8)) (multiplicity none) (ordered false) (nonunique false) (redefines none) (value none) (span (offset 543) (line 19) (column 65) (len 29))))) (action-def (name "Distribute Torque") (specializes none) (body (in-out (direction in) (reference false) (declaration "driveShaftTorque") (type (ref r9)) (multiplicity none) (ordered false) (nonunique false) (redefines none) (value none) (span (offset 610) (line 20) (column 36) (len 28))) (in-out (direction out) (reference false) (declaration "wheelTorque1") (type (ref r10)) (multiplicity none) (ordered false) (nonunique false) (redefines none) (value none) (span (offset 639) (line 20) (column 65) (len 25))) (in-out (direction out) (reference false) (declaration "wheelTorque2") (type (ref r11)) (multiplicity none) (ordered false) (nonunique false) (redefines none) (value none) (span (offset 665) (line 20) (column 91) (len 25))))) (action-def (name "Provide Power") (specializes none) (body (in-out (direction in) (reference false) (declaration "fuelCmd") (type (ref r12)) (multiplicity none) (ordered false) (nonunique false) (redefines none) (value none) (span (offset 727) (line 22) (column 32) (len 20))) (in-out (direction out) (reference false) (declaration "wheelTorque1") (type (ref r13)) (multiplicity none) (ordered false) (nonunique false) (redefines none) (value none) (span (offset 748) (line 22) (column 53) (len 25))) (in-out (direction out) (reference false) (declaration "wheelTorque2") (type (ref r14)) (multiplicity none) (ordered false) (nonunique false) (redefines none) (value none) (span (offset 774) (line 22) (column 79) (len 25))))))) (package (name "Usages") (body (action-usage))))))
)
~~~
