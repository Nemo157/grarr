use std::fmt;
use maud::{ Render, Markup };

macro_rules! fa {
  ($($e:ident => $v:expr,)*) => {
    #[allow(dead_code)]
    #[derive(Clone, Copy)]
    pub enum FA { $($e,)* }
    impl FA {
      fn class(self) -> &'static str {
        match self {
          $(FA::$e => concat!("fa fa-", $v),)*
        }
      }
    }
  };
}

fa! {
  Book      => "book",
  CodeFork  => "code-fork",
  Cog       => "cog",
  File      => "file",
  GitSquare => "git-square",
  Home      => "home",
  Info      => "info",
  LevelUp   => "level-up",
  Question  => "question",
  Sitemap   => "sitemap",
  Tag       => "tag",
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum FAM {
  FixedWidth(FA),
  Lg(FA),
  Li(FA),
  X(u8, FA),
}

impl FAM {
  fn class(self) -> String {
    match self {
      FAM::FixedWidth(fa) => format!("fa-fw {}", fa.class()),
      FAM::Lg(fa) => format!("fa-fw fa-lg {}", fa.class()),
      FAM::Li(fa) => format!("fa-li {}", fa.class()),
      FAM::X(mul, fa) => format!("fa-fw fa-{}x {}", mul, fa.class()),
    }
  }
}

impl Render for FA {
  fn render(&self) -> Markup {
    html!({ i class=(self.class()) { } })
  }
}

impl Render for FAM {
  fn render(&self) -> Markup {
    html!({ i class=(self.class()) { } })
  }
}
