# META
~~~sexpr
(snapshot (type semantic) (description "View definition bodies accept the members their grammar allows: ref declarations with a feature-kind keyword, nested viewpoint usages, and satisfy requirement ... by ... . Each is parsed by the same production the package and part scopes already dispatch; this scope simply had no arm for them, so every one was an unexpected keyword or unsupported grammar. A satisfy body carries a brace-bodied require member, and a usage body carries bare end declarations."))
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
        satisfy requirement viewpointConformance by that {
            require viewpointSatisfactions {
                doc /* The required ViewpointChecks. */
                ref :>> ownedPerformances::this, subperformances::this default that.that;
            }
        }
    }
    part def Ports {
        ref :>> outgoingTransfersFromSelf :> interfacingPorts {
            end ref source;
            end ref target;
        }
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
        satisfy requirement viewpointConformance by that {}
    }
    part def Ports {
        ref outgoingTransfersFromSelf :> interfacingPorts {
            end source;
            end target;
        }
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
    (reference r0 (scope relative) (span (offset 630) (line 16) (column 46) (len 16)) (segments (segment 0 (token "interfacingPorts") (name "interfacingPorts") (separator none) (span (offset 630) (line 16) (column 46) (len 16)))))
  )
  (root (package (name "ViewDefBodyMembers") (body (view-def) (part-def (name "Ports") (body (ref (name "outgoingTransfersFromSelf") (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing none) (redefines none) (subsets (relationship (kind subsets) (implied false) (targets (ref r0)))) (body (end (identity (declaration (name "source") (span (offset 669) (line 17) (column 21) (len 6)))) (typing none) (references none) (redefines none) (crosses none)) (end (identity (declaration (name "target") (span (offset 697) (line 18) (column 21) (len 6)))) (typing none) (references none) (redefines none) (crosses none)))))) (rendering-def))))
)
~~~
