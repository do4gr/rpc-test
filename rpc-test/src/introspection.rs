pub mod introspection {
    use crate::rpc::rpc::call_rpc;

    pub fn introspect(binary_path: &str, connection: &str) {
        let method: &str = "introspect";
        let mut params: String = "{\"url\": \"".into();
        params.push_str(connection);
        params.push_str("\"}");

        call_rpc(binary_path, method, params.as_str());
    }

    pub fn get_database_metadata(binary_path: &str, connection: &str) {
        let method: &str = "getDatabaseMetadata";
        let mut params: String = "{\"url\": \"".into();
        params.push_str(connection);
        params.push_str("\"}");

        call_rpc(binary_path, method, params.as_str());
    }

    pub fn list_databases(binary_path: &str, connection: &str) {
        let method: &str = "listDatabases";
        let mut params: String = "{\"url\": \"".into();
        params.push_str(connection);
        params.push_str("\"}");

        call_rpc(binary_path, method, params.as_str());
    }
}
