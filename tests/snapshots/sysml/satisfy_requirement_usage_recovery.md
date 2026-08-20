# META
~~~sexpr
(snapshot (type recovery) (description "Malformed satisfy syntax stays an explicit recovery node at its authored span and is never turned into an apparently valid shorthand: no cloned reference stands in for a missing `by` target, and no empty declaration stands in for a malformed header. Malformed content before a satisfy usage, a `by` with nothing to name, a header that no alternative continues, a malformed member inside the braced RequirementBody, and delimiters that appear only inside comments and quoted names are each shown with the valid siblings that follow them intact. Recovery synchronizes on `assert` and `not` as well as on `satisfy`, because all three are FIRST tokens of the same production: a malformed member before a prefixed satisfy usage must stop at the prefix rather than scan past it and take that usage terminator with it."))
~~~
# SOURCE
~~~sysml
package SatisfyRecovery {
    requirement def Spec;
    part target;
    part def MalformedBeforeSatisfy {
        part beforeMalformed;
        @@@
        satisfy Spec by target;
        part afterMalformed;
    }
    part def MissingBySubject {
        part beforeMissing;
        satisfy Spec by ;
        part afterMissing;
    }
    part def MalformedHeader {
        part beforeHeader;
        satisfy by target;
        satisfy Spec = ;
        part afterHeader;
    }
    part def MalformedBodyMember {
        part beforeBody;
        satisfy Spec by target {
            @@@
            require constraint stillParsed;
        }
        part afterBody;
        part alsoAfterBody;
    }
    part def DelimitersInLexicalContent {
        satisfy Spec by target {
            doc
            /* a } inside a comment does not close this body */
            require constraint 'a } inside a quoted name';
        }
        part afterLexicalContent;
    }
    part def NestedBraces {
        satisfy Spec by target {
            requirement nested {
                doc
                /* nested */
            }
        }
        part afterNested;
    }
    part def MalformedBeforePrefixedSatisfy {
        bogus token here
        not satisfy Spec by target;
        part afterNegated;
    }
    part beforeAsserted {
        bogus token here
        assert satisfy Spec by target;
        part afterAsserted;
    }
    view assertedInViewBody : SomeView {
        bogus token here
        assert not satisfy Spec by target;
        satisfy Spec;
    }
    requirement def NegatedInRequirementBody {
        bogus token here
        not satisfy Spec by target;
        require constraint afterNegated;
    }
    bogus token here
    assert not satisfy Spec by target;
    part afterPackageLevelRecovery;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "satisfy_requirement_usage_recovery.md"
    (diagnostics
      (diagnostic (code "missing_semicolon") (severity error) (category parseerror) (span (offset 145) (line 6) (column 9) (len 12)) (message "missing semicolon before next declaration"))
      (diagnostic (code "missing_expression_after_operator") (severity error) (category parseerror) (span (offset 284) (line 12) (column 9) (len 26)) (message "expected target after 'by'"))
      (diagnostic (code "recovered_part_def_body_element") (severity error) (category parseerror) (span (offset 401) (line 17) (column 9) (len 27)) (message "unexpected token in part definition body"))
      (diagnostic (code "missing_expression_after_operator") (severity error) (category parseerror) (span (offset 428) (line 18) (column 9) (len 25)) (message "expected expression after '='"))
      (diagnostic (code "missing_semicolon") (severity error) (category parseerror) (span (offset 582) (line 24) (column 13) (len 16)) (message "missing semicolon before next declaration"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 1215) (line 48) (column 9) (len 25)) (message "unrecognized declaration `bogus` in part definition body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 1335) (line 53) (column 9) (len 25)) (message "unrecognized declaration `bogus` in part usage body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 1474) (line 58) (column 9) (len 25)) (message "unrecognized declaration `bogus` in view body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 1617) (line 63) (column 9) (len 25)) (message "unrecognized declaration `bogus` in requirement body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 1721) (line 67) (column 5) (len 21)) (message "unrecognized declaration `bogus` in package body"))
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
    (reference r0 (scope relative) (span (offset 165) (line 7) (column 17) (len 4)) (segments (segment 0 (token "Spec") (name "Spec") (separator none) (span (offset 165) (line 7) (column 17) (len 4)))))
    (reference r1 (scope relative) (span (offset 173) (line 7) (column 25) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 173) (line 7) (column 25) (len 6)))))
    (reference r2 (scope relative) (span (offset 553) (line 23) (column 17) (len 4)) (segments (segment 0 (token "Spec") (name "Spec") (separator none) (span (offset 553) (line 23) (column 17) (len 4)))))
    (reference r3 (scope relative) (span (offset 561) (line 23) (column 25) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 561) (line 23) (column 25) (len 6)))))
    (reference r4 (scope relative) (span (offset 756) (line 31) (column 17) (len 4)) (segments (segment 0 (token "Spec") (name "Spec") (separator none) (span (offset 756) (line 31) (column 17) (len 4)))))
    (reference r5 (scope relative) (span (offset 764) (line 31) (column 25) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 764) (line 31) (column 25) (len 6)))))
    (reference r6 (scope relative) (span (offset 1006) (line 39) (column 17) (len 4)) (segments (segment 0 (token "Spec") (name "Spec") (separator none) (span (offset 1006) (line 39) (column 17) (len 4)))))
    (reference r7 (scope relative) (span (offset 1014) (line 39) (column 25) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 1014) (line 39) (column 25) (len 6)))))
    (reference r8 (scope relative) (span (offset 1252) (line 49) (column 21) (len 4)) (segments (segment 0 (token "Spec") (name "Spec") (separator none) (span (offset 1252) (line 49) (column 21) (len 4)))))
    (reference r9 (scope relative) (span (offset 1260) (line 49) (column 29) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 1260) (line 49) (column 29) (len 6)))))
    (reference r10 (scope relative) (span (offset 1375) (line 54) (column 24) (len 4)) (segments (segment 0 (token "Spec") (name "Spec") (separator none) (span (offset 1375) (line 54) (column 24) (len 4)))))
    (reference r11 (scope relative) (span (offset 1383) (line 54) (column 32) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 1383) (line 54) (column 32) (len 6)))))
    (reference r12 (scope relative) (span (offset 1455) (line 57) (column 31) (len 8)) (segments (segment 0 (token "SomeView") (name "SomeView") (separator none) (span (offset 1455) (line 57) (column 31) (len 8)))))
    (reference r13 (scope relative) (span (offset 1518) (line 59) (column 28) (len 4)) (segments (segment 0 (token "Spec") (name "Spec") (separator none) (span (offset 1518) (line 59) (column 28) (len 4)))))
    (reference r14 (scope relative) (span (offset 1526) (line 59) (column 36) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 1526) (line 59) (column 36) (len 6)))))
    (reference r15 (scope relative) (span (offset 1550) (line 60) (column 17) (len 4)) (segments (segment 0 (token "Spec") (name "Spec") (separator none) (span (offset 1550) (line 60) (column 17) (len 4)))))
    (reference r16 (scope relative) (span (offset 1654) (line 64) (column 21) (len 4)) (segments (segment 0 (token "Spec") (name "Spec") (separator none) (span (offset 1654) (line 64) (column 21) (len 4)))))
    (reference r17 (scope relative) (span (offset 1662) (line 64) (column 29) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 1662) (line 64) (column 29) (len 6)))))
    (reference r18 (scope relative) (span (offset 1761) (line 68) (column 24) (len 4)) (segments (segment 0 (token "Spec") (name "Spec") (separator none) (span (offset 1761) (line 68) (column 24) (len 4)))))
    (reference r19 (scope relative) (span (offset 1769) (line 68) (column 32) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 1769) (line 68) (column 32) (len 6)))))
  )
  (root (package (name "SatisfyRecovery") (body brace (requirement-def (name "Spec") (modifiers) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "target") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-def (name "MalformedBeforeSatisfy") (modifiers) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "beforeMalformed") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (malformed (code "missing_semicolon") (found "@@@") (span (offset 145) (line 6) (column 9) (len 12))) (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r0))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r1)) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "afterMalformed") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "MissingBySubject") (modifiers) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "beforeMissing") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (malformed (code "missing_expression_after_operator") (found "satisfy Spec by ;") (span (offset 284) (line 12) (column 9) (len 26))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "afterMissing") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "MalformedHeader") (modifiers) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "beforeHeader") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (malformed (code "recovered_part_def_body_element") (found "satisfy by target;") (span (offset 401) (line 17) (column 9) (len 27))) (malformed (code "missing_expression_after_operator") (found "satisfy Spec = ;") (span (offset 428) (line 18) (column 9) (len 25))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "afterHeader") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "MalformedBodyMember") (modifiers) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "beforeBody") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r2))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r3)) (body brace (malformed (code "missing_semicolon") (found "@@@") (span (offset 582) (line 24) (column 13) (len 16))) (require-constraint))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "afterBody") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "alsoAfterBody") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "DelimitersInLexicalContent") (modifiers) (body brace (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r4))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r5)) (body brace (doc (name none) (locale none) (body (span (offset 803) (line 33) (column 15) (len 47)) (normalized "a } inside a comment does not close this body "))) (require-constraint))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "afterLexicalContent") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "NestedBraces") (modifiers) (body brace (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r6))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r7)) (body brace (requirement-usage (name "nested") (multiplicity none)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "afterNested") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "MalformedBeforePrefixedSatisfy") (modifiers) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "bogus token here") (span (offset 1215) (line 48) (column 9) (len 25))) (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated true) (requirement (reference (ref r8))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r9)) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "afterNegated") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "beforeAsserted") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "bogus token here") (span (offset 1335) (line 53) (column 9) (len 25))) (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert true) (negated false) (requirement (reference (ref r10))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r11)) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "afterAsserted") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (view (name "assertedInViewBody") (short-name none) (type (ref r12)) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "bogus token here") (span (offset 1474) (line 58) (column 9) (len 25))) (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert true) (negated true) (requirement (reference (ref r13))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r14)) (body semicolon)) (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r15))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (redefines none) (crosses none) (value none) (by none) (body semicolon)))) (requirement-def (name "NegatedInRequirementBody") (modifiers) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "bogus token here") (span (offset 1617) (line 63) (column 9) (len 25))) (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated true) (requirement (reference (ref r16))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r17)) (body semicolon)) (require-constraint))) (malformed (code "unrecognized_declaration_in_scope") (found "bogus token here") (span (offset 1721) (line 67) (column 5) (len 21))) (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert true) (negated true) (requirement (reference (ref r18))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r19)) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "afterPackageLevelRecovery") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))))
)
~~~
