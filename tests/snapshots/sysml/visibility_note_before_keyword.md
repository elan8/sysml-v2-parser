# META
~~~sexpr
(snapshot (type semantic) (description "Notes are trivia after a visibility keyword, with or without surrounding whitespace, on the definitions and aliases that read their prefix through visibility_prefix. The visibility stays on the member and a following sibling stays typed."))
~~~
# SOURCE
~~~sysml
private //* before keyword */ part def Commented;
protected//* no space */part def Tight;
public // line note
part def Line;
private//* before alias */alias Other for Commented;
part later;
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "visibility_note_before_keyword.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
private part def Commented;

protected part def Tight;

public part def Line;

private alias Other for Commented;

part later;
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 167) (line 5) (column 43) (len 9)) (segments (segment 0 (token "Commented") (name "Commented") (separator none) (span (offset 167) (line 5) (column 43) (len 9)))))
  )
  (root (part-def (name "Commented") (modifiers) (body semicolon)) (part-def (name "Tight") (modifiers) (body semicolon)) (part-def (name "Line") (modifiers) (body semicolon)) (alias (name "Other") (target (ref r0)) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "later") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))
)
~~~
