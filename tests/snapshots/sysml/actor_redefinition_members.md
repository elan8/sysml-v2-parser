# META
~~~sexpr
(snapshot (type semantic) (description "SysML ActorUsage is actor followed by Usage: identification is optional, anonymous and named redefinitions preserve every authored target, values are FeatureValue parts, and a bare actor remains an anonymous Usage. Based on SysML 8.2.2.23 and pinned UseCaseTest.sysml."))
~~~
# SOURCE
~~~sysml
package ActorRedefinitions {
    part user;
    part spare;
    use case def UC {
        actor;
        actor :>> user;
        actor named : Operator :>> user;
        private actor <A> redefines user, spare = fallback;
        actor :>> user := replacement;
        actor retained : Operator;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "actor_redefinition_members.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sexpr
(stable-idempotent)
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 115) (line 6) (column 19) (len 4)) (segments (segment 0 (token "user") (name "user") (separator none) (span (offset 115) (line 6) (column 19) (len 4)))))
    (reference r1 (scope relative) (span (offset 143) (line 7) (column 23) (len 8)) (segments (segment 0 (token "Operator") (name "Operator") (separator none) (span (offset 143) (line 7) (column 23) (len 8)))))
    (reference r2 (scope relative) (span (offset 156) (line 7) (column 36) (len 4)) (segments (segment 0 (token "user") (name "user") (separator none) (span (offset 156) (line 7) (column 36) (len 4)))))
    (reference r3 (scope relative) (span (offset 198) (line 8) (column 37) (len 4)) (segments (segment 0 (token "user") (name "user") (separator none) (span (offset 198) (line 8) (column 37) (len 4)))))
    (reference r4 (scope relative) (span (offset 204) (line 8) (column 43) (len 5)) (segments (segment 0 (token "spare") (name "spare") (separator none) (span (offset 204) (line 8) (column 43) (len 5)))))
    (reference r5 (scope relative) (span (offset 212) (line 8) (column 51) (len 8)) (segments (segment 0 (token "fallback") (name "fallback") (separator none) (span (offset 212) (line 8) (column 51) (len 8)))))
    (reference r6 (scope relative) (span (offset 240) (line 9) (column 19) (len 4)) (segments (segment 0 (token "user") (name "user") (separator none) (span (offset 240) (line 9) (column 19) (len 4)))))
    (reference r7 (scope relative) (span (offset 248) (line 9) (column 27) (len 11)) (segments (segment 0 (token "replacement") (name "replacement") (separator none) (span (offset 248) (line 9) (column 27) (len 11)))))
    (reference r8 (scope relative) (span (offset 286) (line 10) (column 26) (len 8)) (segments (segment 0 (token "Operator") (name "Operator") (separator none) (span (offset 286) (line 10) (column 26) (len 8)))))
  )
  (root (package (name "ActorRedefinitions") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "user") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "spare") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (use-case-def (name "UC") (modifiers) (body brace (actor (name none) (short-name none) (type none) (redefines none) (multiplicity none)) (actor (name none) (short-name none) (type none) (redefines (relationship (kind redefines) (implied false) (targets (ref r0)))) (multiplicity none)) (actor (name "named") (short-name none) (type (ref r1)) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (multiplicity none)) (actor (name none) (short-name "A") (type none) (redefines (relationship (kind redefines) (implied false) (targets (ref r3) (ref r4)))) (multiplicity none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 212) (line 8) (column 51) (len 8)) (ref r5)))))) (actor (name none) (short-name none) (type none) (redefines (relationship (kind redefines) (implied false) (targets (ref r6)))) (multiplicity none) (value (feature-value (kind assign) (default false) (expression (expression (span (offset 248) (line 9) (column 27) (len 11)) (ref r7)))))) (actor (name "retained") (short-name none) (type (ref r8)) (redefines none) (multiplicity none)))))))
)
~~~
