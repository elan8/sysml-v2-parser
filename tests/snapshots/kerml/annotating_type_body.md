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
        'rep';
        classifierRep;
        'language';
        "text";
        feature wheels {
            doc
            /* feature body */
            comment
            /* feature aside */
            'rep';
            featureRep;
            'language';
            "text";
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "AnnotatingTypeBody") (body brace (kerml-classifier (keyword classifier) (abstract false) (name "Vehicle") (specializes none)))))
)
~~~
