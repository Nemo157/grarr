use git_appraise::{ Status, CIStatus, CIStatuses };

renderers! {
  CIStatusesRenderer(statuses: CIStatuses) {
    div {
      "CI Statuses: "
      ul {
        #for status in statuses {
          li #(CIStatusRenderer(status))
        }
      }
    }
  }

  CIStatusRenderer(status: CIStatus) {
    #if let Some(url) = status.url() {
      a href={ #url } #status.agent().unwrap_or("<Unknown agent>")
    }
    #if status.url().is_none() {
      #status.agent().unwrap_or("<Unknown agent>")
    }
    ": "
    #status.status().map(|s| match s { Status::Success => "success", Status::Failure => "failure" }).unwrap_or("null")
  }
}
