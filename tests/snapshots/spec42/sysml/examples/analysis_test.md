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
        objective obj : AnalysisObjective  {
            subject = result;
        }
        v.m;
    }
    analysis def AnalysisPlan {
        subject v : V;
        objective  {
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
  (references
    (reference r0 (scope relative) (span (offset 59) (line 7) (column 12) (len 1)) (segments (segment 0 (token "V") (name "V") (separator none) (span (offset 59) (line 7) (column 12) (len 1)))))
  )
  (root (package (name "AnalysisTest") (body brace (part-def (name "V") (modifiers) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "m") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vv") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (requirement-def (name "AnalysisObjective") (modifiers) (body brace (doc (name none) (locale none) (body (span (offset 109) (line 10) (column 9) (len 5)) (normalized "... "))))) (analysis-case-def (modifiers)) (analysis-case-def (modifiers)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "analysisContext") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (analysis-case-usage))))))
)
~~~
