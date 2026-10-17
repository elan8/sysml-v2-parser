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
    (reference r0 (scope relative) (span (offset 80) (line 3) (column 17) (len 10)) (segments (segment 0 (token "SizeChoice") (name "SizeChoice") (separator none) (span (offset 80) (line 3) (column 17) (len 10)))))
    (reference r1 (scope relative) (span (offset 175) (line 5) (column 36) (len 5)) (segments (segment 0 (token "Check") (name "Check") (separator none) (span (offset 175) (line 5) (column 36) (len 5)))))
    (reference r2 (scope relative) (span (offset 342) (line 11) (column 28) (len 17)) (segments (segment 0 (token "LogicalToPhysical") (name "LogicalToPhysical") (separator none) (span (offset 342) (line 11) (column 28) (len 17)))))
    (reference r3 (scope relative) (span (offset 379) (line 12) (column 9) (len 6)) (segments (segment 0 (token "source") (name "source") (separator none) (span (offset 379) (line 12) (column 9) (len 6)))))
    (reference r4 (scope relative) (span (offset 390) (line 12) (column 20) (len 7)) (segments (segment 0 (token "logical") (name "logical") (separator none) (span (offset 390) (line 12) (column 20) (len 7)))))
    (reference r5 (scope relative) (span (offset 407) (line 13) (column 9) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 407) (line 13) (column 9) (len 6)))))
    (reference r6 (scope relative) (span (offset 418) (line 13) (column 20) (len 8)) (segments (segment 0 (token "physical") (name "physical") (separator none) (span (offset 418) (line 13) (column 20) (len 8)))))
    (reference r7 (scope relative) (span (offset 479) (line 16) (column 23) (len 10)) (segments (segment 0 (token "SizeChoice") (name "SizeChoice") (separator none) (span (offset 479) (line 16) (column 23) (len 10)))))
  )
  (root (package (name "P") (body brace (enum-def (name "SizeChoice") (body brace (enum-value (extensions) (enum-keyword present) (visibility none) (name "small") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon) (span (offset 38) (line 2) (column 27) (len 11))) (enum-value (extensions) (enum-keyword present) (visibility none) (name "large") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon) (span (offset 50) (line 2) (column 39) (len 11))))) (enumeration-usage (name "size") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (body semicolon)) (verification-case-def (name "Check") (modifiers) (body semicolon)) (verification-case-usage (declaration (name "check") (short-name none) (abstract false) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (subsets none)) (body semicolon)) (use-case-usage (declaration (name "operate") (short-name none) (abstract false) (type none) (subsets none)) (variation true) (body semicolon)) (state-usage (name "mode") (short-name none) (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "logical") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "physical") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (allocation-def (name "LogicalToPhysical") (modifiers) (body semicolon)) (allocation-usage (name "allocated") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (ends (connector-end (multiplicity none) (target (ref r3)) (references (ref r4))) (connector-end (multiplicity none) (target (ref r5)) (references (ref r6)))) (body semicolon)) (part-def (name "Vehicle") (modifiers) (body brace (enumeration-usage (name "chosen") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (body semicolon)))))))
)
~~~
