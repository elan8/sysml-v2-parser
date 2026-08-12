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
  )
  (root (package (name "Control Structures Example") (body (import (target (span (span (offset 55) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 67) (line 2) (column 29) (len 3))) (separator (span (offset 67) (line 2) (column 29) (len 2))) (marker (span (offset 69) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (attribute-def) (part-usage) (part-usage) (action-def) (action-def) (action-def) (action-def))))
)
~~~
