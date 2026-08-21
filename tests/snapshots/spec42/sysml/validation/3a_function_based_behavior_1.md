# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (03-Function-based Behavior): 3a-Function-based Behavior-1"))
~~~
# SOURCE
~~~sysml
package '3a-Function-based Behavior-1' {
	public import Definitions::*;
	public import Usages::*;

	package Definitions {
		alias Torque for ISQ::TorqueValue {
			/*
			 * The 'TorqueValue' type is aliased as 'Torque'.
			 */
		}
		
		attribute def FuelCmd;
		
		/*
		 * There is no special construct for modeling "signals". Data to be
		 * transmitted asynchronously can simply be modeled using attribute defs.
		 */
		
		attribute def EngineStart;
		attribute def EngineOff;
		
		/*
		 * Black box definitions for actions include their inputs and outputs.
		 */
		
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
			
			bind 'generate torque'.fuelCmd = fuelCmd {
				/*
				 * This is a binding connector, just as was used to
				 * model delegation between ports.
				 */
			}
			
			action 'generate torque': 'Generate Torque' {
				/*
				 * An action usage inherits parameters from its definition.
				 * They act as its "pins".
				 */
			}
			
			flow 'generate torque'.engineTorque 
			    to 'amplify torque'.engineTorque {
				/*
				 * A flow is a connection between two actions that streams items from
				 * an output parameter of one action to an input parameter of the other.
				 * Note that streaming is a property of the connection, not the
				 * actions or their parameters.
				 */
			}
			
			action 'amplify torque': 'Amplify Torque';
			
			flow 'amplify torque'.transmissionTorque 
			    to 'transfer torque'.transmissionTorque;
			
			action 'transfer torque': 'Transfer Torque';
			
			flow 'transfer torque'.driveshaftTorque 
			    to 'distribute torque'.driveShaftTorque;
			
			action 'distribute torque': 'Distribute Torque';
			
			bind wheelTorque1 = 'distribute torque'.wheelTorque1;
			bind wheelTorque2 = 'distribute torque'.wheelTorque2;
			
			// CONTROL FLOW PART

			first start then continue {
				/*
				 * A first is an assertion that one thing must occur
				 * before another, acting like a "control flow". 'start' is
				 * the start snapshot of the action, which acts like an
				 * "initial node".
				 */
			}
			
			merge continue {
				/*
				 * A merge node is necessary to prevent a loop of successions
				 * from being unsatisfiable.
				 */
			}
			first continue then engineStarted;
			
			action engineStarted accept engineStart: EngineStart {
				/*
				 * An accept action accepts an incoming transfer of some item
				 * from outside an action, in this case the "signal" 'EngineStart'.
				 * Note that 'engineStarted' is the name of the action, while
				 * 'engineStart' is the name of the received signal attribute.
				 */
			}			
			first engineStarted then engineStopped;
					
			action engineStopped accept engineOff: EngineOff;	
			first engineStopped then continue;
			
			/*
			 * These successions act to "enable" the torque-related actions.
			 * Each action on the right can only be performed following the
			 * completion of a performance of 'engineStarted'.
			 */
			first engineStarted then 'generate torque';
			first engineStarted then 'amplify torque';
			first engineStarted then 'transfer torque';
			first engineStarted then 'distribute torque';
			
			/*
			 * These successions act to "disable" the torque-related actions.
			 * The performance of the actions on the left cannot continue
			 * once there is a performance of 'engineStopped'.
			 */
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
  (document "3a_function_based_behavior_1.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '3a-Function-based Behavior-1' {
    public import Definitions::*;
    public import Usages::*;
    package Definitions {
        alias Torque for ISQ::TorqueValue {
            /*
			 * The 'TorqueValue' type is aliased as 'Torque'.
			 */
        }
        attribute def FuelCmd;
        /*
		 * There is no special construct for modeling "signals". Data to be
		 * transmitted asynchronously can simply be modeled using attribute defs.
		 */
        attribute def EngineStart;
        attribute def EngineOff;
        /*
		 * Black box definitions for actions include their inputs and outputs.
		 */
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
            bind 'generate torque'.fuelCmd = fuelCmd {
                /*
				 * This is a binding connector, just as was used to
				 * model delegation between ports.
				 */
            }
            action 'generate torque' : 'Generate Torque' {
                /*
				 * An action usage inherits parameters from its definition.
				 * They act as its "pins".
				 */
            }
            flow from 'generate torque'.engineTorque to 'amplify torque'.engineTorque {
                /*
				 * A flow is a connection between two actions that streams items from
				 * an output parameter of one action to an input parameter of the other.
				 * Note that streaming is a property of the connection, not the
				 * actions or their parameters.
				 */
            }
            action 'amplify torque' : 'Amplify Torque';
            flow from 'amplify torque'.transmissionTorque to 'transfer torque'.transmissionTorque;
            action 'transfer torque' : 'Transfer Torque';
            flow from 'transfer torque'.driveshaftTorque to 'distribute torque'.driveShaftTorque;
            action 'distribute torque' : 'Distribute Torque';
            bind wheelTorque1 = 'distribute torque'.wheelTorque1;
            bind wheelTorque2 = 'distribute torque'.wheelTorque2;
            first start then continue {
                /*
				 * A first is an assertion that one thing must occur
				 * before another, acting like a "control flow". 'start' is
				 * the start snapshot of the action, which acts like an
				 * "initial node".
				 */
            }
            merge continue {
                /*
				 * A merge node is necessary to prevent a loop of successions
				 * from being unsatisfiable.
				 */
            }
            first continue then engineStarted;
            action engineStarted accept engineStart : EngineStart {
                /*
				 * An accept action accepts an incoming transfer of some item
				 * from outside an action, in this case the "signal" 'EngineStart'.
				 * Note that 'engineStarted' is the name of the action, while
				 * 'engineStart' is the name of the received signal attribute.
				 */
            }
            first engineStarted then engineStopped;
            action engineStopped accept engineOff : EngineOff;
            first engineStopped then continue;
            /*
			 * These successions act to "enable" the torque-related actions.
			 * Each action on the right can only be performed following the
			 * completion of a performance of 'engineStarted'.
			 */
            first engineStarted then 'generate torque';
            first engineStarted then 'amplify torque';
            first engineStarted then 'transfer torque';
            first engineStarted then 'distribute torque';
            /*
			 * These successions act to "disable" the torque-related actions.
			 * The performance of the actions on the left cannot continue
			 * once there is a performance of 'engineStopped'.
			 */
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
    (reference r3 (scope relative) (span (offset 612) (line 26) (column 46) (len 7)) (segments (segment 0 (token "FuelCmd") (name "FuelCmd") (separator none) (span (offset 612) (line 26) (column 46) (len 7)))))
    (reference r4 (scope relative) (span (offset 639) (line 26) (column 73) (len 6)) (segments (segment 0 (token "Torque") (name "Torque") (separator none) (span (offset 639) (line 26) (column 73) (len 6)))))
    (reference r5 (scope relative) (span (offset 698) (line 27) (column 50) (len 6)) (segments (segment 0 (token "Torque") (name "Torque") (separator none) (span (offset 698) (line 27) (column 50) (len 6)))))
    (reference r6 (scope relative) (span (offset 730) (line 27) (column 82) (len 6)) (segments (segment 0 (token "Torque") (name "Torque") (separator none) (span (offset 730) (line 27) (column 82) (len 6)))))
    (reference r7 (scope relative) (span (offset 796) (line 28) (column 57) (len 6)) (segments (segment 0 (token "Torque") (name "Torque") (separator none) (span (offset 796) (line 28) (column 57) (len 6)))))
    (reference r8 (scope relative) (span (offset 826) (line 28) (column 87) (len 6)) (segments (segment 0 (token "Torque") (name "Torque") (separator none) (span (offset 826) (line 28) (column 87) (len 6)))))
    (reference r9 (scope relative) (span (offset 892) (line 29) (column 57) (len 6)) (segments (segment 0 (token "Torque") (name "Torque") (separator none) (span (offset 892) (line 29) (column 57) (len 6)))))
    (reference r10 (scope relative) (span (offset 918) (line 29) (column 83) (len 6)) (segments (segment 0 (token "Torque") (name "Torque") (separator none) (span (offset 918) (line 29) (column 83) (len 6)))))
    (reference r11 (scope relative) (span (offset 944) (line 29) (column 109) (len 6)) (segments (segment 0 (token "Torque") (name "Torque") (separator none) (span (offset 944) (line 29) (column 109) (len 6)))))
    (reference r12 (scope relative) (span (offset 1000) (line 31) (column 44) (len 7)) (segments (segment 0 (token "FuelCmd") (name "FuelCmd") (separator none) (span (offset 1000) (line 31) (column 44) (len 7)))))
    (reference r13 (scope relative) (span (offset 1027) (line 31) (column 71) (len 6)) (segments (segment 0 (token "Torque") (name "Torque") (separator none) (span (offset 1027) (line 31) (column 71) (len 6)))))
    (reference r14 (scope relative) (span (offset 1053) (line 31) (column 97) (len 6)) (segments (segment 0 (token "Torque") (name "Torque") (separator none) (span (offset 1053) (line 31) (column 97) (len 6)))))
    (reference r15 (scope relative) (span (offset 1116) (line 37) (column 27) (len 15)) (segments (segment 0 (token "'Provide Power'") (name "Provide Power") (separator none) (span (offset 1116) (line 37) (column 27) (len 15)))))
    (reference r16 (scope relative) (span (offset 1438) (line 51) (column 30) (len 17)) (segments (segment 0 (token "'Generate Torque'") (name "Generate Torque") (separator none) (span (offset 1438) (line 51) (column 30) (len 17)))))
    (reference r17 (scope relative) (span (offset 1585) (line 58) (column 9) (len 30)) (segments (segment 0 (token "'generate torque'") (name "generate torque") (separator none) (span (offset 1585) (line 58) (column 9) (len 17))) (segment 1 (token "engineTorque") (name "engineTorque") (separator dot) (span (offset 1603) (line 58) (column 27) (len 12)))))
    (reference r18 (scope relative) (span (offset 1627) (line 59) (column 11) (len 29)) (segments (segment 0 (token "'amplify torque'") (name "amplify torque") (separator none) (span (offset 1627) (line 59) (column 11) (len 16))) (segment 1 (token "engineTorque") (name "engineTorque") (separator dot) (span (offset 1644) (line 59) (column 28) (len 12)))))
    (reference r19 (scope relative) (span (offset 1966) (line 68) (column 29) (len 16)) (segments (segment 0 (token "'Amplify Torque'") (name "Amplify Torque") (separator none) (span (offset 1966) (line 68) (column 29) (len 16)))))
    (reference r20 (scope relative) (span (offset 1996) (line 70) (column 9) (len 35)) (segments (segment 0 (token "'amplify torque'") (name "amplify torque") (separator none) (span (offset 1996) (line 70) (column 9) (len 16))) (segment 1 (token "transmissionTorque") (name "transmissionTorque") (separator dot) (span (offset 2013) (line 70) (column 26) (len 18)))))
    (reference r21 (scope relative) (span (offset 2043) (line 71) (column 11) (len 36)) (segments (segment 0 (token "'transfer torque'") (name "transfer torque") (separator none) (span (offset 2043) (line 71) (column 11) (len 17))) (segment 1 (token "transmissionTorque") (name "transmissionTorque") (separator dot) (span (offset 2061) (line 71) (column 29) (len 18)))))
    (reference r22 (scope relative) (span (offset 2114) (line 73) (column 30) (len 17)) (segments (segment 0 (token "'Transfer Torque'") (name "Transfer Torque") (separator none) (span (offset 2114) (line 73) (column 30) (len 17)))))
    (reference r23 (scope relative) (span (offset 2145) (line 75) (column 9) (len 34)) (segments (segment 0 (token "'transfer torque'") (name "transfer torque") (separator none) (span (offset 2145) (line 75) (column 9) (len 17))) (segment 1 (token "driveshaftTorque") (name "driveshaftTorque") (separator dot) (span (offset 2163) (line 75) (column 27) (len 16)))))
    (reference r24 (scope relative) (span (offset 2191) (line 76) (column 11) (len 36)) (segments (segment 0 (token "'distribute torque'") (name "distribute torque") (separator none) (span (offset 2191) (line 76) (column 11) (len 19))) (segment 1 (token "driveShaftTorque") (name "driveShaftTorque") (separator dot) (span (offset 2211) (line 76) (column 31) (len 16)))))
    (reference r25 (scope relative) (span (offset 2264) (line 78) (column 32) (len 19)) (segments (segment 0 (token "'Distribute Torque'") (name "Distribute Torque") (separator none) (span (offset 2264) (line 78) (column 32) (len 19)))))
    (reference r26 (scope relative) (span (offset 2700) (line 94) (column 10) (len 8)) (segments (segment 0 (token "continue") (name "continue") (separator none) (span (offset 2700) (line 94) (column 10) (len 8)))))
    (reference r27 (scope relative) (span (offset 2916) (line 102) (column 45) (len 11)) (segments (segment 0 (token "EngineStart") (name "EngineStart") (separator none) (span (offset 2916) (line 102) (column 45) (len 11)))))
    (reference r28 (scope relative) (span (offset 3315) (line 112) (column 43) (len 9)) (segments (segment 0 (token "EngineOff") (name "EngineOff") (separator none) (span (offset 3315) (line 112) (column 43) (len 9)))))
  )
  (root (package (name "3a-Function-based Behavior-1") (body brace (import (target (span (span (offset 56) (line 2) (column 16) (len 14))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 67) (line 2) (column 27) (len 3))) (separator (span (offset 67) (line 2) (column 27) (len 2))) (marker (span (offset 69) (line 2) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 87) (line 3) (column 16) (len 9))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 93) (line 3) (column 22) (len 3))) (separator (span (offset 93) (line 3) (column 22) (len 2))) (marker (span (offset 95) (line 3) (column 24) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "Definitions") (body brace (alias (name "Torque") (target (ref r2)) (body brace (element-count 1))) (attribute-def (declaration-name "FuelCmd") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 265) (line 14) (column 5) (len 150)) (normalized "There is no special construct for modeling \"signals\". Data to be\ntransmitted asynchronously can simply be modeled using attribute defs.\n"))) (attribute-def (declaration-name "EngineStart") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "EngineOff") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 484) (line 22) (column 5) (len 77)) (normalized "Black box definitions for actions include their inputs and outputs.\n"))) (action-def (name "Generate Torque") (modifiers) (specializes none) (body brace (in-out (direction in) (reference false) (declaration "fuelCmd") (subsets none) (type (ref r3)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 600) (line 26) (column 34) (len 20))) (in-out (direction out) (reference false) (declaration "engineTorque") (subsets none) (type (ref r4)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 621) (line 26) (column 55) (len 25))))) (action-def (name "Amplify Torque") (modifiers) (specializes none) (body brace (in-out (direction in) (reference false) (declaration "engineTorque") (subsets none) (type (ref r5)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 681) (line 27) (column 33) (len 24))) (in-out (direction out) (reference false) (declaration "transmissionTorque") (subsets none) (type (ref r6)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 706) (line 27) (column 58) (len 31))))) (action-def (name "Transfer Torque") (modifiers) (specializes none) (body brace (in-out (direction in) (reference false) (declaration "transmissionTorque") (subsets none) (type (ref r7)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 773) (line 28) (column 34) (len 30))) (in-out (direction out) (reference false) (declaration "driveshaftTorque") (subsets none) (type (ref r8)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 804) (line 28) (column 65) (len 29))))) (action-def (name "Distribute Torque") (modifiers) (specializes none) (body brace (in-out (direction in) (reference false) (declaration "driveShaftTorque") (subsets none) (type (ref r9)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 871) (line 29) (column 36) (len 28))) (in-out (direction out) (reference false) (declaration "wheelTorque1") (subsets none) (type (ref r10)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 900) (line 29) (column 65) (len 25))) (in-out (direction out) (reference false) (declaration "wheelTorque2") (subsets none) (type (ref r11)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 926) (line 29) (column 91) (len 25))))) (action-def (name "Provide Power") (modifiers) (specializes none) (body brace (in-out (direction in) (reference false) (declaration "fuelCmd") (subsets none) (type (ref r12)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 988) (line 31) (column 32) (len 20))) (in-out (direction out) (reference false) (declaration "wheelTorque1") (subsets none) (type (ref r13)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 1009) (line 31) (column 53) (len 25))) (in-out (direction out) (reference false) (declaration "wheelTorque2") (subsets none) (type (ref r14)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 1035) (line 31) (column 79) (len 25))))))) (package (name "Usages") (body brace (action-usage (name "provide power") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (in-out-declaration) (in-out-declaration) (in-out-declaration) (bind) (action-usage (name "generate torque") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1464) (line 52) (column 7) (len 101)) (normalized "An action usage inherits parameters from its definition.\nThey act as its \"pins\".\n"))))) (flow-usage (kind flow) (visibility none) (declaration (endpoint-only (from (connector-end (multiplicity none) (target (ref r17)) (references none))) (to (connector-end (multiplicity none) (target (ref r18)) (references none))))) (body (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1665) (line 60) (column 7) (len 261)) (normalized "A flow is a connection between two actions that streams items from\nan output parameter of one action to an input parameter of the other.\nNote that streaming is a property of the connection, not the\nactions or their parameters.\n")))))) (action-usage (name "amplify torque") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r19)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (flow-usage (kind flow) (visibility none) (declaration (endpoint-only (from (connector-end (multiplicity none) (target (ref r20)) (references none))) (to (connector-end (multiplicity none) (target (ref r21)) (references none))))) (body (body semicolon))) (action-usage (name "transfer torque") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r22)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (flow-usage (kind flow) (visibility none) (declaration (endpoint-only (from (connector-end (multiplicity none) (target (ref r23)) (references none))) (to (connector-end (multiplicity none) (target (ref r24)) (references none))))) (body (body semicolon))) (action-usage (name "distribute torque") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r25)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (bind) (bind) (first) (merge (declaration (named (expression (span (offset 2700) (line 94) (column 10) (len 8)) (ref r26)))) (body brace (open-brace (span (offset 2709) (line 94) (column 19) (len 1))) (members (comment (keyword none) (name none) (about) (locale none) (body (span (offset 2717) (line 95) (column 7) (len 105)) (normalized "A merge node is necessary to prevent a loop of successions\nfrom being unsatisfiable.\n")))) (close-brace (span (offset 2828) (line 99) (column 4) (len 1))))) (first) (action-usage (name "engineStarted") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (accept (payload (name "engineStart") (type (ref r27)) (via none))) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 2936) (line 103) (column 7) (len 277)) (normalized "An accept action accepts an incoming transfer of some item\nfrom outside an action, in this case the \"signal\" 'EngineStart'.\nNote that 'engineStarted' is the name of the action, while\n'engineStart' is the name of the received signal attribute.\n"))))) (first) (action-usage (name "engineStopped") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (accept (payload (name "engineOff") (type (ref r28)) (via none))) (body semicolon)) (first) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 3374) (line 115) (column 6) (len 194)) (normalized "These successions act to \"enable\" the torque-related actions.\nEach action on the right can only be performed following the\ncompletion of a performance of 'engineStarted'.\n"))) (first) (first) (first) (first) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 3769) (line 125) (column 6) (len 193)) (normalized "These successions act to \"disable\" the torque-related actions.\nThe performance of the actions on the left cannot continue\nonce there is a performance of 'engineStopped'.\n"))) (first) (first) (first) (first))))))))
)
~~~
