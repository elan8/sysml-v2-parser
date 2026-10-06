# META
~~~sexpr
(snapshot (type semantic) (description "KerML TypeBodyElement reaches NonFeatureMember -> MemberElement -> NonFeatureElement -> Multiplicity, so a classifier body owns `multiplicity` members in their bounded MultiplicityRange form, bare or bodied and with a MemberPrefix, exactly as a package body does, instead of shredding them into result expressions."))
~~~
# SOURCE
~~~sysml
package Multiplicities {
    classifier One[1];
    classifier Two[1] {
        multiplicity extra [2];
        private multiplicity bounded [0..*];
        multiplicity zeroOrOne [0..1] {
            doc /* bodied */
        }
    }
    multiplicity packageLevel [1];
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "type_body_multiplicity_member.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Multiplicities {
    classifier One[1];
    classifier Two[1] {
        multiplicity extra[2];
        private multiplicity bounded[0..*];
        multiplicity zeroOrOne[0..1] {
            doc
            /* bodied */
        }
    }
    multiplicity packageLevel[1];
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "Multiplicities") (body brace (kerml-classifier (keyword classifier) (abstract false) (name "One") (multiplicity (lower none) (upper (expression (span (offset 44) (line 2) (column 20) (len 1)) (integer 1)))) (specializes none) (conjugates none) (body semicolon)) (kerml-classifier (keyword classifier) (abstract false) (name "Two") (multiplicity (lower none) (upper (expression (span (offset 67) (line 3) (column 20) (len 1)) (integer 1)))) (specializes none) (conjugates none) (body brace (kerml-classifier (keyword multiplicity) (abstract false) (name "extra") (multiplicity (lower none) (upper (expression (span (offset 100) (line 4) (column 29) (len 1)) (integer 2)))) (specializes none) (conjugates none) (body semicolon)) (kerml-classifier (keyword multiplicity) (abstract false) (name "bounded") (multiplicity (lower (expression (span (offset 142) (line 5) (column 39) (len 1)) (integer 0))) (upper (expression (span (offset 145) (line 5) (column 42) (len 1)) (infinity)))) (specializes none) (conjugates none) (body semicolon)) (kerml-classifier (keyword multiplicity) (abstract false) (name "zeroOrOne") (multiplicity (lower (expression (span (offset 181) (line 6) (column 33) (len 1)) (integer 0))) (upper (expression (span (offset 184) (line 6) (column 36) (len 1)) (integer 1)))) (specializes none) (conjugates none) (body brace (doc (name none) (locale none) (body (span (offset 207) (line 7) (column 19) (len 8)) (normalized "bodied "))))))) (kerml-classifier (keyword multiplicity) (abstract false) (name "packageLevel") (multiplicity (lower none) (upper (expression (span (offset 265) (line 10) (column 32) (len 1)) (integer 1)))) (specializes none) (conjugates none) (body semicolon)))))
)
~~~
