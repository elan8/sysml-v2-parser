# META
~~~sexpr
(snapshot (type malformed) (description "A malformed member inside a subject declaration's DefinitionBody recovers at the next typed sibling. The body remains attached to the subject rather than being skipped wholesale."))
~~~
# SOURCE
~~~sysml
package SubjectDeclarationBodyRecovery {
    requirement def R {
        subject vehicle : Vehicle {
            bogus ???;
            part after : Vehicle;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "subject_decl_body_recovery.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 113) (line 4) (column 13) (len 23)) (message "unrecognized declaration `bogus` in definition body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package SubjectDeclarationBodyRecovery {
    requirement def R {
        subject vehicle : Vehicle {
            bogus ???;
            part 'after' : Vehicle;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 91) (line 3) (column 27) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 91) (line 3) (column 27) (len 7)))))
    (reference r1 (scope relative) (span (offset 149) (line 5) (column 26) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 149) (line 5) (column 26) (len 7)))))
  )
  (root (package (name "SubjectDeclarationBodyRecovery") (body brace (requirement-def (name "R") (modifiers) (body brace (subject (name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "bogus ???;") (span (offset 113) (line 4) (column 13) (len 23))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "after") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))))))))
)
~~~
