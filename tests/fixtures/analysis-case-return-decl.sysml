package AnalysisCaseReturnDecl {
    verification def MyVerification {
        subject sys : System;
        return verdict : VerdictKind = evaluateData.verdict;
        return :>> result = computeResult();
        return attribute score : Real = 42.0;
    }

    analysis def MyAnalysis {
        subject sys : System;
        return thrust : ForceValue;
    }
}
