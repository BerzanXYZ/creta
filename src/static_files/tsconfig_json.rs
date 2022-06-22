// tsconfig.json file is for configuring TypeScript compiler

pub const TSCONFIG_JSON: &str =
r#"{
    "compilerOptions": {
        "target": "ES6",
        "module": "CommonJS",
        "declaration": true,
        "outDir": "./bin",
        "strict": true,
        "skipLibCheck": true,
    },
    "include": [
        "src/**/*"
    ]
}
"#;