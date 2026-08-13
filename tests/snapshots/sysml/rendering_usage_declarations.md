# META
~~~sexpr
(snapshot (type semantic) (description "Rendering usages retain abstract, multiplicity, ordered/nonunique, subsets, redefines, and value clauses; the anonymous redefinition form nests inside a rendering usage body."))
~~~
# SOURCE
~~~sysml
package Views {
    abstract rendering renderings : Rendering[0..*] nonunique :> parts {
        doc /* renderings is the base rendering. */
    }
    rendering asElementTable : TabularRendering[1] :> renderings {
        view columnView[0..*] ordered {
            doc /* The Views to be rendered in the column cells. */
        }
        rendering :>> subrenderings[0..*] = columnView.viewRendering;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "rendering_usage_declarations.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Views {
    abstract rendering renderings : Rendering[0..*] nonunique :> parts {
        doc
        /* renderings is the base rendering. */
    }
    rendering asElementTable : TabularRendering[1] :> renderings {
        view columnView[0..*] ordered {
            doc
            /* The Views to be rendered in the column cells. */
        }
        rendering[0..*] :>> subrenderings = columnView.viewRendering;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "Views") (body (rendering-usage) (rendering-usage))))
)
~~~
