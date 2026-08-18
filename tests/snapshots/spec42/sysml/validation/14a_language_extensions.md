# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (14-Language Extensions): 14a-Language Extensions"))
~~~
# SOURCE
~~~sysml
package '14a-Language Extensions' {
	private import 'User Defined Extensions'::*;
	
	package 'User Defined Extensions' {
		
		enum def ClassificationLevel {
			uncl;
			conf;
			secret;
		}
		
		metadata def Classified {
			ref :>> annotatedElement : SysML::PartUsage;
			attribute classificationLevel : ClassificationLevel[1];
		}
	}
	
	part part_X {
		metadata Classified {
			classificationLevel = ClassificationLevel::conf;
		}
	}
	
	// Alternative shorthand notation
	part part_Y {
		@Classified {
			classificationLevel = ClassificationLevel::conf;
		}
	}

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "14a_language_extensions.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '14a-Language Extensions' {
    private import 'User Defined Extensions'::*;
    package 'User Defined Extensions' {
        enum def ClassificationLevel {
            uncl;
            conf;
            secret;
        }
        metadata def Classified {
            attribute :>> annotatedElement : SysML::PartUsage;
            attribute classificationLevel : ClassificationLevel[1];
        }
    }
    part part_X {
        metadata Classified {
            attribute classificationLevel = ClassificationLevel::conf;
        }
    }
    part part_Y {
        @Classified {
            attribute classificationLevel = ClassificationLevel::conf;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 52) (line 2) (column 17) (len 25)) (segments (segment 0 (token "'User Defined Extensions'") (name "User Defined Extensions") (separator none) (span (offset 52) (line 2) (column 17) (len 25)))))
    (reference r1 (scope relative) (span (offset 490) (line 26) (column 4) (len 10)) (segments (segment 0 (token "Classified") (name "Classified") (separator none) (span (offset 490) (line 26) (column 4) (len 10)))))
  )
  (root (package (name "14a-Language Extensions") (body brace (import (target (span (span (offset 52) (line 2) (column 17) (len 28))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 77) (line 2) (column 42) (len 3))) (separator (span (offset 77) (line 2) (column 42) (len 2))) (marker (span (offset 79) (line 2) (column 44) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "User Defined Extensions") (body brace (enum-def (name "ClassificationLevel") (body brace (enum-value (name "uncl") (short-name none) (value none) (body semicolon) (span (offset 160) (line 7) (column 4) (len 5))) (enum-value (name "conf") (short-name none) (value none) (body semicolon) (span (offset 169) (line 8) (column 4) (len 5))) (enum-value (name "secret") (short-name none) (value none) (body semicolon) (span (offset 178) (line 9) (column 4) (len 7))))) (metadata-def))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "part_X") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (metadata-usage))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "part_Y") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r1)) (about) (body brace (element-count 1))))))))
)
~~~
