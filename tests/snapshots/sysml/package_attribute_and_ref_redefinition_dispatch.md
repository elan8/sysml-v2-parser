# META
~~~sexpr
(snapshot (type semantic) (description "Package members distinguish the mandatory-def attribute definition production from ordinary attribute usages, and an anonymous ref redefinition in a part usage body retains its target as a relationship rather than turning it into a declaration name."))
~~~
# SOURCE
~~~sysml
package PackageAttributeAndRefRedefinitionDispatch {
    attribute bare;
    attribute typed : T;
    attribute valued = 1;
    attribute typedValued : T = 2;
    attribute def explicit :> T;
    part holder {
        ref :>> system;
        ref named : T :>> inherited;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "package_attribute_and_ref_redefinition_dispatch.md"
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
    (reference r0 (scope relative) (span (offset 189) (line 6) (column 31) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 189) (line 6) (column 31) (len 1)))))
    (reference r1 (scope relative) (span (offset 226) (line 8) (column 17) (len 6)) (segments (segment 0 (token "system") (name "system") (separator none) (span (offset 226) (line 8) (column 17) (len 6)))))
    (reference r2 (scope relative) (span (offset 254) (line 9) (column 21) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 254) (line 9) (column 21) (len 1)))))
    (reference r3 (scope relative) (span (offset 260) (line 9) (column 27) (len 9)) (segments (segment 0 (token "inherited") (name "inherited") (separator none) (span (offset 260) (line 9) (column 27) (len 9)))))
  )
  (root (package (name "PackageAttributeAndRefRedefinitionDispatch") (body brace (attribute-usage) (attribute-usage) (attribute-usage) (attribute-usage) (attribute-def (declaration-name "explicit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "holder") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (ref (name "") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1)))) (subsets none) (body semicolon)) (ref (name "named") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (redefines (relationship (kind redefines) (implied false) (targets (ref r3)))) (subsets none) (body semicolon)))))))
)
~~~
