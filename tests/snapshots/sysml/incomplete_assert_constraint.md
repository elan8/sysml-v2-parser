# META
~~~sexpr
(snapshot (type recovery) (description "An assert without a constraint keyword must name a referenced constraint. The incomplete member is diagnosed and the following valid assertions survive recovery."))
~~~
# SOURCE
~~~sysml
part def T {
    assert not ;
    assert not target;
    assert constraint { true }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "incomplete_assert_constraint.md"
    (diagnostics
      (diagnostic (code "recovered_part_def_body_element") (severity error) (category parseerror) (span (offset 17) (line 2) (column 5) (len 17)) (message "unexpected token in part definition body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
part def T {
    assert not ;
    assert not target;
    assert constraint {
        true
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
  )
  (root (part-def (name "T") (modifiers) (body brace (malformed (code "recovered_part_def_body_element") (found "assert not ;") (span (offset 17) (line 2) (column 5) (len 17))) (assert-constraint) (assert-constraint))))
)
~~~
