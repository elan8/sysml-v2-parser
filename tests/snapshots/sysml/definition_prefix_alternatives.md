# META
~~~sexpr
(snapshot (type semantic) (description "BasicDefinitionPrefix (SysML BNF 219) is one slot with two alternatives, reached by every definition kind through DefinitionPrefix, OccurrenceDefinitionPrefix, IndividualDefinition or ExtendedDefinition. Each kind retains the authored spelling with its exact span, and the unprefixed state stays distinguishable. The two productions that differ are covered too: EnumerationDefinition spells no prefix at all and MetadataDefinition inlines `abstract` with no `variation` alternative, so both refuse what they do not spell."))
~~~
# SOURCE
~~~sysml
package DefinitionPrefixAlternatives {
    abstract attribute def AbstractAttribute;
    variation attribute def VariationAttribute;
    attribute def PlainAttribute;
    abstract item def AbstractItem;
    variation item def VariationItem;
    abstract action def AbstractAction;
    variation action def VariationAction;
    abstract calc def AbstractCalc;
    variation calc def VariationCalc;
    abstract state def AbstractState;
    variation state def VariationState;
    abstract port def AbstractPort;
    variation port def VariationPort;
    abstract requirement def AbstractRequirement;
    variation requirement def VariationRequirement;
    abstract constraint def AbstractConstraint;
    variation constraint def VariationConstraint;
    abstract view def AbstractView;
    variation view def VariationView;
    abstract viewpoint def AbstractViewpoint;
    variation viewpoint def VariationViewpoint;
    abstract rendering def AbstractRendering;
    variation rendering def VariationRendering;
    abstract occurrence def AbstractOccurrence;
    variation occurrence def VariationOccurrence;
    abstract case def AbstractCase;
    variation case def VariationCase;
    abstract analysis def AbstractAnalysis;
    variation analysis def VariationAnalysis;
    abstract verification def AbstractVerification;
    variation verification def VariationVerification;
    abstract use case def AbstractUseCase;
    variation use case def VariationUseCase;
    abstract individual def AbstractIndividual;
    abstract metadata def AbstractMetadata;
    metadata def PlainMetadata;
    enum def PlainEnum;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "definition_prefix_alternatives.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sexpr
(stable-idempotent)
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "DefinitionPrefixAlternatives") (body brace (attribute-def (declaration-name "AbstractAttribute") (short-name none) (modifiers (abstract (span (offset 43) (line 2) (column 5) (len 8)))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "VariationAttribute") (short-name none) (modifiers (variation (span (offset 89) (line 3) (column 5) (len 9)))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "PlainAttribute") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (item-def (name "AbstractItem") (modifiers (abstract (span (offset 171) (line 5) (column 5) (len 8)))) (individual false) (specializes none) (body semicolon)) (item-def (name "VariationItem") (modifiers (variation (span (offset 207) (line 6) (column 5) (len 9)))) (individual false) (specializes none) (body semicolon)) (action-def (name "AbstractAction") (modifiers (abstract (span (offset 245) (line 7) (column 5) (len 8)))) (specializes none) (body semicolon)) (action-def (name "VariationAction") (modifiers (variation (span (offset 285) (line 8) (column 5) (len 9)))) (specializes none) (body semicolon)) (calc-def (name "AbstractCalc") (modifiers (abstract (span (offset 327) (line 9) (column 5) (len 8)))) (body semicolon)) (calc-def (name "VariationCalc") (modifiers (variation (span (offset 363) (line 10) (column 5) (len 9)))) (body semicolon)) (state-def (name "AbstractState") (modifiers (abstract (span (offset 401) (line 11) (column 5) (len 8)))) (body semicolon)) (state-def (name "VariationState") (modifiers (variation (span (offset 439) (line 12) (column 5) (len 9)))) (body semicolon)) (port-def (name "AbstractPort") (modifiers (abstract (span (offset 479) (line 13) (column 5) (len 8)))) (specializes none) (body semicolon)) (port-def (name "VariationPort") (modifiers (variation (span (offset 515) (line 14) (column 5) (len 9)))) (specializes none) (body semicolon)) (requirement-def (name "AbstractRequirement") (modifiers (abstract (span (offset 553) (line 15) (column 5) (len 8)))) (body semicolon)) (requirement-def (name "VariationRequirement") (modifiers (variation (span (offset 603) (line 16) (column 5) (len 9)))) (body semicolon)) (constraint-def (name "AbstractConstraint") (modifiers (abstract (span (offset 655) (line 17) (column 5) (len 8)))) (specializes none) (body semicolon)) (constraint-def (name "VariationConstraint") (modifiers (variation (span (offset 703) (line 18) (column 5) (len 9)))) (specializes none) (body semicolon)) (view-def (name "AbstractView") (short-name none) (modifiers (abstract (span (offset 753) (line 19) (column 5) (len 8)))) (specializes none) (body semicolon)) (view-def (name "VariationView") (short-name none) (modifiers (variation (span (offset 789) (line 20) (column 5) (len 9)))) (specializes none) (body semicolon)) (viewpoint-def (modifiers (abstract (span (offset 827) (line 21) (column 5) (len 8))))) (viewpoint-def (modifiers (variation (span (offset 873) (line 22) (column 5) (len 9))))) (rendering-def (modifiers (abstract (span (offset 921) (line 23) (column 5) (len 8))))) (rendering-def (modifiers (variation (span (offset 967) (line 24) (column 5) (len 9))))) (occurrence-def (modifiers (abstract (span (offset 1015) (line 25) (column 5) (len 8))))) (occurrence-def (modifiers (variation (span (offset 1063) (line 26) (column 5) (len 9))))) (case-def (modifiers (abstract (span (offset 1113) (line 27) (column 5) (len 8))))) (case-def (modifiers (variation (span (offset 1149) (line 28) (column 5) (len 9))))) (analysis-case-def (modifiers (abstract (span (offset 1187) (line 29) (column 5) (len 8))))) (analysis-case-def (modifiers (variation (span (offset 1231) (line 30) (column 5) (len 9))))) (verification-case-def (name "AbstractVerification") (modifiers (abstract (span (offset 1277) (line 31) (column 5) (len 8)))) (body semicolon)) (verification-case-def (name "VariationVerification") (modifiers (variation (span (offset 1329) (line 32) (column 5) (len 9)))) (body semicolon)) (use-case-def (name "AbstractUseCase") (modifiers (abstract (span (offset 1383) (line 33) (column 5) (len 8)))) (body semicolon)) (use-case-def (name "VariationUseCase") (modifiers (variation (span (offset 1426) (line 34) (column 5) (len 9)))) (body semicolon)) (individual-def (modifiers (abstract (span (offset 1471) (line 35) (column 5) (len 8))))) (metadata-def (name "AbstractMetadata") (abstract true) (specializes none) (body semicolon)) (metadata-def (name "PlainMetadata") (abstract false) (specializes none) (body semicolon)) (enum-def (name "PlainEnum") (body semicolon)))))
)
~~~
