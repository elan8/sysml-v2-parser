# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 37 (Dependencies): Dependency Example"))
~~~
# SOURCE
~~~sysml
package 'Dependency Example' {
	
	part 'System Assembly' {
		part 'Computer Subsystem' {
			// ...
		}
		
		part 'Storage Subsystem' {
			// ...
		}
	}
	
	package 'Software Design' {
		item def MessageSchema {
			// ...
		}
		item def DataSchema {
			// ...
		}
	}
	
	dependency from 'System Assembly'::'Computer Subsystem' to 'Software Design';
	
	dependency Schemata 
		from 'System Assembly'::'Storage Subsystem' 
		to 'Software Design'::MessageSchema, 'Software Design'::DataSchema;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "37_dependency_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Dependency Example' {
    part 'System Assembly' {
        part 'Computer Subsystem' {}
        part 'Storage Subsystem' {}
    }
    package 'Software Design' {
        item def MessageSchema {
        }
        item def DataSchema {
        }
    }
    dependency from 'System Assembly'::'Computer Subsystem' to 'Software Design';
    dependency Schemata from 'System Assembly'::'Storage Subsystem' to 'Software Design'::MessageSchema, 'Software Design'::DataSchema;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 284) (line 22) (column 18) (len 39)) (segments (segment 0 (token "'System Assembly'") (name "System Assembly") (separator none) (span (offset 284) (line 22) (column 18) (len 17))) (segment 1 (token "'Computer Subsystem'") (name "Computer Subsystem") (separator colon-colon) (span (offset 303) (line 22) (column 37) (len 20)))))
    (reference r1 (scope relative) (span (offset 327) (line 22) (column 61) (len 17)) (segments (segment 0 (token "'Software Design'") (name "Software Design") (separator none) (span (offset 327) (line 22) (column 61) (len 17)))))
    (reference r2 (scope relative) (span (offset 377) (line 25) (column 8) (len 38)) (segments (segment 0 (token "'System Assembly'") (name "System Assembly") (separator none) (span (offset 377) (line 25) (column 8) (len 17))) (segment 1 (token "'Storage Subsystem'") (name "Storage Subsystem") (separator colon-colon) (span (offset 396) (line 25) (column 27) (len 19)))))
    (reference r3 (scope relative) (span (offset 422) (line 26) (column 6) (len 32)) (segments (segment 0 (token "'Software Design'") (name "Software Design") (separator none) (span (offset 422) (line 26) (column 6) (len 17))) (segment 1 (token "MessageSchema") (name "MessageSchema") (separator colon-colon) (span (offset 441) (line 26) (column 25) (len 13)))))
    (reference r4 (scope relative) (span (offset 456) (line 26) (column 40) (len 29)) (segments (segment 0 (token "'Software Design'") (name "Software Design") (separator none) (span (offset 456) (line 26) (column 40) (len 17))) (segment 1 (token "DataSchema") (name "DataSchema") (separator colon-colon) (span (offset 475) (line 26) (column 59) (len 10)))))
  )
  (root (package (name "Dependency Example") (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "System Assembly") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "Computer Subsystem") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "Storage Subsystem") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace)))) (package (name "Software Design") (body brace (item-def) (item-def))) (dependency (clients (ref r0)) (suppliers (ref r1)) (body semicolon)) (dependency (clients (ref r2)) (suppliers (ref r3) (ref r4)) (body semicolon)))))
)
~~~
