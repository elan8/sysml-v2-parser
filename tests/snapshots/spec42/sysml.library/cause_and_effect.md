# META
~~~sexpr
(snapshot (type semantic) (description "Standard Library: Domain Libraries/Cause and Effect/CauseAndEffect"))
~~~
# SOURCE
~~~sysml
standard library package CauseAndEffect {
	doc /* This package provides language-extension metadata for cause-effect modeling. */
	
	public import CausationConnections::*;
	private import ScalarValues::*;
	private import Metaobjects::SemanticMetadata;

	metadata def <cause> CauseMetadata :> SemanticMetadata {
		doc
		/*
		 * CauseMetadata identifies a usage as being a cause occurrence.
		 * It is intended to be used to tag the cause ends of a Multicausation.
		 */
		 
		ref :>> annotatedElement : SysML::Usage;
		ref :>> baseType = causes as SysML::Usage;
	}
	
	metadata def <effect> EffectMetadata :> SemanticMetadata {
		doc
		/*
		 * EffectMetadata identifies a usage as being an effect occurrence.
		 * It is intended to be used to tag the effect ends of a Multicausation.
		 */
		 
		ref :>> annotatedElement : SysML::Usage;
		ref :>> baseType = effects as SysML::Usage;
	}
	
	metadata def CausationMetadata {
		doc
		/*
		 * CausationMetadata allows for the specification of additional metadata about
		 * a cause-effect connection definition or usage.
		 */
		 
		ref :> annotatedElement : SysML::ConnectionDefinition;
		ref :> annotatedElement : SysML::ConnectionUsage;
		
		attribute isNecessary : Boolean default false {
			doc 
			/* 
			 * Whether all the causes are necessary for all the effects to occur.
			 * If this is false (the default), then some or all of the effects may 
			 * still have occurred even if some of the causes did not.
			 */
		}
		
		attribute isSufficient : Boolean default false {
			doc
			/*
			 * Whether the causes were sufficient for all the effects to occur.
			 * If this is false (the default), then it may be the case that some
			 * other occurrences were also necessary for some or all of the effects
			 * to have occurred.
			 */
		}
		
		attribute probability : Real[0..1] {
			doc /* The probability that the causes will actually result in effects occurring. */
		}	
	}
	
	metadata def <multicausation> MulticausationSemanticMetadata :> CausationMetadata, SemanticMetadata {
		doc
		/*
		 * MulticausationMetadata is SemanticMetadata for a Multicausation connection.
		 */
		 
		ref :>> baseType = multicausations meta SysML::Usage;
	}
	
	metadata def <causation> CausationSemanticMetadadata :> CausationMetadata, SemanticMetadata {
		doc
		/*
		 * CausationMetadata is SemanticMetadata for a Causation connection.
		 */
		 
		ref :>> baseType = causations meta SysML::Usage;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "cause_and_effect.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package CauseAndEffect {
    doc
    /* This package provides language-extension metadata for cause-effect modeling. */
    public import CausationConnections::*;
    private import ScalarValues::*;
    private import Metaobjects::SemanticMetadata;
    metadata def <cause> CauseMetadata :> SemanticMetadata {
        doc
        /*
		 * CauseMetadata identifies a usage as being a cause occurrence.
		 * It is intended to be used to tag the cause ends of a Multicausation.
		 */
        attribute :>> annotatedElement : SysML::Usage;
        attribute :>> baseType = causes as SysML::Usage;
    }
    metadata def <effect> EffectMetadata :> SemanticMetadata {
        doc
        /*
		 * EffectMetadata identifies a usage as being an effect occurrence.
		 * It is intended to be used to tag the effect ends of a Multicausation.
		 */
        attribute :>> annotatedElement : SysML::Usage;
        attribute :>> baseType = effects as SysML::Usage;
    }
    metadata def CausationMetadata {
        doc
        /*
		 * CausationMetadata allows for the specification of additional metadata about
		 * a cause-effect connection definition or usage.
		 */
        attribute :> annotatedElement : SysML::ConnectionDefinition;
        attribute :> annotatedElement : SysML::ConnectionUsage;
        attribute isNecessary : Boolean default false {
            doc
            /* 
			 * Whether all the causes are necessary for all the effects to occur.
			 * If this is false (the default), then some or all of the effects may 
			 * still have occurred even if some of the causes did not.
			 */
        }
        attribute isSufficient : Boolean default false {
            doc
            /*
			 * Whether the causes were sufficient for all the effects to occur.
			 * If this is false (the default), then it may be the case that some
			 * other occurrences were also necessary for some or all of the effects
			 * to have occurred.
			 */
        }
        attribute probability : Real[0..1] {
            doc
            /* The probability that the causes will actually result in effects occurring. */
        }
    }
    metadata def <multicausation> MulticausationSemanticMetadata :> CausationMetadata, SemanticMetadata {
        doc
        /*
		 * MulticausationMetadata is SemanticMetadata for a Multicausation connection.
		 */
        attribute :>> baseType = multicausations meta SysML::Usage;
    }
    metadata def <causation> CausationSemanticMetadadata :> CausationMetadata, SemanticMetadata {
        doc
        /*
		 * CausationMetadata is SemanticMetadata for a Causation connection.
		 */
        attribute :>> baseType = causations meta SysML::Usage;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 147) (line 4) (column 16) (len 20)) (segments (segment 0 (token "CausationConnections") (name "CausationConnections") (separator none) (span (offset 147) (line 4) (column 16) (len 20)))))
    (reference r1 (scope relative) (span (offset 188) (line 5) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 188) (line 5) (column 17) (len 12)))))
    (reference r2 (scope relative) (span (offset 221) (line 6) (column 17) (len 29)) (segments (segment 0 (token "Metaobjects") (name "Metaobjects") (separator none) (span (offset 221) (line 6) (column 17) (len 11))) (segment 1 (token "SemanticMetadata") (name "SemanticMetadata") (separator colon-colon) (span (offset 234) (line 6) (column 30) (len 16)))))
  )
  (root (library-package (name "CauseAndEffect") (standard true) (body brace (doc) (import (target (span (span (offset 147) (line 4) (column 16) (len 23))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 167) (line 4) (column 36) (len 3))) (separator (span (offset 167) (line 4) (column 36) (len 2))) (marker (span (offset 169) (line 4) (column 38) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 188) (line 5) (column 17) (len 15))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 200) (line 5) (column 29) (len 3))) (separator (span (offset 200) (line 5) (column 29) (len 2))) (marker (span (offset 202) (line 5) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 221) (line 6) (column 17) (len 29))) (all none) (ref r2) (shape (membership (recursive-suffix none))))) (metadata-def) (metadata-def) (metadata-def) (metadata-def) (metadata-def))))
)
~~~
