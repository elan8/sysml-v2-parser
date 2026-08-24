# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 17 (Control): Control Structures Example"))
~~~
# SOURCE
~~~sysml
package 'Control Structures Example' {
	private import ScalarValues::*;
	
	attribute def BatteryCharged;
	
	part battery;
	part powerSystem;
	
	action def MonitorBattery { out charge : Real; }
	action def AddCharge { in charge : Real; }
	action def EndCharging;
	
	action def ChargeBattery {
		loop action charging {
			action monitor : MonitorBattery {
				out charge;
			}
			
			then if monitor.charge < 100 {
				action addCharge : AddCharge {
					in charge = monitor.charge;
				}
			}				
		} until charging.monitor.charge >= 100;
		
		then action endCharging : EndCharging;
		then done;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "17_control_structures_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Control Structures Example' {
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
        loop action charging {
            action monitor : MonitorBattery {
                out charge;
            }
            then if monitor.charge < 100 {
                action addCharge : AddCharge {
                    in charge = monitor.charge;
                }
            }
        } until charging.monitor.charge >= 100;
        then action endCharging : EndCharging;
        then done;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 55) (line 2) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 55) (line 2) (column 17) (len 12)))))
    (reference r1 (scope relative) (span (offset 185) (line 9) (column 43) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 185) (line 9) (column 43) (len 4)))))
    (reference r2 (scope relative) (span (offset 229) (line 10) (column 37) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 229) (line 10) (column 37) (len 4)))))
    (reference r3 (scope relative) (span (offset 337) (line 15) (column 21) (len 14)) (segments (segment 0 (token "MonitorBattery") (name "MonitorBattery") (separator none) (span (offset 337) (line 15) (column 21) (len 14)))))
    (reference r4 (scope relative) (span (offset 390) (line 19) (column 12) (len 7)) (segments (segment 0 (token "monitor") (name "monitor") (separator none) (span (offset 390) (line 19) (column 12) (len 7)))))
    (reference r5 (scope relative) (span (offset 398) (line 19) (column 20) (len 6)) (segments (segment 0 (token "charge") (name "charge") (separator none) (span (offset 398) (line 19) (column 20) (len 6)))))
    (reference r6 (scope relative) (span (offset 436) (line 20) (column 24) (len 9)) (segments (segment 0 (token "AddCharge") (name "AddCharge") (separator none) (span (offset 436) (line 20) (column 24) (len 9)))))
    (reference r7 (scope relative) (span (offset 506) (line 24) (column 11) (len 8)) (segments (segment 0 (token "charging") (name "charging") (separator none) (span (offset 506) (line 24) (column 11) (len 8)))))
    (reference r8 (scope relative) (span (offset 515) (line 24) (column 20) (len 7)) (segments (segment 0 (token "monitor") (name "monitor") (separator none) (span (offset 515) (line 24) (column 20) (len 7)))))
    (reference r9 (scope relative) (span (offset 523) (line 24) (column 28) (len 6)) (segments (segment 0 (token "charge") (name "charge") (separator none) (span (offset 523) (line 24) (column 28) (len 6)))))
  )
  (root (package (name "Control Structures Example") (body brace (import (target (span (span (offset 55) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 67) (line 2) (column 29) (len 3))) (separator (span (offset 67) (line 2) (column 29) (len 2))) (marker (span (offset 69) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (attribute-def (declaration-name "BatteryCharged") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "battery") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "powerSystem") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (action-def (name "MonitorBattery") (modifiers) (specializes none) (body brace (in-out (direction out) (reference false) (declaration "charge") (subsets none) (type (ref r1)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 172) (line 9) (column 30) (len 18))))) (action-def (name "AddCharge") (modifiers) (specializes none) (body brace (in-out (direction in) (reference false) (declaration "charge") (subsets none) (type (ref r2)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 217) (line 10) (column 25) (len 17))))) (action-def (name "EndCharging") (modifiers) (specializes none) (body semicolon)) (action-def (name "ChargeBattery") (modifiers) (specializes none) (body brace (loop (prefix (action-node-prefix (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (action-declaration none))) (condition none) (body-parameter (action-declaration (name "charging") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (body brace (action-usage (keyword action) (name "monitor") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (in-out-declaration))) (then-if (if (condition (expression (span (offset 390) (line 19) (column 12) (len 20)) (binary (operator "<") (left (expression (span (offset 390) (line 19) (column 12) (len 14)) (member-access (base (expression (span (offset 390) (line 19) (column 12) (len 7)) (ref r4))) (separator dot) (member (ref r5))))) (right (expression (span (offset 407) (line 19) (column 29) (len 3)) (integer 100)))))) (then (body brace (action-usage (keyword action) (name "addCharge") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (in-out-declaration))))) (else none))))) (until (expression (span (offset 506) (line 24) (column 11) (len 30)) (binary (operator ">=") (left (expression (span (offset 506) (line 24) (column 11) (len 23)) (member-access (base (expression (span (offset 506) (line 24) (column 11) (len 16)) (member-access (base (expression (span (offset 506) (line 24) (column 11) (len 8)) (ref r7))) (separator dot) (member (ref r8))))) (separator dot) (member (ref r9))))) (right (expression (span (offset 533) (line 24) (column 38) (len 3)) (integer 100))))))) (then-action) (then-action))))))
)
~~~
