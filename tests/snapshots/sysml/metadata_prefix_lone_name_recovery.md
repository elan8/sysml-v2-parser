# META
~~~sexpr
(snapshot (type recovery) (description "A `#tag` prefixes a declaration, so a lone name after it with no terminator is malformed and reports missing_semicolon. It used to split silently into a bare tag and, in a case or analysis body, a result expression, with no diagnostic. A terminated `#tag name;` stays ExtendedUsage, and a tag before a keyword stays a prefix."))
~~~
# SOURCE
~~~sysml
case def Result {
    #q de
}
analysis def Analysis {
    #q ins
}
part def P {
    #q retained;
    #q part keyword;
    #q broken
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "metadata_prefix_lone_name_recovery.md"
    (diagnostics
      (diagnostic (code "missing_semicolon") (severity error) (category parseerror) (span (offset 22) (line 2) (column 5) (len 6)) (message "missing semicolon before next declaration"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 22) (line 2) (column 5) (len 6)) (message "suppressed 2 cascading missing_semicolon diagnostics after earlier recovery errors"))
    )
  )
)
~~~
# FORMAT
~~~sysml
case def Result {
    #q de
}

analysis def Analysis {
    #q ins
}

part def P {
    #q retained;
    #q part keyword;
    #q broken
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 85) (line 8) (column 6) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 85) (line 8) (column 6) (len 1)))))
    (reference r1 (scope relative) (span (offset 102) (line 9) (column 6) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 102) (line 9) (column 6) (len 1)))))
  )
  (root (case-def (name "Result") (modifiers) (body brace (malformed (code "missing_semicolon") (found "#q de") (span (offset 22) (line 2) (column 5) (len 6))))) (analysis-case-def (name "Analysis") (modifiers) (body brace (malformed (code "missing_semicolon") (found "#q ins") (span (offset 58) (line 5) (column 5) (len 7))))) (part-def (name "P") (modifiers) (body brace (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r0)) (declaration (name "retained") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r1))) (declaration-name "keyword") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (malformed (code "missing_semicolon") (found "#q broken") (span (offset 122) (line 10) (column 5) (len 10))))))
)
~~~
