# META
~~~sexpr
(snapshot (type semantic) (description "ActionBodyItem and StateBodyItem both begin with NonBehaviorBodyItem, which reaches DefinitionMember -> DefinitionElement -> AnnotatingElement, so behavioural bodies own the same annotating production as definition bodies. This fixture holds the action definition, action usage, state definition and control-node scopes."))
~~~
# SOURCE
~~~sysml
package AnnotatingBehaviorScopes {
    action def Launch {
        doc /* action definition */
        comment /* action aside */
        rep actionRep language "text" /* action rendering */
        @Approved;
        action step;
    }
    action countdown {
        doc /* action usage */
        comment /* action usage aside */
        rep actionUsageRep language "text" /* action usage rendering */
        @Approved;
    }
    state def Modes {
        doc /* state definition */
        comment /* state aside */
        rep stateRep language "text" /* state rendering */
        @Approved;
        entry; then idle;
        state idle;
    }
    action fanout {
        fork f {
            doc /* fork */
            comment /* fork aside */
            rep forkRep language "text" /* fork rendering */
            @Approved;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "annotating_behavior_scopes.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package AnnotatingBehaviorScopes {
    action def Launch {
        doc
        /* action definition */
        comment
        /* action aside */
        rep actionRep language "text"
        /* action rendering */
        @Approved;
        action step;
    }
    action countdown {
        doc
        /* action usage */
        comment
        /* action usage aside */
        rep actionUsageRep language "text"
        /* action usage rendering */
        @Approved;
    }
    state def Modes {
        doc
        /* state definition */
        comment
        /* state aside */
        rep stateRep language "text"
        /* state rendering */
        @Approved;
        entry;
        then idle;
        state idle;
    }
    action fanout {
        fork f {
            doc
            /* fork */
            comment
            /* fork aside */
            rep forkRep language "text"
            /* fork rendering */
            @Approved;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 618) (line 20) (column 21) (len 4)) (segments (segment 0 (token "idle") (name "idle") (separator none) (span (offset 618) (line 20) (column 21) (len 4)))))
  )
  (root (package (name "AnnotatingBehaviorScopes") (body brace (action-def (name "Launch") (specializes none) (body brace (doc) (comment (keyword (span (offset 103) (line 4) (column 9) (len 7))) (name none) (locale none)) (textual-rep) (metadata-annotation) (action-usage (declaration "step") (type none)))) (action-usage) (state-def (name "Modes") (body brace (doc) (comment (keyword (span (offset 494) (line 17) (column 9) (len 7))) (name none) (locale none)) (textual-rep) (metadata-annotation) (entry (action-keyword false) (target none) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)) (then (state (ref r0))) (state-usage))) (action-usage))))
)
~~~
