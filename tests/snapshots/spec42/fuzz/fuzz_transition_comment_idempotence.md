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
      (diagnostic (code "missing_closing_brace") (severity none) (category parseerror) (span (offset 420) (line 11) (column 98) (len 1)) (message "missing closing '}'"))
    )
  )
)
~~~
# FORMAT
~~~sexpr
(stable-idempotent)
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (malformed (code "missing_closing_brace") (found none) (span (offset 0) (line 1) (column 1) (len 420))))
)
~~~
