pub const PACKAGE_JSON: &str = 
r#"{
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