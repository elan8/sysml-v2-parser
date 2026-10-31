# META
~~~sexpr
(snapshot (type malformed) (description "A prefixed part usage whose brace body is never closed. The bodies still open at end of input keep the members read before it and record a missing close rather than a } span (body brace (unterminated)), while missing_closing_brace reports end of input. Its own fixture because the prefix must not change that: the prefixed usage keeps its prefix and members, and the formatter writes no closing brace that was never authored."))
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
~~~sysml
package PartPrefixUnterminated {
    part def UnmatchedBrace {
        ref individual snapshot part unterminated : Engine {}
        part def AfterUnmatchedBrace {
            part recovered : Engine;
        }
    }
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 115) (line 3) (column 53) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 115) (line 3) (column 53) (len 6)))))
    (reference r1 (scope relative) (span (offset 190) (line 6) (column 26) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 190) (line 6) (column 26) (len 6)))))
  )
  (root (package (visibility none) (name "PartPrefixUnterminated") (body brace (unterminated) (part-def (name "UnmatchedBrace") (modifiers) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual true) (portion snapshot) (extensions)) (declaration-name "unterminated") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace)) (part-def (name "AfterUnmatchedBrace") (modifiers) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "recovered") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))))
)
~~~
