# META
~~~sexpr
(snapshot (type semantic) (description "KerML reaches the same production by its own route: TypeBodyElement -> NonFeatureMember -> MemberElement -> AnnotatingElement. A classifier body and a nested feature body therefore own all four alternatives, exactly as a SysML definition body does."))
~~~
# SOURCE
~~~sysml
package AnnotatingTypeBody {
    classifier Vehicle {
        doc /* classifier body */
        comment /* classifier aside */
        rep classifierRep language "text" /* classifier rendering */
        feature wheels {
            doc /* feature body */
            comment /* feature aside */
            rep featureRep language "text" /* feature rendering */
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "annotating_type_body.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package AnnotatingTypeBody {
    classifier Vehicle {
        doc
        /* classifier body */
        comment
        /* classifier aside */
        rep classifierRep language "text"
        /* classifier rendering */
        feature wheels {
            doc
            /* feature body */
            comment
            /* feature aside */
            rep featureRep language "text"
            /* feature rendering */
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "AnnotatingTypeBody") (body brace (kerml-classifier (keyword classifier) (abstract false) (name "Vehicle") (specializes none) (conjugates none) (body brace (doc (name none) (locale none) (body (span (offset 68) (line 3) (column 15) (len 17)) (normalized "classifier body "))) (comment (keyword (span (offset 96) (line 4) (column 9) (len 7))) (name none) (about) (locale none) (body (span (offset 106) (line 4) (column 19) (len 18)) (normalized "classifier aside "))) (textual-rep (name "classifierRep") (language "text") (body (span (offset 171) (line 5) (column 45) (len 22)) (normalized "classifier rendering "))) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "wheels") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body brace (doc (name none) (locale none) (body (span (offset 239) (line 7) (column 19) (len 14)) (normalized "feature body "))) (comment (keyword (span (offset 268) (line 8) (column 13) (len 7))) (name none) (about) (locale none) (body (span (offset 278) (line 8) (column 23) (len 15)) (normalized "feature aside "))) (textual-rep (name "featureRep") (language "text") (body (span (offset 341) (line 9) (column 46) (len 19)) (normalized "feature rendering "))))))))))
)
~~~
