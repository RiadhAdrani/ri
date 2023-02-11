pub fn use_eslint() -> String {
    String::from("")
        + "{\n"
        + "\"root\": true,"
        + "\"parser\": \"@typescript-eslint/parser\","
        + "\"plugins\": [\"@typescript-eslint\"],\n"
        + "\"extends\": [\n"
        + "\"eslint:recommended\",\n"
        + "\"plugin:@typescript-eslint/eslint-recommended\",\n"
        + "\"plugin:@typescript-eslint/recommended\""
        + "]\n"
        + "}"
}

pub fn use_eslint_ignore() -> String {
    String::from("") + "node_modules\n" + "dist\n" + "build\n"
}
