# META
~~~sexpr
(snapshot (type semantic) (description "EnumeratedValue = UsageExtensionKeyword* 'enum'? Usage (reference SysML.xtext 784-786): a `#Tag` run ahead of an enumerated value is retained in authored order, with or without the optional keyword (MetadataTest.sysml)"))
~~~
# SOURCE
~~~sysml
package P {
    enum def ClassificationLevel :> ScalarValues::Natural {
        uncl : ClassificationLevel = 0;
        #Security enum secret : ClassificationLevel = 2;
        #Security #Classified topSecret = 3;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "enumerated_value_extension_keywords.md"
    (diagnostics
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
    (reference r0 (scope relative) (span (offset 87) (line 3) (column 16) (len 19)) (segments (segment 0 (token "ClassificationLevel") (name "ClassificationLevel") (separator none) (span (offset 87) (line 3) (column 16) (len 19)))))
    (reference r1 (scope relative) (span (offset 121) (line 4) (column 10) (len 8)) (segments (segment 0 (token "Security") (name "Security") (separator none) (span (offset 121) (line 4) (column 10) (len 8)))))
    (reference r2 (scope relative) (span (offset 144) (line 4) (column 33) (len 19)) (segments (segment 0 (token "ClassificationLevel") (name "ClassificationLevel") (separator none) (span (offset 144) (line 4) (column 33) (len 19)))))
    (reference r3 (scope relative) (span (offset 178) (line 5) (column 10) (len 8)) (segments (segment 0 (token "Security") (name "Security") (separator none) (span (offset 178) (line 5) (column 10) (len 8)))))
    (reference r4 (scope relative) (span (offset 188) (line 5) (column 20) (len 10)) (segments (segment 0 (token "Classified") (name "Classified") (separator none) (span (offset 188) (line 5) (column 20) (len 10)))))
  )
  (root (package (name "P") (body brace (enum-def (name "ClassificationLevel") (body brace (enum-value (extensions) (enum-keyword none) (visibility none) (name "uncl") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 109) (line 3) (column 38) (len 1)) (integer 0))))) (body semicolon) (span (offset 80) (line 3) (column 9) (len 31))) (enum-value (extensions (ref r1)) (enum-keyword present) (visibility none) (name "secret") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 166) (line 4) (column 55) (len 1)) (integer 2))))) (body semicolon) (span (offset 120) (line 4) (column 9) (len 48))) (enum-value (extensions (ref r3) (ref r4)) (enum-keyword none) (visibility none) (name "topSecret") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 211) (line 5) (column 43) (len 1)) (integer 3))))) (body semicolon) (span (offset 177) (line 5) (column 9) (len 36))))))))
)
~~~
