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
  )
  (root (package (name "CommentLoopFeatureBinding") (body brace (kerml-classifier (keyword class) (abstract false) (name "C") (specializes none) (body brace (binding (name none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 95) (line 4) (column 15) (len 30)) (normalized "feature binding body comment "))) (default-reference-usage))))))))
)
~~~
