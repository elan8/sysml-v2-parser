# META
~~~sexpr
(snapshot (type semantic) (description "`;` and `{}` are different authored syntax and the shared `ast::Body<E>` container keeps them different: one declares that the element owns no members, the other that it owns an empty body. The projection writes `(body semicolon)` against `(body ...)` with its members, and members appear in authored order. Both spellings are byte-identical after formatting, so neither collapses into the other. Note the empty brace body projects as `(body )` -- the brace form lists its members and an empty one has none, so the trailing space is the whole of it. That is the intended output, not a stray space: what distinguishes it from `;` is the absence of the `semicolon` marker."))
~~~
# SOURCE
~~~sysml
package P {
    part def SemicolonBody;
    part def EmptyBraceBody {
    }
    part def OrderedMembers {
        attribute one : Real;
        attribute two : Real;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "body_container_forms.md"
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
    (reference r0 (scope relative) (span (offset 130) (line 6) (column 25) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 130) (line 6) (column 25) (len 4)))))
    (reference r1 (scope relative) (span (offset 160) (line 7) (column 25) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 160) (line 7) (column 25) (len 4)))))
  )
  (root (package (name "P") (body (part-def (name "SemicolonBody") (body semicolon)) (part-def (name "EmptyBraceBody") (body )) (part-def (name "OrderedMembers") (body (attribute-usage (declaration-name "one") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "two") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
