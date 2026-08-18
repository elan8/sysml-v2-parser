# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 05 (Redefinition): Redefinition Example"))
~~~
# SOURCE
~~~sysml
package 'Redefinition Example' {

	part def Vehicle {
		part eng : Engine;
	}
	part def SmallVehicle :> Vehicle {
		part smallEng : SmallEngine redefines eng;
	}
	part def BigVehicle :> Vehicle {
		part bigEng : BigEngine :>> eng;
	}

	part def Engine {
		part cyl : Cylinder[4..6];
	}
	part def SmallEngine :> Engine {
		part redefines cyl[4];
	}
	part def BigEngine :> Engine {
		part redefines cyl[6];
	}

	part def Cylinder;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "05_redefinition_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Redefinition Example' {
    part def Vehicle {
        part eng : Engine;
    }
    part def SmallVehicle :> Vehicle {
        part smallEng : SmallEngine :>> eng;
    }
    part def BigVehicle :> Vehicle {
        part bigEng : BigEngine :>> eng;
    }
    part def Engine {
        part cyl : Cylinder[4..6];
    }
    part def SmallEngine :> Engine {
        part :>> cyl[4];
    }
    part def BigEngine :> Engine {
        part :>> cyl[6];
    }
    part def Cylinder;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "Redefinition Example") (body brace (part-def (name "Vehicle") (body brace (part-usage))) (part-def (name "SmallVehicle") (body brace (part-usage))) (part-def (name "BigVehicle") (body brace (part-usage))) (part-def (name "Engine") (body brace (part-usage))) (part-def (name "SmallEngine") (body brace (part-usage))) (part-def (name "BigEngine") (body brace (part-usage))) (part-def (name "Cylinder") (body semicolon)))))
)
~~~
