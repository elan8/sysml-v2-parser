# META
~~~sexpr
(snapshot (type malformed) (description "An annotating member whose REGULAR_COMMENT body is never closed is a lexical failure, not a member-level one: everything after the opening `/*` is comment text, so there is no later sibling left to preserve: the part definition keeps the comment as one recovery member running to end of input, and it and the package record a missing close. Recorded so the difference between this and a member-level malformed annotating node stays visible."))
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
      (diagnostic (code "recovered_part_def_body_element") (severity error) (category parseerror) (span (offset 75) (line 3) (column 9) (len 82)) (message "unexpected token in part definition body"))
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
  (name-language sysml)
  (references
  )
  (root (package (name "AnnotatingMemberUnterminated") (body brace (unterminated) (part-def (name "Unterminated") (modifiers) (body brace (unterminated) (malformed (code "recovered_part_def_body_element") (found "comment /* never closed") (span (offset 75) (line 3) (column 9) (len 82))))))))
)
~~~
