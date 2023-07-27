use super::{nginx_ops, ArgMatches};

pub(crate) fn match_force(matches: &ArgMatches) {
    if matches.get_flag("renew_certificate") {
        let domain_name = matches
            .get_one::<String>("domain_name")
            .expect("contains_id")
            .to_owned();
        match nginx_ops::remake_ssl(&domain_name) {
            Ok(()) => println!("Successfully Regenerated SSL"),
            Err((code, message)) => eprintln!("Error {code}\nError Message: {message}"),
        }
    } else if matches.get_flag("db_migration") {
        libnginx_wrapper::init_migration(true);
        println!("Finished!")
    }
}
