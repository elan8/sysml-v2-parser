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
  (root (package (name "Diamond") (body brace (part-def (name "Base") (body brace (part-def (name "Member") (body semicolon)))) (part-def (name "Left") (body semicolon)) (part-def (name "Right") (body semicolon)) (part-def (name "Diamond") (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "p") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))))))
)
~~~
