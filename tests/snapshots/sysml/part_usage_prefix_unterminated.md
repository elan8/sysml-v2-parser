# META
~~~sexpr
(snapshot (type malformed) (description "A prefixed part usage whose brace body is never closed. There is deliberately no 'body with a missing close' state (see planning/shared-grammar.md), so the enclosing declaration's parse fails and the scope above keeps the text as one recovery node reporting missing_closing_brace. Its own fixture because a document-wide recovery span subsumes every other member, which is exactly the property being pinned: the prefix does not change it, and nothing is silently reinterpreted as a terminated usage."))
~~~
# SOURCE
~~~sysml
package PartPrefixUnterminated {
    part def UnmatchedBrace {
        ref individual snapshot part unterminated : Engine {
    }
    part def AfterUnmatchedBrace {
        part recovered : Engine;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "part_usage_prefix_unterminated.md"
    (diagnostics
      (diagnostic (code "missing_closing_brace") (severity none) (category parseerror) (span (offset 205) (line 8) (column 2) (len 1)) (message "missing closing '}'"))
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
  )
  (root (malformed (code "missing_closing_brace") (found none) (span (offset 0) (line 1) (column 1) (len 205))))
)
~~~
