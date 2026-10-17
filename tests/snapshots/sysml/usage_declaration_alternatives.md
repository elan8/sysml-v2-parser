# META
~~~sexpr
(snapshot (type semantic) (description "SysML usage forms that have typed parsers: an enumeration usage's ValuePart, the variation prefix on verification, use case and state usages, and the n-ary allocate ConnectorPart."))
~~~
# SOURCE
~~~sysml
package P {
    enum def SizeChoice { enum small; enum large; }
    enum size : SizeChoice = SizeChoice::small;
    verification def Check;
    variation verification check : Check;
    variation use case operate;
    variation state mode;
    part logical;
    part physical;
    allocation def LogicalToPhysical;
    allocation allocated : LogicalToPhysical allocate (
        source ::> logical,
        target ::> physical
    );
    part def Vehicle {
        enum chosen : SizeChoice = SizeChoice::large;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "usage_declaration_alternatives.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package P {
    enum def SizeChoice {
        enum small;
        enum large;
    }
    enum size : SizeChoice = SizeChoice::small;
    verification def Check;
    variation verification check : Check;
    variation use case operate;
    variation state mode;
    part logical;
    part physical;
    allocation def LogicalToPhysical;
    allocation allocated : LogicalToPhysical allocate (source references logical, target references physical);
    part def Vehicle {
        enum chosen : SizeChoice = SizeChoice::large;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
  )
  (root (package (name "P") (body brace (enum-def (name "SizeChoice") (body brace (enum-value (extensions) (enum-keyword present) (visibility none) (name "small") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon) (span (offset 38) (line 2) (column 27) (len 11))) (enum-value (extensions) (enum-keyword present) (visibility none) (name "large") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon) (span (offset 50) (line 2) (column 39) (len 11))))) (enumeration-usage (name "size") (short-name none)) (verification-case-def (name "Check") (modifiers) (body semicolon)) (verification-case-usage (name "check") (short-name none)) (use-case-usage) (state-usage (name "mode") (short-name none) (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "logical") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "physical") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (allocation-def (name "LogicalToPhysical") (modifiers)) (allocation-usage (name "allocated") (short-name none)) (part-def (name "Vehicle") (modifiers) (body brace (enumeration-usage (name "chosen") (short-name none)))))))
)
~~~
