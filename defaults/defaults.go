package defaults

const (
	animal = ""
	clx    = ""
	git    = ""
	globe  = ""
	graph  = ""
	man    = ""
	node   = ""
	pager  = ""
	pencil = "פֿ"
	python = ""
	ruby   = ""
	search = ""
	shell  = ""
	ssh    = ""
	sudo   = ""
	tmux   = "﬿"
	vim    = ""
)

func GetMap() map[string]string {
	return map[string]string{
		"Python":  python,
		"[tmux]":  tmux,
		"ack":     search,
		"bash":    shell,
		"bat":     animal,
		"cat":     animal,
		"clx":     clx,
		"fd":      search,
		"find":    search,
		"fish":    shell,
		"fzf":     search,
		"git":     git,
		"grep":    search,
		"htop":    graph,
		"lazygit": git,
		"less":    pager,
		"man":     man,
		"more":    pager,
		"nano":    pencil,
		"node":    node,
		"nvim":    vim,
		"pico":    pencil,
		"ping":    globe,
		"rg":      search,
		"ruby":    ruby,
		"ssh":     ssh,
		"sudo":    sudo,
		"tig":     git,
		"top":     graph,
		"vi":      vim,
		"vim":     vim,
		"zsh":     shell,
	}
}
