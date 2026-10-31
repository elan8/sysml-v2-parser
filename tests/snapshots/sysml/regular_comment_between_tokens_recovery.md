# META
~~~sexpr
(snapshot (type recovery) (description "Only tokens are significant (KerML 8.2.1), so notes may sit anywhere, but a REGULAR_COMMENT is a token the grammar places only as the body of a Comment, Documentation or TextualRepresentation. Between the tokens of a declaration it is an error. At a member position it stays a Comment member, and later siblings still parse."))
~~~
# SOURCE
~~~sysml
package Outer {
    private /* comment */ part def Rejected;
    public/* comment */import Lib::*;
    part /* comment */ usage;
    private /* comment */ package Nested;
    attribute a : /* comment */ Real;
    /* a member, not an error */
    private //* note */ part def Kept;
    public // note
    part usageKept;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "regular_comment_between_tokens_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_package_body_element") (severity error) (category parseerror) (span (offset 20) (line 2) (column 5) (len 40)) (message "unexpected token in package body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 20) (line 2) (column 5) (len 40)) (message "suppressed 4 cascading recovered diagnostics after earlier recovery errors"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package Outer {
    private /* comment */ part def Rejected;
    public/* comment */import Lib::*;
    part /* comment */ usage;
    private /* comment */ package Nested;
    attribute a : /* comment */ Real;
    /* a member, not an error */
    private part def Kept;
    public part usageKept;
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
  )
  (root (package (visibility none) (name "Outer") (body brace (malformed (code "recovered_package_body_element") (found "private /* comment */ part def Rejected;") (span (offset 20) (line 2) (column 5) (len 40))) (malformed (code "recovered_package_body_element") (found "public/* comment */import Lib::*;") (span (offset 65) (line 3) (column 5) (len 33))) (malformed (code "recovered_package_body_element") (found "part /* comment */ usage;") (span (offset 103) (line 4) (column 5) (len 25))) (malformed (code "recovered_package_body_element") (found "private /* comment */ package Nested;") (span (offset 133) (line 5) (column 5) (len 37))) (malformed (code "recovered_package_body_element") (found "attribute a : /* comment */ Real;") (span (offset 175) (line 6) (column 5) (len 33))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 215) (line 7) (column 7) (len 24)) (normalized "a member, not an error "))) (part-def (name "Kept") (modifiers) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "usageKept") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))
)
~~~
