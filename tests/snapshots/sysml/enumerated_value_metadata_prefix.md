# META
~~~sexpr
(snapshot (type semantic) (description "EnumeratedValue = UsageExtensionKeyword* 'enum'? Usage (reference SysML.xtext 784-786): a `#Tag` prefix on an enumerated value is retained rather than recovered. The published .kebnf (528-535) omits the run; the reference grammar and MetadataTest.sysml author it."))
~~~
# SOURCE
~~~sysml
package EnumeratedValuePilotMetadataRecovery {
    metadata def Security;
    enum def ClassificationLevel {
        enum uncl = 0;
        #Security enum secret : ClassificationLevel = 2;
        enum conf = 1;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "enumerated_value_metadata_prefix.md"
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
    (reference r0 (scope relative) (span (offset 141) (line 5) (column 10) (len 8)) (segments (segment 0 (token "Security") (name "Security") (separator none) (span (offset 141) (line 5) (column 10) (len 8)))))
    (reference r1 (scope relative) (span (offset 164) (line 5) (column 33) (len 19)) (segments (segment 0 (token "ClassificationLevel") (name "ClassificationLevel") (separator none) (span (offset 164) (line 5) (column 33) (len 19)))))
  )
  (root (package (name "EnumeratedValuePilotMetadataRecovery") (body brace (metadata-def (name "Security") (abstract false) (specializes none) (body semicolon)) (enum-def (name "ClassificationLevel") (body brace (enum-value (extensions) (enum-keyword present) (visibility none) (name "uncl") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 129) (line 4) (column 21) (len 1)) (integer 0))))) (body semicolon) (span (offset 117) (line 4) (column 9) (len 14))) (enum-value (extensions (ref r0)) (enum-keyword present) (visibility none) (name "secret") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 186) (line 5) (column 55) (len 1)) (integer 2))))) (body semicolon) (span (offset 140) (line 5) (column 9) (len 48))) (enum-value (extensions) (enum-keyword present) (visibility none) (name "conf") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 209) (line 6) (column 21) (len 1)) (integer 1))))) (body semicolon) (span (offset 197) (line 6) (column 9) (len 14))))))))
)
~~~
