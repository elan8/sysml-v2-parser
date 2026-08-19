# META
~~~sexpr
(snapshot (type semantic) (description "`AllocationUsage`, `ConnectionUsage`, `BindingConnectorAsUsage` and `SuccessionAsUsage` all end in `UsageBody`, and `UsageBody = DefinitionBody`, so each owns the whole usage member set. All four held a `ConnectBody` marker whose brace form was parsed by `advance_to_closing_brace` and kept nothing -- and a braced `connect` body did not merely lose its members, it aborted emission with an opaque-body error, so the document could not be formatted at all. `BindingConnectorAsUsage` is converted with them but has no case here: `binding a = b;` is dispatched in neither a package body nor a definition body, which is a member-dispatch gap recorded separately."))
~~~
# SOURCE
~~~sysml
package UsageBodyConnectorOwners {
    part def Rig {
        port left;
        port right;
        connect left to right {
            doc /* a connect usage body used to abort emission */
            attribute gauge;
        }
        allocate left to right {
            doc /* an allocate body used to be discarded */
        }
    }
    occurrence def Timeline {
        occurrence early;
        occurrence late;
        succession first early then late {
            doc /* a succession body used to be discarded */
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "usage_body_connector_owners.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package UsageBodyConnectorOwners {
    part def Rig {
        port left;
        port right;
        connect left to right {
            doc
            /* a connect usage body used to abort emission */
            attribute gauge;
        }
        allocate left to right {
            doc
            /* an allocate body used to be discarded */
        }
    }
    occurrence def Timeline {
        occurrence early;
        occurrence late;
        succession first early then late {
            doc
            /* a succession body used to be discarded */
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 109) (line 5) (column 17) (len 4)) (segments (segment 0 (token "left") (name "left") (separator none) (span (offset 109) (line 5) (column 17) (len 4)))))
    (reference r1 (scope relative) (span (offset 117) (line 5) (column 25) (len 5)) (segments (segment 0 (token "right") (name "right") (separator none) (span (offset 117) (line 5) (column 25) (len 5)))))
  )
  (root (package (name "UsageBodyConnectorOwners") (body brace (part-def (name "Rig") (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "left") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "right") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (connect (from (expression (span (offset 109) (line 5) (column 17) (len 4)) (ref r0))) (to (expression (span (offset 117) (line 5) (column 25) (len 5)) (ref r1))) (body brace (doc (name none) (locale none) (body (span (offset 143) (line 6) (column 19) (len 45)) (normalized "a connect usage body used to abort emission "))) (attribute-usage (declaration-name "gauge") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon))) (subsets none) (redefines none)) (allocate))) (occurrence-def))))
)
~~~
