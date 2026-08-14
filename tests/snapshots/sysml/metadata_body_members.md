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
  )
  (root (package (name "MetadataBodyMembers") (body (metadata-def))))
)
~~~
