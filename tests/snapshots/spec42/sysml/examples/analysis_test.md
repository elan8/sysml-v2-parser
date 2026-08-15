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
  )
  (root (package (name "AnalysisTest") (body brace (part-def (name "V") (body brace (default-reference-usage))) (part-usage) (requirement-def (name "AnalysisObjective") (body brace (doc))) (analysis-case-def) (analysis-case-def) (part-usage))))
)
~~~
