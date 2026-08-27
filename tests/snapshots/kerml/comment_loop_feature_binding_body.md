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
    (reference r0 (scope relative) (span (offset 66) (line 3) (column 17) (len 5)) (segments (segment 0 (token "value") (name "value") (separator none) (span (offset 66) (line 3) (column 17) (len 5)))))
    (reference r1 (scope relative) (span (offset 74) (line 3) (column 25) (len 4)) (segments (segment 0 (token "self") (name "self") (separator none) (span (offset 74) (line 3) (column 25) (len 4)))))
    (reference r2 (scope relative) (span (offset 149) (line 5) (column 22) (len 4)) (segments (segment 0 (token "self") (name "self") (separator none) (span (offset 149) (line 5) (column 22) (len 4)))))
  )
  (root (package (name "CommentLoopFeatureBinding") (body brace (kerml-classifier (keyword class) (abstract false) (name "C") (specializes none) (conjugates none) (body brace (binding (all false) (name none) (multiplicity none) (inline-ends (pair (of false) (left (connector-end (multiplicity none) (target (ref r0)) (references none))) (right (connector-end (multiplicity none) (target (ref r1)) (references none))))) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 95) (line 4) (column 15) (len 30)) (normalized "feature binding body comment "))) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "nested") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 149) (line 5) (column 22) (len 4)) (ref r2))))) (body semicolon)))))))))
)
~~~
