# META
~~~sexpr
(snapshot (type semantic) (description "Alias target and binding resolution coverage"))
~~~
# SOURCE
~~~sysml
package AliasCoverage {
    part def Device;
    alias DeviceAlias for Device;
    part device : DeviceAlias;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "alias_target_binding.md"
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
    (reference r0 (scope relative) (span (offset 71) (line 3) (column 27) (len 6)) (segments (segment 0 (token "Device") (name "Device") (separator none) (span (offset 71) (line 3) (column 27) (len 6)))))
  )
  (root (package (name "AliasCoverage") (body brace (part-def (name "Device") (body semicolon)) (alias (name "DeviceAlias") (target (ref r0)) (body semicolon)) (part-usage))))
)
~~~
