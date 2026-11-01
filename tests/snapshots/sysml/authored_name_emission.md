# META
~~~sexpr
(snapshot (type semantic) (description "Exact source-backed declaration-name tokens preserve their authored quotes and escapes through formatting: part, attribute, port, ref, action and payload, binding connector, connection end, final state, case return, collection-operator parameter, and a KerML bare declaration. SysML Identification (pinned BNF 42-44) admits both BASIC_NAME and UNRESTRICTED_NAME, while the pinned Pilot grammar has the same Name alternative; these owners already retain the exact token span, unlike aggregate Identification spans deliberately outside this migration."))
~~~
# SOURCE
~~~sysml
package AuthoredNameEmission {
    part def Holder {
        part 'part label';
        part 'can\'t';
        attribute 'attribute label';
        port 'port label';
        ref 'ref label';
        action 'action label';
        binding 'binding label' bind left = right;
        datatype 'bare label';
    }

    action def Payloads {
        accept 'payload label' : Payload;
    }

    connection def Link {
        end 'end label';
    }

    state def States {
    }

    verification def Verification {
        return 'return label';
    }

    private import Filtered[items->forAll { in 'parameter label'; 'parameter label' == selected }];
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "authored_name_emission.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package AuthoredNameEmission {
    part def Holder {
        part 'part label';
        part 'can\'t';
        attribute 'attribute label';
        port 'port label';
        ref 'ref label';
        action 'action label';
        binding 'binding label' bind left = right;
        datatype 'bare label';
    }
    action def Payloads {
        accept 'payload label' : Payload;
    }
    connection def Link {
        end 'end label';
    }
    state def States {
    }
    verification def Verification {
        return 'return label';
    }
    private import Filtered [items->forAll { in 'parameter label'; 'parameter label' == selected }];
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 371) (line 14) (column 34) (len 7)) (segments (segment 0 (token "Payload") (name "Payload") (separator none) (span (offset 371) (line 14) (column 34) (len 7)))))
    (reference r1 (scope relative) (span (offset 568) (line 28) (column 20) (len 8)) (segments (segment 0 (token "Filtered") (name "Filtered") (separator none) (span (offset 568) (line 28) (column 20) (len 8)))))
    (reference r2 (scope relative) (span (offset 577) (line 28) (column 29) (len 5)) (segments (segment 0 (token "items") (name "items") (separator none) (span (offset 577) (line 28) (column 29) (len 5)))))
    (reference r3 (scope relative) (span (offset 615) (line 28) (column 67) (len 17)) (segments (segment 0 (token "'parameter label'") (name "parameter label") (separator none) (span (offset 615) (line 28) (column 67) (len 17)))))
    (reference r4 (scope relative) (span (offset 636) (line 28) (column 88) (len 8)) (segments (segment 0 (token "selected") (name "selected") (separator none) (span (offset 636) (line 28) (column 88) (len 8)))))
  )
  (root (package (visibility none) (name "AuthoredNameEmission") (body brace (part-def (name "Holder") (modifiers) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "part label") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "can't") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "attribute label") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "port label") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (ref (name "ref label") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)) (action-usage) (bind) (kerml-classifier (keyword datatype) (abstract false) (name "bare label") (multiplicity none) (specializes none) (conjugates none) (body semicolon)))) (action-def (name "Payloads") (modifiers) (specializes none) (body brace (action-usage (keyword accept) (name none) (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (accept (payload (name "payload label") (type (ref r0)) (via none))) (body semicolon)))) (connection-def (name "Link") (modifiers) (extensions) (specializes none) (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions) (short-name none) (identity (declaration (name "end label") (span (offset 425) (line 18) (column 13) (len 11)))) (typing none) (references none) (multiplicity none) (redefines none) (crosses none)))) (state-def (name "States") (modifiers) (body brace)) (verification-case-def (name "Verification") (modifiers) (body brace (case-return (declaration "return label") (target none) (type none) (redefines none) (feature-kind none) (subsetting false) (value none)))) (import (visibility private) (target (span (span (offset 568) (line 28) (column 20) (len 79))) (all none) (ref r1) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 576) (line 28) (column 28) (len 71))) (open (span (offset 576) (line 28) (column 28) (len 1))) (expression (expression (span (offset 577) (line 28) (column 29) (len 69)) (collection-op (operator "forAll") (base (expression (span (offset 577) (line 28) (column 29) (len 5)) (ref r2))) (arguments) (brace-body (body (span (offset 591) (line 28) (column 43) (len 55)) (open-brace (span (offset 591) (line 28) (column 43) (len 1))) (parameters (parameter (span (offset 593) (line 28) (column 45) (len 21)) (direction in (span (offset 593) (line 28) (column 45) (len 2))) (reference-keyword none) (declaration (name "parameter label") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (terminator (semicolon (span (offset 613) (line 28) (column 65) (len 1)))))) (result (expression (span (offset 615) (line 28) (column 67) (len 29)) (binary (operator "==") (left (expression (span (offset 615) (line 28) (column 67) (len 17)) (ref r3))) (right (expression (span (offset 636) (line 28) (column 88) (len 8)) (ref r4)))))) (close-brace (span (offset 645) (line 28) (column 97) (len 1)))))))) (close (span (offset 646) (line 28) (column 98) (len 1)))))))) (body semicolon)))))
)
~~~
