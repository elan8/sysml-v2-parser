# META
~~~sexpr
(snapshot (type semantic) (description "OccurrenceUsagePrefix admits EndUsagePrefix as the alternative to its basic slots (reference SysML.xtext 836-843), so the kind-keyworded `end port`, `end [1] part`, `end item` and `end occurrence` usages the example corpus and the normative library author (`Interfaces.sysml:72`, `Flows.sysml:82`) are each their family's own node with an `end` head, retaining an owned cross feature when one is written. The published .kebnf (SysML BNF 564-570) omits the alternative; see planning/spec42-upstream-gap-audit.md. The keyword-less `end valid : T;` remains an EndDecl."))
~~~
# SOURCE
~~~sysml
package PilotOccurrenceEndPrefixRecovery {
    connection def C {
        end port p : P;
        end [1] part q : Q;
        end item r : R;
        end occurrence s : S;
        end valid : T;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "occurrence_end_prefix.md"
    (diagnostics
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
    (reference r0 (scope relative) (span (offset 87) (line 3) (column 22) (len 1)) (segments (segment 0 (token "P") (name "P") (separator none) (span (offset 87) (line 3) (column 22) (len 1)))))
    (reference r1 (scope relative) (span (offset 115) (line 4) (column 26) (len 1)) (segments (segment 0 (token "Q") (name "Q") (separator none) (span (offset 115) (line 4) (column 26) (len 1)))))
    (reference r2 (scope relative) (span (offset 139) (line 5) (column 22) (len 1)) (segments (segment 0 (token "R") (name "R") (separator none) (span (offset 139) (line 5) (column 22) (len 1)))))
    (reference r3 (scope relative) (span (offset 192) (line 7) (column 21) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 192) (line 7) (column 21) (len 1)))))
  )
  (root (package (name "PilotOccurrenceEndPrefixRecovery") (body brace (connection-def (name "C") (modifiers) (role ordinary) (specializes none) (body brace (port-usage (prefix (end (cross none)) (extensions)) (declaration-name "p") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage (then false) (prefix (end (cross ((direction none) (derived false) (variance none) (constant false) (reference false) (name none) (short-name none) (typing none) (multiplicity (lower (expression (span (offset 103) (line 4) (column 14) (len 1)) (integer 1))) (upper (expression (span (offset 103) (line 4) (column 14) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)))) (extensions)) (declaration-name "q") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (item-usage (prefix (end (cross none)) (extensions)) (declaration "r") (short-name none) (type (ref r2)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (occurrence (prefix (end (cross none)) (extensions)) (declaration "s") (short-name none) (target none) (body semicolon)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (short-name none) (identity (declaration (name "valid") (span (offset 184) (line 7) (column 13) (len 5)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (references none) (multiplicity none) (redefines none) (crosses none)))))))
)
~~~
