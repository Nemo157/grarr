use git2::{ Oid, Commit };

const HEX: &'static [u8; 0x10] = b"0123456789abcdef";
fn short(oid: Oid) -> String {
  oid.as_bytes().iter().take(3).flat_map(|b| vec![HEX[((b >> 4) & 0xFu8) as usize] as char, HEX[(b & 0xFu8) as usize] as char]).collect()
}

fn summary<'a>(commit: &'a Commit<'a>) -> Option<&'a str> {
  commit.message()
    .and_then(|message| message.lines().nth(0))
}

renderers! {
  CommitsRenderer(commits: &'a Vec<Commit<'a>>) {
    #for commit in commits {
      #CommitStubRenderer(commit)
    }
  }

  CommitStubRenderer(commit: &'a Commit<'a>) {
    div class="commit-stub" {
      a href={ "commits/" #commit.id() } {
        span class="id"
          #short(commit.id())
        " "
        #if let Some(summary) = summary(commit) {
          #summary
        }
        #if let None = summary(commit) {
          "<No summary provided>"
        }
      }
    }
  }

  CommitRenderer(commit: &'a Commit<'a>) {
    #CommitStubRenderer(commit)
  }
}