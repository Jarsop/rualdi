// TODO: Add completions for __rualdi_fzf
pub const ZSH_COMPLETION_REP: &[(&str, &str)] = &[
    (
        r#"            (a)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':alias -- Alias to path:_files' \
'::path -- Path to aliasing, if not provided current directory is used:_files' \
&& ret=0
;;
(add)"#,
        r#"(add|a)"#
    ),
    (
        r#"(ax)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':alias -- Alias to link:_files' \
'::var -- Environment variable to link on alias, if not provided alias is used:_files' \
&& ret=0
;;
(add-env)"#,
        r#"(add-env|ax)"#
    ),
    (
        r#"(i)
_arguments "${_arguments_options[@]}" \
'--cmd=[Renames the '\''rad'\'' command and corresponding aliases]' \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':shell:(bash zsh)' \
&& ret=0
;;
(init)"#,
        r#"(init|i)"#
    ),
    (
        r#"(l)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
(la)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
(list-alias)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
(lx)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
(list-env)"#,
        r#"(list|l|list-alias|la|list-env|lx)"#
    ),
    (
        r#"(r)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::alias -- Alias to remove:_files' \
&& ret=0
;;
(remove)"#,
        r#"(remove|r)"#
    ),
    (
        r#"(rx)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':alias -- Alias for which to remove the environment variable:_files' \
&& ret=0
;;
(remove-env)"#,
        r#"(remove-env|rx)"#
    ),
    (
        r#"(res)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':path -- Path to resolve alias:_files' \
&& ret=0
;;
(resolve)"#,
        r#"(resolve|res)"#
    ),
    (
        r#"(resx)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':alias -- Alias for which to find environment variable:_files' \
&& ret=0
;;
(resolve-env)"#,
        r#"(resolve-env|resx)"#
    ),
    (
        r#"(comp)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':comp-type -- Variable type to list for help with completions:(alias env shell)' \
'::shell -- Shell to be used for completions (can only be used with shell):(zsh bash)' \
&& ret=0
;;
(completions)"#,
        r#"(completions|comp)"#
    ),
    (
        "(( $+functions[_a_commands] )) ||
_a_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'a commands' commands \"$@\"
}
(( $+functions[_rualdi__a_commands] )) ||
_rualdi__a_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'rualdi a commands' commands \"$@\"
}
(( $+functions[_rualdi__add_commands] )) ||
_rualdi__add_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'rualdi add commands' commands \"$@\"
}
(( $+functions[_rualdi__add-env_commands] )) ||
_rualdi__add-env_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'rualdi add-env commands' commands \"$@\"
}
(( $+functions[_ax_commands] )) ||
_ax_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'ax commands' commands \"$@\"
}
(( $+functions[_rualdi__ax_commands] )) ||
_rualdi__ax_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'rualdi ax commands' commands \"$@\"
}
(( $+functions[_comp_commands] )) ||
_comp_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'comp commands' commands \"$@\"
}
(( $+functions[_rualdi__comp_commands] )) ||
_rualdi__comp_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'rualdi comp commands' commands \"$@\"
}
(( $+functions[_rualdi__completions_commands] )) ||
_rualdi__completions_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'rualdi completions commands' commands \"$@\"
}
(( $+functions[_i_commands] )) ||
_i_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'i commands' commands \"$@\"
}
(( $+functions[_rualdi__i_commands] )) ||
_rualdi__i_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'rualdi i commands' commands \"$@\"
}
(( $+functions[_rualdi__init_commands] )) ||
_rualdi__init_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'rualdi init commands' commands \"$@\"
}
(( $+functions[_l_commands] )) ||
_l_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'l commands' commands \"$@\"
}
(( $+functions[_rualdi__l_commands] )) ||
_rualdi__l_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'rualdi l commands' commands \"$@\"
}
(( $+functions[_la_commands] )) ||
_la_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'la commands' commands \"$@\"
}
(( $+functions[_rualdi__la_commands] )) ||
_rualdi__la_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'rualdi la commands' commands \"$@\"
}
(( $+functions[_rualdi__list_commands] )) ||
_rualdi__list_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'rualdi list commands' commands \"$@\"
}
(( $+functions[_rualdi__list-alias_commands] )) ||
_rualdi__list-alias_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'rualdi list-alias commands' commands \"$@\"
}
(( $+functions[_rualdi__list-env_commands] )) ||
_rualdi__list-env_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'rualdi list-env commands' commands \"$@\"
}
(( $+functions[_lx_commands] )) ||
_lx_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'lx commands' commands \"$@\"
}
(( $+functions[_rualdi__lx_commands] )) ||
_rualdi__lx_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'rualdi lx commands' commands \"$@\"
}
(( $+functions[_r_commands] )) ||
_r_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'r commands' commands \"$@\"
}
(( $+functions[_rualdi__r_commands] )) ||
_rualdi__r_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'rualdi r commands' commands \"$@\"
}
(( $+functions[_rualdi__remove_commands] )) ||
_rualdi__remove_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'rualdi remove commands' commands \"$@\"
}
(( $+functions[_rualdi__remove-env_commands] )) ||
_rualdi__remove-env_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'rualdi remove-env commands' commands \"$@\"
}
(( $+functions[_res_commands] )) ||
_res_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'res commands' commands \"$@\"
}
(( $+functions[_rualdi__res_commands] )) ||
_rualdi__res_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'rualdi res commands' commands \"$@\"
}
(( $+functions[_rualdi__resolve_commands] )) ||
_rualdi__resolve_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'rualdi resolve commands' commands \"$@\"
}
(( $+functions[_rualdi__resolve-env_commands] )) ||
_rualdi__resolve-env_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'rualdi resolve-env commands' commands \"$@\"
}
(( $+functions[_resx_commands] )) ||
_resx_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'resx commands' commands \"$@\"
}
(( $+functions[_rualdi__resx_commands] )) ||
_rualdi__resx_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'rualdi resx commands' commands \"$@\"
}
(( $+functions[_rualdi__rx_commands] )) ||
_rualdi__rx_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'rualdi rx commands' commands \"$@\"
}
(( $+functions[_rx_commands] )) ||
_rx_commands() {
    local commands; commands=(
\x20\x20\x20\x20\x20\x20\x20\x20
    )
    _describe -t commands 'rx commands' commands \"$@\"
}
",
        r#""#,
    ),
