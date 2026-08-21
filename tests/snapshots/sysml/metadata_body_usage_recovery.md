# META
~~~sexpr
(snapshot (type recovery) (description "A malformed MetadataBodyUsage remains one source-backed recovery element and does not consume the later valid nested metadata redefinition. The body uses its own recursive reference-member grammar, not AttributeBody/DefaultReferenceUsage (SysML textual BNF 1678-1693; pinned Pilot KerML.xtext 1098-1115)."))
~~~
# SOURCE
~~~sysml
package MetadataBodyRecovery {
    part vehicle {
        @Risk {
            totalRisk = ;
            ref :>> technicalRisk = medium;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "metadata_body_usage_recovery.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 78) (line 4) (column 13) (len 26)) (message "unrecognized declaration `totalRisk` in metadata body"))
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
    (reference r0 (scope relative) (span (offset 59) (line 3) (column 10) (len 4)) (segments (segment 0 (token "Risk") (name "Risk") (separator none) (span (offset 59) (line 3) (column 10) (len 4)))))
    (reference r1 (scope relative) (span (offset 112) (line 5) (column 21) (len 13)) (segments (segment 0 (token "technicalRisk") (name "technicalRisk") (separator none) (span (offset 112) (line 5) (column 21) (len 13)))))
    (reference r2 (scope relative) (span (offset 128) (line 5) (column 37) (len 6)) (segments (segment 0 (token "medium") (name "medium") (separator none) (span (offset 128) (line 5) (column 37) (len 6)))))
  )
  (root (package (name "MetadataBodyRecovery") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r0)) (about) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "totalRisk = ;") (span (offset 78) (line 4) (column 13) (len 26))) (metadata-body-usage (reference true) (redefinition-operator colon-greater-greater) (target (ref r1)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 128) (line 5) (column 37) (len 6)) (ref r2))))) (body semicolon)))))))))
)
~~~
