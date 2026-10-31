# META
~~~sexpr
(snapshot (type semantic) (description "Migrated verification return-ref control preserves its declaration and nested qualified return expression."))
~~~
# SOURCE
~~~sysml
package P { verification def V { return ref verdictResult { return VerdictKind::unknown; } } }
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "verification_return_ref_expression.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package P {
    verification def V {
        return ref verdictResult {
            return VerdictKind::unknown;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 67) (line 1) (column 68) (len 20)) (segments (segment 0 (token "VerdictKind") (name "VerdictKind") (separator none) (span (offset 67) (line 1) (column 68) (len 11))) (segment 1 (token "unknown") (name "unknown") (separator colon-colon) (span (offset 80) (line 1) (column 81) (len 7)))))
  )
  (root (package (visibility none) (name "P") (body brace (verification-case-def (name "V") (modifiers) (body brace (return-ref (name "verdictResult") (short-name none) (body-span (span (offset 58) (line 1) (column 59) (len 32))) (body brace (result (expression (span (offset 67) (line 1) (column 68) (len 20)) (ref r0))))))))))
)
~~~
