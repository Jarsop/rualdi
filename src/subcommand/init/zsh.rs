use super::Init;
use crate::config;

use anyhow::Result;
use std::io::Write;

pub fn run<W: Write>(writer: &mut W, options: &Init) -> Result<()> {
    let __rualdi_pwd = if config::rad_resolve_symlinks() {
        r#"
function __rualdi_pwd() {
    local cpwd colp
    # (D) will replace expanded $HOME with ~
    cpwd=${(D)"$(builtin pwd -P)"}
    colp=${(l:(COLUMNS-$#cpwd-2)/2::=:):-}
    builtin print -Pr "%F{11}$colp%f %F{14}%B$cpwd%f%b %F{11}$colp%f"
}"#
    } else {
        r#"
function __rualdi_pwd() {
    local cpwd colp
    # (D) will replace expanded $HOME with ~
    cpwd=${(D)"$(builtin pwd -L)"}
    colp=${(l:(COLUMNS-$#cpwd-2)/2::=:):-}
    builtin print -Pr "%F{11}$colp%f %F{14}%B$cpwd%f%b %F{11}$colp%f"
}"#
    };

    let __rualdi_cd = if config::rad_no_echo() {
        r#"
function __rualdi_cd() {
    builtin pushd "$@" || return "$?"
}"#
    } else {
        r#"
function __rualdi_cd() {
    builtin pushd "$@" || return "$?"
    __rualdi_pwd
}"#
    };

    let aliases = format!(
        r#"
alias {cmd}='__rualdi_rad'
alias {cmd}f='__rualdi_fzf'
alias {cmd}x='__rualdi_radx'
alias {cmd}xn='__rualdi_radxn'
alias {cmd}a='__rualdi_rada'
alias {cmd}ax='__rualdi_radax'
alias {cmd}l='__rualdi_radl'
alias {cmd}r='__rualdi_radr'
alias {cmd}rx='__rualdi_radrx'"#,
        cmd = options.cmd
    );

    write!(
        writer,
        r###"
# =============================================================================
# Utility functions for rualdi
#
# pwd based on the value of `_RAD_RESOLVE_SYMLINKS`
{__rualdi_pwd}
# cd + custom logic + resolving based on the value of `_RAD_NO_ECHO`
{__rualdi_cd}
# =============================================================================

setopt extendedglob noshortloops rcexpandparam
zmodload -Fa zsh/parameter p:commands p:dirstack

typeset -gaH rualdi_aliases

rualdi_aliases=( ${{(@f)"$(rualdi list-alias)"}} )

# Error wrapper
function __rualdi_error() {{ builtin print -Pr "%F{{1}}%BError:%b%f $@"; }}

# Use fzf if it is installed to help display directories to jump to
function __rualdi_fzf_() {{
    local list
    (( $+commands[fzf] )) || __rualdi_error "%F{{2}}fzf%f is needed to use this feature"

    ( (( $+commands[exa] )) || whence -va exa &>/dev/null ) &&
        list='exa -lbhg --git' || list='ls -l'

    fzf --ansi +m \
        --height 45% \
        --preview="echo {{}} |sed 's#.*→  ##'| xargs $list --color=always" \
        --preview-window="right:50%" \
        "$@"
}}

# Modified from wfxr/formarks, used to color output
function __rualdi_colorize() {{
    local field='\(\S\+\s*\)'
    local esc=$(builtin print '\033')
    local N="${{esc}}[0m"
    local R="${{esc}}[31m"
    local G="${{esc}}[32m"
    local Y="${{esc}}[38;5;14m"
    local B="${{esc}}[34m"
    local pattern="s#^${{field}}${{field}}${{field}}#$Y\1$R\2$N$B\3$N#"
    (( $+commands[gsed] )) && gsed "$pattern" || sed "$pattern"
}}

# List directories with fzf
function __rualdi_fzf_list() {{
    # This adds support for file paths with spaces
    rualdi_aliases=( ${{${{rualdi_aliases[@]// /__}}//__=>__/ → }} )
    builtin print -rl -- "$rualdi_aliases[@]" \
        | __rualdi_colorize \
        | column -t
}}

# Combine above fzf functions into one. Same as __rualdi_cd; however, fzf is involved
# Has an option to switch to recent directories as well using '-d'
function __rualdi_fzf {{
    if [[ $# -eq 1 && "$1" = '-' ]]; then
        if [[ -n "$OLDPWD" ]]; then
            __rualdi_cd "$OLDPWD"
        else
            __rualdi_error "%F{{2}}\$OLDPWD%f is not set"
            return 1
        fi
    elif [[ $# -eq 1 && "$1" = -<-> ]]; then
        __rualdi_cd "$1"
    elif [[ $# -le 2 && "$1" = (#i)(-d|--dir) ]]; then
        local dir; shift
        dir=$(builtin print -rl "$dirstack[@]" | __rualdi_fzf_ --query="${{argv:-}}")
        [[ -d "$dir" ]] && __rualdi_cd "$dir"
    else
        local sel
        # Wrapper for regular rad [[[
        [[ -n "$argv" ]] && {{
            local -a alias_dirs
            alias_dirs=( ${{rualdi_aliases[@]//(#m)*/${{(@)${{(@As: => :)${{MATCH}}}}[1]}}}} )
            # Note: :* checks whether argv is contained in alias_dirs
            [[ -n "${{argv:*alias_dirs}}" ]] && {{
                __rualdi_cd "$(rualdi resolve -- "$argv")" && return
            }}
        }}
        # ]]]

        sel=${{${{(@s:→:)$(__rualdi_fzf_list \
            | __rualdi_colorize \
            | __rualdi_fzf_ --query="${{argv:-}}")}}[1]}}

        [[ -n "$sel" ]] &&
            __rualdi_cd "$(rualdi resolve -- "$sel")"
    fi
}}

# Jump to a directory using alias
function __rualdi_rad() {{
    if [[ $# -eq 1 && "$1" = '-' ]]; then
        if [[ -n "$OLDPWD" ]]; then
            __rualdi_cd "$OLDPWD"
        else
            __rualdi_error "%F{{2}}\$OLDPWD%f is not set"
            return 1
        fi
    elif [[ $# -eq 1 && "$1" = -<-> ]]; then
        [[ ! -o pushdminus ]] && __rualdi_cd "${{1/-/+}}" || __rualdi_cd "$1"
    else
        local __rualdi_result
        __rualdi_result="$(rualdi resolve -- "$@")" &&
            __rualdi_cd "$__rualdi_result"
    fi
}}

# Add a new alias to the rualdi aliases configuration file
function __rualdi_rada() {{
    rualdi add -- "$@"
}}

# Add a new alias environment variable to the current environment
# and fill rualdi aliases configuration file
function __rualdi_radx() {{
    local __rualdi_alias=$1 __rualdi_var
    rualdi add-env -- "$@" && \
    __rualdi_var="$(rualdi resolve-env -- $__rualdi_alias)" && \
    export RAD_$__rualdi_var="$(rualdi resolve -- "$__rualdi_alias")" && \
    echo "Environment variable 'RAD_$__rualdi_var' added to current environment"
}}

# Add a new alias environment variable to the current environment
# without filling rualdi aliases configuration file
function __rualdi_radxn() {{
    local __rualdi_alias=$1 __rualdi_var=$2
    test "$__rualdi_var" && __rualdi_var=${{__rualdi_var:u}} || __rualdi_var=${{__rualdi_alias:u}}
    export RAD_$__rualdi_var="$(rualdi resolve -- "$__rualdi_alias")" && \
    echo "Environment variable 'RAD_$__rualdi_var' added to current environment without filling rualdi alias configuration file"
}}

# Add a new alias to the rualdi aliases configuration file with environment variable associated
function __rualdi_radax() {{
    local __rualdi_alias=$1 __rualdi_path=$2 __rualdi_var=$3 __rualdi_get
    rualdi add -- $__rualdi_alias $__rualdi_path && \
    rualdi add-env -- $__rualdi_alias $__rualdi_var && \
    __rualdi_get="$(rualdi resolve-env -- $__rualdi_alias)" && \
    export RAD_$__rualdi_get=$(rualdi resolve -- $__rualdi_alias)
}}

# Remove an alias to the rualdi aliases configuration file
function __rualdi_radr() {{
    rualdi remove -- "$@"
}}

# Remove an alias environment variable to the rualdi aliases configuration file.
function __rualdi_radrx() {{
    rualdi remove-env -- "$@"
}}

# List aliases and alias environment variables to the rualdi aliases configuration file.
function __rualdi_radl() {{
    rualdi list
}}

# =============================================================================
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
# Convenient aliases for rualdi
{aliases}
# =============================================================================
# To initialize rualdi with zsh, add the following line to your zsh
# configuration file (usually ~/.zshrc):
#
# eval "$(rualdi init zsh)"
"###,
        __rualdi_pwd = __rualdi_pwd,
        __rualdi_cd = __rualdi_cd,
        aliases = aliases,
    )?;

    Ok(())
}
