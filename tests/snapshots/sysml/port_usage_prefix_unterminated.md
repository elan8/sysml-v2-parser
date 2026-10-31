# META
~~~sexpr
(snapshot (type malformed) (description "A prefixed port usage whose brace body is never closed. The bodies still open at end of input keep the members read before it and record a missing close rather than a } span (body brace (unterminated)), while missing_closing_brace reports end of input. Its own fixture because the prefix must not change that: the prefixed usage keeps its prefix and members, and the formatter writes no closing brace that was never authored."))
~~~
# SOURCE
~~~sysml
package PortPrefixUnterminated {
    port def UnmatchedBrace {
        ref individual snapshot port unterminated : PowerPort {
    }
    part def AfterUnmatchedBrace {
        port recovered : PowerPort;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "port_usage_prefix_unterminated.md"
    (diagnostics
      (diagnostic (code "missing_body_or_semicolon") (severity error) (category parseerror) (span (offset 137) (line 5) (column 5) (len 73)) (message "expected ';' or '{' after part definition header"))
      (diagnostic (code "missing_closing_brace") (severity none) (category parseerror) (span (offset 211) (line 8) (column 2) (len 1)) (message "missing closing '}'"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package PortPrefixUnterminated {
    port def UnmatchedBrace {
        ref individual snapshot port unterminated : PowerPort {}
        part def AfterUnmatchedBrace {
        port recovered : PowerPort;
    }
    }
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 115) (line 3) (column 53) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 115) (line 3) (column 53) (len 9)))))
  )
  (root (package (name "PortPrefixUnterminated") (body brace (unterminated) (port-def (name "UnmatchedBrace") (modifiers) (specializes none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual true) (portion snapshot) (extensions)) (declaration-name "unterminated") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace)) (malformed (code "missing_body_or_semicolon") (found "part def AfterUnmatchedBrace {") (span (offset 137) (line 5) (column 5) (len 73))))))))
)
~~~
