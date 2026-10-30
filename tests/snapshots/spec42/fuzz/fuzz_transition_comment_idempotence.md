# META
~~~sexpr
(snapshot (type semantic) (description "Fuzz: transition with line comment in absorbed tokens stops before comment"))
~~~
# SOURCE
~~~sysml
package j {
state def S {
    entry; then off;
    state off;
    transition t first accept X state package Timebehavior TakePicture          //ce [0..1];
                member step 'm' : ControlPerformances::MergePerformance [0..1] featured by TakePicture_snapshoure {
        public import 'merge';
}
                }

                // var step focus [0..1];               member step package RiskMetadataExEmple {
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz_transition_comment_idempotence.md"
    (diagnostics
      (diagnostic (code "recovered_state_body_element") (severity error) (category parseerror) (span (offset 66) (line 5) (column 5) (len 254)) (message "unexpected token in state body"))
      (diagnostic (code "missing_closing_brace") (severity none) (category parseerror) (span (offset 420) (line 11) (column 98) (len 1)) (message "missing closing '}'"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package j {
    state def S {
        entry;
        then off;
        state off;
        transition t first accept X state package Timebehavior TakePicture          //ce [0..1];
                member step 'm' : ControlPerformances::MergePerformance [0..1] featured by TakePicture_snapshoure {
        public import 'merge';
}
    }
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 42) (line 3) (column 17) (len 3)) (segments (segment 0 (token "off") (name "off") (separator none) (span (offset 42) (line 3) (column 17) (len 3)))))
  )
  (root (package (name "j") (body brace (unterminated) (state-def (name "S") (modifiers) (body brace (entry (action empty) (body semicolon)) (then (state (ref r0))) (state-usage (name "off") (short-name none) (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (malformed (code "recovered_state_body_element") (found "transition t first accept X state package Timebehavior TakeP") (span (offset 66) (line 5) (column 5) (len 254))))))))
)
~~~
