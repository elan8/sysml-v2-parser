# META
~~~sexpr
(snapshot (type semantic) (description "The anonymous `:>` and `:>>` specialization shorthand publishes no declared name: repeated shorthand members in one body report `(declaration-name none)` and keep distinct arena references to their targets, so neither aliases the other nor subsets itself (spec42 Gap 57)."))
~~~
# SOURCE
~~~sysml
metadata def M {
    :> annotatedElement : SysML::Usage;
    :> annotatedElement : SysML::Definition;
    :>> baseType = SysML::PartUsage;
    attribute named :> annotatedElement;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "anonymous_specialization_shorthand.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
metadata def M {
    attribute :> annotatedElement : SysML::Usage;
    attribute :> annotatedElement : SysML::Definition;
    attribute :>> baseType = SysML::PartUsage;
    attribute named :> annotatedElement;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 43) (line 2) (column 27) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 43) (line 2) (column 27) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 50) (line 2) (column 34) (len 5)))))
    (reference r1 (scope relative) (span (offset 24) (line 2) (column 8) (len 16)) (segments (segment 0 (token "annotatedElement") (name "annotatedElement") (separator none) (span (offset 24) (line 2) (column 8) (len 16)))))
    (reference r2 (scope relative) (span (offset 83) (line 3) (column 27) (len 17)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 83) (line 3) (column 27) (len 5))) (segment 1 (token "Definition") (name "Definition") (separator colon-colon) (span (offset 90) (line 3) (column 34) (len 10)))))
    (reference r3 (scope relative) (span (offset 64) (line 3) (column 8) (len 16)) (segments (segment 0 (token "annotatedElement") (name "annotatedElement") (separator none) (span (offset 64) (line 3) (column 8) (len 16)))))
    (reference r4 (scope relative) (span (offset 110) (line 4) (column 9) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 110) (line 4) (column 9) (len 8)))))
    (reference r5 (scope relative) (span (offset 121) (line 4) (column 20) (len 16)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 121) (line 4) (column 20) (len 5))) (segment 1 (token "PartUsage") (name "PartUsage") (separator colon-colon) (span (offset 128) (line 4) (column 27) (len 9)))))
    (reference r6 (scope relative) (span (offset 162) (line 5) (column 24) (len 16)) (segments (segment 0 (token "annotatedElement") (name "annotatedElement") (separator none) (span (offset 162) (line 5) (column 24) (len 16)))))
  )
  (root (metadata-def (name "M") (abstract false) (specializes none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets (relationship (kind subsets) (implied false) (targets (ref r1)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets (relationship (kind subsets) (implied false) (targets (ref r3)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 121) (line 4) (column 20) (len 16)) (ref r5))))) (body semicolon)) (attribute-usage (declaration-name "named") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r6)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))
)
~~~
