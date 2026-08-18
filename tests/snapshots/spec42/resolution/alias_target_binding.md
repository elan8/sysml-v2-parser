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
    (reference r1 (scope relative) (span (offset 97) (line 4) (column 19) (len 11)) (segments (segment 0 (token "DeviceAlias") (name "DeviceAlias") (separator none) (span (offset 97) (line 4) (column 19) (len 11)))))
  )
  (root (package (name "AliasCoverage") (body brace (part-def (name "Device") (body semicolon)) (alias (name "DeviceAlias") (target (ref r0)) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "device") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))))
)
~~~
