# META
~~~sexpr
(snapshot (type semantic) (description "The shared occurrence-style definition body dispatches its structured members before falling back to opaque capture. A visibility prefix therefore no longer hides the member behind it, and ref members reach the body at all; a form the parser does not model is still retained with a diagnostic, which is what the fallback is for. Written as an occurrence definition rather than the flow definition the library shapes come from, because flow definitions have no emitter yet. An occurrence definition body still projects as a bare marker, so the format section is what shows the structured members survived: (stable-idempotent) means the emitted text is byte-identical to the source, so a dropped visibility prefix or ref member would break it. The diagnostics section pins the fallback."))
~~~
# SOURCE
~~~sysml
package OccurrenceBodyMembers {
    occurrence def Message {
        private attribute seBeforeNum : Natural[1];
        protected attribute teAfterNum : Natural[1];
        private part p : T;
        ref self : SuccessionFlow :>> Flow::self;
        private ref action thisConnection = self;
        connection :>> c connect a to b;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "occurrence_body_members.md"
    (diagnostics
      (diagnostic (code "unsupported_grammar_form") (severity warning) (category unsupportedgrammarform) (span (offset 302) (line 8) (column 9) (len 32)) (message "this definition body member is spec-valid but not structurally implemented"))
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
  )
  (root (package (name "OccurrenceBodyMembers") (body (occurrence-def))))
)
~~~
