# META
~~~sexpr
(snapshot (type semantic) (description "`InterfaceUsage = OccurrenceUsagePrefix 'interface' InterfaceUsageDeclaration InterfaceBody`, and `InterfaceUsageDeclaration`'s `UsageDeclaration` makes the `: Type` optional and its `( 'connect' InterfacePart )?` optional too. A name was therefore reachable only through the typed spelling or the connect spelling: `interface i;` and `interface i { ... }` left the name unconsumed, the body parser failed on it, and the whole member went to recovery. The anonymous forms are here because the lookahead that fixes it must not claim the first connector end of `interface a to b` as a declared name."))
~~~
# SOURCE
~~~sysml
package InterfaceUsageDeclarationForms {
    part host {
        interface bare;
        interface documented {
            doc /* a declared interface usage with no typing */
        }
        interface specialized :> BaseInterface;
        interface typed : Bus {
            doc /* the spelling that already parsed */
        }
        interface connect leftPort to rightPort;
        interface named connect leftPort to rightPort;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "interface_usage_declaration_forms.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package InterfaceUsageDeclarationForms {
    part host {
        interface bare;
        interface documented {
            doc
            /* a declared interface usage with no typing */
        }
        interface specialized :> BaseInterface;
        interface typed : Bus {
            doc
            /* the spelling that already parsed */
        }
        interface connect leftPort to rightPort;
        interface named connect leftPort to rightPort;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "InterfaceUsageDeclarationForms") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "host") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (interface-usage) (interface-usage) (interface-usage) (interface-usage) (interface-usage) (interface-usage))))))
)
~~~
