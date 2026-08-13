# META
~~~sexpr
(snapshot (type recovery) (description "Structured return-reference bodies, portion kinds, KerML feature/class declarations, and reference connection usages retain semantics, while unimplemented KerML semantic/feature-form declarations and malformed annotation syntax are explicitly unsupported/malformed."))
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
    class def DeferredClass;
    multiplicity exactlyOne [1];
    interaction DeferredInteraction;
    predicate deferredPredicate;
    verification def Verify {
        return ref result[0..*] {
            doc
            /* Result documentation. */
            return Results::accepted.'item';
        }
        return ref empty;
    }
    part def Timeline {
        snapshot initial;
        then timeslice later;
        ref connection unresolved :> Ghost::subset {
        }
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
  (root (package (name "TypedContracts") (body (kerml-bare-declaration (keyword "datatype") (name "DeferredType") (multiplicity none)) (default-reference-usage) (class-def) (kerml-bare-declaration (keyword "multiplicity") (name "exactlyOne") (multiplicity (lower (expression (span (offset 150) (line 5) (column 30) (len 1)) (integer 1))) (upper (expression (span (offset 153) (line 5) (column 33) (len 1)) (integer 1))))) (kerml-bare-declaration (keyword "interaction") (name "DeferredInteraction") (multiplicity none)) (kerml-bare-declaration (keyword "predicate") (name "deferredPredicate") (multiplicity none)) (verification-case-def (name "Verify") (body (return-ref (name "result") (body-span (span (offset 290) (line 10) (column 33) (len 98))) (body (doc) (result (expression (span (offset 355) (line 12) (column 20) (len 22)) (member-access (base (expression (span (offset 355) (line 12) (column 20) (len 17)) (ref r0))) (separator dot) (member (ref r1))))))) (return-ref (name "empty") (body-span (span (offset 413) (line 14) (column 25) (len 1))) (body semicolon)))) (part-def (name "Timeline") (body (occurrence (portion snapshot) (declaration "initial") (target none)) (occurrence (portion timeslice) (declaration "later") (target none)) (connection))) (interface-def (name "BrokenInterface") (specializes none) (body (malformed (code "unsupported_annotation_syntax") (found "@@@ bogus ###;") (span (offset 609) (line 24) (column 9) (len 19))))))))
)
~~~
