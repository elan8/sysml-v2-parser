# META
~~~sexpr
(snapshot (type semantic) (description "Short names on usage declarations previously lacking an owning AST field survive parsing and typed emission."))
~~~
# SOURCE
~~~sysml
package UpstreamGapShortNames {
    action <act> run;
    occurrence <occ> happening;
    constraint <con> check;
    connection def Link {
        ref <reference> subject : Thing;
        end <source> source : Thing;
        end <target> target : Thing;
    }
    calc def Calculation {
        return <result> result : Thing;
    }
    view <vw> overview;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "upstream_gap_short_names.md"
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
    (reference r0 (scope relative) (span (offset 174) (line 6) (column 35) (len 5)) (segments (segment 0 (token "Thing") (name "Thing") (separator none) (span (offset 174) (line 6) (column 35) (len 5)))))
    (reference r1 (scope relative) (span (offset 211) (line 7) (column 31) (len 5)) (segments (segment 0 (token "Thing") (name "Thing") (separator none) (span (offset 211) (line 7) (column 31) (len 5)))))
    (reference r2 (scope relative) (span (offset 248) (line 8) (column 31) (len 5)) (segments (segment 0 (token "Thing") (name "Thing") (separator none) (span (offset 248) (line 8) (column 31) (len 5)))))
  )
  (root (package (name "UpstreamGapShortNames") (body brace (action-usage (name "run") (short-name "act") (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "happening") (short-name "occ") (target none) (body semicolon)) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "check") (short-name "con") (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (connection-def (name "Link") (modifiers) (role ordinary) (specializes none) (body brace (ref (name "subject") (short-name "reference") (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)) (end (introducer bare) (short-name "source") (identity (declaration (name "source") (span (offset 202) (line 7) (column 22) (len 6)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (references none) (multiplicity none) (redefines none) (crosses none) (nested-usage none)) (end (introducer bare) (short-name "target") (identity (declaration (name "target") (span (offset 239) (line 8) (column 22) (len 6)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (references none) (multiplicity none) (redefines none) (crosses none) (nested-usage none)))) (calc-def (name "Calculation") (modifiers) (body brace (return-declaration (name "result") (short-name "result")))) (view (name "overview") (short-name "vw") (type none) (body semicolon)))))
)
~~~
