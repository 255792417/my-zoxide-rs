_my_zoxide_auto_add() {
    my-zoxide add "$PWD" >/dev/null 2>&1
}

autoload -Uz add-zsh-hook
add-zsh-hook chpwd _my_zoxide_auto_add
_my_zoxide_auto_add

function my-z() {
    local initial_query="$*"
    local reload_command='my-zoxide list {q}'
    local target

    target=$(my-zoxide list "$initial_query" | fzf \
        --query="$initial_query" \
        --disabled \
        --bind "change:reload:${reload_command}" \
        --height 40% \
        --reverse \
        --border \
        --header "Jump to Directory" \
        --preview 'ls {}')

    if [[ -n "$target" ]]; then
        cd "$target" || return
    fi
}
