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
      (diagnostic (code "recovered_action_body_element") (severity error) (category parseerror) (span (offset 294) (line 14) (column 3) (len 206)) (message "unexpected token in action body"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 500) (line 24) (column 5) (len 43)) (message "unexpected keyword `until` in action body"))
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
		}
        until charging.monitor.charge >= 100;
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
  )
  (root (package (name "Control Structures Example") (body brace (import (target (span (span (offset 55) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 67) (line 2) (column 29) (len 3))) (separator (span (offset 67) (line 2) (column 29) (len 2))) (marker (span (offset 69) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (attribute-def (name "BatteryCharged") (multiplicity none)) (part-usage (declaration-name "battery") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body semicolon)) (part-usage (declaration-name "powerSystem") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body semicolon)) (action-def (name "MonitorBattery") (specializes none) (body brace (in-out (direction out) (reference false) (declaration "charge") (subsets none) (type (ref r1)) (multiplicity none) (ordered false) (nonunique false) (redefines none) (value none) (span (offset 172) (line 9) (column 30) (len 18))))) (action-def (name "AddCharge") (specializes none) (body brace (in-out (direction in) (reference false) (declaration "charge") (subsets none) (type (ref r2)) (multiplicity none) (ordered false) (nonunique false) (redefines none) (value none) (span (offset 217) (line 10) (column 25) (len 17))))) (action-def (name "EndCharging") (specializes none) (body semicolon)) (action-def (name "ChargeBattery") (specializes none) (body brace (malformed (code "recovered_action_body_element") (found "loop action charging {") (span (offset 294) (line 14) (column 3) (len 206))) (malformed (code "unexpected_keyword_in_scope") (found "until charging.monitor.charge >= 100;") (span (offset 500) (line 24) (column 5) (len 43))) (then-action) (then-action))))))
)
~~~
