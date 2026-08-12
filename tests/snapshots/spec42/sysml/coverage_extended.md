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
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 42) (line 2) (column 16) (len 17)) (message "unexpected keyword `def` in package body"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 70) (line 3) (column 16) (len 25)) (message "unexpected keyword `def` in package body"))
      (diagnostic (code "recovered_package_body_element") (severity error) (category parseerror) (span (offset 95) (line 4) (column 5) (len 45)) (message "unexpected token in package body"))
      (diagnostic (code "unsupported_annotation_syntax") (severity warning) (category unsupportedgrammarform) (span (offset 140) (line 5) (column 5) (len 51)) (message "incomplete parser support for annotation syntax in package body"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 202) (line 6) (column 16) (len 28)) (message "unexpected keyword `def` in package body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 241) (line 7) (column 16) (len 16)) (message "unrecognized declaration `batteryLow` in package body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 268) (line 8) (column 16) (len 11)) (message "unrecognized declaration `x` in package body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 290) (line 9) (column 16) (len 14)) (message "unrecognized declaration `x` in package body"))
      (diagnostic (code "recovered_package_body_element") (severity error) (category parseerror) (span (offset 304) (line 10) (column 5) (len 28)) (message "unexpected token in package body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package ExtendedExamples {
    #situation;
    def Failure;
    #situation;
    def Failure :> Base;
    abstract #situation def AbstractFailure;
    #SecurityRelated #situation def Vulnerability;
    #situation;
    def Failure { part p; }
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
  )
  (root (package (name "ExtendedExamples") (body (metadata-keyword-usage) (malformed (code "unexpected_keyword_in_scope") (found "def Failure;") (span (offset 42) (line 2) (column 16) (len 17))) (metadata-keyword-usage) (malformed (code "unexpected_keyword_in_scope") (found "def Failure :> Base;") (span (offset 70) (line 3) (column 16) (len 25))) (malformed (code "recovered_package_body_element") (found "abstract #situation def AbstractFailure;") (span (offset 95) (line 4) (column 5) (len 45))) (malformed (code "unsupported_annotation_syntax") (found "#SecurityRelated #situation def Vulnerability;") (span (offset 140) (line 5) (column 5) (len 51))) (metadata-keyword-usage) (malformed (code "unexpected_keyword_in_scope") (found "def Failure { part p; }") (span (offset 202) (line 6) (column 16) (len 28))) (metadata-keyword-usage) (malformed (code "unrecognized_declaration_in_scope") (found "batteryLow;") (span (offset 241) (line 7) (column 16) (len 16))) (metadata-keyword-usage) (malformed (code "unrecognized_declaration_in_scope") (found "x : T;") (span (offset 268) (line 8) (column 16) (len 11))) (metadata-keyword-usage) (malformed (code "unrecognized_declaration_in_scope") (found "x : T { }") (span (offset 290) (line 9) (column 16) (len 14))) (malformed (code "recovered_package_body_element") (found "variation #situation def V;") (span (offset 304) (line 10) (column 5) (len 28))))))
)
~~~
