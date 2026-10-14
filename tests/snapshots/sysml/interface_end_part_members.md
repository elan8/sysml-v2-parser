# META
~~~sexpr
(snapshot (type semantic) (description "InterfaceOccurrenceUsageElement includes StructureUsageElement: an end-prefixed PartUsage remains a part with an authored end prefix and reference subsetting, allowing validateInterfaceUsageEnd to report its semantic type error. The part keyword is not a declaration name. Pinned SysML 8.2.2.14 and OpenSysML validateInterfaceUsageEnd probe."))
~~~
# SOURCE
~~~sysml
package InterfaceEndParts {
    part def Fuel;
    port def P;
    part vehicle {
        part fuel : Fuel;
        port inlet : P;
        interface bad {
            end part ::> fuel;
            end port ::> inlet;
            end part named ::> fuel;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "interface_end_part_members.md"
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
    (reference r0 (scope relative) (span (offset 102) (line 5) (column 21) (len 4)) (segments (segment 0 (token "Fuel") (name "Fuel") (separator none) (span (offset 102) (line 5) (column 21) (len 4)))))
    (reference r1 (scope relative) (span (offset 129) (line 6) (column 22) (len 1)) (segments (segment 0 (token "P") (name "P") (separator none) (span (offset 129) (line 6) (column 22) (len 1)))))
    (reference r2 (scope relative) (span (offset 181) (line 8) (column 26) (len 4)) (segments (segment 0 (token "fuel") (name "fuel") (separator none) (span (offset 181) (line 8) (column 26) (len 4)))))
    (reference r3 (scope relative) (span (offset 212) (line 9) (column 26) (len 5)) (segments (segment 0 (token "inlet") (name "inlet") (separator none) (span (offset 212) (line 9) (column 26) (len 5)))))
    (reference r4 (scope relative) (span (offset 250) (line 10) (column 32) (len 4)) (segments (segment 0 (token "fuel") (name "fuel") (separator none) (span (offset 250) (line 10) (column 32) (len 4)))))
  )
  (root (package (name "InterfaceEndParts") (body brace (part-def (name "Fuel") (modifiers) (body semicolon)) (port-def (name "P") (modifiers) (specializes none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "fuel") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "inlet") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (interface-usage (form declaration) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "bad") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (part none) (body brace (part-usage (then false) (prefix (end (cross none)) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references (relationship (kind references) (implied false) (targets (ref r2)))) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (end (cross none)) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references (relationship (kind references) (implied false) (targets (ref r3)))) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage (then false) (prefix (end (cross none)) (extensions)) (declaration-name "named") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references (relationship (kind references) (implied false) (targets (ref r4)))) (crosses none) (intersects none) (value none) (body semicolon)))))))))
)
~~~
