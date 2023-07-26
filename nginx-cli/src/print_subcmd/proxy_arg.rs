use super::{Arg, ArgAction};

pub(crate) fn proxy_arg() -> Arg {
    Arg::new("proxy_feature")
        .short('p')
        .long("proxy")
        .help("List Proxy Feature")
        .required(true)
        .conflicts_with_all(["all", "redirect_feature", "filehost_feature", "spa_feature"])
        .action(ArgAction::SetTrue)
}