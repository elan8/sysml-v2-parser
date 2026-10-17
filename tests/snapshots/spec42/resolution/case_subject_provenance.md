# META
~~~sexpr
(snapshot (type semantic) (description "Case subject derived relationship retains explicit provenance"))
~~~
# SOURCE
~~~sysml
package M {
    part def P;
    analysis def A {
        subject s : P;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "case_subject_provenance.md"
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
    (reference r0 (scope relative) (span (offset 69) (line 4) (column 21) (len 1)) (segments (segment 0 (token "P") (name "P") (separator none) (span (offset 69) (line 4) (column 21) (len 1)))))
  )
  (root (package (name "M") (body brace (part-def (name "P") (modifiers) (body semicolon)) (analysis-case-def (name "A") (modifiers) (body brace (subject (name "s") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
