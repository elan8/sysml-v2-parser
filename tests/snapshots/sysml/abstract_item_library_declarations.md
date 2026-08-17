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
  )
  (root (library-package (name "Items") (standard false) (body brace (item-usage (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "items")) (item-usage (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "metadataItems")))))
)
~~~
