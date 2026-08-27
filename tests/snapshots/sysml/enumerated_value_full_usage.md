# META
~~~sexpr
(snapshot (type semantic) (description "An EnumerationUsageMember is MemberPrefix plus EnumeratedValue, and the pinned EnumeratedValue is optional `enum` followed by full Usage (SysML-textual-bnf.kebnf 528-535). This fixture retains its visibility, optional enum token, Identification, FeatureSpecializationPart relationships, multiplicity modifiers, value, and usage body without admitting Pilot-only UsageExtensionKeyword metadata prefixes."))
~~~
# SOURCE
~~~sysml
package EnumeratedValueFullUsage {
    enum def Level {
        private enum <lo> low : Level[0..1] ordered nonunique = 0;
        enum 'quoted';
        enum 'can\'t';
        protected medium : Level subsets low = 1;
        public high : Level redefines medium references low crosses medium intersects low = 2;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "enumerated_value_full_usage.md"
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
    (reference r0 (scope relative) (span (offset 88) (line 3) (column 33) (len 5)) (segments (segment 0 (token "Level") (name "Level") (separator none) (span (offset 88) (line 3) (column 33) (len 5)))))
    (reference r1 (scope relative) (span (offset 196) (line 6) (column 28) (len 5)) (segments (segment 0 (token "Level") (name "Level") (separator none) (span (offset 196) (line 6) (column 28) (len 5)))))
    (reference r2 (scope relative) (span (offset 210) (line 6) (column 42) (len 3)) (segments (segment 0 (token "low") (name "low") (separator none) (span (offset 210) (line 6) (column 42) (len 3)))))
    (reference r3 (scope relative) (span (offset 241) (line 7) (column 23) (len 5)) (segments (segment 0 (token "Level") (name "Level") (separator none) (span (offset 241) (line 7) (column 23) (len 5)))))
    (reference r4 (scope relative) (span (offset 257) (line 7) (column 39) (len 6)) (segments (segment 0 (token "medium") (name "medium") (separator none) (span (offset 257) (line 7) (column 39) (len 6)))))
    (reference r5 (scope relative) (span (offset 275) (line 7) (column 57) (len 3)) (segments (segment 0 (token "low") (name "low") (separator none) (span (offset 275) (line 7) (column 57) (len 3)))))
    (reference r6 (scope relative) (span (offset 287) (line 7) (column 69) (len 6)) (segments (segment 0 (token "medium") (name "medium") (separator none) (span (offset 287) (line 7) (column 69) (len 6)))))
    (reference r7 (scope relative) (span (offset 305) (line 7) (column 87) (len 3)) (segments (segment 0 (token "low") (name "low") (separator none) (span (offset 305) (line 7) (column 87) (len 3)))))
  )
  (root (package (name "EnumeratedValueFullUsage") (body brace (enum-def (name "Level") (body brace (enum-value (extensions) (enum-keyword present) (visibility private) (name "low") (short-name "lo") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity (lower (expression (span (offset 94) (line 3) (column 39) (len 1)) (integer 0))) (upper (expression (span (offset 97) (line 3) (column 42) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering ordered) (uniqueness nonunique)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 120) (line 3) (column 65) (len 1)) (integer 0))))) (body semicolon) (span (offset 64) (line 3) (column 9) (len 58))) (enum-value (extensions) (enum-keyword present) (visibility none) (name "quoted") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon) (span (offset 131) (line 4) (column 9) (len 14))) (enum-value (extensions) (enum-keyword present) (visibility none) (name "can't") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon) (span (offset 154) (line 5) (column 9) (len 14))) (enum-value (extensions) (enum-keyword none) (visibility protected) (name "medium") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r2))) (value (expression (span (offset 216) (line 6) (column 48) (len 1)) (integer 1)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon) (span (offset 177) (line 6) (column 9) (len 41))) (enum-value (extensions) (enum-keyword none) (visibility public) (name "high") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (references (relationship (kind references) (implied false) (targets (ref r5)))) (crosses (relationship (kind crosses) (implied false) (targets (ref r6)))) (intersects (relationship (kind intersects) (implied false) (targets (ref r7)))) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 311) (line 7) (column 93) (len 1)) (integer 2))))) (body semicolon) (span (offset 227) (line 7) (column 9) (len 86))))))))
)
~~~
