# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Timeslice and Snapshot): TimeVaryingAttribute"))
~~~
# SOURCE
~~~sysml
package TimeVaryingAttribute {
    private import SI::s;
    
    item def PwrCmd {
        attribute pwrLevel: ScalarValues::Integer;
    }
    
    part def Transport2 {
        private import Time::*;
        attribute startTime = TimeOf(start);
        attribute elapseTime :> ISQ::duration;
        attribute :>> localClock.currentTime = startTime + elapseTime;
        
        out item pwrCmd:PwrCmd;
        // Lifetime conditions
        timeslice :>> portionOfLife {
            snapshot :>> start {
                :>> elapseTime = 0 [s];
                :>> pwrCmd.pwrLevel = 0;
            }
            snapshot :>> done {
                :>> elapseTime = 2 [s];
                :>> pwrCmd.pwrLevel = 1;
            }
        }
        
 //     Alternative:
 //       // initial conditions
 //       :>> portionOfLife.start : C {
 //           :>> elapseTime = 0 [s];
 //           :>> pwrCmd.pwrLevel = 0;
 //       }
 
        timeslice transportPeriod {
            snapshot :>> start{
                :>> elapseTime = 1 [s];
            }
            snapshot :>> done {
                :>> elapseTime = 1.5 [s];
            }
           :>> pwrCmd.pwrLevel = 2*elapseTime.num;
        }
        
//      Alternative:
//        // final conditions
//        :>> portionOfLife.done {
//            :>> elapseTime = 2 [s];
//            :>> pwrCmd.pwrLevel = 1;
//        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "time_varying_attribute.md"
    (diagnostics
      (diagnostic (code "recovered_occurrence_body_element") (severity error) (category parseerror) (span (offset 566) (line 19) (column 17) (len 37)) (message "unexpected token in occurrence body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 566) (line 19) (column 17) (len 37)) (message "suppressed 2 cascading recovered diagnostics after earlier recovery errors"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package TimeVaryingAttribute {
    private import SI::s;
    item def PwrCmd {
        attribute pwrLevel : ScalarValues::Integer;
    }
    part def Transport2 {
        private import Time::*;
        attribute startTime = TimeOf(start);
        attribute elapseTime :> ISQ::duration;
        attribute  :>> localClock.currentTime = startTime + elapseTime;
        out item pwrCmd : PwrCmd;
        timeslice  :>> portionOfLife {
            snapshot  :>> start {
                attribute  :>> elapseTime = 0 [s];
                :>> pwrCmd.pwrLevel = 0;
            }
            snapshot  :>> done {
                attribute  :>> elapseTime = 2 [s];
                :>> pwrCmd.pwrLevel = 1;
            }
        }
        timeslice transportPeriod {
            snapshot  :>> start {
                attribute  :>> elapseTime = 1 [s];
            }
            snapshot :>> done {
                :>> elapseTime = 1.5 [s];
            }
           :>> pwrCmd.pwrLevel = 2*elapseTime.num;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 50) (line 2) (column 20) (len 5)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 50) (line 2) (column 20) (len 2))) (segment 1 (token "s") (name "s") (separator colon-colon) (span (offset 54) (line 2) (column 24) (len 1)))))
    (reference r1 (scope relative) (span (offset 195) (line 9) (column 24) (len 4)) (segments (segment 0 (token "Time") (name "Time") (separator none) (span (offset 195) (line 9) (column 24) (len 4)))))
    (reference r2 (scope relative) (span (offset 234) (line 10) (column 31) (len 6)) (segments (segment 0 (token "TimeOf") (name "TimeOf") (separator none) (span (offset 234) (line 10) (column 31) (len 6)))))
    (reference r3 (scope relative) (span (offset 241) (line 10) (column 38) (len 5)) (segments (segment 0 (token "start") (name "start") (separator none) (span (offset 241) (line 10) (column 38) (len 5)))))
    (reference r4 (scope relative) (span (offset 281) (line 11) (column 33) (len 13)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 281) (line 11) (column 33) (len 3))) (segment 1 (token "duration") (name "duration") (separator colon-colon) (span (offset 286) (line 11) (column 38) (len 8)))))
    (reference r5 (scope relative) (span (offset 318) (line 12) (column 23) (len 22)) (segments (segment 0 (token "localClock") (name "localClock") (separator none) (span (offset 318) (line 12) (column 23) (len 10))) (segment 1 (token "currentTime") (name "currentTime") (separator dot) (span (offset 329) (line 12) (column 34) (len 11)))))
    (reference r6 (scope relative) (span (offset 343) (line 12) (column 48) (len 9)) (segments (segment 0 (token "startTime") (name "startTime") (separator none) (span (offset 343) (line 12) (column 48) (len 9)))))
    (reference r7 (scope relative) (span (offset 355) (line 12) (column 60) (len 10)) (segments (segment 0 (token "elapseTime") (name "elapseTime") (separator none) (span (offset 355) (line 12) (column 60) (len 10)))))
  )
  (root (package (name "TimeVaryingAttribute") (body (import (target (span (span (offset 50) (line 2) (column 20) (len 5))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (item-def) (part-def (name "Transport2") (body (import (target (span (span (offset 195) (line 9) (column 24) (len 7))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 199) (line 9) (column 28) (len 3))) (separator (span (offset 199) (line 9) (column 28) (len 2))) (marker (span (offset 201) (line 9) (column 30) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (attribute-usage (declaration-name "startTime") (direction none) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 234) (line 10) (column 31) (len 13)) (invocation (callee (expression (span (offset 234) (line 10) (column 31) (len 6)) (ref r2))) (arguments (argument (parameter none) (value (expression (span (offset 241) (line 10) (column 38) (len 5)) (ref r3)))))))))) (body semicolon)) (attribute-usage (declaration-name "elapseTime") (direction none) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r4)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r5)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 343) (line 12) (column 48) (len 22)) (binary (operator "+") (left (expression (span (offset 343) (line 12) (column 48) (len 9)) (ref r6))) (right (expression (span (offset 355) (line 12) (column 60) (len 10)) (ref r7)))))))) (body semicolon)) (item-usage) (occurrence (portion timeslice) (declaration "") (target none)) (occurrence (portion timeslice) (declaration "transportPeriod") (target none)))))))
)
~~~
