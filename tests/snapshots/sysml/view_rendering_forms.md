# META
~~~sexpr
(snapshot (type semantic) (description "A view render member preserves its reference-subsetting target separately from an inline rendering declaration, including a qualified target and an owned usage body."))
~~~
# SOURCE
~~~sysml
package Views {
    rendering def Diagram;
    rendering tree : Diagram;
    view def Structure {
        render Views::tree;
    }
    view instance : Structure {
        render rendering custom : Diagram[0..1] :> tree {
            doc /* Inline rendering body. */
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "view_rendering_forms.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Views {
    rendering def Diagram;
    rendering tree : Diagram;
    view def Structure {
        render Views::tree;
    }
    view instance : Structure {
        render rendering custom : Diagram[0..1] :> tree {
            doc
            /* Inline rendering body. */
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 113) (line 5) (column 16) (len 11)) (segments (segment 0 (token "Views") (name "Views") (separator none) (span (offset 113) (line 5) (column 16) (len 5))) (segment 1 (token "tree") (name "tree") (separator colon-colon) (span (offset 120) (line 5) (column 23) (len 4)))))
    (reference r1 (scope relative) (span (offset 152) (line 7) (column 21) (len 9)) (segments (segment 0 (token "Structure") (name "Structure") (separator none) (span (offset 152) (line 7) (column 21) (len 9)))))
    (reference r2 (scope relative) (span (offset 198) (line 8) (column 35) (len 7)) (segments (segment 0 (token "Diagram") (name "Diagram") (separator none) (span (offset 198) (line 8) (column 35) (len 7)))))
    (reference r3 (scope relative) (span (offset 215) (line 8) (column 52) (len 4)) (segments (segment 0 (token "tree") (name "tree") (separator none) (span (offset 215) (line 8) (column 52) (len 4)))))
  )
  (root (package (name "Views") (body brace (rendering-def (modifiers)) (rendering-usage) (view-def (name "Structure") (short-name none) (modifiers) (specializes none) (body brace (view-rendering (form reference (ref r0)) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))) (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body brace (view-rendering (form inline "custom") (type (ref r2)) (multiplicity (lower (expression (span (offset 206) (line 8) (column 43) (len 1)) (integer 0))) (upper (expression (span (offset 209) (line 8) (column 46) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r3)))) (redefines none) (body brace (doc (name none) (locale none) (body (span (offset 240) (line 9) (column 19) (len 24)) (normalized "Inline rendering body. "))))))))))
)
~~~
