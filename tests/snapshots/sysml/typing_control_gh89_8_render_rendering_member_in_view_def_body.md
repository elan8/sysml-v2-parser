# META
~~~sexpr
(snapshot (type semantic) (description "Preserves gh89_8_render_rendering_member_in_view_def_body through the canonical parser projection while replacing scalar typing fields."))
~~~
# SOURCE
~~~sysml
package ViewTest {
            rendering def R;
            view def V {
                render rendering r1: R[0..1];
            }
        }
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "typing_control_gh89_8_render_rendering_member_in_view_def_body.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ViewTest {
    rendering def R;
    view def V {
        render rendering r1 : R[0..1];
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 110) (line 4) (column 38) (len 1)) (segments (segment 0 (token "R") (name "R") (separator none) (span (offset 110) (line 4) (column 38) (len 1)))))
  )
  (root (package (visibility none) (name "ViewTest") (body brace (rendering-def (name "R") (short-name none) (modifiers) (specializes none) (body semicolon)) (view-def (name "V") (short-name none) (modifiers) (specializes none) (body brace (view-rendering (form inline "r1") (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity (lower (expression (span (offset 112) (line 4) (column 40) (len 1)) (integer 0))) (upper (expression (span (offset 115) (line 4) (column 43) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))))))
)
~~~
