# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Simple Tests): MetadataTest. Its `#Security enum secret : ClassificationLevel = 2;` is the Pilot-only metadata-prefixed EnumeratedValue extension (Pilot SysML.xtext 765-771) absent from the 2026-04 pin's EnumeratedValue production (SysML-textual-bnf.kebnf 528-535), so it is retained as exact recovery."))
~~~
# SOURCE
~~~sysml
package MetadataTest {
	private import 'User Defined Extensions'::*;
	
	library package 'User Defined Extensions' {
		
		#Security enum def ClassificationLevel :> ScalarValues::Natural {
			uncl : ClassificationLevel = 0;
			conf : ClassificationLevel = 1;
			#Security enum secret : ClassificationLevel = 2;
		}
		
		metadata def Classified {
			ref :>> annotatedElement : SysML::Usage;
			ref classificationLevel : ClassificationLevel;
		}
		
		metadata def Security;
	}
	
	ref x {
		metadata Classified {
			classificationLevel = ClassificationLevel::conf;
		}
	}
	
	ref y {
		@Classified {
			classificationLevel = ClassificationLevel::conf;
		}
		@Security;
	}
	
	private ref #Classified #Security z1;
	abstract #Classified z2;
	
	ref z {
	    #Security #Classified metadata Classified {
	        classificationLevel = ClassificationLevel::secret;
	    }
	}	
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "metadata_test.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 190) (line 7) (column 4) (len 35)) (message "unrecognized declaration `uncl` in enumeration body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 225) (line 8) (column 4) (len 35)) (message "unrecognized declaration `conf` in enumeration body"))
      (diagnostic (code "unsupported_annotation_syntax") (severity warning) (category unsupportedgrammarform) (span (offset 260) (line 9) (column 4) (len 51)) (message "incomplete parser support for metadata syntax in enumeration body"))
      (diagnostic (code "recovered_package_body_element") (severity error) (category parseerror) (span (offset 669) (line 33) (column 2) (len 39)) (message "unexpected token in package body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package MetadataTest {
    private import 'User Defined Extensions'::*;
    library package 'User Defined Extensions' {
        #Security
        enum def ClassificationLevel :> ScalarValues::Natural {
            uncl : ClassificationLevel = 0;
            conf : ClassificationLevel = 1;
            #Security enum secret : ClassificationLevel = 2;
        }
        metadata def Classified {
            attribute :>> annotatedElement : SysML::Usage;
            attribute classificationLevel : ClassificationLevel;
        }
        metadata def Security;
    }
    ref x {
        metadata Classified {
            attribute classificationLevel = ClassificationLevel::conf;
        }
    }
    ref y {
        @Classified {
            attribute classificationLevel = ClassificationLevel::conf;
        }
        @Security;
    }
    private ref #Classified #Security z1;
    abstract #Classified z2;
    ref z {
        #Security
        #Classified
        metadata Classified {
            attribute classificationLevel = ClassificationLevel::secret;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 39) (line 2) (column 17) (len 25)) (segments (segment 0 (token "'User Defined Extensions'") (name "User Defined Extensions") (separator none) (span (offset 39) (line 2) (column 17) (len 25)))))
    (reference r1 (scope relative) (span (offset 122) (line 6) (column 4) (len 8)) (segments (segment 0 (token "Security") (name "Security") (separator none) (span (offset 122) (line 6) (column 4) (len 8)))))
    (reference r2 (scope relative) (span (offset 374) (line 13) (column 31) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 374) (line 13) (column 31) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 381) (line 13) (column 38) (len 5)))))
    (reference r3 (scope relative) (span (offset 355) (line 13) (column 12) (len 16)) (segments (segment 0 (token "annotatedElement") (name "annotatedElement") (separator none) (span (offset 355) (line 13) (column 12) (len 16)))))
    (reference r4 (scope relative) (span (offset 417) (line 14) (column 30) (len 19)) (segments (segment 0 (token "ClassificationLevel") (name "ClassificationLevel") (separator none) (span (offset 417) (line 14) (column 30) (len 19)))))
    (reference r5 (scope relative) (span (offset 533) (line 22) (column 26) (len 25)) (segments (segment 0 (token "ClassificationLevel") (name "ClassificationLevel") (separator none) (span (offset 533) (line 22) (column 26) (len 19))) (segment 1 (token "conf") (name "conf") (separator colon-colon) (span (offset 554) (line 22) (column 47) (len 4)))))
    (reference r6 (scope relative) (span (offset 581) (line 27) (column 4) (len 10)) (segments (segment 0 (token "Classified") (name "Classified") (separator none) (span (offset 581) (line 27) (column 4) (len 10)))))
    (reference r7 (scope relative) (span (offset 619) (line 28) (column 26) (len 25)) (segments (segment 0 (token "ClassificationLevel") (name "ClassificationLevel") (separator none) (span (offset 619) (line 28) (column 26) (len 19))) (segment 1 (token "conf") (name "conf") (separator colon-colon) (span (offset 640) (line 28) (column 47) (len 4)))))
    (reference r8 (scope relative) (span (offset 653) (line 30) (column 4) (len 8)) (segments (segment 0 (token "Security") (name "Security") (separator none) (span (offset 653) (line 30) (column 4) (len 8)))))
    (reference r9 (scope relative) (span (offset 718) (line 34) (column 12) (len 10)) (segments (segment 0 (token "Classified") (name "Classified") (separator none) (span (offset 718) (line 34) (column 12) (len 10)))))
    (reference r10 (scope relative) (span (offset 750) (line 37) (column 7) (len 8)) (segments (segment 0 (token "Security") (name "Security") (separator none) (span (offset 750) (line 37) (column 7) (len 8)))))
    (reference r11 (scope relative) (span (offset 760) (line 37) (column 17) (len 10)) (segments (segment 0 (token "Classified") (name "Classified") (separator none) (span (offset 760) (line 37) (column 17) (len 10)))))
    (reference r12 (scope relative) (span (offset 824) (line 38) (column 32) (len 27)) (segments (segment 0 (token "ClassificationLevel") (name "ClassificationLevel") (separator none) (span (offset 824) (line 38) (column 32) (len 19))) (segment 1 (token "secret") (name "secret") (separator colon-colon) (span (offset 845) (line 38) (column 53) (len 6)))))
  )
  (root (package (name "MetadataTest") (body brace (import (target (span (span (offset 39) (line 2) (column 17) (len 28))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 64) (line 2) (column 42) (len 3))) (separator (span (offset 64) (line 2) (column 42) (len 2))) (marker (span (offset 66) (line 2) (column 44) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (library-package (name "User Defined Extensions") (standard false) (body brace (metadata-keyword-usage (type (ref r1)) (body none)) (enum-def (name "ClassificationLevel") (body brace (malformed (code "unrecognized_declaration_in_scope") (found "uncl : ClassificationLevel = 0;") (span (offset 190) (line 7) (column 4) (len 35))) (malformed (code "unrecognized_declaration_in_scope") (found "conf : ClassificationLevel = 1;") (span (offset 225) (line 8) (column 4) (len 35))) (malformed (code "unsupported_annotation_syntax") (found "#Security enum secret : ClassificationLevel = 2;") (span (offset 260) (line 9) (column 4) (len 51))))) (metadata-def (name "Classified") (abstract false) (specializes none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r3)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "classificationLevel") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (metadata-def (name "Security") (abstract false) (specializes none) (body semicolon)))) (ref (name "x") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing none) (redefines none) (subsets none) (body brace (metadata-usage (declaration-name "Classified") (type none) (about) (body brace (attribute-usage (declaration-name "classificationLevel") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 533) (line 22) (column 26) (len 25)) (ref r5))))) (body semicolon)))))) (ref (name "y") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing none) (redefines none) (subsets none) (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r6)) (about) (body brace (attribute-usage (declaration-name "classificationLevel") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 619) (line 28) (column 26) (len 25)) (ref r7))))) (body semicolon)))) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r8)) (about) (body semicolon)))) (malformed (code "recovered_package_body_element") (found "private ref #Classified #Security z1;") (span (offset 669) (line 33) (column 2) (len 39))) (extended-def (prefix-keywords ((ref r9))) (definition-prefix abstract) (def false) (name "z2") (specializes none) (body semicolon)) (ref (name "z") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing none) (redefines none) (subsets none) (body brace (metadata-keyword-usage (type (ref r10)) (body none)) (metadata-keyword-usage (type (ref r11)) (body none)) (metadata-usage (declaration-name "Classified") (type none) (about) (body brace (attribute-usage (declaration-name "classificationLevel") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 824) (line 38) (column 32) (len 27)) (ref r12))))) (body semicolon)))))))))
)
~~~
