echo "tasks.fish has been loaded!"
echo ""

function .run
    set -x ADVENT_OF_CODE_SESSION (get_env_var ADVENT_OF_CODE_SESSION)

	cargo run -- $argv
end

function get_env_var
    cat .env | grep $argv[1]= | cut -d '=' -f 2
end
