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
      (diagnostic (code "recovered_occurrence_body_element") (severity error) (category parseerror) (span (offset 1069) (line 38) (column 13) (len 135)) (message "unexpected token in occurrence body"))
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
        attribute :>> localClock.currentTime = startTime + elapseTime;
        out item pwrCmd : PwrCmd;
        timeslice :>> portionOfLife {
            snapshot :>> start {
                attribute :>> elapseTime = 0 [s];
                attribute :>> pwrCmd.pwrLevel = 0;
            }
            snapshot :>> done {
                attribute :>> elapseTime = 2 [s];
                attribute :>> pwrCmd.pwrLevel = 1;
            }
        }
        timeslice transportPeriod {
            snapshot :>> start {
                attribute :>> elapseTime = 1 [s];
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
    (reference r1 (scope relative) (span (offset 112) (line 5) (column 29) (len 21)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 112) (line 5) (column 29) (len 12))) (segment 1 (token "Integer") (name "Integer") (separator colon-colon) (span (offset 126) (line 5) (column 43) (len 7)))))
    (reference r2 (scope relative) (span (offset 195) (line 9) (column 24) (len 4)) (segments (segment 0 (token "Time") (name "Time") (separator none) (span (offset 195) (line 9) (column 24) (len 4)))))
    (reference r3 (scope relative) (span (offset 234) (line 10) (column 31) (len 6)) (segments (segment 0 (token "TimeOf") (name "TimeOf") (separator none) (span (offset 234) (line 10) (column 31) (len 6)))))
    (reference r4 (scope relative) (span (offset 241) (line 10) (column 38) (len 5)) (segments (segment 0 (token "start") (name "start") (separator none) (span (offset 241) (line 10) (column 38) (len 5)))))
    (reference r5 (scope relative) (span (offset 281) (line 11) (column 33) (len 13)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 281) (line 11) (column 33) (len 3))) (segment 1 (token "duration") (name "duration") (separator colon-colon) (span (offset 286) (line 11) (column 38) (len 8)))))
    (reference r6 (scope relative) (span (offset 318) (line 12) (column 23) (len 22)) (segments (segment 0 (token "localClock") (name "localClock") (separator none) (span (offset 318) (line 12) (column 23) (len 10))) (segment 1 (token "currentTime") (name "currentTime") (separator dot) (span (offset 329) (line 12) (column 34) (len 11)))))
    (reference r7 (scope relative) (span (offset 343) (line 12) (column 48) (len 9)) (segments (segment 0 (token "startTime") (name "startTime") (separator none) (span (offset 343) (line 12) (column 48) (len 9)))))
    (reference r8 (scope relative) (span (offset 355) (line 12) (column 60) (len 10)) (segments (segment 0 (token "elapseTime") (name "elapseTime") (separator none) (span (offset 355) (line 12) (column 60) (len 10)))))
    (reference r9 (scope relative) (span (offset 400) (line 14) (column 25) (len 6)) (segments (segment 0 (token "PwrCmd") (name "PwrCmd") (separator none) (span (offset 400) (line 14) (column 25) (len 6)))))
    (reference r10 (scope relative) (span (offset 530) (line 18) (column 21) (len 10)) (segments (segment 0 (token "elapseTime") (name "elapseTime") (separator none) (span (offset 530) (line 18) (column 21) (len 10)))))
    (reference r11 (scope relative) (span (offset 570) (line 19) (column 21) (len 15)) (segments (segment 0 (token "pwrCmd") (name "pwrCmd") (separator none) (span (offset 570) (line 19) (column 21) (len 6))) (segment 1 (token "pwrLevel") (name "pwrLevel") (separator dot) (span (offset 577) (line 19) (column 28) (len 8)))))
    (reference r12 (scope relative) (span (offset 657) (line 22) (column 21) (len 10)) (segments (segment 0 (token "elapseTime") (name "elapseTime") (separator none) (span (offset 657) (line 22) (column 21) (len 10)))))
    (reference r13 (scope relative) (span (offset 697) (line 23) (column 21) (len 15)) (segments (segment 0 (token "pwrCmd") (name "pwrCmd") (separator none) (span (offset 697) (line 23) (column 21) (len 6))) (segment 1 (token "pwrLevel") (name "pwrLevel") (separator dot) (span (offset 704) (line 23) (column 28) (len 8)))))
    (reference r14 (scope relative) (span (offset 1023) (line 36) (column 21) (len 10)) (segments (segment 0 (token "elapseTime") (name "elapseTime") (separator none) (span (offset 1023) (line 36) (column 21) (len 10)))))
  )
  (root (package (name "TimeVaryingAttribute") (body brace (import (target (span (span (offset 50) (line 2) (column 20) (len 5))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (item-def (name "PwrCmd") (individual false) (specializes none) (body brace (attribute-usage (declaration-name "pwrLevel") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Transport2") (body brace (import (target (span (span (offset 195) (line 9) (column 24) (len 7))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 199) (line 9) (column 28) (len 3))) (separator (span (offset 199) (line 9) (column 28) (len 2))) (marker (span (offset 201) (line 9) (column 30) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (attribute-usage (declaration-name "startTime") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 234) (line 10) (column 31) (len 13)) (invocation (callee (expression (span (offset 234) (line 10) (column 31) (len 6)) (ref r3))) (arguments (argument (parameter none) (value (expression (span (offset 241) (line 10) (column 38) (len 5)) (ref r4)))))))))) (body semicolon)) (attribute-usage (declaration-name "elapseTime") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r5)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r6)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 343) (line 12) (column 48) (len 22)) (binary (operator "+") (left (expression (span (offset 343) (line 12) (column 48) (len 9)) (ref r7))) (right (expression (span (offset 355) (line 12) (column 60) (len 10)) (ref r8)))))))) (body semicolon)) (item-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "pwrCmd") (short-name none) (type (ref r9)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion timeslice) (extensions)) (declaration "") (short-name none) (target none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration "") (short-name none) (target none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r10)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 543) (line 18) (column 34) (len 5)) (literal-with-unit (value (expression (span (offset 543) (line 18) (column 34) (len 1)) (integer 0))) (unit (expression (span (offset 546) (line 18) (column 37) (len 1)) (bracket (expression (span (offset 546) (line 18) (column 37) (len 1)) (unit "s")))))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r11)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 588) (line 19) (column 39) (len 1)) (integer 0))))) (body semicolon)))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration "") (short-name none) (target none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r12)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 670) (line 22) (column 34) (len 5)) (literal-with-unit (value (expression (span (offset 670) (line 22) (column 34) (len 1)) (integer 2))) (unit (expression (span (offset 673) (line 22) (column 37) (len 1)) (bracket (expression (span (offset 673) (line 22) (column 37) (len 1)) (unit "s")))))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r13)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 715) (line 23) (column 39) (len 1)) (integer 1))))) (body semicolon)))))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion timeslice) (extensions)) (declaration "transportPeriod") (short-name none) (target none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration "") (short-name none) (target none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r14)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1036) (line 36) (column 34) (len 5)) (literal-with-unit (value (expression (span (offset 1036) (line 36) (column 34) (len 1)) (integer 1))) (unit (expression (span (offset 1039) (line 36) (column 37) (len 1)) (bracket (expression (span (offset 1039) (line 36) (column 37) (len 1)) (unit "s")))))))))) (body semicolon)))) (malformed (code "recovered_occurrence_body_element") (found "snapshot :>> done {") (span (offset 1069) (line 38) (column 13) (len 135))))))))))
)
~~~
