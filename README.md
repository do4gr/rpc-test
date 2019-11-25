# rpc-test
Test Prisma Engine RPC Calls

## Old Script
```shell
#!/bin/bash
fileName=$1
folder="../../database-schema-examples/sqlite/basic_tests/models/"

echo "{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"introspect\",\"params\":[{\"url\":\"file:${folder}${fileName}.db\"}]}" | ../target/debug/introspection-engine | jq -r '.result' > "${folder}$fileName".prisma
```

Copy binaries over: 

```shell
$ cp /Users/matthias/repos/work/prisma-engine/target/debug/introspection-engine /Users/matthias/repos/work/rpc-test/rpc-test/binaries/ && cp /Users/matthias/repos/work/prisma-engine/target/debug/migration-engine  /Users/matthias/repos/work/rpc-test/rpc-test/binaries/
```


Needs the connection string
Needs the path to the binaries
Needs the path to a schema.prisma file
Needs a path to store results / output to stdout

Config {
    connection string: String default sqlite db in folder
    mig-engine: default in folder
    intro-engine: default in folder
    schema: default in folder
    result: print + result folder in folder

}

Do relative paths work?


connection strings
mysql://prisma:qd58rcCywPRS4Stk@introspection-public-mysql.clfeqqifnebj.eu-west-1.rds.amazonaws.com:3306/wordpress
postgres://prisma:qd58rcCywPRS4Stk@introspection-public-postgres-cluster.cluster-ro-clfeqqifnebj.eu-west-1.rds.amazonaws.com:5432/basic-blog?schema=public



## Introspection Engine




### List Databases

### Get Metadata

### Introspect

```json
{
    "id":1,
    "jsonrpc":"2.0",
    "method":"introspect",
    "params":[
        {"url":"file:/Users/folder/test.db"}
    ]
}
```


## Migration Engine

### Infer Migration Steps

```json
{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "inferMigrationSteps",
    "params": {
        "projectInfo": "the-project-id",
        "migrationId": "the-migration_id",
        "assumeToBeApplied": [],
        "dataModel": "model Blog {\n    id Int @id\n    name String\n    viewCount Int\n    posts Post[]\n    authors Author[]\n}\n\nmodel Author {\n    id Int @id\n    name String?\n    authors Blog[]\n}\n\nmodel Post {\n    id Int @id\n    title String\n    tags String[]\n    blog Blog\n}"
    }
}
```


### Apply Migration

### DMMF to DML
```json
{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "convertDmmfToDml",
    "params": {
        "projectInfo": "the-project-id",
        "dmmf": "yada yada"
    }
}
```



### List Migrations
```json
{
    "id": 1, 
    
    "jsonrpc": "2.0", 
    "method": "listMigrations", 
    "params": {"projectInfo": "the-project-id"}
}
```


### Migration Progress
```json
{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "migrationProgress",
    "params": {
        "projectInfo": "the-project-id",
        "migrationId": "the-migration-id"
    }
}
```
