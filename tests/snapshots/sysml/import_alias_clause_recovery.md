# META
~~~sexpr
(snapshot (type recovery) (description "SysML imports have no renaming clause: `Import = VisibilityIndicator 'import' 'all'? ImportDeclaration RelationshipBody` (SysML BNF 8.2.2.4). `import A as B;` at root and in a package body, with a membership, namespace or quoted target, reports import_alias_clause exactly on the `as` clause and suggests an alias membership (`alias B for A;`, SysML 7.5.2). The statement is retained as a recovery node, and later siblings, including a valid import, still parse."))
~~~
# SOURCE
~~~sysml
public import A as B;
package A { part def X; }
package C {
    public import A as B;
    private import A::* as 'Other Name';
    private import 'Quoted Pkg'::X as Y;
    part def Kept;
    public import A::X;
    public import A /* note */ as Commented;
    public import A::X[x as Real];
    alias Renamed for A::X;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "import_alias_clause_recovery.md"
    (diagnostics
      (diagnostic (code "import_alias_clause") (severity error) (category parseerror) (span (offset 16) (line 1) (column 17) (len 4)) (message "a SysML import has no 'as' clause; an alias membership names an element differently"))
      (diagnostic (code "import_alias_clause") (severity error) (category parseerror) (span (offset 80) (line 4) (column 21) (len 4)) (message "a SysML import has no 'as' clause; an alias membership names an element differently"))
      (diagnostic (code "import_alias_clause") (severity error) (category parseerror) (span (offset 110) (line 5) (column 25) (len 15)) (message "a SysML import has no 'as' clause; an alias membership names an element differently"))
      (diagnostic (code "import_alias_clause") (severity error) (category parseerror) (span (offset 162) (line 6) (column 36) (len 4)) (message "a SysML import has no 'as' clause; an alias membership names an element differently"))
      (diagnostic (code "import_alias_clause") (severity error) (category parseerror) (span (offset 242) (line 9) (column 32) (len 12)) (message "a SysML import has no 'as' clause; an alias membership names an element differently"))
    )
  )
)
~~~
# FORMAT
~~~sysml
public import A as B;

package A {
    part def X;
}

package C {
    public import A as B;
    private import A::* as 'Other Name';
    private import 'Quoted Pkg'::X as Y;
    part def Kept;
    public import A::X;
    public import A /* note */ as Commented;
    public import A::X [x as Real];
    alias Renamed for A::X;
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 205) (line 8) (column 19) (len 4)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 205) (line 8) (column 19) (len 1))) (segment 1 (token "X") (name "X") (separator colon-colon) (span (offset 208) (line 8) (column 22) (len 1)))))
    (reference r1 (scope relative) (span (offset 274) (line 10) (column 19) (len 4)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 274) (line 10) (column 19) (len 1))) (segment 1 (token "X") (name "X") (separator colon-colon) (span (offset 277) (line 10) (column 22) (len 1)))))
    (reference r2 (scope relative) (span (offset 279) (line 10) (column 24) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 279) (line 10) (column 24) (len 1)))))
    (reference r3 (scope relative) (span (offset 284) (line 10) (column 29) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 284) (line 10) (column 29) (len 4)))))
    (reference r4 (scope relative) (span (offset 313) (line 11) (column 23) (len 4)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 313) (line 11) (column 23) (len 1))) (segment 1 (token "X") (name "X") (separator colon-colon) (span (offset 316) (line 11) (column 26) (len 1)))))
  )
  (root (malformed (code "import_alias_clause") (found "as") (span (offset 0) (line 1) (column 1) (len 21)) (diagnostic-span (span (offset 16) (line 1) (column 17) (len 4)))) (package (name "A") (body brace (part-def (name "X") (modifiers) (body semicolon)))) (package (name "C") (body brace (malformed (code "import_alias_clause") (found "as") (span (offset 64) (line 4) (column 5) (len 21)) (diagnostic-span (span (offset 80) (line 4) (column 21) (len 4)))) (malformed (code "import_alias_clause") (found "as") (span (offset 90) (line 5) (column 5) (len 36)) (diagnostic-span (span (offset 110) (line 5) (column 25) (len 15)))) (malformed (code "import_alias_clause") (found "as") (span (offset 131) (line 6) (column 5) (len 36)) (diagnostic-span (span (offset 162) (line 6) (column 36) (len 4)))) (part-def (name "Kept") (modifiers) (body semicolon)) (import (visibility public) (target (span (span (offset 205) (line 8) (column 19) (len 4))) (all none) (ref r0) (shape (membership (recursive-suffix none)))) (body semicolon)) (malformed (code "import_alias_clause") (found "as") (span (offset 215) (line 9) (column 5) (len 40)) (diagnostic-span (span (offset 242) (line 9) (column 32) (len 12)))) (import (visibility public) (target (span (span (offset 274) (line 10) (column 19) (len 15))) (all none) (ref r1) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 278) (line 10) (column 23) (len 11))) (open (span (offset 278) (line 10) (column 23) (len 1))) (expression (expression (span (offset 279) (line 10) (column 24) (len 9)) (type-check (kind as) (operand (expression (span (offset 279) (line 10) (column 24) (len 1)) (ref r2))) (type (ref r3))))) (close (span (offset 288) (line 10) (column 33) (len 1)))))))) (body semicolon)) (alias (name "Renamed") (target (ref r4)) (body semicolon)))))
)
~~~
