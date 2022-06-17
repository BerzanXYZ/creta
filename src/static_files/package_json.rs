pub const PACKAGE_JSON: &str = 
r#"// Created by Creta app builder
// Modify this file as you want
// For example;
// -set a version
// -change description
// -add the url of your git repo
// -add your name

{
    "name": "PROJECTNAME",
    "version": "0.0.0",
    "description": "DESCRIPTION",
    "main": "src/index.ts",
    "repository": "YOUR_GIT_REPO_URL",
    "author": "YOUR_NAME",
    "license": "MIT",
    "scripts": {
        "build": "yarn tsc",
        "start": "node bin/index.js"
    }
    ],
    "devDependencies": {
        "typescript": "^4.7.3"
    }
}
"#;