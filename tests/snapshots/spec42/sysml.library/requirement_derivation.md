# META
~~~sexpr
(snapshot (type semantic) (description "Standard Library: Domain Libraries/Requirement Derivation/RequirementDerivation"))
~~~
# SOURCE
~~~sysml
standard library package RequirementDerivation {
	doc /* This package provides language-extension metadata for modeling requirement derivation. */
	
	public import DerivationConnections::*;
	private import Metaobjects::SemanticMetadata;
	
	metadata def <original> OriginalRequirementMetadata :> SemanticMetadata {
		doc
		/*
		 * OriginalRequirementMetadata identifies a usage as an original requirement.
		 * It is intended to be used to tag the original requirement end of a Derivation.
		 */
		 
		:> annotatedElement : SysML::Usage;
		:>> baseType = originalRequirements meta SysML::Usage;
	}
	
	metadata def <derive> DerivedRequirementMetadata :> SemanticMetadata {
		doc
		/*
		 * DerivedRequirementMetadata identifies a usage as a derived requirement.
		 * It is intended to be used to tag the derived requirement ends of a Derivation.
		 */
		 
		:> annotatedElement : SysML::Usage;	
		:>> baseType = derivedRequirements meta SysML::Usage;
	}
	
	metadata def <derivation> DerivationMetadata :> SemanticMetadata {
		doc
		/*
		 * DerivationMetadata is SemanticMetadata for a Derivation connection.
		 */
		 
		:> annotatedElement : SysML::ConnectionDefinition;
		:> annotatedElement : SysML::ConnectionUsage;
		:>> baseType = derivations meta SysML::Usage;
	}	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "requirement_derivation.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package RequirementDerivation {
    doc
    /* This package provides language-extension metadata for modeling requirement derivation. */
    public import DerivationConnections::*;
    private import Metaobjects::SemanticMetadata;
    metadata def <original> OriginalRequirementMetadata :> SemanticMetadata {
        doc
        /*
		 * OriginalRequirementMetadata identifies a usage as an original requirement.
		 * It is intended to be used to tag the original requirement end of a Derivation.
		 */
        attribute :> annotatedElement : SysML::Usage;
        attribute :>> baseType = originalRequirements meta SysML::Usage;
    }
    metadata def <derive> DerivedRequirementMetadata :> SemanticMetadata {
        doc
        /*
		 * DerivedRequirementMetadata identifies a usage as a derived requirement.
		 * It is intended to be used to tag the derived requirement ends of a Derivation.
		 */
        attribute :> annotatedElement : SysML::Usage;
        attribute :>> baseType = derivedRequirements meta SysML::Usage;
    }
    metadata def <derivation> DerivationMetadata :> SemanticMetadata {
        doc
        /*
		 * DerivationMetadata is SemanticMetadata for a Derivation connection.
		 */
        attribute :> annotatedElement : SysML::ConnectionDefinition;
        attribute :> annotatedElement : SysML::ConnectionUsage;
        attribute :>> baseType = derivations meta SysML::Usage;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 164) (line 4) (column 16) (len 21)) (segments (segment 0 (token "DerivationConnections") (name "DerivationConnections") (separator none) (span (offset 164) (line 4) (column 16) (len 21)))))
    (reference r1 (scope relative) (span (offset 206) (line 5) (column 17) (len 29)) (segments (segment 0 (token "Metaobjects") (name "Metaobjects") (separator none) (span (offset 206) (line 5) (column 17) (len 11))) (segment 1 (token "SemanticMetadata") (name "SemanticMetadata") (separator colon-colon) (span (offset 219) (line 5) (column 30) (len 16)))))
  )
  (root (library-package (name "RequirementDerivation") (standard true) (body brace (doc) (import (target (span (span (offset 164) (line 4) (column 16) (len 24))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 185) (line 4) (column 37) (len 3))) (separator (span (offset 185) (line 4) (column 37) (len 2))) (marker (span (offset 187) (line 4) (column 39) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 206) (line 5) (column 17) (len 29))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (metadata-def) (metadata-def) (metadata-def))))
)
~~~
