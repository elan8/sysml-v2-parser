# META
~~~sexpr
(snapshot (type semantic) (description "KerML Simple Tests: Types"))
~~~
# SOURCE
~~~sysml
package Types {
	abstract type A specializes Base::Anything;
	type all x specializes A, Base::things;
	
	// This Type has exactly one instance.
	type Singleton[1] specializes Base::Anything;
	
	type Super specializes Base::Anything {
	    private package P {
	        type Sub specializes Super;
	    }
	    protected feature f : P::Sub;
	}
	
	type B :> Base::Anything;
	
	specialization Gen subtype A specializes B;
	specialization subtype x :> Base::things;
	
	type Original specializes Base::Anything {
	    in feature Input; 
	}
	type Conjugate1 specializes Base::Anything;
	type Conjugate2 specializes Base::Anything;
	conjugation c1 conjugate Conjugate1 conjugates Original; 
	conjugation c2 conjugate Conjugate2 ~ Original; 
	
	type Conjugate3 conjugates Original;
	type Conjugate4 ~ Conjugate1;
	
	type C :> B disjoint from A;
	
	type D :> Base::Anything unions A, B;
	type E :> Base::Anything intersects A, B;
	type F :> Base::Anything differences A, B;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "types.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 624) (line 25) (column 2) (len 339)) (message "unrecognized declaration `conjugation` in package body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package Types {
    abstract type A specializes Base::Anything;
    type all x specializes A, Base::things;
    type Singleton[1] specializes Base::Anything;
    type Super specializes Base::Anything {
        private package P {
            type Sub specializes Super;
        }
        protected feature f : P::Sub;
    }
    type B :> Base::Anything;
    specialization Gen subtype A specializes B;
    specialization subtype x specializes Base::things;
    type Original specializes Base::Anything {
        in feature Input;
    }
    type Conjugate1 specializes Base::Anything;
    type Conjugate2 specializes Base::Anything;
    conjugation c1 conjugate Conjugate1 conjugates Original; 
	conjugation c2 conjugate Conjugate2 ~ Original; 
	
	type Conjugate3 conjugates Original;
	type Conjugate4 ~ Conjugate1;
	
	type C :> B disjoint from A;
	
	type D :> Base::Anything unions A, B;
	type E :> Base::Anything intersects A, B;
	type F :> Base::Anything differences A, B;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 45) (line 2) (column 30) (len 14)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 45) (line 2) (column 30) (len 4))) (segment 1 (token "Anything") (name "Anything") (separator colon-colon) (span (offset 51) (line 2) (column 36) (len 8)))))
    (reference r1 (scope relative) (span (offset 85) (line 3) (column 25) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 85) (line 3) (column 25) (len 1)))))
    (reference r2 (scope relative) (span (offset 88) (line 3) (column 28) (len 12)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 88) (line 3) (column 28) (len 4))) (segment 1 (token "things") (name "things") (separator colon-colon) (span (offset 94) (line 3) (column 34) (len 6)))))
    (reference r3 (scope relative) (span (offset 175) (line 6) (column 32) (len 14)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 175) (line 6) (column 32) (len 4))) (segment 1 (token "Anything") (name "Anything") (separator colon-colon) (span (offset 181) (line 6) (column 38) (len 8)))))
    (reference r4 (scope relative) (span (offset 217) (line 8) (column 25) (len 14)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 217) (line 8) (column 25) (len 4))) (segment 1 (token "Anything") (name "Anything") (separator colon-colon) (span (offset 223) (line 8) (column 31) (len 8)))))
    (reference r5 (scope relative) (span (offset 289) (line 10) (column 31) (len 5)) (segments (segment 0 (token "Super") (name "Super") (separator none) (span (offset 289) (line 10) (column 31) (len 5)))))
    (reference r6 (scope relative) (span (offset 330) (line 12) (column 28) (len 6)) (segments (segment 0 (token "P") (name "P") (separator none) (span (offset 330) (line 12) (column 28) (len 1))) (segment 1 (token "Sub") (name "Sub") (separator colon-colon) (span (offset 333) (line 12) (column 31) (len 3)))))
    (reference r7 (scope relative) (span (offset 354) (line 15) (column 12) (len 14)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 354) (line 15) (column 12) (len 4))) (segment 1 (token "Anything") (name "Anything") (separator colon-colon) (span (offset 360) (line 15) (column 18) (len 8)))))
    (reference r8 (scope relative) (span (offset 400) (line 17) (column 29) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 400) (line 17) (column 29) (len 1)))))
    (reference r9 (scope relative) (span (offset 414) (line 17) (column 43) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 414) (line 17) (column 43) (len 1)))))
    (reference r10 (scope relative) (span (offset 441) (line 18) (column 25) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 441) (line 18) (column 25) (len 1)))))
    (reference r11 (scope relative) (span (offset 446) (line 18) (column 30) (len 12)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 446) (line 18) (column 30) (len 4))) (segment 1 (token "things") (name "things") (separator colon-colon) (span (offset 452) (line 18) (column 36) (len 6)))))
    (reference r12 (scope relative) (span (offset 489) (line 20) (column 28) (len 14)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 489) (line 20) (column 28) (len 4))) (segment 1 (token "Anything") (name "Anything") (separator colon-colon) (span (offset 495) (line 20) (column 34) (len 8)))))
    (reference r13 (scope relative) (span (offset 562) (line 23) (column 30) (len 14)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 562) (line 23) (column 30) (len 4))) (segment 1 (token "Anything") (name "Anything") (separator colon-colon) (span (offset 568) (line 23) (column 36) (len 8)))))
    (reference r14 (scope relative) (span (offset 607) (line 24) (column 30) (len 14)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 607) (line 24) (column 30) (len 4))) (segment 1 (token "Anything") (name "Anything") (separator colon-colon) (span (offset 613) (line 24) (column 36) (len 8)))))
  )
  (root (package (name "Types") (body brace (kerml-classifier (keyword type) (abstract true) (name "A") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r0)))) (body semicolon)) (kerml-classifier (keyword type) (abstract false) (name "x") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r1) (ref r2)))) (body semicolon)) (kerml-classifier (keyword type) (abstract false) (name "Singleton") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r3)))) (body semicolon)) (kerml-classifier (keyword type) (abstract false) (name "Super") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r4)))) (body brace (package-member (visibility private) (package (name "P") (body brace (kerml-classifier (keyword type) (abstract false) (name "Sub") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r5)))) (body semicolon))))) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "f") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)))) (kerml-classifier (keyword type) (abstract false) (name "B") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r7)))) (body semicolon)) (kerml-relationship (keyword subtype) (declaration-keyword true) (source (ref r8)) (target (ref r9))) (kerml-relationship (keyword subtype) (declaration-keyword true) (source (ref r10)) (target (ref r11))) (kerml-classifier (keyword type) (abstract false) (name "Original") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r12)))) (body brace (kerml-feature (prefix (head basic) (direction in) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "Input") (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)))) (kerml-classifier (keyword type) (abstract false) (name "Conjugate1") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r13)))) (body semicolon)) (kerml-classifier (keyword type) (abstract false) (name "Conjugate2") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r14)))) (body semicolon)) (malformed (code "unrecognized_declaration_in_scope") (found "conjugation c1 conjugate Conjugate1 conjugates Original;") (span (offset 624) (line 25) (column 2) (len 339))))))
)
~~~
