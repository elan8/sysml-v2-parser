# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 41 (Language Extension): Model Library Example"))
~~~
# SOURCE
~~~sysml
library package 'Model Library Example' {
	private import ScalarValues::Real;
	private import RiskMetadata::Level;
	
	abstract occurrence def Situation;
	
	abstract occurrence situations : Situation[*] nonunique;
	
	abstract occurrence def Cause {
		attribute probability : Real;
	}
	
	abstract occurrence causes : Cause[*] nonunique :> situations;
	
	abstract occurrence def Failure {
		attribute severity : Level;
	}
	
	abstract occurrence failures : Failure[*] nonunique :> situations;
	
	abstract connection def Causation :> Occurrences::HappensBefore {
		end [*] ref cause : Situation;
		end [*] ref effect : Situation;
	}
	
	abstract connection causations : Causation[*] nonunique;
	
	item def Scenario {
		occurrence :>> situations;
		occurrence :>> causes :> situations;
		occurrence :>> failures :> situations;
	}
	
	item scenarios : Scenario[*] nonunique;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "41_model_library_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
library package 'Model Library Example' {
    private import ScalarValues::Real;
    private import RiskMetadata::Level;
    abstract occurrence def Situation;
    abstract occurrence situations : Situation[*];
    abstract occurrence def Cause {
        attribute probability : Real;
    }
    abstract occurrence causes : Cause[*] :> situations;
    abstract occurrence def Failure {
        attribute severity : Level;
    }
    abstract occurrence failures : Failure[*] :> situations;
    abstract connection def Causation :> Occurrences::HappensBefore {
        end cause : Situation[*];
        end effect : Situation[*];
    }
    abstract connection def causations : Causation;
    item def Scenario {
        occurrence :>> situations;
        occurrence :> situations :>> causes;
        occurrence :> situations :>> failures;
    }
    item scenarios : Scenario[*] nonunique;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 58) (line 2) (column 17) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 58) (line 2) (column 17) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 72) (line 2) (column 31) (len 4)))))
    (reference r1 (scope relative) (span (offset 94) (line 3) (column 17) (len 19)) (segments (segment 0 (token "RiskMetadata") (name "RiskMetadata") (separator none) (span (offset 94) (line 3) (column 17) (len 12))) (segment 1 (token "Level") (name "Level") (separator colon-colon) (span (offset 108) (line 3) (column 31) (len 5)))))
    (reference r2 (scope relative) (span (offset 529) (line 21) (column 39) (len 26)) (segments (segment 0 (token "Occurrences") (name "Occurrences") (separator none) (span (offset 529) (line 21) (column 39) (len 11))) (segment 1 (token "HappensBefore") (name "HappensBefore") (separator colon-colon) (span (offset 542) (line 21) (column 52) (len 13)))))
    (reference r3 (scope relative) (span (offset 580) (line 22) (column 23) (len 9)) (segments (segment 0 (token "Situation") (name "Situation") (separator none) (span (offset 580) (line 22) (column 23) (len 9)))))
    (reference r4 (scope relative) (span (offset 614) (line 23) (column 24) (len 9)) (segments (segment 0 (token "Situation") (name "Situation") (separator none) (span (offset 614) (line 23) (column 24) (len 9)))))
    (reference r5 (scope relative) (span (offset 664) (line 26) (column 35) (len 9)) (segments (segment 0 (token "Causation") (name "Causation") (separator none) (span (offset 664) (line 26) (column 35) (len 9)))))
    (reference r6 (scope relative) (span (offset 843) (line 34) (column 19) (len 8)) (segments (segment 0 (token "Scenario") (name "Scenario") (separator none) (span (offset 843) (line 34) (column 19) (len 8)))))
  )
  (root (library-package (name "Model Library Example") (standard false) (body brace (import (target (span (span (offset 58) (line 2) (column 17) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 94) (line 3) (column 17) (len 19))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (occurrence-def) (occurrence (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "situations") (short-name none) (target none) (body semicolon)) (occurrence-def) (occurrence (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "causes") (short-name none) (target none) (body semicolon)) (occurrence-def) (occurrence (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "failures") (short-name none) (target none) (body semicolon)) (connection-def (name "Causation") (modifiers abstract) (role ordinary) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r2)))) (body brace (end (short-name none) (identity (declaration (name "cause") (span (offset 572) (line 22) (column 15) (len 5)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (references none) (redefines none) (crosses none)) (end (short-name none) (identity (declaration (name "effect") (span (offset 605) (line 23) (column 15) (len 6)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (references none) (redefines none) (crosses none)))) (connection-def (name "causations") (modifiers abstract) (role ordinary) (specializes (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (body semicolon)) (item-def (name "Scenario") (individual false) (specializes none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "") (short-name none) (target none) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "") (short-name none) (target none) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "") (short-name none) (target none) (body semicolon)))) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "scenarios") (short-name none) (type (ref r6)) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (subsets none) (redefines none) (value none) (body semicolon)))))
)
~~~
