error() {
	echo -e "\033[1;31merror:\033[0m $@" >&2
}

info() {
	echo -e "\033[1;32minfo:\033[0m $@" >&2
}

warn() {
	echo -e "\033[1;33mwarn:\033[0m $@" >&2
}

debug() {
	local log_level="${LOG_LEVEL:-info}"

	if [[ "$log_level" == "debug" ]]; then
		echo -e "\033[1;34mdebug:\033[0m $@" >&2
	fi

	return
}

die() {
	echo -e "\033[1;31mfatal:\033[0m $@" >&2
	exit 1
}

_find_root() {
	local dir
	dir="$(pwd)"

	local root="/"

	while [[ "$dir" != "$root" ]]; do
		if [[ -f "$dir/flake.nix" ]]; then
			debug "Found project root at $dir."
			echo "$dir"
			return
		else
			dir="$(dirname "$dir")"
		fi
	done

	die "failed to find project root directory"
}

_load_env_files() {
	debug "Loading env.default and .env file"

	local root
	root="$(_find_root)"

	if [ ! -f "$root/.env" ]; then
		[ -f "$root/env.default" ] || die "no .env file found, and no env.default file found."

		warn "no .env file found, creating one from $root/env.default."

		# Copy all empty variables to .env to be overwritten by the user
		grep \"\" "$root/env.default" >"$root/.env"

		warn "Done, created $root/.env file. Please fill in the missing values."
	fi

	set -o allexport

	# shellcheck source=/dev/null
	source "$root/env.default"
	debug "Loaded $(grep -cE "^[A-Z]+=" "$root/env.default") env vars from $root/env.default"

	# shellcheck source=/dev/null
	source "$root/.env"
	debug "Loaded $(grep -cE "^[A-Z]+=" "$root/.env") env vars from $root/.env."

	set +o allexport
}

_load_env() {
  _load_env_files

  # Set env var for root folder
  ROOT="$(_find_root)"
  export ROOT

  info "Loaded environment."
}
