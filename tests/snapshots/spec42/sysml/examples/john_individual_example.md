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
      (diagnostic (code "recovered_occurrence_body_element") (severity error) (category parseerror) (span (offset 1170) (line 50) (column 6) (len 258)) (message "unexpected token in occurrence body"))
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
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "JohnIndividualExample") (body brace (item-def) (item-def) (item-def) (item-def) (occurrence (portion none) (declaration "UnitedStatesWithJohnAsPresident") (short-name none) (target none)))))
)
~~~
