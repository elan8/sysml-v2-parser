# META
~~~sexpr
(snapshot (type semantic) (description "MultiplicityPart's ordering and uniqueness keyword slots: every spelling is retained with its authored span and reproduced in its authored order, so an authored default differs from omission; readonly and variable, which no pinned production spells, reach recovery without consuming the valid sibling that follows (spec42 Gap 52)."))
~~~
# SOURCE
~~~sysml
package MultiplicityModifiers {
    attribute omitted : Real[0..*];
    attribute isOrdered : Real[0..*] ordered;
    attribute isNonordered : Real[0..*] nonordered;
    attribute isNonunique : Real[0..*] nonunique;
    attribute isUnique : Real[0..*] unique;
    attribute orderedFirst : Real[0..*] ordered nonunique;
    attribute nonuniqueFirst : Real[0..*] nonunique ordered;
    attribute leadingSlots [0..*] ordered nonunique : Real;
    attribute notAModifier : Real[0..*] orderedBy;
    readonly attribute rejectedReadonly : Real;
    attribute afterReadonly : Real;
    variable attribute rejectedVariable : Real;
    attribute afterVariable : Real;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "multiplicity_modifier_slots.md"
    (diagnostics
      (diagnostic (code "unsupported_grammar_form") (severity warning) (category unsupportedgrammarform) (span (offset 444) (line 10) (column 5) (len 46)) (message "the spec-valid extended-library declaration production is retained but not structurally implemented"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 495) (line 11) (column 5) (len 48)) (message "unrecognized declaration `readonly` in package body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 579) (line 13) (column 5) (len 48)) (message "unrecognized declaration `variable` in package body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package MultiplicityModifiers {
    attribute omitted : Real[0..*];
    attribute isOrdered : Real[0..*] ordered;
    attribute isNonordered : Real[0..*] nonordered;
    attribute isNonunique : Real[0..*] nonunique;
    attribute isUnique : Real[0..*] unique;
    attribute orderedFirst : Real[0..*] ordered nonunique;
    attribute nonuniqueFirst : Real[0..*] nonunique ordered;
    attribute leadingSlots : Real[0..*] ordered nonunique;
    attribute notAModifier : Real[0..*] orderedBy;
    readonly attribute rejectedReadonly : Real;
    attribute afterReadonly : Real;
    variable attribute rejectedVariable : Real;
    attribute afterVariable : Real;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "MultiplicityModifiers") (body brace (attribute-usage) (attribute-usage) (attribute-usage) (attribute-usage) (attribute-usage) (attribute-usage) (attribute-usage) (attribute-usage) (extended-library-declaration) (malformed (code "unrecognized_declaration_in_scope") (found "readonly attribute rejectedReadonly : Real;") (span (offset 495) (line 11) (column 5) (len 48))) (attribute-usage) (malformed (code "unrecognized_declaration_in_scope") (found "variable attribute rejectedVariable : Real;") (span (offset 579) (line 13) (column 5) (len 48))) (attribute-usage))))
)
~~~
