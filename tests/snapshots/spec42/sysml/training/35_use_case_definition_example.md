# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 35 (Use Cases): Use Case Definition Example"))
~~~
# SOURCE
~~~sysml
package 'Use Case Definition Example' {
	
	part def Vehicle;
	part def Person;
	part def Environment;
	part def 'Fuel Station';
	
	use case def 'Provide Transportation' {
		subject vehicle : Vehicle;
		
		actor driver : Person;
		actor passengers : Person[0..4];
		actor environment : Environment;
		
		objective {
			doc 
			/* Transport driver and passengers from starting location 
			 * to ending location.
			 */
		}		
	}
	
	use case def 'Enter Vehicle' {
		subject vehicle : Vehicle;
		actor driver : Person;
		actor passengers : Person[0..4];
	}
	
	use case def 'Exit Vehicle' {
		subject vehicle : Vehicle;
		actor driver : Person;
		actor passengers : Person[0..4];
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "35_use_case_definition_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Use Case Definition Example' {
    part def Vehicle;
    part def Person;
    part def Environment;
    part def 'Fuel Station';
    use case def 'Provide Transportation' {
        subject vehicle : Vehicle;
        actor driver : Person;
        actor passengers : Person[0..4];
        actor environment : Environment;
        objective  {
            doc
            /* Transport driver and passengers from starting location 
			 * to ending location.
			 */
        }
    }
    use case def 'Enter Vehicle' {
        subject vehicle : Vehicle;
        actor driver : Person;
        actor passengers : Person[0..4];
    }
    use case def 'Exit Vehicle' {
        subject vehicle : Vehicle;
        actor driver : Person;
        actor passengers : Person[0..4];
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 220) (line 11) (column 18) (len 6)) (segments (segment 0 (token "Person") (name "Person") (separator none) (span (offset 220) (line 11) (column 18) (len 6)))))
    (reference r1 (scope relative) (span (offset 249) (line 12) (column 22) (len 6)) (segments (segment 0 (token "Person") (name "Person") (separator none) (span (offset 249) (line 12) (column 22) (len 6)))))
    (reference r2 (scope relative) (span (offset 285) (line 13) (column 23) (len 11)) (segments (segment 0 (token "Environment") (name "Environment") (separator none) (span (offset 285) (line 13) (column 23) (len 11)))))
    (reference r3 (scope relative) (span (offset 507) (line 25) (column 18) (len 6)) (segments (segment 0 (token "Person") (name "Person") (separator none) (span (offset 507) (line 25) (column 18) (len 6)))))
    (reference r4 (scope relative) (span (offset 536) (line 26) (column 22) (len 6)) (segments (segment 0 (token "Person") (name "Person") (separator none) (span (offset 536) (line 26) (column 22) (len 6)))))
    (reference r5 (scope relative) (span (offset 632) (line 31) (column 18) (len 6)) (segments (segment 0 (token "Person") (name "Person") (separator none) (span (offset 632) (line 31) (column 18) (len 6)))))
    (reference r6 (scope relative) (span (offset 661) (line 32) (column 22) (len 6)) (segments (segment 0 (token "Person") (name "Person") (separator none) (span (offset 661) (line 32) (column 22) (len 6)))))
  )
  (root (package (name "Use Case Definition Example") (body brace (part-def (name "Vehicle") (modifiers) (body semicolon)) (part-def (name "Person") (modifiers) (body semicolon)) (part-def (name "Environment") (modifiers) (body semicolon)) (part-def (name "Fuel Station") (modifiers) (body semicolon)) (use-case-def (name "Provide Transportation") (modifiers) (body brace (subject) (actor (name "driver") (short-name none) (type (ref r0)) (multiplicity none)) (actor (name "passengers") (short-name none) (type (ref r1)) (multiplicity (lower (expression (span (offset 256) (line 12) (column 29) (len 1)) (integer 0))) (upper (expression (span (offset 259) (line 12) (column 32) (len 1)) (integer 4))))) (actor (name "environment") (short-name none) (type (ref r2)) (multiplicity none)) (objective))) (use-case-def (name "Enter Vehicle") (modifiers) (body brace (subject) (actor (name "driver") (short-name none) (type (ref r3)) (multiplicity none)) (actor (name "passengers") (short-name none) (type (ref r4)) (multiplicity (lower (expression (span (offset 543) (line 26) (column 29) (len 1)) (integer 0))) (upper (expression (span (offset 546) (line 26) (column 32) (len 1)) (integer 4))))))) (use-case-def (name "Exit Vehicle") (modifiers) (body brace (subject) (actor (name "driver") (short-name none) (type (ref r5)) (multiplicity none)) (actor (name "passengers") (short-name none) (type (ref r6)) (multiplicity (lower (expression (span (offset 668) (line 32) (column 29) (len 1)) (integer 0))) (upper (expression (span (offset 671) (line 32) (column 32) (len 1)) (integer 4))))))))))
)
~~~
