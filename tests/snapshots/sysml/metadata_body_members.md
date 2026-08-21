# META
~~~sexpr
(snapshot (type semantic) (description "metadata def bodies dispatch the same structured members as other attribute-shaped bodies (ref redefinitions, nested items, connects) instead of dropping them to opaque capture (spec42 Gap 40)."))
~~~
# SOURCE
~~~sysml
package MetadataBodyMembers {
    metadata def MetadataItem :> Metaobject, Item {
        ref self : MetadataItem redefines Metaobject::self, Item::self;
        item picture : Picture;
        attribute rationale : String;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "metadata_body_members.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package MetadataBodyMembers {
    metadata def MetadataItem :> Metaobject, Item {
        ref self : MetadataItem :>> Metaobject::self, Item::self;
        item picture : Picture;
        attribute rationale : String;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 63) (line 2) (column 34) (len 10)) (segments (segment 0 (token "Metaobject") (name "Metaobject") (separator none) (span (offset 63) (line 2) (column 34) (len 10)))))
    (reference r1 (scope relative) (span (offset 75) (line 2) (column 46) (len 4)) (segments (segment 0 (token "Item") (name "Item") (separator none) (span (offset 75) (line 2) (column 46) (len 4)))))
    (reference r2 (scope relative) (span (offset 101) (line 3) (column 20) (len 12)) (segments (segment 0 (token "MetadataItem") (name "MetadataItem") (separator none) (span (offset 101) (line 3) (column 20) (len 12)))))
    (reference r3 (scope relative) (span (offset 124) (line 3) (column 43) (len 16)) (segments (segment 0 (token "Metaobject") (name "Metaobject") (separator none) (span (offset 124) (line 3) (column 43) (len 10))) (segment 1 (token "self") (name "self") (separator colon-colon) (span (offset 136) (line 3) (column 55) (len 4)))))
    (reference r4 (scope relative) (span (offset 142) (line 3) (column 61) (len 10)) (segments (segment 0 (token "Item") (name "Item") (separator none) (span (offset 142) (line 3) (column 61) (len 4))) (segment 1 (token "self") (name "self") (separator colon-colon) (span (offset 148) (line 3) (column 67) (len 4)))))
    (reference r5 (scope relative) (span (offset 177) (line 4) (column 24) (len 7)) (segments (segment 0 (token "Picture") (name "Picture") (separator none) (span (offset 177) (line 4) (column 24) (len 7)))))
    (reference r6 (scope relative) (span (offset 216) (line 5) (column 31) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 216) (line 5) (column 31) (len 6)))))
  )
  (root (package (name "MetadataBodyMembers") (body brace (metadata-def (name "MetadataItem") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r0) (ref r1)))) (body brace (ref (name "self") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines (relationship (kind redefines) (implied false) (targets (ref r3) (ref r4)))) (subsets none) (body semicolon)) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "picture") (short-name none) (type (ref r5)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (attribute-usage (declaration-name "rationale") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
