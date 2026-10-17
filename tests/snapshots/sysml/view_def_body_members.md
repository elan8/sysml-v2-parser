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
        ref rendering : Rendering :>> self;
        abstract ref rendering subrenderings : Rendering[0..*] :> renderings;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 95) (line 3) (column 38) (len 4)) (segments (segment 0 (token "View") (name "View") (separator none) (span (offset 95) (line 3) (column 38) (len 4)))))
    (reference r1 (scope relative) (span (offset 109) (line 3) (column 52) (len 5)) (segments (segment 0 (token "views") (name "views") (separator none) (span (offset 109) (line 3) (column 52) (len 5)))))
    (reference r2 (scope relative) (span (offset 209) (line 6) (column 34) (len 14)) (segments (segment 0 (token "ViewpointCheck") (name "ViewpointCheck") (separator none) (span (offset 209) (line 6) (column 34) (len 14)))))
    (reference r3 (scope relative) (span (offset 202) (line 6) (column 27) (len 4)) (segments (segment 0 (token "self") (name "self") (separator none) (span (offset 202) (line 6) (column 27) (len 4)))))
    (reference r4 (scope relative) (span (offset 268) (line 7) (column 44) (len 14)) (segments (segment 0 (token "ViewpointCheck") (name "ViewpointCheck") (separator none) (span (offset 268) (line 7) (column 44) (len 14)))))
    (reference r5 (scope relative) (span (offset 336) (line 8) (column 53) (len 4)) (segments (segment 0 (token "that") (name "that") (separator none) (span (offset 336) (line 8) (column 53) (len 4)))))
    (reference r6 (scope relative) (span (offset 363) (line 9) (column 21) (len 22)) (segments (segment 0 (token "viewpointSatisfactions") (name "viewpointSatisfactions") (separator none) (span (offset 363) (line 9) (column 21) (len 22)))))
    (reference r7 (scope relative) (span (offset 523) (line 11) (column 80) (len 4)) (segments (segment 0 (token "that") (name "that") (separator none) (span (offset 523) (line 11) (column 80) (len 4)))))
    (reference r8 (scope relative) (span (offset 528) (line 11) (column 85) (len 4)) (segments (segment 0 (token "that") (name "that") (separator none) (span (offset 528) (line 11) (column 85) (len 4)))))
    (reference r9 (scope relative) (span (offset 468) (line 11) (column 25) (len 23)) (segments (segment 0 (token "ownedPerformances") (name "ownedPerformances") (separator none) (span (offset 468) (line 11) (column 25) (len 17))) (segment 1 (token "this") (name "this") (separator colon-colon) (span (offset 487) (line 11) (column 44) (len 4)))))
    (reference r10 (scope relative) (span (offset 493) (line 11) (column 50) (len 21)) (segments (segment 0 (token "subperformances") (name "subperformances") (separator none) (span (offset 493) (line 11) (column 50) (len 15))) (segment 1 (token "this") (name "this") (separator colon-colon) (span (offset 510) (line 11) (column 67) (len 4)))))
    (reference r11 (scope relative) (span (offset 601) (line 16) (column 17) (len 25)) (segments (segment 0 (token "outgoingTransfersFromSelf") (name "outgoingTransfersFromSelf") (separator none) (span (offset 601) (line 16) (column 17) (len 25)))))
    (reference r12 (scope relative) (span (offset 630) (line 16) (column 46) (len 16)) (segments (segment 0 (token "interfacingPorts") (name "interfacingPorts") (separator none) (span (offset 630) (line 16) (column 46) (len 16)))))
    (reference r13 (scope relative) (span (offset 793) (line 22) (column 34) (len 9)) (segments (segment 0 (token "Rendering") (name "Rendering") (separator none) (span (offset 793) (line 22) (column 34) (len 9)))))
    (reference r14 (scope relative) (span (offset 786) (line 22) (column 27) (len 4)) (segments (segment 0 (token "self") (name "self") (separator none) (span (offset 786) (line 22) (column 27) (len 4)))))
    (reference r15 (scope relative) (span (offset 851) (line 23) (column 48) (len 9)) (segments (segment 0 (token "Rendering") (name "Rendering") (separator none) (span (offset 851) (line 23) (column 48) (len 9)))))
    (reference r16 (scope relative) (span (offset 870) (line 23) (column 67) (len 10)) (segments (segment 0 (token "renderings") (name "renderings") (separator none) (span (offset 870) (line 23) (column 67) (len 10)))))
  )
  (root (package (name "ViewDefBodyMembers") (body brace (view-def (name "View") (short-name none) (modifiers (abstract (span (offset 33) (line 2) (column 5) (len 8)))) (specializes none) (body brace (ref (name "subviews") (short-name none) (prefix (direction none) (derived false) (usage-prefix abstract) (constant false)) (extensions) (kind view) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity (lower (expression (span (offset 100) (line 3) (column 43) (len 1)) (integer 0))) (upper (expression (span (offset 103) (line 3) (column 46) (len 1)) (infinity)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets (relationship (kind subsets) (implied false) (targets (ref r1)))) (body brace (doc (name none) (short-name none) (locale none) (body (span (offset 135) (line 4) (column 19) (len 28)) (normalized "The subviews of this view. "))))) (ref (name none) (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind viewpoint) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines (relationship (kind redefines) (implied false) (targets (ref r3)))) (subsets none) (body semicolon)) (viewpoint-usage (name "viewpointSatisfactions") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (body semicolon)) (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (declaration (name "viewpointConformance") (short-name none))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r5)) (body brace (require-constraint (kind require) (constraint-keyword false) (name none) (target (ref r6)) (specializations) (value none) (body brace (doc (name none) (short-name none) (locale none) (body (span (offset 410) (line 10) (column 23) (len 31)) (normalized "The required ViewpointChecks. "))) (ref (name none) (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 523) (line 11) (column 80) (len 9)) (member-access (base (expression (span (offset 523) (line 11) (column 80) (len 4)) (ref r7))) (separator dot) (member (ref r8))))))) (redefines (relationship (kind redefines) (implied false) (targets (ref r9) (ref r10)))) (subsets none) (body semicolon)))))))) (part-def (name "Ports") (modifiers) (body brace (ref (name none) (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines (relationship (kind redefines) (implied false) (targets (ref r11)))) (subsets (relationship (kind subsets) (implied false) (targets (ref r12)))) (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer (reference (span (offset 665) (line 17) (column 17) (len 3)))) (extensions) (short-name none) (identity (declaration (name "source") (span (offset 669) (line 17) (column 21) (len 6)))) (typing none) (references none) (multiplicity none) (redefines none) (crosses none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer (reference (span (offset 693) (line 18) (column 17) (len 3)))) (extensions) (short-name none) (identity (declaration (name "target") (span (offset 697) (line 18) (column 21) (len 6)))) (typing none) (references none) (multiplicity none) (redefines none) (crosses none)))))) (rendering-def (name "Rendering") (short-name none) (modifiers (abstract (span (offset 725) (line 21) (column 5) (len 8)))) (specializes none) (body brace (ref (name none) (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind rendering) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines (relationship (kind redefines) (implied false) (targets (ref r14)))) (subsets none) (body semicolon)) (ref (name "subrenderings") (short-name none) (prefix (direction none) (derived false) (usage-prefix abstract) (constant false)) (extensions) (kind rendering) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (multiplicity (lower (expression (span (offset 861) (line 23) (column 58) (len 1)) (integer 0))) (upper (expression (span (offset 864) (line 23) (column 61) (len 1)) (infinity)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets (relationship (kind subsets) (implied false) (targets (ref r16)))) (body semicolon)))))))
)
~~~
