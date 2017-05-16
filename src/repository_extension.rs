use git2;

pub trait RepositoryExtension {
    fn origin_url(&self) -> Option<String>;
    fn mirrors(&self) -> Option<Vec<(String, String)>>;
}

impl RepositoryExtension for git2::Repository {
    fn origin_url(&self) -> Option<String> {
        self.find_remote("origin").ok()
            .and_then(|remote| remote.url().map(ToOwned::to_owned))
    }
    fn mirrors(&self) -> Option<Vec<(String, String)>> {
        fn mirrors(repo: &git2::Repository) -> Result<Vec<(String, String)>, git2::Error> {
            let config = try!(repo.config());
            let entries = try!(config.entries(Some("remotes.mirrors")));
            let mut result = vec![];
            for entry in &entries {
                let entry = try!(entry);
                if let Some(name) = entry.value() {
                    let url = repo.find_remote(name).ok()
                        .and_then(|remote| remote.url().map(ToOwned::to_owned));
                    if let Some(url) = url {
                        result.push((name.to_owned(), url));
                    }
                }
            }
            Ok(result)
        }

        mirrors(self).ok()
            .and_then(|mirrors| if mirrors.is_empty() { None } else { Some(mirrors) })
    }
}
