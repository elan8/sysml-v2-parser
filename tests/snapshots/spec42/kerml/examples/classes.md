# META
~~~sexpr
(snapshot (type semantic) (description "Pinned KerML Simple Tests/Classes.kerml source: the private nested struct TypeBody retains `alias z for y;` between its typed feature and composite feature siblings (KerML textual BNF TypeBodyElement; Pilot agrees)."))
~~~
# SOURCE
~~~sysml
package Classes {
	
	feature f: A;

	public class <'1'> A { 
		feature b: B;
		protected in c: C;
		portion feature p : A;
	}
	
	abstract class <'2'> B {
		public abstract feature a: A {
			composite feature aa: A;
		}
		public composite feature a1: A;
		feature x {
			composite feature a: A {
			    portion feature q : A;
			}
			portion feature q : A;
		}
		package P { }
	}
	
	private struct C specializes Classes::'2' {
		private y: A, '2'[0..*];
		alias z for y;
		composite feature c : C {
			composite feature cc : C;
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "classes.md"
    (diagnostics
      (diagnostic (code "recovered_calc_body_element") (severity error) (category parseerror) (span (offset 437) (line 26) (column 12) (len 18)) (message "unexpected token in calc body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package Classes {
    feature f : A;
    public class <'1'> A {
        feature b : B;
        protected;
        in c : C;
        portion feature p : A;
    }
    abstract class <'2'> B {
        public abstract feature a : A {
            composite feature aa : A;
        }
        public composite feature a1 : A;
        feature x {
            composite feature a : A {
                portion feature q : A;
            }
            portion feature q : A;
        }
        package P {
        }
    }
    private struct C specializes Classes::'2' {
        private;
        y;
        : A, '2'[0..*];
        alias z for y;
        composite feature c : C {
            composite feature cc : C;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 32) (line 3) (column 13) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 32) (line 3) (column 13) (len 1)))))
    (reference r1 (scope relative) (span (offset 74) (line 6) (column 14) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 74) (line 6) (column 14) (len 1)))))
    (reference r2 (scope relative) (span (offset 79) (line 7) (column 3) (len 9)) (segments (segment 0 (token "protected") (name "protected") (separator none) (span (offset 79) (line 7) (column 3) (len 9)))))
    (reference r3 (scope relative) (span (offset 95) (line 7) (column 19) (len 1)) (segments (segment 0 (token "C") (name "C") (separator none) (span (offset 95) (line 7) (column 19) (len 1)))))
    (reference r4 (scope relative) (span (offset 120) (line 8) (column 23) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 120) (line 8) (column 23) (len 1)))))
    (reference r5 (scope relative) (span (offset 183) (line 12) (column 30) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 183) (line 12) (column 30) (len 1)))))
    (reference r6 (scope relative) (span (offset 212) (line 13) (column 26) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 212) (line 13) (column 26) (len 1)))))
    (reference r7 (scope relative) (span (offset 250) (line 15) (column 32) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 250) (line 15) (column 32) (len 1)))))
    (reference r8 (scope relative) (span (offset 291) (line 17) (column 25) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 291) (line 17) (column 25) (len 1)))))
    (reference r9 (scope relative) (span (offset 322) (line 18) (column 28) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 322) (line 18) (column 28) (len 1)))))
    (reference r10 (scope relative) (span (offset 353) (line 20) (column 24) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 353) (line 20) (column 24) (len 1)))))
    (reference r11 (scope relative) (span (offset 411) (line 25) (column 31) (len 12)) (segments (segment 0 (token "Classes") (name "Classes") (separator none) (span (offset 411) (line 25) (column 31) (len 7))) (segment 1 (token "'2'") (name "2") (separator colon-colon) (span (offset 420) (line 25) (column 40) (len 3)))))
    (reference r12 (scope relative) (span (offset 428) (line 26) (column 3) (len 7)) (segments (segment 0 (token "private") (name "private") (separator none) (span (offset 428) (line 26) (column 3) (len 7)))))
    (reference r13 (scope relative) (span (offset 436) (line 26) (column 11) (len 1)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 436) (line 26) (column 11) (len 1)))))
    (reference r14 (scope relative) (span (offset 467) (line 27) (column 15) (len 1)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 467) (line 27) (column 15) (len 1)))))
    (reference r15 (scope relative) (span (offset 494) (line 28) (column 25) (len 1)) (segments (segment 0 (token "C") (name "C") (separator none) (span (offset 494) (line 28) (column 25) (len 1)))))
    (reference r16 (scope relative) (span (offset 524) (line 29) (column 27) (len 1)) (segments (segment 0 (token "C") (name "C") (separator none) (span (offset 524) (line 29) (column 27) (len 1)))))
  )
  (root (package (name "Classes") (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "f") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)) (kerml-classifier (keyword class) (abstract false) (name "A") (specializes none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "b") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)) (expression (expression (span (offset 79) (line 7) (column 3) (len 9)) (ref r2))) (kerml-feature (prefix (head basic) (direction in) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind none) (member false) (all false) (name "c") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion portion) (variability none) (metadata)) (kind feature) (member false) (all false) (name "p") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)))) (kerml-classifier (keyword class) (abstract true) (name "B") (specializes none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract true) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "a") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion composite) (variability none) (metadata)) (kind feature) (member false) (all false) (name "aa") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)))) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion composite) (variability none) (metadata)) (kind feature) (member false) (all false) (name "a1") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "x") (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion composite) (variability none) (metadata)) (kind feature) (member false) (all false) (name "a") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion portion) (variability none) (metadata)) (kind feature) (member false) (all false) (name "q") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)))) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion portion) (variability none) (metadata)) (kind feature) (member false) (all false) (name "q") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)))) (package-member (visibility none) (package (name "P") (body brace))))) (kerml-classifier (keyword struct) (abstract false) (name "C") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r11)))) (body brace (expression (expression (span (offset 428) (line 26) (column 3) (len 7)) (ref r12))) (expression (expression (span (offset 436) (line 26) (column 11) (len 1)) (ref r13))) (malformed (code "recovered_calc_body_element") (found ": A, '2'[0..*];") (span (offset 437) (line 26) (column 12) (len 18))) (alias (name "z") (target (ref r14)) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion composite) (variability none) (metadata)) (kind feature) (member false) (all false) (name "c") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion composite) (variability none) (metadata)) (kind feature) (member false) (all false) (name "cc") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)))))))))
)
~~~
