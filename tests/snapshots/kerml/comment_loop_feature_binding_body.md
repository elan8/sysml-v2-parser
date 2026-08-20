# META
~~~sexpr
(snapshot (type semantic) (description "A KerML feature binding body owns AnnotatingElement members. Its manual loop must retain a regular comment between the binding header and its nested binding member."))
~~~
# SOURCE
~~~sysml
package CommentLoopFeatureBinding {
    class C {
        binding value = self {
            /* feature binding body comment */
            nested = self;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "comment_loop_feature_binding_body.md"
    (diagnostics
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
    (reference r0 (scope relative) (span (offset 149) (line 5) (column 22) (len 4)) (segments (segment 0 (token "self") (name "self") (separator none) (span (offset 149) (line 5) (column 22) (len 4)))))
  )
  (root (package (name "CommentLoopFeatureBinding") (body brace (kerml-classifier (keyword class) (abstract false) (name "C") (specializes none) (body brace (binding (name none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 95) (line 4) (column 15) (len 30)) (normalized "feature binding body comment "))) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "nested") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 149) (line 5) (column 22) (len 4)) (ref r0))))) (body semicolon)))))))))
)
~~~
