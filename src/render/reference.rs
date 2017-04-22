use git2;
use referenced_commit::ReferencedCommit;

const HEX: &'static [u8; 0x10] = b"0123456789abcdef";
fn short(oid: git2::Oid) -> String {
  oid.as_bytes().iter().take(3).flat_map(|b| vec![HEX[((b >> 4) & 0xFu8) as usize] as char, HEX[(b & 0xFu8) as usize] as char]).collect()
}

pub fn Commit(commit: &git2::Commit) -> ::maud::Markup {
  html! {
    span.id title=(commit.id()) { (short(commit.id())) }
  }
}

pub fn Reference(commit: &ReferencedCommit) -> ::maud::Markup {
  html! {
    @match commit.reference.as_ref().and_then(|r| r.shorthand()) {
      Some(ref reff) => span.ref title=(commit.commit.id()) { (reff) },
      None => (Commit(&commit.commit)),
    }
  }
}
