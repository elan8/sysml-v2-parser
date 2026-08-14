# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 41 (Language Extension): Semantic Metadata Example"))
~~~
# SOURCE
~~~sysml
library package 'Semantic Metadata Example' {
	private import 'Model Library Example'::*;
	private import Metaobjects::SemanticMetadata;

	metadata def situation :> SemanticMetadata {
		:>> baseType = situations meta SysML::Usage;
	}
	
	metadata def cause :> SemanticMetadata {
		:>> baseType = causes meta SysML::Usage;
	}
	
	metadata def failure :> SemanticMetadata {
		:>> baseType = failures meta SysML::Usage;
	}
	
	metadata def causation :> SemanticMetadata {
		:>> baseType = causations meta SysML::Usage;
	}
	
	metadata def scenario :> SemanticMetadata {
		:>> baseType = scenarios meta SysML::Usage;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "41_semantic_metadata_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
library package 'Semantic Metadata Example' {
    private import 'Model Library Example'::*;
    private import Metaobjects::SemanticMetadata;
    metadata def situation :> SemanticMetadata {
        attribute baseType :>> baseType = situations meta SysML::Usage;
    }
    metadata def cause :> SemanticMetadata {
        attribute baseType :>> baseType = causes meta SysML::Usage;
    }
    metadata def failure :> SemanticMetadata {
        attribute baseType :>> baseType = failures meta SysML::Usage;
    }
    metadata def causation :> SemanticMetadata {
        attribute baseType :>> baseType = causations meta SysML::Usage;
    }
    metadata def scenario :> SemanticMetadata {
        attribute baseType :>> baseType = scenarios meta SysML::Usage;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 62) (line 2) (column 17) (len 23)) (segments (segment 0 (token "'Model Library Example'") (name "Model Library Example") (separator none) (span (offset 62) (line 2) (column 17) (len 23)))))
    (reference r1 (scope relative) (span (offset 106) (line 3) (column 17) (len 29)) (segments (segment 0 (token "Metaobjects") (name "Metaobjects") (separator none) (span (offset 106) (line 3) (column 17) (len 11))) (segment 1 (token "SemanticMetadata") (name "SemanticMetadata") (separator colon-colon) (span (offset 119) (line 3) (column 30) (len 16)))))
  )
  (root (library-package (name "Semantic Metadata Example") (standard false) (body (import (target (span (span (offset 62) (line 2) (column 17) (len 26))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 85) (line 2) (column 40) (len 3))) (separator (span (offset 85) (line 2) (column 40) (len 2))) (marker (span (offset 87) (line 2) (column 42) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 106) (line 3) (column 17) (len 29))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (metadata-def) (metadata-def) (metadata-def) (metadata-def) (metadata-def))))
)
~~~
