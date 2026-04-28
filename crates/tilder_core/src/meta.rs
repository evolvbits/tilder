use crate::traits::Capitalize;

pub fn authors() -> String {
  let authors = std::env::var("CARGO_PKG_AUTHORS").expect("CARGO_PKG_AUTHORS not set");

  authors
    .replace("://", "\x00PROTO\x00") // protect the ://
    .split(':')
    .map(|a| {
      let restored = a.trim().replace("\x00PROTO\x00", "://");
      format!("- {}", restored)
    })
    .collect::<Vec<_>>()
    .join("\n")
}

pub struct CreditsInfo<'a> {
  pub name: &'a str,
  pub version: &'a str,
  pub maintainer: &'a str,
  pub repository: &'a str,
  pub commit: &'a str,
  pub date: &'a str,
  pub license: &'a str,
  pub site: &'a str,
}

/// Get full credits string
pub fn credits_string(info: &CreditsInfo) -> String {
  let sep: String = "-".repeat(50);
  let lst_authors = authors();

  format!(
    "\n{} - Version {}\n\n\
        Credits:\n\n\
        Authors:\n{lst_authors}\n\
        Maintainer: {}\n\n\
        Info:\n\n\
        Code repository: {}\n\
        Commit hash: {}\n\
        Commit date: {}\n\n\
        {sep}\n\
        {}\n\
        Site: {}\n\
        {sep}",
    info.name.capitalize(),
    info.version,
    info.maintainer,
    info.repository,
    info.commit,
    info.date,
    info.license,
    info.site
  )
}
