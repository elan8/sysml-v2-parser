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
    ref self :>> Item::self : Part;
    ref helper :>> Item::helper :> related : Part;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (part-def (name "Part") (body (ref) (ref))))
)
~~~
