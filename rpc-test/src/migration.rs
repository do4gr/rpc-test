pub mod migration {
    use crate::rpc::rpc::call_rpc;

    pub fn infer_migration_steps(binary_path: &str, connection: &str) {
        let method: &str = "inferMigrationSteps";
        let mut params: String = "{\"url\": \"".into();
        params.push_str(connection);
        params.push_str("\"}");

        call_rpc(binary_path, method, params.as_str());
    }

    pub fn list_migrations(binary_path: &str, connection: &str) {
        let method: &str = "listMigrations";
        let mut params: String = "{\"url\": \"".into();
        params.push_str(connection);
        params.push_str("\"}");

        call_rpc(binary_path, method, params.as_str());
    }

    pub fn migration_progress(binary_path: &str, connection: &str) {
        let method: &str = "migrationProgress";
        let mut params: String = "{\"url\": \"".into();
        params.push_str(connection);
        params.push_str("\"}");

        call_rpc(binary_path, method, params.as_str());
    }

    pub fn apply_migration(binary_path: &str, connection: &str) {
        let method: &str = "applyMigration";
        let mut params: String = "{\"url\": \"".into();
        params.push_str(connection);
        params.push_str("\"}");

        call_rpc(binary_path, method, params.as_str());
    }

    pub fn unapply_migration(binary_path: &str, connection: &str) {
        let method: &str = "unapplyMigration";
        let mut params: String = "{\"url\": \"".into();
        params.push_str(connection);
        params.push_str("\"}");

        call_rpc(binary_path, method, params.as_str());
    }

    pub fn reset(binary_path: &str, connection: &str) {
        let method: &str = "reset";
        let mut params: String = "{\"url\": \"".into();
        params.push_str(connection);
        params.push_str("\"}");

        call_rpc(binary_path, method, params.as_str());
    }

    pub fn calculate_datamodel(binary_path: &str, connection: &str) {
        let method: &str = "calculateDatamodel";
        let mut params: String = "{\"url\": \"".into();
        params.push_str(connection);
        params.push_str("\"}");

        call_rpc(binary_path, method, params.as_str());
    }

    pub fn calculate_database_steps(binary_path: &str, connection: &str) {
        let method: &str = "calculateDatabaseSteps";
        let mut params: String = "{\"url\": \"".into();
        params.push_str(connection);
        params.push_str("\"}");

        call_rpc(binary_path, method, params.as_str());
    }
}
