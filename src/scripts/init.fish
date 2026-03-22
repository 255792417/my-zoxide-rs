function _my_zoxide_auto_add --on-variable PWD
    my-zoxide add "$PWD" >/dev/null 2>&1
end

# 核心跳转函数 `my-z`
function my-z --description "Jump to a directory using my-zoxide, with fzf integration"
    set -l initial_query "$argv"

    set -l reload_command "my-zoxide list {q}"

    set -l fzf_opts --query="$initial_query" --disabled --bind "change:reload:$reload_command" --height 40% --reverse --border --header "Jump to Directory" --preview 'ls {}'

    set -l target (my-zoxide list $initial_query | fzf $fzf_opts)

    if test -n "$target"
        cd "$target"
    end
end
