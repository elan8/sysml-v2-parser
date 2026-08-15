# META
~~~sexpr
(snapshot (type semantic) (description "Group 12: Extended Definitions and Usages (SysML §8.2.2.27)"))
~~~
# SOURCE
~~~sysml
package ExtendedExamples {
    #situation def Failure;
    #situation def Failure :> Base;
    abstract #situation def AbstractFailure;
    #SecurityRelated #situation def Vulnerability;
    #situation def Failure { part p; }
    #situation batteryLow;
    #situation x : T;
    #situation x : T { }
    variation #situation def V;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "coverage_extended.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 290) (line 9) (column 16) (len 14)) (message "unrecognized declaration `x` in package body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package ExtendedExamples {
    #situation def Failure;
    #situation def Failure :> Base;
    abstract #situation def AbstractFailure;
    #SecurityRelated #situation def Vulnerability;
    #situation def Failure {
        part p;
    }
    #situation batteryLow;
    #situation
    x : T;
    #situation
    x : T { }
    variation #situation def V;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 85) (line 3) (column 31) (len 4)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 85) (line 3) (column 31) (len 4)))))
  )
  (root (package (name "ExtendedExamples") (body brace (extended-def (prefix-keywords ("situation")) (definition-prefix none) (def true) (name "Failure") (specializes none) (body semicolon)) (extended-def (prefix-keywords ("situation")) (definition-prefix none) (def true) (name "Failure") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r0)))) (body semicolon)) (extended-def (prefix-keywords ("situation")) (definition-prefix abstract) (def true) (name "AbstractFailure") (specializes none) (body semicolon)) (extended-def (prefix-keywords ("SecurityRelated" "situation")) (definition-prefix none) (def true) (name "Vulnerability") (specializes none) (body semicolon)) (extended-def (prefix-keywords ("situation")) (definition-prefix none) (def true) (name "Failure") (specializes none) (body brace (part-usage))) (extended-def (prefix-keywords ("situation")) (definition-prefix none) (def false) (name "batteryLow") (specializes none) (body semicolon)) (metadata-keyword-usage) (default-reference-usage) (metadata-keyword-usage) (malformed (code "unrecognized_declaration_in_scope") (found "x : T { }") (span (offset 290) (line 9) (column 16) (len 14))) (extended-def (prefix-keywords ("situation")) (definition-prefix variation) (def true) (name "V") (specializes none) (body semicolon)))))
)
~~~
