#[macro_use]
extern crate clap;
use serde::{Deserialize, Serialize};
use std::io::Error as IoError;
mod introspection;
mod migration;
mod rpc;
use crate::introspection::introspection::*;
use crate::migration::migration::*;

fn main() -> Result<(), IoError> {
    let matches = clap_app!(myapp =>
        (@setting SubcommandRequiredElseHelp)
        (version: "0.1")
        (author: "Matthias Oertel")
        (about: "Tests Prisma Engines RPC Calls")
        //Config
        (@subcommand config =>                  
            (about: "Stores the configuration for the calls.")
            (@arg connection: -s +takes_value "The connection string to use." ) 
            (@arg intro_path: -i +takes_value "The path to the migration engine." )
            (@arg mig_path: -m +takes_value "The path to the introspection engine." )
        )
        (@subcommand delete => (about: "Deletes the configuration for the calls."))
        // // Introspection
        (@subcommand introspect =>              
            (about: "Introspects the specified database and returns its Prisma schema.")
            (@arg connection: "connection")
        )
        (@subcommand listDatabases =>           (about: "Lists the available databases."))
        (@subcommand getDatabaseMetadata =>     (about: "Shows metadata for the specified database."))
        // Migration
        (@subcommand inferMigrationSteps =>     (about: "inferMigrationSteps.")) 
        (@subcommand listMigrations =>          (about: "listMigrations.")) 
        (@subcommand migrationProgress =>       (about: "migrationProgress."))
        (@subcommand applyMigration =>          (about: "applyMigration"))
        (@subcommand unapplyMigration =>        (about: "unapplyMigration"))
        (@subcommand reset =>                   (about: "reset")) 
        (@subcommand calculateDatamodel =>      (about: "calculateDatamodel")) 
        (@subcommand calculateDatabaseSteps =>  (about: "calculateDatabaseSteps"))
    )
    .get_matches();

    // Config
    if let Some(submatches) = matches.subcommand_matches("config") {
        // old config

        let new_cfg: MyConfig = MyConfig {
            version: 0,
            connection_string: submatches
                .value_of("connection")
                .unwrap_or(&cfg.connection_string)
                .to_string(),
            intro_path: submatches
                .value_of("intro_path")
                .unwrap_or(&cfg.intro_path)
                .to_string(),
            mig_path: submatches
                .value_of("mig_path")
                .unwrap_or(&cfg.mig_path)
                .to_string(),
        };

        confy::store(config, new_cfg)?;
    }

    if let Some(submatches) = matches.subcommand_matches("delete") {
        // delete old config

        let new_cfg: MyConfig = MyConfig {
            version: 0,
            connection_string: submatches
                .value_of("connection")
                .unwrap_or(&cfg.connection_string)
                .to_string(),
            intro_path: submatches
                .value_of("intro_path")
                .unwrap_or(&cfg.intro_path)
                .to_string(),
            mig_path: submatches
                .value_of("mig_path")
                .unwrap_or(&cfg.mig_path)
                .to_string(),
        };

        confy::store(config, new_cfg)?;
    }

    // Gets a value for config if supplied by user, or defaults to "rpc-test"
    let config = "rpc-test";
    let cfg: MyConfig = confy::load(config)?;
    let intro_path = &cfg.intro_path.as_str();
    let mig_path = &cfg.mig_path.as_str();
    let connection_string = &cfg.connection_string.as_str();

    // Introspection
    if let Some(submatches) = matches.subcommand_matches("introspect") {
        introspect(
            intro_path,
            submatches
                .value_of("connection")
                .unwrap_or(&cfg.connection_string),
        );
    }

    if let Some(_submatches) = matches.subcommand_matches("getDatabaseMetadata") {
        get_database_metadata(intro_path, connection_string);
    }

    if let Some(_submatches) = matches.subcommand_matches("listDatabases") {
        list_databases(intro_path, connection_string);
    }

    // Migration
    if let Some(_submatches) = matches.subcommand_matches("inferMigrationSteps") {
        infer_migration_steps(mig_path, connection_string);
    }

    if let Some(_submatches) = matches.subcommand_matches("listMigrations") {
        list_migrations(mig_path, connection_string);
    }

    if let Some(_submatches) = matches.subcommand_matches("migrationProgress") {
        migration_progress(mig_path, connection_string);
    }

    if let Some(_submatches) = matches.subcommand_matches("applyMigration") {
        apply_migration(mig_path, connection_string);
    }

    if let Some(_submatches) = matches.subcommand_matches("unapplyMigration") {
        unapply_migration(mig_path, connection_string);
    }

    if let Some(_submatches) = matches.subcommand_matches("reset") {
        reset(mig_path, connection_string);
    }
    if let Some(_submatches) = matches.subcommand_matches("calculateDatamodel") {
        calculate_datamodel(mig_path, connection_string);
    }

    if let Some(_submatches) = matches.subcommand_matches("calculateDatabaseSteps") {
        calculate_database_steps(mig_path, connection_string);
    }

    std::result::Result::Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct MyConfig {
    version: u8,
    connection_string: String,
    mig_path: String,
    intro_path: String,
}

impl ::std::default::Default for MyConfig {
    fn default() -> Self {
        Self {
            version: 0,
            connection_string:
                "file:/Users/matthias/repos/work/rpc-test/rpc-test/final/test/introspection-engine.db"
                    .into(),
            mig_path: "/Users/matthias/repos/work/rpc-test/rpc-test/final/binaries/migration-engine".into(),
            intro_path: "/Users/matthias/repos/work/rpc-test/rpc-test/final/binaries/introspection-engine".into(),
        }
    }
}
