# META
~~~sexpr
(snapshot (type semantic) (description "A malformed member after a keyword-less /* ... */ in a calc-shaped body recovers as its own node with an exact span: the comment ahead of it keeps a member of its own rather than being pulled into the recovered slice, and the valid sibling written after the malformed member is not consumed (spec42 Gap 60)."))
~~~
# SOURCE
~~~sysml
package BareCommentMemberRecovery {
    behavior Recovering {
        /* before the malformed member */
        feature f : ;
        feature g : Anything unions h;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "bare_comment_member_recovery.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 112) (line 4) (column 9) (len 22)) (message "unrecognized declaration `feature` in calc body"))
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
    (reference r0 (scope relative) (span (offset 162) (line 5) (column 37) (len 1)) (segments (segment 0 (token "h") (name "h") (separator none) (span (offset 162) (line 5) (column 37) (len 1)))))
  )
  (root (package (name "BareCommentMemberRecovery") (body brace (kerml-classifier (keyword behavior) (abstract false) (name "Recovering") (specializes none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 72) (line 3) (column 11) (len 29)) (normalized "before the malformed member "))) (malformed (code "unrecognized_declaration_in_scope") (found "feature f : ;") (span (offset 112) (line 4) (column 9) (len 22))) (kerml-feature (name "g") (relationships (type-relationship (keyword unions) (targets (ref r0)))) (value none) (body semicolon)))))))
)
~~~
