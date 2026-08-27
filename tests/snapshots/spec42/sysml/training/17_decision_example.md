# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 17 (Control): Decision Example"))
~~~
# SOURCE
~~~sysml
package 'Decision Example' {
	private import ScalarValues::*;
	
	attribute def BatteryCharged;
	
	part battery;
	part powerSystem;
	
	action def MonitorBattery { out charge : Real; }
	action def AddCharge { in charge : Real; }
	action def EndCharging;
	
	action def ChargeBattery {
		first start;

		then merge continueCharging;
		
		then action monitor : MonitorBattery {
			out batteryCharge : Real;
		}
		
		then decide;
			if monitor.batteryCharge < 100 then addCharge;
			if monitor.batteryCharge >= 100 then endCharging;
			
		action addCharge : AddCharge {
			in charge = monitor.batteryCharge;
		}
		then continueCharging;
		
		action endCharging : EndCharging;
		then done;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "17_decision_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Decision Example' {
    private import ScalarValues::*;
    attribute def BatteryCharged;
    part battery;
    part powerSystem;
    action def MonitorBattery {
        out charge : Real;
    }
    action def AddCharge {
        in charge : Real;
    }
    action def EndCharging;
    action def ChargeBattery {
        first start;
        then merge continueCharging;
        then action monitor : MonitorBattery {
            out batteryCharge : Real;
        }
        then decide;
        if monitor.batteryCharge < 100 then addCharge;
        if monitor.batteryCharge >= 100 then endCharging;
        action addCharge : AddCharge {
            in charge = monitor.batteryCharge;
        }
        then continueCharging;
        action endCharging : EndCharging;
        then done;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 45) (line 2) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 45) (line 2) (column 17) (len 12)))))
    (reference r1 (scope relative) (span (offset 175) (line 9) (column 43) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 175) (line 9) (column 43) (len 4)))))
    (reference r2 (scope relative) (span (offset 219) (line 10) (column 37) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 219) (line 10) (column 37) (len 4)))))
    (reference r3 (scope relative) (span (offset 290) (line 14) (column 9) (len 5)) (segments (segment 0 (token "start") (name "start") (separator none) (span (offset 290) (line 14) (column 9) (len 5)))))
    (reference r4 (scope relative) (span (offset 311) (line 16) (column 14) (len 16)) (segments (segment 0 (token "continueCharging") (name "continueCharging") (separator none) (span (offset 311) (line 16) (column 14) (len 16)))))
    (reference r5 (scope relative) (span (offset 430) (line 23) (column 7) (len 7)) (segments (segment 0 (token "monitor") (name "monitor") (separator none) (span (offset 430) (line 23) (column 7) (len 7)))))
    (reference r6 (scope relative) (span (offset 438) (line 23) (column 15) (len 13)) (segments (segment 0 (token "batteryCharge") (name "batteryCharge") (separator none) (span (offset 438) (line 23) (column 15) (len 13)))))
    (reference r7 (scope relative) (span (offset 480) (line 24) (column 7) (len 7)) (segments (segment 0 (token "monitor") (name "monitor") (separator none) (span (offset 480) (line 24) (column 7) (len 7)))))
    (reference r8 (scope relative) (span (offset 488) (line 24) (column 15) (len 13)) (segments (segment 0 (token "batteryCharge") (name "batteryCharge") (separator none) (span (offset 488) (line 24) (column 15) (len 13)))))
    (reference r9 (scope relative) (span (offset 552) (line 26) (column 22) (len 9)) (segments (segment 0 (token "AddCharge") (name "AddCharge") (separator none) (span (offset 552) (line 26) (column 22) (len 9)))))
    (reference r10 (scope relative) (span (offset 657) (line 31) (column 24) (len 11)) (segments (segment 0 (token "EndCharging") (name "EndCharging") (separator none) (span (offset 657) (line 31) (column 24) (len 11)))))
  )
  (root (package (name "Decision Example") (body brace (import (target (span (span (offset 45) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 57) (line 2) (column 29) (len 3))) (separator (span (offset 57) (line 2) (column 29) (len 2))) (marker (span (offset 59) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (attribute-def (declaration-name "BatteryCharged") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "battery") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "powerSystem") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (action-def (name "MonitorBattery") (modifiers) (specializes none) (body brace (in-out (direction out) (kind none) (reference false) (declaration "charge") (subsets none) (type (ref r1)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 162) (line 9) (column 30) (len 18))))) (action-def (name "AddCharge") (modifiers) (specializes none) (body brace (in-out (direction in) (kind none) (reference false) (declaration "charge") (subsets none) (type (ref r2)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 207) (line 10) (column 25) (len 17))))) (action-def (name "EndCharging") (modifiers) (specializes none) (body semicolon)) (action-def (name "ChargeBattery") (modifiers) (specializes none) (body brace (first (source (expression (span (offset 290) (line 14) (column 9) (len 5)) (ref r3))) (target none) (body semicolon (span (span (offset 295) (line 14) (column 14) (len 1))))) (then-control (merge (declaration (named (expression (span (offset 311) (line 16) (column 14) (len 16)) (ref r4)))) (body semicolon (span (span (offset 327) (line 16) (column 30) (len 1)))))) (then-action) (then-control (decide (declaration anonymous) (body semicolon (span (span (offset 422) (line 22) (column 14) (len 1)))))) (if (condition (expression (span (offset 430) (line 23) (column 7) (len 27)) (binary (operator "<") (left (expression (span (offset 430) (line 23) (column 7) (len 21)) (member-access (base (expression (span (offset 430) (line 23) (column 7) (len 7)) (ref r5))) (separator dot) (member (ref r6))))) (right (expression (span (offset 454) (line 23) (column 31) (len 3)) (integer 100)))))) (then (body shorthand (then-action))) (else none)) (if (condition (expression (span (offset 480) (line 24) (column 7) (len 28)) (binary (operator ">=") (left (expression (span (offset 480) (line 24) (column 7) (len 21)) (member-access (base (expression (span (offset 480) (line 24) (column 7) (len 7)) (ref r7))) (separator dot) (member (ref r8))))) (right (expression (span (offset 505) (line 24) (column 32) (len 3)) (integer 100)))))) (then (body shorthand (then-action))) (else none)) (action-usage (keyword action) (name "addCharge") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (in-out-declaration))) (then-action) (action-usage (keyword action) (name "endCharging") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (then-action))))))
)
~~~
