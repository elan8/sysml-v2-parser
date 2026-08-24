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
    conjugation c2 conjugate Conjugate2 conjugates Original;
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
    (reference r15 (scope relative) (span (offset 649) (line 25) (column 27) (len 10)) (segments (segment 0 (token "Conjugate1") (name "Conjugate1") (separator none) (span (offset 649) (line 25) (column 27) (len 10)))))
    (reference r16 (scope relative) (span (offset 671) (line 25) (column 49) (len 8)) (segments (segment 0 (token "Original") (name "Original") (separator none) (span (offset 671) (line 25) (column 49) (len 8)))))
    (reference r17 (scope relative) (span (offset 708) (line 26) (column 27) (len 10)) (segments (segment 0 (token "Conjugate2") (name "Conjugate2") (separator none) (span (offset 708) (line 26) (column 27) (len 10)))))
    (reference r18 (scope relative) (span (offset 721) (line 26) (column 40) (len 8)) (segments (segment 0 (token "Original") (name "Original") (separator none) (span (offset 721) (line 26) (column 40) (len 8)))))
    (reference r19 (scope relative) (span (offset 762) (line 28) (column 29) (len 8)) (segments (segment 0 (token "Original") (name "Original") (separator none) (span (offset 762) (line 28) (column 29) (len 8)))))
    (reference r20 (scope relative) (span (offset 791) (line 29) (column 20) (len 10)) (segments (segment 0 (token "Conjugate1") (name "Conjugate1") (separator none) (span (offset 791) (line 29) (column 20) (len 10)))))
    (reference r21 (scope relative) (span (offset 816) (line 31) (column 12) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 816) (line 31) (column 12) (len 1)))))
    (reference r22 (scope relative) (span (offset 848) (line 33) (column 12) (len 14)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 848) (line 33) (column 12) (len 4))) (segment 1 (token "Anything") (name "Anything") (separator colon-colon) (span (offset 854) (line 33) (column 18) (len 8)))))
    (reference r23 (scope relative) (span (offset 887) (line 34) (column 12) (len 14)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 887) (line 34) (column 12) (len 4))) (segment 1 (token "Anything") (name "Anything") (separator colon-colon) (span (offset 893) (line 34) (column 18) (len 8)))))
    (reference r24 (scope relative) (span (offset 930) (line 35) (column 12) (len 14)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 930) (line 35) (column 12) (len 4))) (segment 1 (token "Anything") (name "Anything") (separator colon-colon) (span (offset 936) (line 35) (column 18) (len 8)))))
  )
  (root (package (name "Types") (body brace (kerml-classifier (keyword type) (abstract true) (name "A") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r0)))) (conjugates none) (body semicolon)) (kerml-classifier (keyword type) (abstract false) (name "x") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r1) (ref r2)))) (conjugates none) (body semicolon)) (kerml-classifier (keyword type) (abstract false) (name "Singleton") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r3)))) (conjugates none) (body semicolon)) (kerml-classifier (keyword type) (abstract false) (name "Super") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r4)))) (conjugates none) (body brace (package-member (visibility private) (package (name "P") (body brace (kerml-classifier (keyword type) (abstract false) (name "Sub") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r5)))) (conjugates none) (body semicolon))))) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "f") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)))) (kerml-classifier (keyword type) (abstract false) (name "B") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r7)))) (conjugates none) (body semicolon)) (kerml-relationship (keyword subtype) (declaration-keyword true) (source (ref r8)) (target (ref r9))) (kerml-relationship (keyword subtype) (declaration-keyword true) (source (ref r10)) (target (ref r11))) (kerml-classifier (keyword type) (abstract false) (name "Original") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r12)))) (conjugates none) (body brace (kerml-feature (prefix (head basic) (direction in) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "Input") (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)))) (kerml-classifier (keyword type) (abstract false) (name "Conjugate1") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r13)))) (conjugates none) (body semicolon)) (kerml-classifier (keyword type) (abstract false) (name "Conjugate2") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r14)))) (conjugates none) (body semicolon)) (kerml-relationship (keyword conjugate) (declaration-keyword true) (source (ref r15)) (target (ref r16))) (kerml-relationship (keyword conjugate) (declaration-keyword true) (source (ref r17)) (target (ref r18))) (kerml-classifier (keyword type) (abstract false) (name "Conjugate3") (specializes none) (conjugates (keyword (ref r19))) (body semicolon)) (kerml-classifier (keyword type) (abstract false) (name "Conjugate4") (specializes none) (conjugates (operator (ref r20))) (body semicolon)) (kerml-classifier (keyword type) (abstract false) (name "C") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r21)))) (conjugates none) (body semicolon)) (kerml-classifier (keyword type) (abstract false) (name "D") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r22)))) (conjugates none) (body semicolon)) (kerml-classifier (keyword type) (abstract false) (name "E") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r23)))) (conjugates none) (body semicolon)) (kerml-classifier (keyword type) (abstract false) (name "F") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r24)))) (conjugates none) (body semicolon)))))
)
~~~
