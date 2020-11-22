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
alias {cmd}x='__rualdi_radx'
alias {cmd}xn='__rualdi_radxn'
alias {cmd}a='__rualdi_rada'
alias {cmd}ax='__rualdi_radax'
alias {cmd}l='__rualdi_radl'
alias {cmd}r='__rualdi_radr'
alias {cmd}xr='__rualdi_radxr'"#,
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
    rualdi add -- "$@"
}}
# Add a new alias environment variable to the current environment and fill rualdi aliases configuration file.
__rualdi_radx() {{
    local __rualdi_alias=$1 __rualdi_var
    rualdi add-env -- "$@" && \
    __rualdi_var="$(rualdi resolve-env -- $__rualdi_alias)" && \
    export RAD_$__rualdi_var="$(rualdi resolve -- "$__rualdi_alias")" && \
    echo "Environment variable 'RAD_$__rualdi_var' added to current environment"
}}
# Add a new alias environment variable to the current environment without filling rualdi aliases configuration file.
__rualdi_radxn() {{
    local __rualdi_alias=$1 __rualdi_var=$2
    test "$__rualdi_var" && __rualdi_var=${{__rualdi_var:u}} || __rualdi_var=${{__rualdi_alias:u}}
    export RAD_$__rualdi_var="$(rualdi resolve -- "$__rualdi_alias")" && \
    echo "Environment variable 'RAD_$__rualdi_var' added to current environment without filling rualdi alias configuration file"
}}
# Add a new alias to the rualdi aliases configuration file with environment variable associated.
__rualdi_radax() {{
    local __rualdi_alias=$1 __rualdi_path=$2 __rualdi_var=$3 __rualdi_get
    rualdi add -- $__rualdi_alias $__rualdi_path && \
    rualdi add-env -- $__rualdi_alias $__rualdi_var && \
    __rualdi_get="$(rualdi resolve-env -- $__rualdi_alias)" && \
    export RAD_$__rualdi_get=$(rualdi resolve -- $__rualdi_alias)
}}
# Remove an alias to the rualdi aliases configuration file.
__rualdi_radr() {{
    rualdi remove -- "$@"
}}
# Remove an alias environment variable to the rualdi aliases configuration file.
__rualdi_radxr() {{
    rualdi remove-env -- "$@"
}}

# List aliases and alias environment variables to the rualdi aliases configuration file.
__rualdi_radl() {{
    rualdi list
}}
# =============================================================================
#
# Restore environment variables
OLDIFS="$IFS"
IFS=$'\n'
for line in $(rualdi list-env); do
    local __rualdi_alias __rualdi_path __rualdi__var
    __rualdi_alias=$(echo $line | cut -d' ' -f1)
    __rualdi_var=$(echo $line | cut -d' ' -f2)
    __rualdi_path=$(rualdi resolve -- $__rualdi_alias)
    export RAD_${{__rualdi_var:u}}="$__rualdi_path"
done
IFS="$OLDIFS"
unset OLDIFS
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
