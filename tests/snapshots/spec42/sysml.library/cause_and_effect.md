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
    (reference r3 (scope relative) (span (offset 292) (line 8) (column 40) (len 16)) (segments (segment 0 (token "SemanticMetadata") (name "SemanticMetadata") (separator none) (span (offset 292) (line 8) (column 40) (len 16)))))
    (reference r4 (scope relative) (span (offset 502) (line 15) (column 30) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 502) (line 15) (column 30) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 509) (line 15) (column 37) (len 5)))))
    (reference r5 (scope relative) (span (offset 483) (line 15) (column 11) (len 16)) (segments (segment 0 (token "annotatedElement") (name "annotatedElement") (separator none) (span (offset 483) (line 15) (column 11) (len 16)))))
    (reference r6 (scope relative) (span (offset 526) (line 16) (column 11) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 526) (line 16) (column 11) (len 8)))))
    (reference r7 (scope relative) (span (offset 537) (line 16) (column 22) (len 6)) (segments (segment 0 (token "causes") (name "causes") (separator none) (span (offset 537) (line 16) (column 22) (len 6)))))
    (reference r8 (scope relative) (span (offset 547) (line 16) (column 32) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 547) (line 16) (column 32) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 554) (line 16) (column 39) (len 5)))))
    (reference r9 (scope relative) (span (offset 607) (line 19) (column 42) (len 16)) (segments (segment 0 (token "SemanticMetadata") (name "SemanticMetadata") (separator none) (span (offset 607) (line 19) (column 42) (len 16)))))
    (reference r10 (scope relative) (span (offset 821) (line 26) (column 30) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 821) (line 26) (column 30) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 828) (line 26) (column 37) (len 5)))))
    (reference r11 (scope relative) (span (offset 802) (line 26) (column 11) (len 16)) (segments (segment 0 (token "annotatedElement") (name "annotatedElement") (separator none) (span (offset 802) (line 26) (column 11) (len 16)))))
    (reference r12 (scope relative) (span (offset 845) (line 27) (column 11) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 845) (line 27) (column 11) (len 8)))))
    (reference r13 (scope relative) (span (offset 856) (line 27) (column 22) (len 7)) (segments (segment 0 (token "effects") (name "effects") (separator none) (span (offset 856) (line 27) (column 22) (len 7)))))
    (reference r14 (scope relative) (span (offset 867) (line 27) (column 33) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 867) (line 27) (column 33) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 874) (line 27) (column 40) (len 5)))))
    (reference r15 (scope relative) (span (offset 1102) (line 37) (column 29) (len 27)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 1102) (line 37) (column 29) (len 5))) (segment 1 (token "ConnectionDefinition") (name "ConnectionDefinition") (separator colon-colon) (span (offset 1109) (line 37) (column 36) (len 20)))))
    (reference r16 (scope relative) (span (offset 1083) (line 37) (column 10) (len 16)) (segments (segment 0 (token "annotatedElement") (name "annotatedElement") (separator none) (span (offset 1083) (line 37) (column 10) (len 16)))))
    (reference r17 (scope relative) (span (offset 1159) (line 38) (column 29) (len 22)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 1159) (line 38) (column 29) (len 5))) (segment 1 (token "ConnectionUsage") (name "ConnectionUsage") (separator colon-colon) (span (offset 1166) (line 38) (column 36) (len 15)))))
    (reference r18 (scope relative) (span (offset 1140) (line 38) (column 10) (len 16)) (segments (segment 0 (token "annotatedElement") (name "annotatedElement") (separator none) (span (offset 1140) (line 38) (column 10) (len 16)))))
    (reference r19 (scope relative) (span (offset 1212) (line 40) (column 27) (len 7)) (segments (segment 0 (token "Boolean") (name "Boolean") (separator none) (span (offset 1212) (line 40) (column 27) (len 7)))))
    (reference r20 (scope relative) (span (offset 1502) (line 49) (column 28) (len 7)) (segments (segment 0 (token "Boolean") (name "Boolean") (separator none) (span (offset 1502) (line 49) (column 28) (len 7)))))
    (reference r21 (scope relative) (span (offset 1821) (line 59) (column 27) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 1821) (line 59) (column 27) (len 4)))))
    (reference r22 (scope relative) (span (offset 1997) (line 64) (column 66) (len 17)) (segments (segment 0 (token "CausationMetadata") (name "CausationMetadata") (separator none) (span (offset 1997) (line 64) (column 66) (len 17)))))
    (reference r23 (scope relative) (span (offset 2016) (line 64) (column 85) (len 16)) (segments (segment 0 (token "SemanticMetadata") (name "SemanticMetadata") (separator none) (span (offset 2016) (line 64) (column 85) (len 16)))))
    (reference r24 (scope relative) (span (offset 2147) (line 70) (column 11) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 2147) (line 70) (column 11) (len 8)))))
    (reference r25 (scope relative) (span (offset 2158) (line 70) (column 22) (len 15)) (segments (segment 0 (token "multicausations") (name "multicausations") (separator none) (span (offset 2158) (line 70) (column 22) (len 15)))))
    (reference r26 (scope relative) (span (offset 2179) (line 70) (column 43) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 2179) (line 70) (column 43) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 2186) (line 70) (column 50) (len 5)))))
    (reference r27 (scope relative) (span (offset 2255) (line 73) (column 58) (len 17)) (segments (segment 0 (token "CausationMetadata") (name "CausationMetadata") (separator none) (span (offset 2255) (line 73) (column 58) (len 17)))))
    (reference r28 (scope relative) (span (offset 2274) (line 73) (column 77) (len 16)) (segments (segment 0 (token "SemanticMetadata") (name "SemanticMetadata") (separator none) (span (offset 2274) (line 73) (column 77) (len 16)))))
    (reference r29 (scope relative) (span (offset 2395) (line 79) (column 11) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 2395) (line 79) (column 11) (len 8)))))
    (reference r30 (scope relative) (span (offset 2406) (line 79) (column 22) (len 10)) (segments (segment 0 (token "causations") (name "causations") (separator none) (span (offset 2406) (line 79) (column 22) (len 10)))))
    (reference r31 (scope relative) (span (offset 2422) (line 79) (column 38) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 2422) (line 79) (column 38) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 2429) (line 79) (column 45) (len 5)))))
  )
  (root (library-package (name "CauseAndEffect") (standard true) (body brace (doc) (import (target (span (span (offset 147) (line 4) (column 16) (len 23))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 167) (line 4) (column 36) (len 3))) (separator (span (offset 167) (line 4) (column 36) (len 2))) (marker (span (offset 169) (line 4) (column 38) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 188) (line 5) (column 17) (len 15))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 200) (line 5) (column 29) (len 3))) (separator (span (offset 200) (line 5) (column 29) (len 2))) (marker (span (offset 202) (line 5) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 221) (line 6) (column 17) (len 29))) (all none) (ref r2) (shape (membership (recursive-suffix none))))) (metadata-def (name "CauseMetadata") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r3)))) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r5)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r6)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 537) (line 16) (column 22) (len 22)) (type-check (kind as) (operand (expression (span (offset 537) (line 16) (column 22) (len 6)) (ref r7))) (type (ref r8))))))) (body semicolon)))) (metadata-def (name "EffectMetadata") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r9)))) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r11)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r12)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 856) (line 27) (column 22) (len 23)) (type-check (kind as) (operand (expression (span (offset 856) (line 27) (column 22) (len 7)) (ref r13))) (type (ref r14))))))) (body semicolon)))) (metadata-def (name "CausationMetadata") (abstract false) (specializes none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (subsets (relationship (kind subsets) (implied false) (targets (ref r16)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r17)))) (subsets (relationship (kind subsets) (implied false) (targets (ref r18)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "isNecessary") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r19)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 1228) (line 40) (column 43) (len 5)) (boolean false))))) (body brace (doc))) (attribute-usage (declaration-name "isSufficient") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r20)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 1518) (line 49) (column 44) (len 5)) (boolean false))))) (body brace (doc))) (attribute-usage (declaration-name "probability") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r21)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (doc))))) (metadata-def (name "MulticausationSemanticMetadata") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r22) (ref r23)))) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r24)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2158) (line 70) (column 22) (len 33)) (meta-cast (base (expression (span (offset 2158) (line 70) (column 22) (len 15)) (ref r25))) (metaclass (ref r26))))))) (body semicolon)))) (metadata-def (name "CausationSemanticMetadadata") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r27) (ref r28)))) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r29)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2406) (line 79) (column 22) (len 28)) (meta-cast (base (expression (span (offset 2406) (line 79) (column 22) (len 10)) (ref r30))) (metaclass (ref r31))))))) (body semicolon)))))))
)
~~~
