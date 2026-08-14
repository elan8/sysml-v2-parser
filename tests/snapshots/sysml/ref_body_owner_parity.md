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
  )
  (root (package (name "RefBodyOwnerParity") (body (connection-def (name "C") (role ordinary) (specializes none) (body (ref))) (part-def (name "P") (body (ref))) (action-def (name "A") (specializes none) (body (ref))) (state-def (name "S") (body (ref))))))
)
~~~
