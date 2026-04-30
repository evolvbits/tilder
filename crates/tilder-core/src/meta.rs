use crate::traits::Capitalize;

pub fn authors() -> String {
  let authors = std::env::var("CARGO_PKG_AUTHORS").expect("CARGO_PKG_AUTHORS not set");

  authors
    .replace("://", "\x00PROTO\x00") // protect the ://
    .split(':')
    .map(|a| {
      let restored = a.trim().replace("\x00PROTO\x00", "://");
      format!("  - {}", restored)
    })
    .collect::<Vec<_>>()
    .join("\n")
}

pub struct CreditsInfo<'a> {
  pub name: &'a str,
  pub version: &'a str,
  pub maintainer: &'a str,
  pub repository: &'a str,
  pub license: &'a str,
  pub commit: &'a str,
  pub date: &'a str,
  pub copyright: &'a str,
  pub homepage: &'a str,
}

/// Get full credits string
pub fn credits_string(info: &CreditsInfo) -> String {
  let sep: String = "-".repeat(55);
  let lst_authors = authors();
  let indentation: String = " ".repeat(2);

  format!(
    "\n{} - Version {}\n\n\
        Credits:\n  Authors:\n{indentation}{lst_authors}\n{indentation}Maintainer: {}\n\n\
        Repository: {}\n\
        License: {}\n\
        Commit: {}\n\
        Last update: {}\n\n\
        {sep}\n\
        {}\n\
        Homepage: {}\n\
        {sep}",
    info.name.capitalize(),
    info.version,
    info.maintainer,
    info.repository,
    info.license,
    info.commit,
    info.date,
    info.copyright,
    info.homepage
  )
}
