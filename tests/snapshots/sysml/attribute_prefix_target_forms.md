# META
~~~sexpr
(snapshot (type semantic) (description "GH-113: an attribute usage may be written with a subsetting-family target and no declaration name of its own (attribute :>> target;, attribute ::> target;, attribute :> target;). Emission used to derive a display name from the target and then emit the clause as well, turning `attribute :>> differencesOf[1];` into the self-referential `attribute differencesOf :>> differencesOf;`. The format section shows the authored form coming back unchanged, and the AST section shows the empty declaration name that makes it anonymous."))
~~~
# SOURCE
~~~sysml
package AttributePrefixTargetForms {
    part def Q {
        attribute differencesOf[1];
        attribute :>> differencesOf[1];
        attribute :> differencesOf[1];
        attribute ::> m = ms.m;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "attribute_prefix_target_forms.md"
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
    (reference r0 (scope relative) (span (offset 112) (line 4) (column 23) (len 13)) (segments (segment 0 (token "differencesOf") (name "differencesOf") (separator none) (span (offset 112) (line 4) (column 23) (len 13)))))
    (reference r1 (scope relative) (span (offset 151) (line 5) (column 22) (len 13)) (segments (segment 0 (token "differencesOf") (name "differencesOf") (separator none) (span (offset 151) (line 5) (column 22) (len 13)))))
    (reference r2 (scope relative) (span (offset 191) (line 6) (column 23) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 191) (line 6) (column 23) (len 1)))))
    (reference r3 (scope relative) (span (offset 195) (line 6) (column 27) (len 2)) (segments (segment 0 (token "ms") (name "ms") (separator none) (span (offset 195) (line 6) (column 27) (len 2)))))
    (reference r4 (scope relative) (span (offset 198) (line 6) (column 30) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 198) (line 6) (column 30) (len 1)))))
  )
  (root (package (name "AttributePrefixTargetForms") (body brace (part-def (name "Q") (body brace (attribute-usage (declaration-name "differencesOf") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r0)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r1)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references (relationship (kind references) (implied false) (targets (ref r2)))) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 195) (line 6) (column 27) (len 4)) (member-access (base (expression (span (offset 195) (line 6) (column 27) (len 2)) (ref r3))) (separator dot) (member (ref r4))))))) (body semicolon)))))))
)
~~~
