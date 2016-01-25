use std::str;
use git2::{ Repository };
use maud_pulldown_cmark::markdown;

fn find_readme(repo: &Repository) -> Option<String> {
  let head_id = repo.head().unwrap().resolve().unwrap().target().unwrap();
  let head = repo.find_commit(head_id).unwrap();
  let tree = head.tree().unwrap();
  let s = tree.get_name("README")
    .or_else(|| tree.get_name("README.md"))
    .and_then(|entry| entry.to_object(repo).ok()
      .and_then(|object| object.as_blob()
        .and_then(|blob| str::from_utf8(blob.content()).ok())
          .map(|s| s.to_string())));
  s
}

renderers! {
  RepositoryRenderer(name: &'a str, repo: &'a Repository) {
    h1 #name
    #if let Some(readme) = find_readme(repo) {
      #(markdown::from_string(&*readme))
    }
  }

  RepositoryStubRenderer(name: &'a str, repo: &'a Repository) {
    div class="repo-stub" {
      a href=#name {
        #name
      }
    }
  }

  RepositoriesRenderer(repos: &'a Vec<(String, Repository)>) {
    #for &(ref path, ref repo) in repos {
      #RepositoryStubRenderer(&*path, repo)
    }
  }
}
