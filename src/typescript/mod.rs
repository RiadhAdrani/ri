pub fn use_ts_config() -> String {
    String::from("")
        + "{\n"
        + "\"exclude\": [\"node_modules\", \"src/**/*.test.ts\", \"./jest.config.js\"],\n"
        + "\"compilerOptions\": {\n"
        + "\"target\": \"es2016\",\n"
        + "\"lib\": [\"es6\", \"dom\"],\n"
        + "\"module\": \"commonjs\",\n"
        + "\"rootDir\": \".\",\n"
        + "\"resolveJsonModule\": true,\n"
        + "\"allowJs\": true,\n"
        + "\"declaration\": true,\n"
        + "\"outDir\": \"build\",\n"
        + "\"esModuleInterop\": true,\n"
        + "\"forceConsistentCasingInFileNames\": true,\n"
        + "\"strict\": true,\n"
        + "\"noImplicitAny\": true,\n"
        + "\"skipLibCheck\": true\n"
        + "}\n"
        + "}"
}
