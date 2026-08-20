# META
~~~sexpr
(snapshot (type semantic) (description "Inherited diamond specialization target is deduplicated"))
~~~
# SOURCE
~~~sysml
package Diamond {
    part def Base {
        part def Member;
    }
    part def Left :> Base;
    part def Right :> Base;
    part def Diamond :> Left, Right {
        part p : Member;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "inherited_diamond_dedup.md"
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
  (references
    (reference r0 (scope relative) (span (offset 179) (line 8) (column 18) (len 6)) (segments (segment 0 (token "Member") (name "Member") (separator none) (span (offset 179) (line 8) (column 18) (len 6)))))
  )
  (root (package (name "Diamond") (body brace (part-def (name "Base") (modifiers) (body brace (part-def (name "Member") (modifiers) (body semicolon)))) (part-def (name "Left") (modifiers) (body semicolon)) (part-def (name "Right") (modifiers) (body semicolon)) (part-def (name "Diamond") (modifiers) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "p") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))))))
)
~~~
