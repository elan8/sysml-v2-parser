# META
~~~sexpr
(snapshot (type semantic) (description "The same ref body members parse identically under every owner: UsageBody is DefinitionBody, so a ref body does not depend on whether a connection, part, action, or state declaration owns it."))
~~~
# SOURCE
~~~sysml
package RefBodyOwnerParity {
    connection def C {
        ref underConnection : Anything {
            doc /* shared */
            comment /* shared */
            rep shared language "text" /* shared */
            @Meta about x;
            attribute mass : Real;
            ref nested;
        }
    }
    part def P {
        ref underPart : Anything {
            doc /* shared */
            comment /* shared */
            rep shared language "text" /* shared */
            @Meta about x;
            attribute mass : Real;
            ref nested;
        }
    }
    action def A {
        ref underAction : Anything {
            doc /* shared */
            comment /* shared */
            rep shared language "text" /* shared */
            @Meta about x;
            attribute mass : Real;
            ref nested;
        }
    }
    state def S {
        ref underState : Anything {
            doc /* shared */
            comment /* shared */
            rep shared language "text" /* shared */
            @Meta about x;
            attribute mass : Real;
            ref nested;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "ref_body_owner_parity.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package RefBodyOwnerParity {
    connection def C {
        ref underConnection : Anything {
            doc
            /* shared */
            comment
            /* shared */
            rep shared language "text"
            /* shared */
            @Meta about x;
            attribute mass : Real;
            ref nested;
        }
    }
    part def P {
        ref underPart : Anything {
            doc
            /* shared */
            comment
            /* shared */
            rep shared language "text"
            /* shared */
            @Meta about x;
            attribute mass : Real;
            ref nested;
        }
    }
    action def A {
        ref underAction : Anything {
            doc
            /* shared */
            comment
            /* shared */
            rep shared language "text"
            /* shared */
            @Meta about x;
            attribute mass : Real;
            ref nested;
        }
    }
    state def S {
        ref underState : Anything {
            doc
            /* shared */
            comment
            /* shared */
            rep shared language "text"
            /* shared */
            @Meta about x;
            attribute mass : Real;
            ref nested;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 82) (line 3) (column 31) (len 8)) (segments (segment 0 (token "Anything") (name "Anything") (separator none) (span (offset 82) (line 3) (column 31) (len 8)))))
    (reference r1 (scope relative) (span (offset 263) (line 8) (column 30) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 263) (line 8) (column 30) (len 4)))))
    (reference r2 (scope relative) (span (offset 350) (line 13) (column 25) (len 8)) (segments (segment 0 (token "Anything") (name "Anything") (separator none) (span (offset 350) (line 13) (column 25) (len 8)))))
    (reference r3 (scope relative) (span (offset 531) (line 18) (column 30) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 531) (line 18) (column 30) (len 4)))))
    (reference r4 (scope relative) (span (offset 622) (line 23) (column 27) (len 8)) (segments (segment 0 (token "Anything") (name "Anything") (separator none) (span (offset 622) (line 23) (column 27) (len 8)))))
    (reference r5 (scope relative) (span (offset 803) (line 28) (column 30) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 803) (line 28) (column 30) (len 4)))))
    (reference r6 (scope relative) (span (offset 892) (line 33) (column 26) (len 8)) (segments (segment 0 (token "Anything") (name "Anything") (separator none) (span (offset 892) (line 33) (column 26) (len 8)))))
    (reference r7 (scope relative) (span (offset 1073) (line 38) (column 30) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 1073) (line 38) (column 30) (len 4)))))
  )
  (root (package (name "RefBodyOwnerParity") (body brace (connection-def (name "C") (role ordinary) (specializes none) (body brace (ref (name "underConnection") (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (redefines none) (subsets none) (body brace (doc) (comment (keyword (span (offset 134) (line 5) (column 13) (len 7))) (name none) (locale none)) (textual-rep) (metadata-annotation) (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (ref (name "nested") (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing none) (redefines none) (subsets none) (body semicolon)))))) (part-def (name "P") (body brace (ref (name "underPart") (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (redefines none) (subsets none) (body brace (doc) (comment (keyword (span (offset 402) (line 15) (column 13) (len 7))) (name none) (locale none)) (textual-rep) (metadata-annotation) (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (ref (name "nested") (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing none) (redefines none) (subsets none) (body semicolon)))))) (action-def (name "A") (specializes none) (body brace (ref (name "underAction") (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (redefines none) (subsets none) (body brace (doc) (comment (keyword (span (offset 674) (line 25) (column 13) (len 7))) (name none) (locale none)) (textual-rep) (metadata-annotation) (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (ref (name "nested") (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing none) (redefines none) (subsets none) (body semicolon)))))) (state-def (name "S") (body brace (ref (name "underState") (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (redefines none) (subsets none) (body brace (doc) (comment (keyword (span (offset 944) (line 35) (column 13) (len 7))) (name none) (locale none)) (textual-rep) (metadata-annotation) (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (ref (name "nested") (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing none) (redefines none) (subsets none) (body semicolon)))))))))
)
~~~
