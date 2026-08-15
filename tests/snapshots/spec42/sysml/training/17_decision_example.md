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
        then 'decide';
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
    (reference r4 (scope relative) (span (offset 552) (line 26) (column 22) (len 9)) (segments (segment 0 (token "AddCharge") (name "AddCharge") (separator none) (span (offset 552) (line 26) (column 22) (len 9)))))
    (reference r5 (scope relative) (span (offset 657) (line 31) (column 24) (len 11)) (segments (segment 0 (token "EndCharging") (name "EndCharging") (separator none) (span (offset 657) (line 31) (column 24) (len 11)))))
  )
  (root (package (name "Decision Example") (body brace (import (target (span (span (offset 45) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 57) (line 2) (column 29) (len 3))) (separator (span (offset 57) (line 2) (column 29) (len 2))) (marker (span (offset 59) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (attribute-def) (part-usage) (part-usage) (action-def (name "MonitorBattery") (specializes none) (body brace (in-out (direction out) (reference false) (declaration "charge") (subsets none) (type (ref r1)) (multiplicity none) (ordered false) (nonunique false) (redefines none) (value none) (span (offset 162) (line 9) (column 30) (len 18))))) (action-def (name "AddCharge") (specializes none) (body brace (in-out (direction in) (reference false) (declaration "charge") (subsets none) (type (ref r2)) (multiplicity none) (ordered false) (nonunique false) (redefines none) (value none) (span (offset 207) (line 10) (column 25) (len 17))))) (action-def (name "EndCharging") (specializes none) (body semicolon)) (action-def (name "ChargeBattery") (specializes none) (body brace (first (source (expression (span (offset 290) (line 14) (column 9) (len 5)) (ref r3))) (target none) (body semicolon)) (then-action) (then-action) (then-action) (if) (if) (action-usage (declaration "addCharge") (type (ref r4))) (then-action) (action-usage (declaration "endCharging") (type (ref r5))) (then-action))))))
)
~~~
