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
        satisfy requirement viewpointConformance by that {
            require viewpointSatisfactions {
                doc
                /* The required ViewpointChecks. */
                'ref' :>> ownedPerformances::this, subperformances::this default that.that;
            }
        }
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
    (reference r0 (scope relative) (span (offset 95) (line 3) (column 38) (len 4)) (segments (segment 0 (token "View") (name "View") (separator none) (span (offset 95) (line 3) (column 38) (len 4)))))
    (reference r1 (scope relative) (span (offset 109) (line 3) (column 52) (len 5)) (segments (segment 0 (token "views") (name "views") (separator none) (span (offset 109) (line 3) (column 52) (len 5)))))
    (reference r2 (scope relative) (span (offset 209) (line 6) (column 34) (len 14)) (segments (segment 0 (token "ViewpointCheck") (name "ViewpointCheck") (separator none) (span (offset 209) (line 6) (column 34) (len 14)))))
    (reference r3 (scope relative) (span (offset 202) (line 6) (column 27) (len 4)) (segments (segment 0 (token "self") (name "self") (separator none) (span (offset 202) (line 6) (column 27) (len 4)))))
    (reference r4 (scope relative) (span (offset 336) (line 8) (column 53) (len 4)) (segments (segment 0 (token "that") (name "that") (separator none) (span (offset 336) (line 8) (column 53) (len 4)))))
    (reference r5 (scope relative) (span (offset 630) (line 16) (column 46) (len 16)) (segments (segment 0 (token "interfacingPorts") (name "interfacingPorts") (separator none) (span (offset 630) (line 16) (column 46) (len 16)))))
  )
  (root (package (name "ViewDefBodyMembers") (body brace (view-def (name "View") (short-name none) (modifiers abstract) (specializes none) (body brace (ref (name "subviews") (short-name none) (prefix (direction none) (derived false) (usage-prefix abstract) (constant false)) (kind view) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (redefines none) (subsets (relationship (kind subsets) (implied false) (targets (ref r1)))) (body brace (doc))) (ref (name "") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind viewpoint) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (redefines (relationship (kind redefines) (implied false) (targets (ref r3)))) (subsets none) (body semicolon)) (viewpoint-usage) (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (declaration (name "viewpointConformance") (short-name none))) (typing none) (multiplicity none) (ordered false) (nonunique false) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r4)) (body brace (require-constraint))))) (part-def (name "Ports") (body brace (ref (name "outgoingTransfersFromSelf") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing none) (redefines none) (subsets (relationship (kind subsets) (implied false) (targets (ref r5)))) (body brace (end (short-name none) (identity (declaration (name "source") (span (offset 669) (line 17) (column 21) (len 6)))) (typing none) (references none) (redefines none) (crosses none)) (end (short-name none) (identity (declaration (name "target") (span (offset 697) (line 18) (column 21) (len 6)))) (typing none) (references none) (redefines none) (crosses none)))))) (rendering-def))))
)
~~~
