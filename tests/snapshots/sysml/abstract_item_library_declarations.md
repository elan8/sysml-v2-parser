# META
~~~sexpr
(snapshot (type semantic) (description "Package-level abstract item usages with multiplicity properties and subsets clauses parse as structured item usages instead of the extended-library-decl fallback."))
~~~
# SOURCE
~~~sysml
library package Items {
    abstract item items : Item[0..*] nonunique :> objects {
        doc /* items is the base feature of all ItemUsages. */
    }
    abstract item metadataItems : MetadataItem[0..*] :> metaobjects, items {
        doc /* metadataItems is the base feature of all MetadataUsages. */
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "abstract_item_library_declarations.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
library package Items {
    abstract item items : Item[0..*] nonunique :> objects {
        doc
        /* items is the base feature of all ItemUsages. */
    }
    abstract item metadataItems : MetadataItem[0..*] :> metaobjects, items {
        doc
        /* metadataItems is the base feature of all MetadataUsages. */
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 50) (line 2) (column 27) (len 4)) (segments (segment 0 (token "Item") (name "Item") (separator none) (span (offset 50) (line 2) (column 27) (len 4)))))
    (reference r1 (scope relative) (span (offset 74) (line 2) (column 51) (len 7)) (segments (segment 0 (token "objects") (name "objects") (separator none) (span (offset 74) (line 2) (column 51) (len 7)))))
    (reference r2 (scope relative) (span (offset 187) (line 5) (column 35) (len 12)) (segments (segment 0 (token "MetadataItem") (name "MetadataItem") (separator none) (span (offset 187) (line 5) (column 35) (len 12)))))
    (reference r3 (scope relative) (span (offset 209) (line 5) (column 57) (len 11)) (segments (segment 0 (token "metaobjects") (name "metaobjects") (separator none) (span (offset 209) (line 5) (column 57) (len 11)))))
    (reference r4 (scope relative) (span (offset 222) (line 5) (column 70) (len 5)) (segments (segment 0 (token "items") (name "items") (separator none) (span (offset 222) (line 5) (column 70) (len 5)))))
  )
  (root (library-package (name "Items") (standard false) (body brace (item-usage (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "items") (short-name none) (type (ref r0)) (multiplicity (lower (expression (span (offset 55) (line 2) (column 32) (len 1)) (integer 0))) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (subsets (relationship (kind subsets) (implied false) (targets (ref r1)))) (redefines none) (value none) (body brace (doc (name none) (locale none) (body (span (offset 98) (line 3) (column 15) (len 46)) (normalized "items is the base feature of all ItemUsages. "))))) (item-usage (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "metadataItems") (short-name none) (type (ref r2)) (multiplicity (lower (expression (span (offset 200) (line 5) (column 48) (len 1)) (integer 0))) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r3) (ref r4)))) (redefines none) (value none) (body brace (doc (name none) (locale none) (body (span (offset 244) (line 6) (column 15) (len 58)) (normalized "metadataItems is the base feature of all MetadataUsages. "))))))))
)
~~~
