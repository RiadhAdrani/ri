pub fn use_jest_config() -> String {
    String::from("")
        + "/** @type {import('ts-jest').JestConfigWithTsJest} */\n"
        + "module.exports = {\n"
        + "\tpreset: \"ts-jest\",\n"
        + "};"
}
