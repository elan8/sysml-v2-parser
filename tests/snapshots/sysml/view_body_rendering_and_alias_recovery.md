# META
~~~sexpr
(snapshot (type recovery) (description "Malformed rendering and alias members inside a view body become recovery nodes without consuming the later valid rendering and alias siblings that begin with the newly declared view-body starters."))
~~~
# SOURCE
~~~sysml
package ViewBodyRenderingAndAliasRecovery {
    rendering def RenderingType;
    part def AliasTarget;

    view Recovered {
        rendering = ;
        rendering retainedRendering : RenderingType;
        alias for AliasTarget;
        alias retainedAlias for AliasTarget;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "view_body_rendering_and_alias_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_view_body_element") (severity error) (category parseerror) (span (offset 133) (line 6) (column 9) (len 22)) (message "unexpected token in view body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 133) (line 6) (column 9) (len 22)) (message "suppressed 1 cascading recovered diagnostic after earlier recovery errors"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package ViewBodyRenderingAndAliasRecovery {
    rendering def RenderingType;
    part def AliasTarget;
    view Recovered {
        rendering = ;
        rendering retainedRendering : RenderingType;
        alias for AliasTarget;
        alias retainedAlias for AliasTarget;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 185) (line 7) (column 39) (len 13)) (segments (segment 0 (token "RenderingType") (name "RenderingType") (separator none) (span (offset 185) (line 7) (column 39) (len 13)))))
    (reference r1 (scope relative) (span (offset 263) (line 9) (column 33) (len 11)) (segments (segment 0 (token "AliasTarget") (name "AliasTarget") (separator none) (span (offset 263) (line 9) (column 33) (len 11)))))
  )
  (root (package (name "ViewBodyRenderingAndAliasRecovery") (body brace (rendering-def (modifiers)) (part-def (name "AliasTarget") (modifiers) (body semicolon)) (view (name "Recovered") (short-name none) (type none) (body brace (malformed (code "recovered_view_body_element") (found "rendering = ;") (span (offset 133) (line 6) (column 9) (len 22))) (rendering-usage (abstract false) (name "retainedRendering") (type (ref r0)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (malformed (code "recovered_view_body_element") (found "alias for AliasTarget;") (span (offset 208) (line 8) (column 9) (len 31))) (alias (name "retainedAlias") (target (ref r1)) (body semicolon)))))))
)
~~~
