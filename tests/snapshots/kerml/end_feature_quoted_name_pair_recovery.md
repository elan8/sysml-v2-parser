# META
~~~sexpr
(snapshot (type recovery) (description "`end 'bool' g;` is not a Feature: a cross feature (KerML BNF 585/592) requires a following `feature` keyword or prefix metadata (562-565), and `'bool' g` is not a FeatureDeclaration, so the member is one recovery node while its valid siblings `end y;` and `end bool g;` (TransitionPerformances.kerml) stay features. It must not be shredded into three bare result expressions."))
~~~
# SOURCE
~~~sysml
package P {
    assoc struct A {
        end y;
        end 'bool' g;
        end bool h;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "end_feature_quoted_name_pair_recovery.md"
    (diagnostics
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 56) (line 4) (column 9) (len 22)) (message "unexpected keyword `end` in calc body"))
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
  (root (package (name "P") (body brace (kerml-classifier (keyword assoc struct) (abstract false) (name "A") (multiplicity none) (specializes none) (conjugates none) (body brace (kerml-feature (prefix (head end) (constant false) (cross none) (metadata)) (kind none) (member false) (all false) (name "y") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (malformed (code "unexpected_keyword_in_scope") (found "end 'bool' g;") (span (offset 56) (line 4) (column 9) (len 22))) (kerml-feature (prefix (head end) (constant false) (cross none) (metadata)) (kind bool) (member false) (all false) (name "h") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)))))))
)
~~~
