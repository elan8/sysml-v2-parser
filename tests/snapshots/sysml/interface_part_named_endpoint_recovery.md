# META
~~~sexpr
(snapshot (type recovery) (description "A named InterfaceEnd with its required OwnedReferenceSubsetting missing is recovered as one malformed interface member, while the following sibling remains typed. InterfacePart parsing is transactional, so the failed endpoint does not leak a target reference (SysML textual BNF 767-784; pinned Pilot SysML.xtext 1159-1186)."))
~~~
# SOURCE
~~~sysml
package InterfacePartNamedEndpointRecovery {
    part host {
        interface i : I connect source ::> ;
        part retained;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "interface_part_named_endpoint_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_part_usage_body_element") (severity error) (category parseerror) (span (offset 69) (line 3) (column 9) (len 45)) (message "unexpected token in part usage body"))
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
  )
  (root (package (name "InterfacePartNamedEndpointRecovery") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "host") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (malformed (code "recovered_part_usage_body_element") (found "interface i : I connect source ::> ;") (span (offset 69) (line 3) (column 9) (len 45))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "retained") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))))))
)
~~~
