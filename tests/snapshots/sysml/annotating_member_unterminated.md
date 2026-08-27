# META
~~~sexpr
(snapshot (type malformed) (description "An annotating member whose REGULAR_COMMENT body is never closed is a lexical failure, not a member-level one: everything after the opening `/*` is comment text, so there is no later sibling left to preserve and the document recovers as a whole with the exact span it covers. Recorded so the difference between this and a member-level malformed annotating node stays visible."))
~~~
# SOURCE
~~~sysml
package AnnotatingMemberUnterminated {
    part def Unterminated {
        comment /* never closed
    }
    part def Later {
        attribute mass;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "annotating_member_unterminated.md"
    (diagnostics
      (diagnostic (code "missing_closing_brace") (severity none) (category parseerror) (span (offset 157) (line 8) (column 2) (len 1)) (message "missing closing '}'"))
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
  (root (malformed (code "missing_closing_brace") (found none) (span (offset 0) (line 1) (column 1) (len 157))))
)
~~~
