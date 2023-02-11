pub fn use_git_ignore() -> String {
    String::from("") + "node_modules\n" + "dist\n" + "build\n" + "./docs/.vitepress/dist"
}
