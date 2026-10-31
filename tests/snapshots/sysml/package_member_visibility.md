# META
~~~sexpr
(snapshot (type semantic) (description "PackageMember allows MemberPrefix on a nested package or library package. A visibility prefix stays on Package.membership / LibraryPackage.membership, later siblings stay typed, and the same prefix is legal at file root."))
~~~
# SOURCE
~~~sysml
package Outer {
    private package Vault {
        part def Treasure;
    }
    public package Shown;
    private library package Hidden { }
    part later : Shown;
}
private package RootHidden;
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "package_member_visibility.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Outer {
    private package Vault {
        part def Treasure;
    }
    public package Shown;
    private library package Hidden {
    }
    part later : Shown;
}

private package RootHidden;
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 159) (line 7) (column 18) (len 5)) (segments (segment 0 (token "Shown") (name "Shown") (separator none) (span (offset 159) (line 7) (column 18) (len 5)))))
  )
  (root (package (visibility none) (name "Outer") (body brace (package (visibility private) (name "Vault") (body brace (part-def (name "Treasure") (modifiers) (body semicolon)))) (package (visibility public) (name "Shown") (body semicolon)) (library-package (visibility private) (name "Hidden") (standard false) (body brace)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "later") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (package (visibility private) (name "RootHidden") (body semicolon)))
)
~~~
