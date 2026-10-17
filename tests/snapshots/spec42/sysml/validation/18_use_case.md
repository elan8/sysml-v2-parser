# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (18-Use Case): 18-Use Case"))
~~~
# SOURCE
~~~sysml
package '18-Use Case' {
	
	part def Vehicle;
	part def Person;
	part def Environment;
	part def 'Fuel Station';
	
	use case 'provide transportation' {
		subject vehicle : Vehicle;
		
		actor driver : Person;
		actor passengers : Person[0..4];
		actor environment : Environment;
		
		objective {
			doc 
			/* Satisfy mission requirements to transport driver and passengers 
			 * from starting location to ending location in conformance with 
			 * the driving profile and meet the mission requirements for safety, 
			 * reliability, comfort, and affordability.
			 */
		}
		
		ref :>> start {
			doc /* Mock-up of a pre-condition. */
			assert constraint {
				doc /* Vehicle at starting location */
			}
		}
		
		first start;
		
		then include 'enter vehicle' {
		    subject;
			actor :>> driver = 'provide transportation'::driver;
			actor :>> passengers = 'provide transportation'::passengers;
		}
		
		then use case 'drive vehicle' {
			include 'add fuel'[0..*] {
				doc
				/*
				 * Mock-up of an extension point.
				 * (But reference to 'add fuel' is in the wrong direction, and it doesn't
				 * make the extension condition sufficient to trigger the behavior.)
				 */
                subject;
				actor :>> fueler = driver;
				ref :>> start {
					doc /* Fuel level < 10% max fuel */
				}
			}
		}
		
		then include 'exit vehicle' {
		    subject;
			actor :>> driver = 'provide transportation'::driver;
			actor :>> passengers = 'provide transportation'::passengers;
		}
		
		then done;
		
		ref :>> done {
			doc /* Mock-up of a post-condition. */
			assert constraint {
				doc /* Vehicle at ending location */
			}
		}
		
	}
	
	use case 'enter vehicle' {
		subject vehicle : Vehicle;
		actor driver : Person;
		actor passengers : Person[0..4];
	}
	
	use case 'exit vehicle' {
		subject vehicle : Vehicle;
		actor driver : Person;
		actor passengers : Person[0..4];
	}
		
	use case 'add fuel' {
		subject vehicle : Vehicle;
		actor fueler : Person;
		actor 'fuel station' : 'Fuel Station';
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "18_use_case.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '18-Use Case' {
    part def Vehicle;
    part def Person;
    part def Environment;
    part def 'Fuel Station';
    use case 'provide transportation' {
        subject vehicle : Vehicle;
        actor driver : Person;
        actor passengers : Person[0..4];
        actor environment : Environment;
        objective {
            doc
            /* Satisfy mission requirements to transport driver and passengers 
			 * from starting location to ending location in conformance with 
			 * the driving profile and meet the mission requirements for safety, 
			 * reliability, comfort, and affordability.
			 */
        }
        ref :>> start {
            doc
            /* Mock-up of a pre-condition. */
            assert constraint {
                doc
                /* Vehicle at starting location */
            }
        }
        first start;
        then include 'enter vehicle' {
            subject;
            actor :>> driver = 'provide transportation'::driver;
            actor :>> passengers = 'provide transportation'::passengers;
        }
        then use case 'drive vehicle' {
            include 'add fuel'[0..*] {
                doc
                /*
				 * Mock-up of an extension point.
				 * (But reference to 'add fuel' is in the wrong direction, and it doesn't
				 * make the extension condition sufficient to trigger the behavior.)
				 */
                subject;
                actor :>> fueler = driver;
                ref :>> start {
                    doc
                    /* Fuel level < 10% max fuel */
                }
            }
        }
        then include 'exit vehicle' {
            subject;
            actor :>> driver = 'provide transportation'::driver;
            actor :>> passengers = 'provide transportation'::passengers;
        }
        then done;
        ref :>> done {
            doc
            /* Mock-up of a post-condition. */
            assert constraint {
                doc
                /* Vehicle at ending location */
            }
        }
    }
    use case 'enter vehicle' {
        subject vehicle : Vehicle;
        actor driver : Person;
        actor passengers : Person[0..4];
    }
    use case 'exit vehicle' {
        subject vehicle : Vehicle;
        actor driver : Person;
        actor passengers : Person[0..4];
    }
    use case 'add fuel' {
        subject vehicle : Vehicle;
        actor fueler : Person;
        actor 'fuel station' : 'Fuel Station';
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 171) (line 9) (column 21) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 171) (line 9) (column 21) (len 7)))))
    (reference r1 (scope relative) (span (offset 200) (line 11) (column 18) (len 6)) (segments (segment 0 (token "Person") (name "Person") (separator none) (span (offset 200) (line 11) (column 18) (len 6)))))
    (reference r2 (scope relative) (span (offset 229) (line 12) (column 22) (len 6)) (segments (segment 0 (token "Person") (name "Person") (separator none) (span (offset 229) (line 12) (column 22) (len 6)))))
    (reference r3 (scope relative) (span (offset 265) (line 13) (column 23) (len 11)) (segments (segment 0 (token "Environment") (name "Environment") (separator none) (span (offset 265) (line 13) (column 23) (len 11)))))
    (reference r4 (scope relative) (span (offset 587) (line 24) (column 11) (len 5)) (segments (segment 0 (token "start") (name "start") (separator none) (span (offset 587) (line 24) (column 11) (len 5)))))
    (reference r5 (scope relative) (span (offset 722) (line 31) (column 9) (len 5)) (segments (segment 0 (token "start") (name "start") (separator none) (span (offset 722) (line 31) (column 9) (len 5)))))
    (reference r6 (scope relative) (span (offset 747) (line 33) (column 16) (len 15)) (segments (segment 0 (token "'enter vehicle'") (name "enter vehicle") (separator none) (span (offset 747) (line 33) (column 16) (len 15)))))
    (reference r7 (scope relative) (span (offset 1333) (line 55) (column 16) (len 14)) (segments (segment 0 (token "'exit vehicle'") (name "exit vehicle") (separator none) (span (offset 1333) (line 55) (column 16) (len 14)))))
    (reference r8 (scope relative) (span (offset 1518) (line 63) (column 11) (len 4)) (segments (segment 0 (token "done") (name "done") (separator none) (span (offset 1518) (line 63) (column 11) (len 4)))))
    (reference r9 (scope relative) (span (offset 1696) (line 73) (column 21) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 1696) (line 73) (column 21) (len 7)))))
    (reference r10 (scope relative) (span (offset 1722) (line 74) (column 18) (len 6)) (segments (segment 0 (token "Person") (name "Person") (separator none) (span (offset 1722) (line 74) (column 18) (len 6)))))
    (reference r11 (scope relative) (span (offset 1751) (line 75) (column 22) (len 6)) (segments (segment 0 (token "Person") (name "Person") (separator none) (span (offset 1751) (line 75) (column 22) (len 6)))))
    (reference r12 (scope relative) (span (offset 1817) (line 79) (column 21) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 1817) (line 79) (column 21) (len 7)))))
    (reference r13 (scope relative) (span (offset 1843) (line 80) (column 18) (len 6)) (segments (segment 0 (token "Person") (name "Person") (separator none) (span (offset 1843) (line 80) (column 18) (len 6)))))
    (reference r14 (scope relative) (span (offset 1872) (line 81) (column 22) (len 6)) (segments (segment 0 (token "Person") (name "Person") (separator none) (span (offset 1872) (line 81) (column 22) (len 6)))))
    (reference r15 (scope relative) (span (offset 1935) (line 85) (column 21) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 1935) (line 85) (column 21) (len 7)))))
    (reference r16 (scope relative) (span (offset 1961) (line 86) (column 18) (len 6)) (segments (segment 0 (token "Person") (name "Person") (separator none) (span (offset 1961) (line 86) (column 18) (len 6)))))
    (reference r17 (scope relative) (span (offset 1994) (line 87) (column 26) (len 14)) (segments (segment 0 (token "'Fuel Station'") (name "Fuel Station") (separator none) (span (offset 1994) (line 87) (column 26) (len 14)))))
  )
  (root (package (name "18-Use Case") (body brace (part-def (name "Vehicle") (modifiers) (body semicolon)) (part-def (name "Person") (modifiers) (body semicolon)) (part-def (name "Environment") (modifiers) (body semicolon)) (part-def (name "Fuel Station") (modifiers) (body semicolon)) (use-case-usage (declaration (name "provide transportation") (short-name none) (abstract false) (type none) (subsets none)) (variation false) (body brace (subject (name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (actor (name "driver") (short-name none) (type (ref r1)) (redefines none) (multiplicity none)) (actor (name "passengers") (short-name none) (type (ref r2)) (redefines none) (multiplicity (lower (expression (span (offset 236) (line 12) (column 29) (len 1)) (integer 0))) (upper (expression (span (offset 239) (line 12) (column 32) (len 1)) (integer 4))))) (actor (name "environment") (short-name none) (type (ref r3)) (redefines none) (multiplicity none)) (objective (visibility none) (name none) (type none) (body brace (doc (name none) (short-name none) (locale none) (body (span (offset 308) (line 17) (column 6) (len 259)) (normalized "Satisfy mission requirements to transport driver and passengers \nfrom starting location to ending location in conformance with \nthe driving profile and meet the mission requirements for safety, \nreliability, comfort, and affordability.\n"))))) (ref-redefinition (target (ref r4)) (body-span (span (offset 593) (line 24) (column 17) (len 117))) (body brace (doc (name none) (short-name none) (locale none) (body (span (offset 604) (line 25) (column 10) (len 29)) (normalized "Mock-up of a pre-condition. "))) (assert-constraint))) (first (source (expression (span (offset 722) (line 31) (column 9) (len 5)) (ref r5))) (target none) (body semicolon (span (span (offset 727) (line 31) (column 14) (len 1))))) (then-include (target (ref r6))) (then-use-case) (then-include (target (ref r7))) (then-action) (ref-redefinition (target (ref r8)) (body-span (span (offset 1523) (line 63) (column 16) (len 116))) (body brace (doc (name none) (short-name none) (locale none) (body (span (offset 1534) (line 64) (column 10) (len 30)) (normalized "Mock-up of a post-condition. "))) (assert-constraint))))) (use-case-usage (declaration (name "enter vehicle") (short-name none) (abstract false) (type none) (subsets none)) (variation false) (body brace (subject (name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (actor (name "driver") (short-name none) (type (ref r10)) (redefines none) (multiplicity none)) (actor (name "passengers") (short-name none) (type (ref r11)) (redefines none) (multiplicity (lower (expression (span (offset 1758) (line 75) (column 29) (len 1)) (integer 0))) (upper (expression (span (offset 1761) (line 75) (column 32) (len 1)) (integer 4))))))) (use-case-usage (declaration (name "exit vehicle") (short-name none) (abstract false) (type none) (subsets none)) (variation false) (body brace (subject (name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (actor (name "driver") (short-name none) (type (ref r13)) (redefines none) (multiplicity none)) (actor (name "passengers") (short-name none) (type (ref r14)) (redefines none) (multiplicity (lower (expression (span (offset 1879) (line 81) (column 29) (len 1)) (integer 0))) (upper (expression (span (offset 1882) (line 81) (column 32) (len 1)) (integer 4))))))) (use-case-usage (declaration (name "add fuel") (short-name none) (abstract false) (type none) (subsets none)) (variation false) (body brace (subject (name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (actor (name "fueler") (short-name none) (type (ref r16)) (redefines none) (multiplicity none)) (actor (name "fuel station") (short-name none) (type (ref r17)) (redefines none) (multiplicity none)))))))
)
~~~
