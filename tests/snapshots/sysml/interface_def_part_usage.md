# META
~~~sexpr
(snapshot (type semantic) (description "InterfaceOccurrenceUsageElement includes StructureUsageElement and therefore PartUsage, in an interface definition body as in an interface usage body, including the end-prefixed spelling; part is the usage kind, not the end name."))
~~~
# SOURCE
~~~sysml
package InterfaceDefPartUsage {
    part def P;
    interface def I {
        end part p : P;
        part q;
    }
    interface i : I {
        end part p : P;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "interface_def_part_usage.md"
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
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 91) (line 4) (column 22) (len 1)) (segments (segment 0 (token "P") (name "P") (separator none) (span (offset 91) (line 4) (column 22) (len 1)))))
    (reference r1 (scope relative) (span (offset 134) (line 7) (column 19) (len 1)) (segments (segment 0 (token "I") (name "I") (separator none) (span (offset 134) (line 7) (column 19) (len 1)))))
    (reference r2 (scope relative) (span (offset 159) (line 8) (column 22) (len 1)) (segments (segment 0 (token "P") (name "P") (separator none) (span (offset 159) (line 8) (column 22) (len 1)))))
  )
  (root (package (visibility none) (name "InterfaceDefPartUsage") (body brace (part-def (name "P") (modifiers) (body semicolon)) (interface-def (name "I") (modifiers) (specializes none) (body brace (part-usage (then false) (prefix (end (cross none)) (extensions)) (declaration-name "p") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "q") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (interface-usage (form declaration) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "i") (short-name none) (type (ref r1)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (part none) (body brace (part-usage (then false) (prefix (end (cross none)) (extensions)) (declaration-name "p") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
