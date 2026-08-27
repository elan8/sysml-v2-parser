# META
~~~sexpr
(snapshot (type semantic) (description "`individual` on every definition kind that reaches `OccurrenceDefinitionPrefix` (SysML BNF 541; Pilot SysML.xtext 804)"))
~~~
# SOURCE
~~~sysml
package IndividualDefinitions {
    individual state def D6;
    individual calc def D8;
    individual constraint def D9;
    individual requirement def D10;
    individual concern def D11;
    individual case def D12;
    individual verification def D13;
    individual use case def D14;
    individual view def D15;
    individual viewpoint def D16;
    individual rendering def D17;
    abstract individual calc def D18 :> D8;
    private individual concern def D19;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "individual_occurrence_definitions.md"
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
  (root (package (name "IndividualDefinitions") (body brace (state-def (name "D6") (modifiers individual) (body semicolon)) (calc-def (name "D8") (modifiers individual) (body semicolon)) (constraint-def (name "D9") (modifiers individual) (specializes none) (body semicolon)) (requirement-def (name "D10") (modifiers individual) (body semicolon)) (concern-usage (name "D11") (visibility none) (abstract false) (individual true) (definition true) (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (case-def (modifiers)) (verification-case-def (name "D13") (modifiers) (body semicolon)) (use-case-def (name "D14") (modifiers individual) (body semicolon)) (view-def (name "D15") (short-name none) (modifiers individual) (specializes none) (body semicolon)) (viewpoint-def (modifiers)) (rendering-def (modifiers)) (calc-def (name "D18") (modifiers (abstract (span (offset 391) (line 13) (column 5) (len 8))) individual) (body semicolon)) (concern-usage (name "D19") (visibility private) (abstract false) (individual true) (definition true) (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon)))))
)
~~~
