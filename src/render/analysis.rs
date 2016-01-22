use git_appraise::{ Analysis, Analyses };

renderers! {
  AnalysesRenderer(analyses: Analyses) {
    div {
      "Analyses: "
      ul {
        #for analysis in analyses {
          #(AnalysisRenderer(analysis))
        }
      }
    }
  }

  AnalysisRenderer(analysis: Analysis) {
    #if let Some(url) = analysis.url() {
      li {
        a href={ #url } #url
      }
    }
  }
}
