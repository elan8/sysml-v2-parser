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
    (reference r0 (scope relative) (span (offset 75) (line 5) (column 7) (len 10)) (segments (segment 0 (token "Classified") (name "Classified") (separator none) (span (offset 75) (line 5) (column 7) (len 10)))))
    (reference r1 (scope relative) (span (offset 92) (line 5) (column 24) (len 9)) (segments (segment 0 (token "Annotated") (name "Annotated") (separator none) (span (offset 92) (line 5) (column 24) (len 9)))))
    (reference r2 (scope relative) (span (offset 205) (line 12) (column 6) (len 10)) (segments (segment 0 (token "Classified") (name "Classified") (separator none) (span (offset 205) (line 12) (column 6) (len 10)))))
    (reference r3 (scope relative) (span (offset 246) (line 14) (column 6) (len 8)) (segments (segment 0 (token "Approval") (name "Approval") (separator none) (span (offset 246) (line 14) (column 6) (len 8)))))
    (reference r4 (scope relative) (span (offset 256) (line 14) (column 16) (len 10)) (segments (segment 0 (token "Classified") (name "Classified") (separator none) (span (offset 256) (line 14) (column 16) (len 10)))))
  )
  (root (metadata-def) (metadata-def) (package (name "Annotated") (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r0)) (about (ref r1)) (body semicolon)) (part-def (name "Vehicle") (body semicolon)) (part-def (name "Engine") (body semicolon)) (metadata-usage) (metadata-keyword-usage (type (ref r2)) (body none)) (part-def (name "AnnotatedPart") (body semicolon)) (metadata-keyword-usage (type (ref r3)) (body none)) (metadata-keyword-usage (type (ref r4)) (body none)) (part-def (name "MultiAnnotated") (body semicolon)))))
)
~~~
