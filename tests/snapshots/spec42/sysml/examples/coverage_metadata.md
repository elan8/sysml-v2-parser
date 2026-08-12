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
      (diagnostic (code "unsupported_annotation_syntax") (severity warning) (category unsupportedgrammarform) (span (offset 73) (line 5) (column 5) (len 35)) (message "incomplete parser support for annotation syntax in package body"))
      (diagnostic (code "unsupported_annotation_syntax") (severity warning) (category unsupportedgrammarform) (span (offset 245) (line 14) (column 5) (len 47)) (message "incomplete parser support for annotation syntax in package body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
metadata def Classified;

metadata def Approval;

package Annotated {
    @ Classified about Annotated;
    part def Vehicle;
    part def Engine;
    metadata m : Classified about Vehicle, Engine;
    #Classified;
    part def AnnotatedPart;
    #Approval #Classified part def MultiAnnotated;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (metadata-def) (metadata-def) (package (name "Annotated") (body (malformed (code "unsupported_annotation_syntax") (found "@ Classified about Annotated;") (span (offset 73) (line 5) (column 5) (len 35))) (part-def (name "Vehicle") (body semicolon)) (part-def (name "Engine") (body semicolon)) (metadata-usage) (metadata-keyword-usage) (part-def (name "AnnotatedPart") (body semicolon)) (malformed (code "unsupported_annotation_syntax") (found "#Approval #Classified part def MultiAnnotated;") (span (offset 245) (line 14) (column 5) (len 47))))))
)
~~~
