# META
~~~sexpr
(snapshot (type semantic) (description "Every slot of the shared OccurrenceUsagePrefix on a ConstraintUsage (ConstraintUsage = OccurrenceUsagePrefix 'constraint' ConstraintUsageDeclaration CalculationBody, SysML BNF 1382), and `abstract` on a ConstraintDefinition, which the parser consumed and discarded. Each modifier appears alone, then in the full legal order and in materially different combinations, in every scope that owns a constraint usage: package, constraint def body, part def body, part usage body, item body and requirement def body. Both body forms appear, and a part usage appears inside a calculation-shaped body, which CalculationBodyItem -> ActionBodyItem -> NonBehaviorBodyItem -> StructureUsageMember admits. MemberPrefix visibility belongs to the membership rather than to the prefix, so it is shown separately and before it."))
~~~
# SOURCE
~~~sysml
package ConstraintPrefixAlternatives {
    metadata def Tag;
    part def Engine;
    abstract constraint def Base;
    constraint def Plain {
        in constraint directedIn;
        out constraint directedOut;
        inout constraint directedInOut;
        derived constraint isDerived;
        abstract constraint isAbstract;
        variation constraint isVariation;
        constant constraint isConstant;
        ref constraint isReference;
        individual constraint isIndividual;
        snapshot constraint isSnapshot;
        timeslice constraint isTimeslice;
        #Tag constraint oneKeyword;
        #Tag #Tag constraint twoKeywords;
        in derived abstract constant ref individual snapshot constraint everySlot;
        ref constraint self : Base :>> Plain::self;
        private ref constraint hidden;
        part inCalculationBody : Engine;
        constraint braced {
        }
    }
    abstract constraint packageLevel : Base;
    #Tag constraint taggedAtPackageLevel;
    part def PartScope {
        ref constraint inPartDef;
    }
    part partUsageScope : Engine {
        abstract constraint inPartUsage;
    }
    item def ItemScope {
        ref constraint inItemBody;
    }
    requirement def RequirementScope {
        abstract constraint inRequirementDef;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "constraint_usage_prefix_alternatives.md"
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
    (reference r0 (scope relative) (span (offset 584) (line 17) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 584) (line 17) (column 10) (len 3)))))
    (reference r1 (scope relative) (span (offset 620) (line 18) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 620) (line 18) (column 10) (len 3)))))
    (reference r2 (scope relative) (span (offset 625) (line 18) (column 15) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 625) (line 18) (column 15) (len 3)))))
    (reference r3 (scope relative) (span (offset 766) (line 20) (column 31) (len 4)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 766) (line 20) (column 31) (len 4)))))
    (reference r4 (scope relative) (span (offset 775) (line 20) (column 40) (len 11)) (segments (segment 0 (token "Plain") (name "Plain") (separator none) (span (offset 775) (line 20) (column 40) (len 5))) (segment 1 (token "self") (name "self") (separator colon-colon) (span (offset 782) (line 20) (column 47) (len 4)))))
    (reference r5 (scope relative) (span (offset 860) (line 22) (column 34) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 860) (line 22) (column 34) (len 6)))))
    (reference r6 (scope relative) (span (offset 951) (line 26) (column 40) (len 4)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 951) (line 26) (column 40) (len 4)))))
    (reference r7 (scope relative) (span (offset 962) (line 27) (column 6) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 962) (line 27) (column 6) (len 3)))))
    (reference r8 (scope relative) (span (offset 1090) (line 31) (column 27) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 1090) (line 31) (column 27) (len 6)))))
  )
  (root (package (name "ConstraintPrefixAlternatives") (body brace (metadata-def (name "Tag") (abstract false) (specializes none) (body semicolon)) (part-def (name "Engine") (modifiers) (body semicolon)) (constraint-def (name "Base") (modifiers (abstract (span (offset 86) (line 4) (column 5) (len 8)))) (specializes none) (body semicolon)) (constraint-def (name "Plain") (modifiers) (specializes none) (body brace (constraint-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "directedIn") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (constraint-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "directedOut") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (constraint-usage (prefix (direction inout) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "directedInOut") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (constraint-usage (prefix (direction none) (derived true) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "isDerived") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (constraint-usage (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "isAbstract") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (constraint-usage (prefix (direction none) (derived false) (variance variation) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "isVariation") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant true) (reference false) (individual false) (portion none) (extensions)) (declaration-name "isConstant") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "isReference") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion none) (extensions)) (declaration-name "isIndividual") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration-name "isSnapshot") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion timeslice) (extensions)) (declaration-name "isTimeslice") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r0))) (declaration-name "oneKeyword") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r1) (ref r2))) (declaration-name "twoKeywords") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (constraint-usage (prefix (direction in) (derived true) (variance abstract) (constant true) (reference true) (individual true) (portion snapshot) (extensions)) (declaration-name "everySlot") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "self") (short-name none) (type (ref r3)) (multiplicity none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (body semicolon)) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "hidden") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "inCalculationBody") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "braced") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body brace)))) (constraint-usage (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "packageLevel") (short-name none) (type (ref r6)) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r7))) (declaration-name "taggedAtPackageLevel") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (part-def (name "PartScope") (modifiers) (body brace (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "inPartDef") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "partUsageScope") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (constraint-usage (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "inPartUsage") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon)))) (item-def (name "ItemScope") (modifiers) (individual false) (specializes none) (body brace (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "inItemBody") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon)))) (requirement-def (name "RequirementScope") (modifiers) (body brace (constraint))))))
)
~~~
