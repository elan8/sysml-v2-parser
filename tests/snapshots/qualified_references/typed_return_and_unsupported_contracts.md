# META
~~~sexpr
(snapshot (type recovery) (description "Structured return-reference bodies and portion kinds retain semantics, while unimplemented connection-like members are explicitly unsupported and failed return parsing cannot publish references."))
~~~
# SOURCE
~~~sysml
package TypedContracts {
    datatype DeferredType;
    feature deferredFeature : DeferredType;
    class DeferredClass;
    multiplicity exactlyOne [1..1];
    interaction DeferredInteraction;
    predicate deferredPredicate;

    verification def Verify {
        return ref result[0..*] {
            doc /* Result documentation. */
            return Results::accepted.item;
        }
        return ref empty;
    }

    part def Timeline {
        snapshot initial;
        then timeslice later;
        ref connection unresolved { } :> Ghost::subset;
    }

    interface def BrokenInterface {
        @@@ bogus ###;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "typed_return_and_unsupported_contracts.md"
    (diagnostics
      (diagnostic (code "unsupported_grammar_form") (severity warning) (category unsupportedgrammarform) (span (offset 29) (line 2) (column 5) (len 22)) (message "the spec-valid KerML semantic declaration production is retained but not structurally implemented"))
      (diagnostic (code "unsupported_grammar_form") (severity warning) (category unsupportedgrammarform) (span (offset 56) (line 3) (column 5) (len 39)) (message "the spec-valid KerML feature declaration production is retained but not structurally implemented"))
      (diagnostic (code "unsupported_grammar_form") (severity warning) (category unsupportedgrammarform) (span (offset 100) (line 4) (column 5) (len 20)) (message "the spec-valid KerML classifier declaration production is retained but not structurally implemented"))
      (diagnostic (code "unsupported_grammar_form") (severity warning) (category unsupportedgrammarform) (span (offset 125) (line 5) (column 5) (len 31)) (message "the spec-valid KerML semantic declaration production is retained but not structurally implemented"))
      (diagnostic (code "unsupported_grammar_form") (severity warning) (category unsupportedgrammarform) (span (offset 161) (line 6) (column 5) (len 32)) (message "the spec-valid KerML semantic declaration production is retained but not structurally implemented"))
      (diagnostic (code "unsupported_grammar_form") (severity warning) (category unsupportedgrammarform) (span (offset 198) (line 7) (column 5) (len 28)) (message "the spec-valid KerML feature form production is retained but not structurally implemented"))
      (diagnostic (code "unsupported_grammar_form") (severity error) (category unsupportedgrammarform) (span (offset 510) (line 20) (column 9) (len 47)) (message "spec-valid connection-like member is not implemented in part definitions"))
      (diagnostic (code "unsupported_annotation_syntax") (severity warning) (category unsupportedgrammarform) (span (offset 609) (line 24) (column 9) (len 19)) (message "incomplete parser support for annotation syntax in interface definition body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package TypedContracts {
    datatype DeferredType;
    feature deferredFeature : DeferredType;
    class DeferredClass;
    multiplicity exactlyOne [1..1];
    interaction DeferredInteraction;
    predicate deferredPredicate;
    verification def Verify {
        return ref result[0..*] {
            doc
            /* Result documentation. */
            return Results::accepted.item;
        }
        return ref empty;
    }
    part def Timeline {
        snapshot initial;
        then timeslice later;
        ref connection unresolved { } :> Ghost::subset;
    }
    interface def BrokenInterface {
        @@@ bogus ###;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 355) (line 12) (column 20) (len 17)) (segments (segment 0 (token "Results") (name "Results") (separator none) (span (offset 355) (line 12) (column 20) (len 7))) (segment 1 (token "accepted") (name "accepted") (separator colon-colon) (span (offset 364) (line 12) (column 29) (len 8)))))
    (reference r1 (scope relative) (span (offset 373) (line 12) (column 38) (len 4)) (segments (segment 0 (token "item") (name "item") (separator none) (span (offset 373) (line 12) (column 38) (len 4)))))
  )
  (root (package (name "TypedContracts") (body (kerml-semantic-declaration) (feature-declaration) (classifier-declaration) (kerml-semantic-declaration) (kerml-semantic-declaration) (kerml-feature-declaration) (verification-case-def (name "Verify") (body (return-ref (name "result") (body-span (span (offset 290) (line 10) (column 33) (len 98))) (body (doc) (result (expression (span (offset 355) (line 12) (column 20) (len 22)) (member-access (base (expression (span (offset 355) (line 12) (column 20) (len 17)) (ref r0))) (separator dot) (member (ref r1))))))) (return-ref (name "empty") (body-span (span (offset 413) (line 14) (column 25) (len 1))) (body semicolon)))) (part-def (name "Timeline") (body (occurrence (portion snapshot) (declaration "initial") (target none)) (occurrence (portion timeslice) (declaration "later") (target none)) (unsupported (production reference-connection-usage) (code "unsupported_grammar_form") (found none) (span (offset 510) (line 20) (column 9) (len 47))))) (interface-def (name "BrokenInterface") (specializes none) (body (malformed (code "unsupported_annotation_syntax") (found "@@@ bogus ###;") (span (offset 609) (line 24) (column 9) (len 19))))))))
)
~~~
