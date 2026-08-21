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
    (reference r0 (scope relative) (span (offset 79) (line 7) (column 3) (len 9)) (segments (segment 0 (token "protected") (name "protected") (separator none) (span (offset 79) (line 7) (column 3) (len 9)))))
    (reference r1 (scope relative) (span (offset 411) (line 25) (column 31) (len 12)) (segments (segment 0 (token "Classes") (name "Classes") (separator none) (span (offset 411) (line 25) (column 31) (len 7))) (segment 1 (token "'2'") (name "2") (separator colon-colon) (span (offset 420) (line 25) (column 40) (len 3)))))
    (reference r2 (scope relative) (span (offset 428) (line 26) (column 3) (len 7)) (segments (segment 0 (token "private") (name "private") (separator none) (span (offset 428) (line 26) (column 3) (len 7)))))
    (reference r3 (scope relative) (span (offset 436) (line 26) (column 11) (len 1)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 436) (line 26) (column 11) (len 1)))))
    (reference r4 (scope relative) (span (offset 467) (line 27) (column 15) (len 1)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 467) (line 27) (column 15) (len 1)))))
  )
  (root (package (name "Classes") (body brace (kerml-feature (name "f") (relationships) (value none) (body semicolon)) (kerml-classifier (keyword class) (abstract false) (name "A") (specializes none) (body brace (kerml-feature (name "b") (relationships) (value none) (body semicolon)) (expression (expression (span (offset 79) (line 7) (column 3) (len 9)) (ref r0))) (in-out-declaration) (kerml-feature (name "p") (relationships) (value none) (body semicolon)))) (kerml-classifier (keyword class) (abstract true) (name "B") (specializes none) (body brace (kerml-feature (name "a") (relationships) (value none) (body brace (kerml-feature (name "aa") (relationships) (value none) (body semicolon)))) (kerml-feature (name "a1") (relationships) (value none) (body semicolon)) (kerml-feature (name "x") (relationships) (value none) (body brace (kerml-feature (name "a") (relationships) (value none) (body brace (kerml-feature (name "q") (relationships) (value none) (body semicolon)))) (kerml-feature (name "q") (relationships) (value none) (body semicolon)))) (package-member (visibility none) (package (name "P") (body brace))))) (kerml-classifier (keyword struct) (abstract false) (name "C") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r1)))) (body brace (expression (expression (span (offset 428) (line 26) (column 3) (len 7)) (ref r2))) (expression (expression (span (offset 436) (line 26) (column 11) (len 1)) (ref r3))) (malformed (code "recovered_calc_body_element") (found ": A, '2'[0..*];") (span (offset 437) (line 26) (column 12) (len 18))) (alias (name "z") (target (ref r4)) (body semicolon)) (kerml-feature (name "c") (relationships) (value none) (body brace (kerml-feature (name "cc") (relationships) (value none) (body semicolon)))))))))
)
~~~
