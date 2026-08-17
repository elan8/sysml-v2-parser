# META
~~~sexpr
(snapshot (type semantic) (description "Every alternative of the two sigil productions, side by side. `@` is MetadataFeature, whose MetadataFeatureDeclaration ends at a required OwnedFeatureTyping and may be preceded by a declared Identification separated by `:` or `typed by`; `#` is PrefixMetadataFeature, which is that typing alone -- no declaration, no separator, no about clause. Absolute and relative references, `::` separators, quoted names, and the `;`/`{}` body forms all appear so the projection distinguishes them."))
~~~
# SOURCE
~~~sysml
package MetadataSigilAlternatives {
    package Profile {
        metadata def Tag;
        metadata def 'safety critical';
    }
    part def AnnotatedByType {
        @Profile::Tag;
        @$::MetadataSigilAlternatives::Profile::Tag;
        @Profile::'safety critical';
    }
    part def AnnotatedByDeclaration {
        @named : Profile::Tag;
        @spelled typed by Profile::Tag;
        @<short> : Profile::Tag;
        @<short> named : Profile::Tag;
    }
    part def AnnotatedAbout {
        @Profile::Tag about AnnotatedByType, Profile::Tag;
    }
    part def AnnotatedBodies {
        @Profile::Tag;
        @Profile::Tag {
            doc /* braced metadata body */
        }
    }
    #Profile::Tag part def HashPrefixed;
    part def HashMembers {
        #Profile::Tag;
        #Profile::'safety critical';
        #Profile::Tag {
            doc /* braced extended-usage body */
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "metadata_sigil_alternatives.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package MetadataSigilAlternatives {
    package Profile {
        metadata def Tag;
        metadata def 'safety critical';
    }
    part def AnnotatedByType {
        @Profile::Tag;
        @$::MetadataSigilAlternatives::Profile::Tag;
        @Profile::'safety critical';
    }
    part def AnnotatedByDeclaration {
        @named : Profile::Tag;
        @spelled typed by Profile::Tag;
        @<short> : Profile::Tag;
        @<short> named : Profile::Tag;
    }
    part def AnnotatedAbout {
        @Profile::Tag about AnnotatedByType, Profile::Tag;
    }
    part def AnnotatedBodies {
        @Profile::Tag;
        @Profile::Tag {
            doc
            /* braced metadata body */
        }
    }
    #Profile::Tag
    part def HashPrefixed;
    part def HashMembers {
        #Profile::Tag;
        #Profile::'safety critical';
        #Profile::Tag {
            doc
            /* braced extended-usage body */
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 170) (line 7) (column 10) (len 12)) (segments (segment 0 (token "Profile") (name "Profile") (separator none) (span (offset 170) (line 7) (column 10) (len 7))) (segment 1 (token "Tag") (name "Tag") (separator colon-colon) (span (offset 179) (line 7) (column 19) (len 3)))))
    (reference r1 (scope absolute) (span (offset 193) (line 8) (column 10) (len 42)) (segments (segment 0 (token "MetadataSigilAlternatives") (name "MetadataSigilAlternatives") (separator none) (span (offset 196) (line 8) (column 13) (len 25))) (segment 1 (token "Profile") (name "Profile") (separator colon-colon) (span (offset 223) (line 8) (column 40) (len 7))) (segment 2 (token "Tag") (name "Tag") (separator colon-colon) (span (offset 232) (line 8) (column 49) (len 3)))))
    (reference r2 (scope relative) (span (offset 246) (line 9) (column 10) (len 26)) (segments (segment 0 (token "Profile") (name "Profile") (separator none) (span (offset 246) (line 9) (column 10) (len 7))) (segment 1 (token "'safety critical'") (name "safety critical") (separator colon-colon) (span (offset 255) (line 9) (column 19) (len 17)))))
    (reference r3 (scope relative) (span (offset 335) (line 12) (column 18) (len 12)) (segments (segment 0 (token "Profile") (name "Profile") (separator none) (span (offset 335) (line 12) (column 18) (len 7))) (segment 1 (token "Tag") (name "Tag") (separator colon-colon) (span (offset 344) (line 12) (column 27) (len 3)))))
    (reference r4 (scope relative) (span (offset 375) (line 13) (column 27) (len 12)) (segments (segment 0 (token "Profile") (name "Profile") (separator none) (span (offset 375) (line 13) (column 27) (len 7))) (segment 1 (token "Tag") (name "Tag") (separator colon-colon) (span (offset 384) (line 13) (column 36) (len 3)))))
    (reference r5 (scope relative) (span (offset 408) (line 14) (column 20) (len 12)) (segments (segment 0 (token "Profile") (name "Profile") (separator none) (span (offset 408) (line 14) (column 20) (len 7))) (segment 1 (token "Tag") (name "Tag") (separator colon-colon) (span (offset 417) (line 14) (column 29) (len 3)))))
    (reference r6 (scope relative) (span (offset 447) (line 15) (column 26) (len 12)) (segments (segment 0 (token "Profile") (name "Profile") (separator none) (span (offset 447) (line 15) (column 26) (len 7))) (segment 1 (token "Tag") (name "Tag") (separator colon-colon) (span (offset 456) (line 15) (column 35) (len 3)))))
    (reference r7 (scope relative) (span (offset 506) (line 18) (column 10) (len 12)) (segments (segment 0 (token "Profile") (name "Profile") (separator none) (span (offset 506) (line 18) (column 10) (len 7))) (segment 1 (token "Tag") (name "Tag") (separator colon-colon) (span (offset 515) (line 18) (column 19) (len 3)))))
    (reference r8 (scope relative) (span (offset 525) (line 18) (column 29) (len 15)) (segments (segment 0 (token "AnnotatedByType") (name "AnnotatedByType") (separator none) (span (offset 525) (line 18) (column 29) (len 15)))))
    (reference r9 (scope relative) (span (offset 542) (line 18) (column 46) (len 12)) (segments (segment 0 (token "Profile") (name "Profile") (separator none) (span (offset 542) (line 18) (column 46) (len 7))) (segment 1 (token "Tag") (name "Tag") (separator colon-colon) (span (offset 551) (line 18) (column 55) (len 3)))))
    (reference r10 (scope relative) (span (offset 602) (line 21) (column 10) (len 12)) (segments (segment 0 (token "Profile") (name "Profile") (separator none) (span (offset 602) (line 21) (column 10) (len 7))) (segment 1 (token "Tag") (name "Tag") (separator colon-colon) (span (offset 611) (line 21) (column 19) (len 3)))))
    (reference r11 (scope relative) (span (offset 625) (line 22) (column 10) (len 12)) (segments (segment 0 (token "Profile") (name "Profile") (separator none) (span (offset 625) (line 22) (column 10) (len 7))) (segment 1 (token "Tag") (name "Tag") (separator colon-colon) (span (offset 634) (line 22) (column 19) (len 3)))))
    (reference r12 (scope relative) (span (offset 704) (line 26) (column 6) (len 12)) (segments (segment 0 (token "Profile") (name "Profile") (separator none) (span (offset 704) (line 26) (column 6) (len 7))) (segment 1 (token "Tag") (name "Tag") (separator colon-colon) (span (offset 713) (line 26) (column 15) (len 3)))))
    (reference r13 (scope relative) (span (offset 776) (line 28) (column 10) (len 12)) (segments (segment 0 (token "Profile") (name "Profile") (separator none) (span (offset 776) (line 28) (column 10) (len 7))) (segment 1 (token "Tag") (name "Tag") (separator colon-colon) (span (offset 785) (line 28) (column 19) (len 3)))))
    (reference r14 (scope relative) (span (offset 799) (line 29) (column 10) (len 26)) (segments (segment 0 (token "Profile") (name "Profile") (separator none) (span (offset 799) (line 29) (column 10) (len 7))) (segment 1 (token "'safety critical'") (name "safety critical") (separator colon-colon) (span (offset 808) (line 29) (column 19) (len 17)))))
    (reference r15 (scope relative) (span (offset 836) (line 30) (column 10) (len 12)) (segments (segment 0 (token "Profile") (name "Profile") (separator none) (span (offset 836) (line 30) (column 10) (len 7))) (segment 1 (token "Tag") (name "Tag") (separator colon-colon) (span (offset 845) (line 30) (column 19) (len 3)))))
  )
  (root (package (name "MetadataSigilAlternatives") (body brace (package (name "Profile") (body brace (metadata-def) (metadata-def))) (part-def (name "AnnotatedByType") (body brace (metadata-annotation (declared-name none) (type (ref r0)) (about) (body semicolon)) (metadata-annotation (declared-name none) (type (ref r1)) (about) (body semicolon)) (metadata-annotation (declared-name none) (type (ref r2)) (about) (body semicolon)))) (part-def (name "AnnotatedByDeclaration") (body brace (metadata-annotation (declared-name (name "named") (short-name none) (typed-by colon)) (type (ref r3)) (about) (body semicolon)) (metadata-annotation (declared-name (name "spelled") (short-name none) (typed-by typed-by)) (type (ref r4)) (about) (body semicolon)) (metadata-annotation (declared-name (name none) (short-name "short") (typed-by colon)) (type (ref r5)) (about) (body semicolon)) (metadata-annotation (declared-name (name "named") (short-name "short") (typed-by colon)) (type (ref r6)) (about) (body semicolon)))) (part-def (name "AnnotatedAbout") (body brace (metadata-annotation (declared-name none) (type (ref r7)) (about (ref r8) (ref r9)) (body semicolon)))) (part-def (name "AnnotatedBodies") (body brace (metadata-annotation (declared-name none) (type (ref r10)) (about) (body semicolon)) (metadata-annotation (declared-name none) (type (ref r11)) (about) (body brace (element-count 1))))) (metadata-keyword-usage (type (ref r12)) (body none)) (part-def (name "HashPrefixed") (body semicolon)) (part-def (name "HashMembers") (body brace (metadata-keyword-usage (type (ref r13)) (body semicolon)) (metadata-keyword-usage (type (ref r14)) (body semicolon)) (metadata-keyword-usage (type (ref r15)) (body brace (element-count 1))))))))
)
~~~
