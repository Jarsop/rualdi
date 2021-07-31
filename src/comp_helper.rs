// TODO: Add completions for __rualdi_fzf
pub const ZSH_COMPLETION_REP: &[(&str, &str)] = &[
    (
        r#"#compdef rualdi"#,
        r#"#compdef rualdi __rualdi_rad __rualdi_radr __rualdi_radx __rualdi_radrx __rualdi_fzf"#
    ),
    (
        r#"local ret=1"#,
        r#"local ret=1

    local -a aliases envs
    aliases=( ${(f)"$(rualdi completions alias)"} )
    envs=( ${(f)"$(rualdi completions envs)"} )"#
    ),
];
