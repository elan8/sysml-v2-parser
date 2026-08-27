# META
~~~sexpr
(snapshot (type semantic) (description "A nested snapshot occurrence retains an anonymous redefinition binding whose value is a decimal quantity expression, without depending on the declaration name (SysML textual BNF 573-585; pinned Pilot SysML.xtext 839-855)."))
~~~
# SOURCE
~~~sysml
package TimesliceSnapshotDecimalBinding {
    timeslice transportPeriod {
        snapshot :>> done {
            :>> elapseTime = 1.5 [s];
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "timeslice_snapshot_decimal_binding.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package TimesliceSnapshotDecimalBinding {
    timeslice transportPeriod {
        snapshot :>> done {
            attribute :>> elapseTime = 1.5[s];
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 118) (line 4) (column 17) (len 10)) (segments (segment 0 (token "elapseTime") (name "elapseTime") (separator none) (span (offset 118) (line 4) (column 17) (len 10)))))
    (reference r1 (scope relative) (span (offset 136) (line 4) (column 35) (len 1)) (segments (segment 0 (token "s") (name "s") (separator none) (span (offset 136) (line 4) (column 35) (len 1)))))
  )
  (root (package (name "TimesliceSnapshotDecimalBinding") (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion timeslice) (extensions)) (declaration "transportPeriod") (short-name none) (target none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration none) (short-name none) (target none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r0)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 131) (line 4) (column 30) (len 7)) (bracket (base (expression (span (offset 131) (line 4) (column 30) (len 3)) (real "1.5"))) (operands (sequence-list (element first (expression (span (offset 136) (line 4) (column 35) (len 1)) (ref r1)))))))))) (body semicolon)))))))))
)
~~~
