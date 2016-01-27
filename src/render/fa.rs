use std::fmt;
use maud::RenderOnce;

pub enum FA {
  LevelUp,
  GitSquare,
  Sitemap,
  Tag,
}

pub enum FAM {
  Li(FA),
}

impl FA {
  fn class(self) -> &'static str {
    match self {
      FA::LevelUp => "fa fa-level-up",
      FA::GitSquare => "fa fa-git-square",
      FA::Sitemap => "fa fa-sitemap",
      FA::Tag => "fa fa-tag",
    }
  }
}

impl FAM {
  fn class(self) -> String {
    match self {
      FAM::Li(fa) => "fa-li ".to_string() + fa.class()
    }
  }
}

impl RenderOnce for FA {
  fn render_once(self, mut w: &mut fmt::Write) -> fmt::Result {
    html!(w, { i class=#self.class() { } })
  }
}

impl RenderOnce for FAM {
  fn render_once(self, mut w: &mut fmt::Write) -> fmt::Result {
    html!(w, { i class=#self.class() { } })
  }
}
