# META
~~~sexpr
(snapshot (type semantic) (description "A keyword-less ref usage in a part definition body retains a :>> redefinition trailing its typing (and a :> subsets clause), instead of falling through to recovery."))
~~~
# SOURCE
~~~sysml
part def Part {
    ref self: Part :>> Item::self;
    ref helper: Part :>> Item::helper :> related;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "part_ref_trailing_redefinition.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
part def Part {
    ref self : Part :>> Item::self;
    ref helper : Part :>> Item::helper :> related;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 30) (line 2) (column 15) (len 4)) (segments (segment 0 (token "Part") (name "Part") (separator none) (span (offset 30) (line 2) (column 15) (len 4)))))
    (reference r1 (scope relative) (span (offset 39) (line 2) (column 24) (len 10)) (segments (segment 0 (token "Item") (name "Item") (separator none) (span (offset 39) (line 2) (column 24) (len 4))) (segment 1 (token "self") (name "self") (separator colon-colon) (span (offset 45) (line 2) (column 30) (len 4)))))
    (reference r2 (scope relative) (span (offset 67) (line 3) (column 17) (len 4)) (segments (segment 0 (token "Part") (name "Part") (separator none) (span (offset 67) (line 3) (column 17) (len 4)))))
    (reference r3 (scope relative) (span (offset 76) (line 3) (column 26) (len 12)) (segments (segment 0 (token "Item") (name "Item") (separator none) (span (offset 76) (line 3) (column 26) (len 4))) (segment 1 (token "helper") (name "helper") (separator colon-colon) (span (offset 82) (line 3) (column 32) (len 6)))))
    (reference r4 (scope relative) (span (offset 92) (line 3) (column 42) (len 7)) (segments (segment 0 (token "related") (name "related") (separator none) (span (offset 92) (line 3) (column 42) (len 7)))))
  )
  (root (part-def (name "Part") (body (ref (name "self") (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (redefines (relationship (kind redefines) (implied false) (targets (ref r1)))) (subsets none) (body semicolon)) (ref (name "helper") (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (redefines (relationship (kind redefines) (implied false) (targets (ref r3)))) (subsets (relationship (kind subsets) (implied false) (targets (ref r4)))) (body semicolon)))))
)
~~~