// // FIX: use the other two functions in this one?
//     (
//         r#"_rualdi "$@""#,
//         r#"(( $+functions[_rualdi_variables] )) ||
// _rualdi_variables() {
//     [[ $PREFIX = -* ]] && return 1
//     integer ret=1
//     local -a aliases; aliases=(
//         ${(@f)$(_call_program commands rualdi completions alias)}
//     )
//     local -a env_vars; env_vas=(
//         ${(@f)$(_call_program commands rualdi completions env)}
//     )
//
//     _alternative \
//       'aliases:aliases:compadd -a - aliases' \
//       'env_vars:environment vars:compadd -a - env_vars' && \
//           ret=0
//
//     return ret
// }
//
// (( $+functions[_rualdi_aliases] )) ||
// _rualdi_aliases() {
//     [[ $PREFIX = -* ]] && return 1
//     integer ret=1
//     local -a aliases; aliases=(
//         ${(@f)$(_call_program commands rualdi completions alias)}
//     )
//
//     _describe -t aliases 'aliases' aliases && ret=0
//
//     return ret
// }
//
// (( $+functions[_rualdi_envs] )) ||
// _rualdi_envs() {
//     [[ $PREFIX = -* ]] && return 1
//     integer ret=1
//     local -a env_vars; env_vars=(
//         ${(@f)$(_call_program commands rualdi completions env)}
//     )
//
//     _describe -t env_vars 'environment vars' env_vars && ret=0
//
//     return ret
// }
//
// _rualdi "$@""#
//     ),
//     (
//         r#"'::alias -- Alias to remove:_files' \"#,
//         r#"'::alias -- Alias to remove:_rualdi_variables' \"#
//     ),
//     (
//         r#"':alias -- Alias for which to remove the environment variable:_files' \"#,
//         r#"':alias -- Alias for which to remove the environment variable:_rualdi_envs' \"#
//     ),
//     (
//         r#"':path -- Path to resolve alias:_files' \"#,
//         r#"':path -- Path to resolve alias:_rualdi_envs' \"#
//     ),
//     (
//         r#"':alias -- Alias for which to find environment variable:_files' \"#,
//         r#"':alias -- Alias for which to find environment variable:_rualdi_aliases' \"#
//     )
];

// _files => _files -/
