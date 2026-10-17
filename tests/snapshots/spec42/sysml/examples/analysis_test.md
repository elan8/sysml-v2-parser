# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Simple Tests): AnalysisTest"))
~~~
# SOURCE
~~~sysml
package AnalysisTest {

	part def V {
		m;
	}
	
	part vv : V;
	
	requirement def AnalysisObjective {
		doc /* ... */
	}

	analysis def AnalysisCase {
		subject v : V;
		
		objective obj : AnalysisObjective { 
			subject = result;
		}
		
		v.m
	}
	
	analysis def AnalysisPlan {
		subject v : V;
		
		objective {
			doc /* ... */
		}
		
		analysis analysisCase : AnalysisCase { return mass; }
	}
	
	part analysisContext {
		analysis analysisPlan : AnalysisPlan {
			subject v = vv;
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "analysis_test.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package AnalysisTest {
    part def V {
        m;
    }
    part vv : V;
    requirement def AnalysisObjective {
        doc
        /* ... */
    }
    analysis def AnalysisCase {
        subject v : V;
        objective obj  : AnalysisObjective {
            subject = result;
        }
        v.m;
    }
    analysis def AnalysisPlan {
        subject v : V;
        objective {
            doc
            /* ... */
        }
        analysis analysisCase : AnalysisCase {
            return mass;
        }
    }
    part analysisContext {
        analysis analysisPlan : AnalysisPlan {
            subject v = vv;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 59) (line 7) (column 12) (len 1)) (segments (segment 0 (token "V") (name "V") (separator none) (span (offset 59) (line 7) (column 12) (len 1)))))
    (reference r1 (scope relative) (span (offset 164) (line 14) (column 15) (len 1)) (segments (segment 0 (token "V") (name "V") (separator none) (span (offset 164) (line 14) (column 15) (len 1)))))
    (reference r2 (scope relative) (span (offset 188) (line 16) (column 19) (len 17)) (segments (segment 0 (token "AnalysisObjective") (name "AnalysisObjective") (separator none) (span (offset 188) (line 16) (column 19) (len 17)))))
    (reference r3 (scope relative) (span (offset 222) (line 17) (column 14) (len 6)) (segments (segment 0 (token "result") (name "result") (separator none) (span (offset 222) (line 17) (column 14) (len 6)))))
    (reference r4 (scope relative) (span (offset 239) (line 20) (column 3) (len 1)) (segments (segment 0 (token "v") (name "v") (separator none) (span (offset 239) (line 20) (column 3) (len 1)))))
    (reference r5 (scope relative) (span (offset 241) (line 20) (column 5) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 241) (line 20) (column 5) (len 1)))))
    (reference r6 (scope relative) (span (offset 291) (line 24) (column 15) (len 1)) (segments (segment 0 (token "V") (name "V") (separator none) (span (offset 291) (line 24) (column 15) (len 1)))))
    (reference r7 (scope relative) (span (offset 361) (line 30) (column 27) (len 12)) (segments (segment 0 (token "AnalysisCase") (name "AnalysisCase") (separator none) (span (offset 361) (line 30) (column 27) (len 12)))))
    (reference r8 (scope relative) (span (offset 446) (line 34) (column 27) (len 12)) (segments (segment 0 (token "AnalysisPlan") (name "AnalysisPlan") (separator none) (span (offset 446) (line 34) (column 27) (len 12)))))
    (reference r9 (scope relative) (span (offset 476) (line 35) (column 16) (len 2)) (segments (segment 0 (token "vv") (name "vv") (separator none) (span (offset 476) (line 35) (column 16) (len 2)))))
  )
  (root (package (name "AnalysisTest") (body brace (part-def (name "V") (modifiers) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "m") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vv") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (requirement-def (name "AnalysisObjective") (modifiers) (body brace (doc (name none) (short-name none) (locale none) (body (span (offset 109) (line 10) (column 9) (len 5)) (normalized "... "))))) (analysis-case-def (name "AnalysisCase") (modifiers) (body brace (subject (name "v") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (objective (visibility none) (name "obj") (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (body brace (subject (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 222) (line 17) (column 14) (len 6)) (ref r3))))) (body semicolon)))) (expression (span (offset 239) (line 20) (column 3) (len 3)) (member-access (base (expression (span (offset 239) (line 20) (column 3) (len 1)) (ref r4))) (separator dot) (member (ref r5)))))) (analysis-case-def (name "AnalysisPlan") (modifiers) (body brace (subject (name "v") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (objective (visibility none) (name none) (type none) (body brace (doc (name none) (short-name none) (locale none) (body (span (offset 320) (line 27) (column 10) (len 5)) (normalized "... "))))) (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "analysisCase") (short-name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (subsets none) (redefines none) (body brace (case-return (declaration "mass") (target none) (type none) (redefines none) (feature-kind none) (subsetting false) (value none)))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "analysisContext") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "analysisPlan") (short-name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (subsets none) (redefines none) (body brace (subject (name "v") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 476) (line 35) (column 16) (len 2)) (ref r9))))) (body semicolon)))))))))
)
~~~
