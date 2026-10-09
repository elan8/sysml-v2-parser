# META
~~~sexpr
(snapshot (type semantic) (description "KerML successions in a type body with every SuccessionDeclaration shape: ends only, the `first` keyword with no declaration, a named declaration, and a declaration of specializations alone, with a multiplicity directly after `then` (#163). Documentation whose Identification is a short name alone keeps its own body (#164)."))
~~~
# SOURCE
~~~sysml
package Sequences {
    class Finishing {
        doc <a> /* Documentation comment on Finishing */
        doc named /* Named documentation */
        doc <s> both /* Short and declared name */
        doc /* Plain documentation */
        feature paint;
        feature dry;
        feature ship;
        succession paint then dry;
        succession first paint then dry;
        succession ordered first [0..1] paint then [1] dry;
        succession redefines p_before_d : Before [1] first paint then dry;
        succession first [1] dry then[0..1] ship;
        private succession all [*] paint then [*] ship;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "succession_declarations_and_named_documentation.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Sequences {
    class Finishing {
        doc <a>
        /* Documentation comment on Finishing */
        doc named
        /* Named documentation */
        doc <s> both
        /* Short and declared name */
        doc
        /* Plain documentation */
        feature paint;
        feature dry;
        feature ship;
        succession paint then dry;
        succession first paint then dry;
        succession ordered first [0..1] paint then [1] dry;
        succession redefines p_before_d : Before[1] first paint then dry;
        succession first [1] dry then [0..1] ship;
        private succession all [*] paint then [*] ship;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "Sequences") (body brace (kerml-classifier (keyword class) (abstract false) (name "Finishing") (multiplicity none) (specializes none) (conjugates none) (body brace (doc (name none) (locale none) (body (span (offset 60) (line 3) (column 19) (len 36)) (normalized "Documentation comment on Finishing "))) (doc (name "named") (locale none) (body (span (offset 119) (line 4) (column 21) (len 21)) (normalized "Named documentation "))) (doc (name "both") (locale none) (body (span (offset 166) (line 5) (column 24) (len 25)) (normalized "Short and declared name "))) (doc (name none) (locale none) (body (span (offset 208) (line 6) (column 15) (len 21)) (normalized "Plain documentation "))) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "paint") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "dry") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "ship") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (succession) (succession) (succession) (succession) (succession) (succession))))))
)
~~~
