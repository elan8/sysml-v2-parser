# META
~~~sexpr
(snapshot (type semantic) (description "`Expose = 'expose' ( MembershipExpose | NamespaceExpose ) RelationshipBody` and the view-body `satisfy` member take the same body a dependency does. All three held a `ConnectBody` marker -- two variants, no delimiter spans -- whose brace form was skipped wholesale, so an annotating member written inside `expose ... { ... }` was discarded with no node and no diagnostic and the body re-emitted as `{}`. A dependency additionally carried the members in a second field beside the marker. All three now hold one `Body<RelationshipBodyElement>`."))
~~~
# SOURCE
~~~sysml
package RelationshipBodyOwners {
    part def Subject;
    dependency client to supplier {
        doc /* a dependency body already kept its members */
    }
    view overview {
        expose Subject {
            doc /* an expose body used to be discarded */
            comment /* and so was this */
        }
        satisfy Concern {
            rep satisfyRep language "text" /* a view-body satisfy body too */
        }
        expose Subject;
        satisfy Concern {}
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "relationship_body_owners.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package RelationshipBodyOwners {
    part def Subject;
    dependency from client to supplier {
        doc
        /* a dependency body already kept its members */
    }
    view overview {
        expose Subject {
            doc
            /* an expose body used to be discarded */
            comment
            /* and so was this */
        }
        satisfy Concern {
            rep satisfyRep language "text"
            /* a view-body satisfy body too */
        }
        expose Subject;
        satisfy Concern {
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 70) (line 3) (column 16) (len 6)) (segments (segment 0 (token "client") (name "client") (separator none) (span (offset 70) (line 3) (column 16) (len 6)))))
    (reference r1 (scope relative) (span (offset 80) (line 3) (column 26) (len 8)) (segments (segment 0 (token "supplier") (name "supplier") (separator none) (span (offset 80) (line 3) (column 26) (len 8)))))
    (reference r2 (scope relative) (span (offset 193) (line 7) (column 16) (len 7)) (segments (segment 0 (token "Subject") (name "Subject") (separator none) (span (offset 193) (line 7) (column 16) (len 7)))))
    (reference r3 (scope relative) (span (offset 329) (line 11) (column 17) (len 7)) (segments (segment 0 (token "Concern") (name "Concern") (separator none) (span (offset 329) (line 11) (column 17) (len 7)))))
    (reference r4 (scope relative) (span (offset 442) (line 14) (column 16) (len 7)) (segments (segment 0 (token "Subject") (name "Subject") (separator none) (span (offset 442) (line 14) (column 16) (len 7)))))
    (reference r5 (scope relative) (span (offset 467) (line 15) (column 17) (len 7)) (segments (segment 0 (token "Concern") (name "Concern") (separator none) (span (offset 467) (line 15) (column 17) (len 7)))))
  )
  (root (package (name "RelationshipBodyOwners") (body brace (part-def (name "Subject") (body semicolon)) (dependency (clients (ref r0)) (suppliers (ref r1)) (body brace (doc))) (view (name "overview") (short-name none) (type none) (body brace (expose (target (span (span (offset 193) (line 7) (column 16) (len 7))) (all none) (ref r2) (shape (membership (recursive-suffix none)))) (body brace (doc) (comment (keyword (span (offset 273) (line 9) (column 13) (len 7))) (name none) (about) (locale none)))) (satisfy (assert false) (negated false) (requirement (reference (ref r3))) (typing none) (multiplicity none) (ordered false) (nonunique false) (subsets none) (references none) (redefines none) (crosses none) (value none) (by none) (body brace (textual-rep))) (expose (target (span (span (offset 442) (line 14) (column 16) (len 7))) (all none) (ref r4) (shape (membership (recursive-suffix none)))) (body semicolon)) (satisfy (assert false) (negated false) (requirement (reference (ref r5))) (typing none) (multiplicity none) (ordered false) (nonunique false) (subsets none) (references none) (redefines none) (crosses none) (value none) (by none) (body brace)))))))
)
~~~
