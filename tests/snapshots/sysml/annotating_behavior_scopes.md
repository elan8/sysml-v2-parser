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
    (reference r0 (scope relative) (span (offset 200) (line 6) (column 10) (len 8)) (segments (segment 0 (token "Approved") (name "Approved") (separator none) (span (offset 200) (line 6) (column 10) (len 8)))))
    (reference r1 (scope relative) (span (offset 413) (line 13) (column 10) (len 8)) (segments (segment 0 (token "Approved") (name "Approved") (separator none) (span (offset 413) (line 13) (column 10) (len 8)))))
    (reference r2 (scope relative) (span (offset 588) (line 19) (column 10) (len 8)) (segments (segment 0 (token "Approved") (name "Approved") (separator none) (span (offset 588) (line 19) (column 10) (len 8)))))
    (reference r3 (scope relative) (span (offset 618) (line 20) (column 21) (len 4)) (segments (segment 0 (token "idle") (name "idle") (separator none) (span (offset 618) (line 20) (column 21) (len 4)))))
  )
  (root (package (name "AnnotatingBehaviorScopes") (body brace (action-def (name "Launch") (specializes none) (body brace (doc) (comment (keyword (span (offset 103) (line 4) (column 9) (len 7))) (name none) (about) (locale none)) (textual-rep) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r0)) (about) (body semicolon)) (action-usage (declaration "step") (type none)))) (action-usage (name "countdown") (short-name none) (body brace (doc) (comment (keyword (span (offset 299) (line 11) (column 9) (len 7))) (name none) (about) (locale none)) (textual-rep) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r1)) (about) (body semicolon)))) (state-def (name "Modes") (body brace (doc) (comment (keyword (span (offset 494) (line 17) (column 9) (len 7))) (name none) (about) (locale none)) (textual-rep) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r2)) (about) (body semicolon)) (entry (action-keyword false) (target none) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)) (then (state (ref r3))) (state-usage))) (action-usage (name "fanout") (short-name none) (body brace (fork))))))
)
~~~
