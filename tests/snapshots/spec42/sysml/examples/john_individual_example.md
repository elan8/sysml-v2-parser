# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Individuals): JohnIndividualExample"))
~~~
# SOURCE
~~~sysml
package JohnIndividualExample {
	
	item def Person {
		doc	
		/*
		 * This is the definition of the class of persons, each of whom has an age.
		 */

		attribute age : ScalarValues::Natural;		
		
		timeslice asPresident : Person [0..*] {
			doc
			/*
			 * These are the periods during which a Person is president.
			 */
		}
	}
	
	individual item def John :> Person {
		doc
		/*
		 * This the definition of the individual Person who is John.
		 * There is at most one such person.
		 */
	}
	
	item def Country {
		doc
		/*
		 * This is the definition of the class of countries, each of which may have 
		 * at most one president (at any point in time).
		 */
		ref presidentOfCountry[0..1] : Person :> presidentOfCountry.asPresident;
	}
	
	individual item def UnitedStates :> Country {
		doc
		/*
		 * This is the definition of the individual country that is the
		 * United States. It contains a single instance. The United States
		 * always has a president who must be at least 35 years old.
		 */
		 
		ref presidentOfUS[1] :>> presidentOfCountry {
	   		assert constraint { age >= 35 } 
	  	}
	}
	
	individual UnitedStatesWithJohnAsPresident : UnitedStates {
    	timeslice item UnitedStatesWhenJohnIsPresident[*] : UnitedStates {
    		doc
    		/*
    		 * These are the time slices of the United States during
    		 * which John is president of the United States.
    		 */
    		ref :>> presidentOfUS : John;
    	}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "john_individual_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package JohnIndividualExample {
    item def Person {
        doc
        /*
		 * This is the definition of the class of persons, each of whom has an age.
		 */
        attribute age : ScalarValues::Natural;
        timeslice asPresident : Person[0..*] {
            doc
            /*
			 * These are the periods during which a Person is president.
			 */
        }
    }
    individual item def John :> Person {
        doc
        /*
		 * This the definition of the individual Person who is John.
		 * There is at most one such person.
		 */
    }
    item def Country {
        doc
        /*
		 * This is the definition of the class of countries, each of which may have 
		 * at most one president (at any point in time).
		 */
        ref presidentOfCountry : Person[0..1] :> presidentOfCountry.asPresident;
    }
    individual item def UnitedStates :> Country {
        doc
        /*
		 * This is the definition of the individual country that is the
		 * United States. It contains a single instance. The United States
		 * always has a president who must be at least 35 years old.
		 */
        ref presidentOfUS[1] :>> presidentOfCountry {
            assert constraint {
                age >= 35;
            }
        }
    }
    individual UnitedStatesWithJohnAsPresident : UnitedStates {
        timeslice item UnitedStatesWhenJohnIsPresident : UnitedStates[*] {
            doc
            /*
    		 * These are the time slices of the United States during
    		 * which John is president of the United States.
    		 */
            ref : John :>> presidentOfUS;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 168) (line 9) (column 19) (len 21)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 168) (line 9) (column 19) (len 12))) (segment 1 (token "Natural") (name "Natural") (separator colon-colon) (span (offset 182) (line 9) (column 33) (len 7)))))
    (reference r1 (scope relative) (span (offset 360) (line 19) (column 30) (len 6)) (segments (segment 0 (token "Person") (name "Person") (separator none) (span (offset 360) (line 19) (column 30) (len 6)))))
    (reference r2 (scope relative) (span (offset 693) (line 33) (column 34) (len 6)) (segments (segment 0 (token "Person") (name "Person") (separator none) (span (offset 693) (line 33) (column 34) (len 6)))))
    (reference r3 (scope relative) (span (offset 703) (line 33) (column 44) (len 30)) (segments (segment 0 (token "presidentOfCountry") (name "presidentOfCountry") (separator none) (span (offset 703) (line 33) (column 44) (len 18))) (segment 1 (token "asPresident") (name "asPresident") (separator dot) (span (offset 722) (line 33) (column 63) (len 11)))))
    (reference r4 (scope relative) (span (offset 777) (line 36) (column 38) (len 7)) (segments (segment 0 (token "Country") (name "Country") (separator none) (span (offset 777) (line 36) (column 38) (len 7)))))
    (reference r5 (scope relative) (span (offset 1033) (line 44) (column 28) (len 18)) (segments (segment 0 (token "presidentOfCountry") (name "presidentOfCountry") (separator none) (span (offset 1033) (line 44) (column 28) (len 18)))))
    (reference r6 (scope relative) (span (offset 1222) (line 50) (column 58) (len 12)) (segments (segment 0 (token "UnitedStates") (name "UnitedStates") (separator none) (span (offset 1222) (line 50) (column 58) (len 12)))))
    (reference r7 (scope relative) (span (offset 1414) (line 56) (column 31) (len 4)) (segments (segment 0 (token "John") (name "John") (separator none) (span (offset 1414) (line 56) (column 31) (len 4)))))
    (reference r8 (scope relative) (span (offset 1398) (line 56) (column 15) (len 13)) (segments (segment 0 (token "presidentOfUS") (name "presidentOfUS") (separator none) (span (offset 1398) (line 56) (column 15) (len 13)))))
  )
  (root (package (name "JohnIndividualExample") (body brace (item-def (name "Person") (modifiers) (individual false) (specializes none) (body brace (doc (name none) (locale none) (body (span (offset 64) (line 5) (column 5) (len 82)) (normalized "This is the definition of the class of persons, each of whom has an age.\n"))) (attribute-usage (declaration-name "age") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion timeslice) (extensions)) (declaration "asPresident") (short-name none) (target none) (body brace (doc (name none) (locale none) (body (span (offset 250) (line 13) (column 6) (len 69)) (normalized "These are the periods during which a Person is president.\n"))))))) (item-def (name "John") (modifiers) (individual true) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r1)))) (body brace (doc (name none) (locale none) (body (span (offset 379) (line 21) (column 5) (len 106)) (normalized "This the definition of the individual Person who is John.\nThere is at most one such person.\n"))))) (item-def (name "Country") (modifiers) (individual false) (specializes none) (body brace (doc (name none) (locale none) (body (span (offset 523) (line 29) (column 5) (len 134)) (normalized "This is the definition of the class of countries, each of which may have \nat most one president (at any point in time).\n"))) (ref (name "presidentOfCountry") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity (lower (expression (span (offset 685) (line 33) (column 26) (len 1)) (integer 0))) (upper (expression (span (offset 688) (line 33) (column 29) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets (relationship (kind subsets) (implied false) (targets (ref r3)))) (body semicolon)))) (item-def (name "UnitedStates") (modifiers) (individual true) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r4)))) (body brace (doc (name none) (locale none) (body (span (offset 797) (line 38) (column 5) (len 202)) (normalized "This is the definition of the individual country that is the\nUnited States. It contains a single instance. The United States\nalways has a president who must be at least 35 years old.\n"))) (ref (name "presidentOfUS") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing none) (multiplicity (lower (expression (span (offset 1026) (line 44) (column 21) (len 1)) (integer 1))) (upper (expression (span (offset 1026) (line 44) (column 21) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines (relationship (kind redefines) (implied false) (targets (ref r5)))) (subsets none) (body brace (assert-constraint))))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion none) (extensions)) (declaration "UnitedStatesWithJohnAsPresident") (short-name none) (target none) (body brace (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion timeslice) (extensions)) (declaration "UnitedStatesWhenJohnIsPresident") (short-name none) (type (ref r6)) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (doc (name none) (locale none) (body (span (offset 1255) (line 52) (column 9) (len 126)) (normalized "These are the time slices of the United States during\nwhich John is president of the United States.\n"))) (ref (name none) (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines (relationship (kind redefines) (implied false) (targets (ref r8)))) (subsets none) (body semicolon)))))))))
)
~~~
