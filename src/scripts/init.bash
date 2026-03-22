_my_zoxide_auto_add() {
    my-zoxide add "$PWD" >/dev/null 2>&1
}

if [[ -n "${PROMPT_COMMAND:-}" ]]; then
    PROMPT_COMMAND="_my_zoxide_auto_add; ${PROMPT_COMMAND}"
else
    PROMPT_COMMAND="_my_zoxide_auto_add"
fi

my-z() {
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
