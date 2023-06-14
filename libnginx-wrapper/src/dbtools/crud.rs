use super::{open_database, params, NginxObj};

pub(crate) fn create_tables() {
    open_database()
        .execute_batch(
            "
BEGIN;
CREATE TABLE tblNginxConf(
    ServerName VARCHAR(100) NOT NULL UNIQUE PRIMARY KEY
    ProxyPass VARCHAR(100) NOT NULL NOT NULL
);
COMMIT;",
        )
        .unwrap();
}

pub fn select_all_from_tbl_nginxconf() -> Vec<NginxObj> {
    open_database()
        .prepare("SELECT ServerName,ProxyPass FROM tblNginxConf")
        .unwrap()
        .query_map([], |each_row| {
            Ok(NginxObj::new(
                each_row.get::<usize, String>(0).unwrap(),
                each_row.get::<usize, String>(1).unwrap(),
            )
            .unwrap())
        })
        .unwrap()
        .map(|each| each.unwrap())
        .collect::<Vec<NginxObj>>()
}

pub(crate) fn insert_tbl_nginxconf(server_name: &str, proxy_pass: &str) {
    open_database()
        .execute(
            "INSERT INTO tblNginxConf(ServerName, ProxyPass) VALUES(?1, ?2);",
            params![server_name, proxy_pass],
        )
        .unwrap();
}

pub(crate) fn delete_from_tbl_nginxconf(server_name: &str) {
    open_database()
        .execute(
            "DELETE FROM tblNginxConf WHERE ServerName = ?1);",
            params![server_name],
        )
        .unwrap();
}
