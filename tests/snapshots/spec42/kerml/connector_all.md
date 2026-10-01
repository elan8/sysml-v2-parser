# META
~~~sexpr
(snapshot (type semantic) (description "KerML connector with 'all' keyword (stdlib patterns from OccurrenceFunctions/TransitionPerformances)"))
~~~
# SOURCE
~~~sysml
package ConnectorAll {
    connector all during: HappensDuring from self to occ;
    connector all guardConstraint: TPCGuardConstraint[*] from transitionLink to guard;
    connector all x from a to b;
    connector all from a to b;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "connector_all.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ConnectorAll {
    connector all during : HappensDuring from self to occ;
    connector all guardConstraint : TPCGuardConstraint[*] from transitionLink to guard;
    connector all x from a to b;
    connector all from a to b;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 49) (line 2) (column 27) (len 13)) (segments (segment 0 (token "HappensDuring") (name "HappensDuring") (separator none) (span (offset 49) (line 2) (column 27) (len 13)))))
    (reference r1 (scope relative) (span (offset 68) (line 2) (column 46) (len 4)) (segments (segment 0 (token "self") (name "self") (separator none) (span (offset 68) (line 2) (column 46) (len 4)))))
    (reference r2 (scope relative) (span (offset 76) (line 2) (column 54) (len 3)) (segments (segment 0 (token "occ") (name "occ") (separator none) (span (offset 76) (line 2) (column 54) (len 3)))))
    (reference r3 (scope relative) (span (offset 116) (line 3) (column 36) (len 18)) (segments (segment 0 (token "TPCGuardConstraint") (name "TPCGuardConstraint") (separator none) (span (offset 116) (line 3) (column 36) (len 18)))))
    (reference r4 (scope relative) (span (offset 143) (line 3) (column 63) (len 14)) (segments (segment 0 (token "transitionLink") (name "transitionLink") (separator none) (span (offset 143) (line 3) (column 63) (len 14)))))
    (reference r5 (scope relative) (span (offset 161) (line 3) (column 81) (len 5)) (segments (segment 0 (token "guard") (name "guard") (separator none) (span (offset 161) (line 3) (column 81) (len 5)))))
    (reference r6 (scope relative) (span (offset 193) (line 4) (column 26) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 193) (line 4) (column 26) (len 1)))))
    (reference r7 (scope relative) (span (offset 198) (line 4) (column 31) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 198) (line 4) (column 31) (len 1)))))
    (reference r8 (scope relative) (span (offset 224) (line 5) (column 24) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 224) (line 5) (column 24) (len 1)))))
    (reference r9 (scope relative) (span (offset 229) (line 5) (column 29) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 229) (line 5) (column 29) (len 1)))))
  )
  (root (package (name "ConnectorAll") (body brace (kerml-connector (all true) (name "during") (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (from (connector-end (multiplicity none) (target (ref r1)) (references none))) (to (connector-end (multiplicity none) (target (ref r2)) (references none))) (body semicolon)) (kerml-connector (all true) (name "guardConstraint") (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3))))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness none)) (from (connector-end (multiplicity none) (target (ref r4)) (references none))) (to (connector-end (multiplicity none) (target (ref r5)) (references none))) (body semicolon)) (kerml-connector (all true) (name "x") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (from (connector-end (multiplicity none) (target (ref r6)) (references none))) (to (connector-end (multiplicity none) (target (ref r7)) (references none))) (body semicolon)) (kerml-connector (all true) (name none) (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (from (connector-end (multiplicity none) (target (ref r8)) (references none))) (to (connector-end (multiplicity none) (target (ref r9)) (references none))) (body semicolon)))))
)
~~~
