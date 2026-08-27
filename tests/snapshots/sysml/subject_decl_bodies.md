# META
~~~sexpr
(snapshot (type semantic) (description "SubjectUsage completes through the shared UsageBody = DefinitionBody production. Requirement and use-case subject declarations retain annotating members and later typed siblings in authored order."))
~~~
# SOURCE
~~~sysml
package SubjectDeclarationBodies {
    requirement def RequirementSubject {
        subject vehicle : Vehicle {
            /* requirement subject comment */
            part context : Vehicle;
        }
    }

    use case def UseCaseSubject {
        subject scenario : Scenario {
            doc /* use-case subject documentation */
            part context : Scenario;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "subject_decl_bodies.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package SubjectDeclarationBodies {
    requirement def RequirementSubject {
        subject vehicle : Vehicle {
            /* requirement subject comment */
            part context : Vehicle;
        }
    }
    use case def UseCaseSubject {
        subject scenario : Scenario {
            doc
            /* use-case subject documentation */
            part context : Scenario;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 102) (line 3) (column 27) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 102) (line 3) (column 27) (len 7)))))
    (reference r1 (scope relative) (span (offset 185) (line 5) (column 28) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 185) (line 5) (column 28) (len 7)))))
    (reference r2 (scope relative) (span (offset 272) (line 10) (column 28) (len 8)) (segments (segment 0 (token "Scenario") (name "Scenario") (separator none) (span (offset 272) (line 10) (column 28) (len 8)))))
    (reference r3 (scope relative) (span (offset 363) (line 12) (column 28) (len 8)) (segments (segment 0 (token "Scenario") (name "Scenario") (separator none) (span (offset 363) (line 12) (column 28) (len 8)))))
  )
  (root (package (name "SubjectDeclarationBodies") (body brace (requirement-def (name "RequirementSubject") (modifiers) (body brace (subject (name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 126) (line 4) (column 15) (len 29)) (normalized "requirement subject comment "))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "context") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))))) (use-case-def (name "UseCaseSubject") (modifiers) (body brace (subject (name "scenario") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (doc (name none) (locale none) (body (span (offset 301) (line 11) (column 19) (len 32)) (normalized "use-case subject documentation "))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "context") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))))))))
)
~~~
