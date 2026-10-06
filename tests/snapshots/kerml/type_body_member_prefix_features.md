# META
~~~sexpr
(snapshot (type semantic) (description "KerML TypeBody members take a MemberPrefix before the FeaturePrefix, so `protected in c : C;` is one directed feature member and not a `protected` result expression beside it."))
~~~
# SOURCE
~~~sysml
package MemberPrefixFeatures {
    classifier C;
    class K {
        protected in c : C;
        private out d : C;
        public inout e;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "type_body_member_prefix_features.md"
    (diagnostics
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
    (reference r0 (scope relative) (span (offset 88) (line 4) (column 26) (len 1)) (segments (segment 0 (token "C") (name "C") (separator none) (span (offset 88) (line 4) (column 26) (len 1)))))
    (reference r1 (scope relative) (span (offset 115) (line 5) (column 25) (len 1)) (segments (segment 0 (token "C") (name "C") (separator none) (span (offset 115) (line 5) (column 25) (len 1)))))
  )
  (root (package (name "MemberPrefixFeatures") (body brace (kerml-classifier (keyword classifier) (abstract false) (name "C") (multiplicity none) (specializes none) (conjugates none) (body semicolon)) (kerml-classifier (keyword class) (abstract false) (name "K") (multiplicity none) (specializes none) (conjugates none) (body brace (kerml-feature (prefix (head basic) (direction in) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind none) (member false) (all false) (name "c") (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction out) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind none) (member false) (all false) (name "d") (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction inout) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind none) (member false) (all false) (name "e") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)))))))
)
~~~
