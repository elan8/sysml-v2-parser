# META
~~~sexpr
(snapshot (type semantic) (description "Coverage: Metadata features with about clause and named metadata"))
~~~
# SOURCE
~~~sysml
metadata def Classified;
metadata def Approval;

package Annotated {
    @ Classified about Annotated;

    part def Vehicle;
    part def Engine;

    metadata m : Classified about Vehicle, Engine;

    #Classified part def AnnotatedPart;

    #Approval #Classified part def MultiAnnotated;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "coverage_metadata.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
metadata def Classified;

metadata def Approval;

package Annotated {
    @Classified about Annotated;
    part def Vehicle;
    part def Engine;
    metadata m : Classified about Vehicle, Engine;
    #Classified
    part def AnnotatedPart;
    #Approval
    #Classified
    part def MultiAnnotated;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (metadata-def) (metadata-def) (package (name "Annotated") (body brace (metadata-annotation) (part-def (name "Vehicle") (body semicolon)) (part-def (name "Engine") (body semicolon)) (metadata-usage) (metadata-keyword-usage) (part-def (name "AnnotatedPart") (body semicolon)) (metadata-keyword-usage) (metadata-keyword-usage) (part-def (name "MultiAnnotated") (body semicolon)))))
)
~~~
