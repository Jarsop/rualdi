use super::Init;
use crate::config;

use anyhow::Result;
use std::io::Write;

pub fn run<W: Write>(writer: &mut W, options: &Init) -> Result<()> {
    let __rualdi_pwd = if config::rad_resolve_symlinks() {
        "
__rualdi_pwd() {
    pwd -P
}"
    } else {
        "
__rualdi_pwd() {
    pwd -L
}"
    };

    let __rualdi_cd = if config::rad_no_echo() {
        r#"
__rualdi_cd() {
    cd "$@" || return "$?"
}"#
    } else {
        r#"
__rualdi_cd() {
    cd "$@" || return "$?"
    __rualdi_pwd
}"#
    };

    let aliases = format!(
        r#"
alias {cmd}='__rualdi_rad'
alias {cmd}a='__rualdi_rada'
alias {cmd}l='__rualdi_radl'
alias {cmd}r='__rualdi_radr'"#,
        cmd = options.cmd
    );

    write!(
        writer,
        r#"\
# =============================================================================
#
# Utility functions for rualdi.
#
# pwd based on the value of _RAD_RESOLVE_SYMLINKS.
{__rualdi_pwd}
# cd + custom logic + resolving based on the value of _RAD_NO_ECHO.
{__rualdi_cd}
# =============================================================================
#
# Jump to a directory using alias.
__rualdi_rad() {{
    if [ "$#" -eq 0 ]; then
        __rualdi_cd ~
    elif [ "$#" -eq 1 ] && [ "$1" = '-' ]; then
        if [ -n "$OLDPWD" ]; then
            __rualdi_cd "$OLDPWD"
        else
            echo "rualdi: \$OLDPWD is not set"
            return 1
        fi
    else
        local __rualdi_result
        __rualdi_result="$(rualdi resolve -- "$@")" && __rualdi_cd "$__rualdi_result"
    fi
}}
# Add a new alias to the rualdi aliases configuration file.
__rualdi_rada() {{
    rualdi add "$@"
}}
# Remove an alias to the rualdi aliases configuration file.
__rualdi_radr() {{
    rualdi remove "$@"
}}
# List alias to the rualdi aliases configuration file.
__rualdi_radl() {{
    rualdi list
}}
# =============================================================================
#
# Convenient aliases for rualdi.
#
{aliases}
# =============================================================================
#
# To initialize rualdi with zsh, add the following line to your zsh
# configuration file (usually ~/.zshrc):
#
# eval "$(rualdi init zsh)"
"#,
        __rualdi_pwd = __rualdi_pwd,
        __rualdi_cd = __rualdi_cd,
        aliases = aliases,
    )?;

    Ok(())
}
