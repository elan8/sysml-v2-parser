# META
~~~sexpr
(snapshot (type semantic) (description "Both view definition and view usage bodies inherit DefinitionBodyItem, so they admit ordinary rendering usages and alias members in addition to their dedicated render members. This fixture proves the two typed alternatives retain names, targets, and rendering headers in both legal scopes."))
~~~
# SOURCE
~~~sysml
package ViewBodyRenderingAndAlias {
    rendering def RenderingType;
    part def AliasTarget;

    view def Defined {
        rendering definitionRendering : RenderingType;
        alias definitionAlias for AliasTarget;
    }

    view Used {
        rendering usedRendering : RenderingType;
        alias usedAlias for AliasTarget;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "view_body_rendering_and_alias.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ViewBodyRenderingAndAlias {
    rendering def RenderingType;
    part def AliasTarget;
    view def Defined {
        rendering definitionRendering : RenderingType;
        alias definitionAlias for AliasTarget;
    }
    view Used {
        rendering usedRendering : RenderingType;
        alias usedAlias for AliasTarget;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 159) (line 6) (column 41) (len 13)) (segments (segment 0 (token "RenderingType") (name "RenderingType") (separator none) (span (offset 159) (line 6) (column 41) (len 13)))))
    (reference r1 (scope relative) (span (offset 208) (line 7) (column 35) (len 11)) (segments (segment 0 (token "AliasTarget") (name "AliasTarget") (separator none) (span (offset 208) (line 7) (column 35) (len 11)))))
    (reference r2 (scope relative) (span (offset 278) (line 11) (column 35) (len 13)) (segments (segment 0 (token "RenderingType") (name "RenderingType") (separator none) (span (offset 278) (line 11) (column 35) (len 13)))))
    (reference r3 (scope relative) (span (offset 321) (line 12) (column 29) (len 11)) (segments (segment 0 (token "AliasTarget") (name "AliasTarget") (separator none) (span (offset 321) (line 12) (column 29) (len 11)))))
  )
  (root (package (name "ViewBodyRenderingAndAlias") (body brace (rendering-def (modifiers)) (part-def (name "AliasTarget") (modifiers) (body semicolon)) (view-def (name "Defined") (short-name none) (modifiers) (specializes none) (body brace (rendering-usage (abstract false) (name "definitionRendering") (type (ref r0)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (alias (name "definitionAlias") (target (ref r1)) (body semicolon)))) (view (name "Used") (short-name none) (type none) (body brace (rendering-usage (abstract false) (name "usedRendering") (type (ref r2)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (alias (name "usedAlias") (target (ref r3)) (body semicolon)))))))
)
~~~
