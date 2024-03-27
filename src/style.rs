pub const DARK_STYLE: &str = r#"
:root {
    --bg-color: #24273a;
    --text-color: #cad3f5;
    --header-color: #b7bdf8;
    --link-color: #7dc4e4;
    --blockquote-color: #b8c0e0;
    --code-bg-color: #363a4f;
    --code-text-color: #b8c0e0;
    --table-border-color: #b7bdf8;
    --table-text-color: #b8c0e0;
}
body {
    background-color: var(--bg-color);
    color: var(--text-color);
}

h1, h2, h3, h4, h5, h6 {
    color: var(--header-color);
}

a {
    color: var(--link-color);
}

blockquote {
    color: var(--blockquote-color);
}

code {
    background-color: var(--code-bg-color);
    color: var(--code-text-color);
}

table {
    border-color: var(--table-border-color);
}

th, td {
    color: var(--table-text-color);
}
"#;
