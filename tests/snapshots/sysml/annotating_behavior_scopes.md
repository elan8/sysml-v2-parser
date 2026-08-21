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
    (reference r4 (scope relative) (span (offset 683) (line 24) (column 14) (len 1)) (segments (segment 0 (token "f") (name "f") (separator none) (span (offset 683) (line 24) (column 14) (len 1)))))
    (reference r5 (scope relative) (span (offset 825) (line 28) (column 14) (len 8)) (segments (segment 0 (token "Approved") (name "Approved") (separator none) (span (offset 825) (line 28) (column 14) (len 8)))))
  )
  (root (package (name "AnnotatingBehaviorScopes") (body brace (action-def (name "Launch") (modifiers) (specializes none) (body brace (doc (name none) (locale none) (body (span (offset 73) (line 3) (column 15) (len 19)) (normalized "action definition "))) (comment (keyword (span (offset 103) (line 4) (column 9) (len 7))) (name none) (about) (locale none) (body (span (offset 113) (line 4) (column 19) (len 14)) (normalized "action aside "))) (textual-rep (name "actionRep") (language "text") (body (span (offset 170) (line 5) (column 41) (len 18)) (normalized "action rendering "))) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r0)) (about) (body semicolon)) (action-usage (name "step") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))) (action-usage (name "countdown") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (doc (name none) (locale none) (body (span (offset 274) (line 10) (column 15) (len 14)) (normalized "action usage "))) (comment (keyword (span (offset 299) (line 11) (column 9) (len 7))) (name none) (about) (locale none) (body (span (offset 309) (line 11) (column 19) (len 20)) (normalized "action usage aside "))) (textual-rep (name "actionUsageRep") (language "text") (body (span (offset 377) (line 12) (column 46) (len 24)) (normalized "action usage rendering "))) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r1)) (about) (body semicolon)))) (state-def (name "Modes") (modifiers) (body brace (doc (name none) (locale none) (body (span (offset 465) (line 16) (column 15) (len 18)) (normalized "state definition "))) (comment (keyword (span (offset 494) (line 17) (column 9) (len 7))) (name none) (about) (locale none) (body (span (offset 504) (line 17) (column 19) (len 13)) (normalized "state aside "))) (textual-rep (name "stateRep") (language "text") (body (span (offset 559) (line 18) (column 40) (len 17)) (normalized "state rendering "))) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r2)) (about) (body semicolon)) (entry (action-keyword false) (target none) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)) (then (state (ref r3))) (state-usage (name "idle") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))) (action-usage (name "fanout") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (fork (declaration (named (expression (span (offset 683) (line 24) (column 14) (len 1)) (ref r4)))) (body brace (open-brace (span (offset 685) (line 24) (column 16) (len 1))) (members (doc (name none) (locale none) (body (span (offset 705) (line 25) (column 19) (len 6)) (normalized "fork "))) (comment (keyword (span (offset 726) (line 26) (column 13) (len 7))) (name none) (about) (locale none) (body (span (offset 736) (line 26) (column 23) (len 12)) (normalized "fork aside "))) (textual-rep (name "forkRep") (language "text") (body (span (offset 793) (line 27) (column 43) (len 16)) (normalized "fork rendering "))) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r5)) (about) (body semicolon))) (close-brace (span (offset 843) (line 29) (column 9) (len 1))))))))))
)
~~~
