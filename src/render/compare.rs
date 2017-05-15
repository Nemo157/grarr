use maud::{ Render, Markup };
use git2;
use repository_context::RepositoryContext;
use referenced_commit::ReferencedCommit;

pub struct Compare<'r> {
    pub context: &'r RepositoryContext,
    pub new: ReferencedCommit<'r>,
    pub old: ReferencedCommit<'r>,
    pub base: git2::Commit<'r>,
    pub commits: Vec<git2::Commit<'r>>,
}

impl<'r> Render for Compare<'r> {
    fn render(&self) -> Markup {
        html!({
            div.block div.block-header h2 { "Comparing base " (super::Reference(&self.old)) " to " (super::Reference(&self.new)) }
            div.block {
                div.block-header h3 { "Commits" }
                div.block-details {
                    @for commit in &self.commits {
                        (super::CommitStub(&self.context, commit))
                    }
                }
            }
            div.block {
                div.block-header h3 { "File changes" }
                div.block-details {
                    (super::DiffCommits(&self.context, &Some(&self.base), &self.new.commit))
                }
            }
        })
    }
}
