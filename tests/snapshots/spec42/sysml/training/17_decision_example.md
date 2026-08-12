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
        if monitor.batteryCharge < 100  {
            then addCharge;
        }
        if monitor.batteryCharge >= 100  {
            then endCharging;
        }
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
  )
  (root (package (name "Decision Example") (body (import (target (span (span (offset 45) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 57) (line 2) (column 29) (len 3))) (separator (span (offset 57) (line 2) (column 29) (len 2))) (marker (span (offset 59) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (attribute-def) (part-usage) (part-usage) (action-def) (action-def) (action-def) (action-def))))
)
~~~
