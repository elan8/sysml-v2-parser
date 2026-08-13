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
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 241) (line 7) (column 16) (len 16)) (message "unrecognized declaration `batteryLow` in package body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 268) (line 8) (column 16) (len 11)) (message "unrecognized declaration `x` in package body"))
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
    #situation;
    batteryLow;
    #situation;
    x : T;
    #situation;
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
  (root (package (name "ExtendedExamples") (body (extended-def (prefix-keywords ("situation")) (definition-prefix none) (name "Failure") (specializes none) (body semicolon)) (extended-def (prefix-keywords ("situation")) (definition-prefix none) (name "Failure") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r0)))) (body semicolon)) (extended-def (prefix-keywords ("situation")) (definition-prefix abstract) (name "AbstractFailure") (specializes none) (body semicolon)) (extended-def (prefix-keywords ("SecurityRelated" "situation")) (definition-prefix none) (name "Vulnerability") (specializes none) (body semicolon)) (extended-def (prefix-keywords ("situation")) (definition-prefix none) (name "Failure") (specializes none) (body (part-usage))) (metadata-keyword-usage) (malformed (code "unrecognized_declaration_in_scope") (found "batteryLow;") (span (offset 241) (line 7) (column 16) (len 16))) (metadata-keyword-usage) (malformed (code "unrecognized_declaration_in_scope") (found "x : T;") (span (offset 268) (line 8) (column 16) (len 11))) (metadata-keyword-usage) (malformed (code "unrecognized_declaration_in_scope") (found "x : T { }") (span (offset 290) (line 9) (column 16) (len 14))) (extended-def (prefix-keywords ("situation")) (definition-prefix variation) (name "V") (specializes none) (body semicolon)))))
)
~~~
