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
    (reference r2 (scope relative) (span (offset 165) (line 10) (column 18) (len 10)) (segments (segment 0 (token "Classified") (name "Classified") (separator none) (span (offset 165) (line 10) (column 18) (len 10)))))
    (reference r3 (scope relative) (span (offset 182) (line 10) (column 35) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 182) (line 10) (column 35) (len 7)))))
    (reference r4 (scope relative) (span (offset 191) (line 10) (column 44) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 191) (line 10) (column 44) (len 6)))))
    (reference r5 (scope relative) (span (offset 205) (line 12) (column 6) (len 10)) (segments (segment 0 (token "Classified") (name "Classified") (separator none) (span (offset 205) (line 12) (column 6) (len 10)))))
    (reference r6 (scope relative) (span (offset 246) (line 14) (column 6) (len 8)) (segments (segment 0 (token "Approval") (name "Approval") (separator none) (span (offset 246) (line 14) (column 6) (len 8)))))
    (reference r7 (scope relative) (span (offset 256) (line 14) (column 16) (len 10)) (segments (segment 0 (token "Classified") (name "Classified") (separator none) (span (offset 256) (line 14) (column 16) (len 10)))))
  )
  (root (metadata-def (name "Classified") (abstract false) (specializes none) (body semicolon)) (metadata-def (name "Approval") (abstract false) (specializes none) (body semicolon)) (package (name "Annotated") (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r0)) (about (ref r1)) (body semicolon)) (part-def (name "Vehicle") (modifiers) (body semicolon)) (part-def (name "Engine") (modifiers) (body semicolon)) (metadata-usage (declaration-name "m") (type (ref r2)) (about (ref r3) (ref r4)) (body semicolon)) (metadata-keyword-usage (type (ref r5)) (body none)) (part-def (name "AnnotatedPart") (modifiers) (body semicolon)) (metadata-keyword-usage (type (ref r6)) (body none)) (metadata-keyword-usage (type (ref r7)) (body none)) (part-def (name "MultiAnnotated") (modifiers) (body semicolon)))))
)
~~~
