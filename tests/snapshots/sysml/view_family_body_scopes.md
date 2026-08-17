# META
~~~sexpr
(snapshot (type semantic) (description "`ViewUsage`, `ViewpointUsage` and `RenderingUsage` are usage-element alternatives and the three matching definitions are DefinitionElement alternatives, so `UsageBody = DefinitionBody` admits all six in a part usage body exactly as in a part definition body. The part usage scope modelled none of them, so `rendering r { ... }` there reached recovery; the part definition scope parsed all six and its emitter refused every one, so a document containing a nested view definition parsed and could not be formatted."))
~~~
# SOURCE
~~~sysml
package ViewFamilyBodyScopes {
    part def Host {
        view def Overview;
        view summary {
            doc /* a view usage in a part definition body */
        }
        viewpoint def Concern;
        viewpoint concernCheck;
        rendering def AsTree;
        rendering asTree;
    }
    part host {
        view def NestedOverview;
        view nestedSummary {
            doc /* a view usage in a part usage body */
        }
        viewpoint def NestedConcern;
        viewpoint nestedStakeholder;
        rendering def NestedAsTree;
        rendering nestedAsTree {
            doc /* the member the audit recorded as reaching recovery */
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "view_family_body_scopes.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ViewFamilyBodyScopes {
    part def Host {
        view def Overview;
        view summary {
            doc
            /* a view usage in a part definition body */
        }
        viewpoint def Concern;
        viewpoint concernCheck;
        rendering def AsTree;
        rendering asTree;
    }
    part host {
        view def NestedOverview;
        view nestedSummary {
            doc
            /* a view usage in a part usage body */
        }
        viewpoint def NestedConcern;
        viewpoint nestedStakeholder;
        rendering def NestedAsTree;
        rendering nestedAsTree {
            doc
            /* the member the audit recorded as reaching recovery */
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "ViewFamilyBodyScopes") (body brace (part-def (name "Host") (body brace (view-def) (view (name "summary") (short-name none) (type none) (body brace (doc))) (viewpoint-def) (viewpoint-usage) (rendering-def) (rendering-usage))) (part-usage (declaration-name "host") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body brace (view-def) (view (name "nestedSummary") (short-name none) (type none) (body brace (doc))) (viewpoint-def) (viewpoint-usage) (rendering-def) (rendering-usage))))))
)
~~~
