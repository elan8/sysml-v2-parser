# META
~~~sexpr
(snapshot (type semantic) (description "A PartDefinition owns DefinitionBodyItem, whose DefinitionMember -> DefinitionElement alternatives include both Package and LibraryPackage (SysML textual BNF 180-207 and 234-248; pinned Pilot SysML agrees). The nested namespaces retain distinct typed forms and their bodies."))
~~~
# SOURCE
~~~sysml
part def Container {
    package Child {
        part component : Component;
    }
    library package LibraryChild {
        attribute value : ScalarValues::Integer;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "part_def_nested_packages.md"
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
    (reference r0 (scope relative) (span (offset 66) (line 3) (column 26) (len 9)) (segments (segment 0 (token "Component") (name "Component") (separator none) (span (offset 66) (line 3) (column 26) (len 9)))))
  )
  (root (part-def (name "Container") (modifiers) (body brace (package (name "Child") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "component") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (library-package (name "LibraryChild") (standard false) (body brace (attribute-usage))))))
)
~~~
