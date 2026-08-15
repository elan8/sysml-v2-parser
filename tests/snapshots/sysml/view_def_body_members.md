# META
~~~sexpr
(snapshot (type semantic) (description "View definition bodies accept the members their grammar allows: ref declarations with a feature-kind keyword, nested viewpoint usages, and satisfy requirement ... by ... . Each is parsed by the same production the package and part scopes already dispatch; this scope simply had no arm for them, so every one was an unexpected keyword or unsupported grammar."))
~~~
# SOURCE
~~~sysml
package ViewDefBodyMembers {
    abstract view def View {
        abstract ref view subviews : View[0..*] :> views {
            doc /* The subviews of this view. */
        }
        ref viewpoint :>> self : ViewpointCheck;
        viewpoint viewpointSatisfactions : ViewpointCheck;
        satisfy requirement viewpointConformance by this;
    }
    abstract rendering def Rendering {
        ref rendering :>> self : Rendering;
        abstract ref rendering subrenderings : Rendering[0..*] :> renderings;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "view_def_body_members.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ViewDefBodyMembers {
    abstract view def View {
        abstract ref view subviews : View[0..*] :> views {
            doc
            /* The subviews of this view. */
        }
        ref viewpoint : ViewpointCheck :>> self;
        viewpoint viewpointSatisfactions : ViewpointCheck;
        satisfy requirement viewpointConformance by this;
    }
    abstract rendering def Rendering {
        ref rendering : Rendering :>> self;
        abstract ref rendering subrenderings : Rendering[0..*] :> renderings;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "ViewDefBodyMembers") (body (view-def) (rendering-def))))
)
~~~
