package defaults

import (
	"github.com/bensadeh/despell/colors"
	"github.com/bensadeh/despell/core"
	"github.com/bensadeh/despell/emoji"
	"github.com/bensadeh/despell/nerdfont"
)

func GetDefaults() map[string]core.Icon {
	return map[string]core.Icon{
		"Python":     {Icon: nerdfont.Python, Color: colors.Yellow, Emoji: emoji.Snake},
		"[tmux]":     {Icon: nerdfont.Tmux, Color: colors.Green, Emoji: emoji.NutAndBold},
		"ack":        {Icon: nerdfont.Search, Color: colors.Cyan, Emoji: emoji.MagnifyingGlass},
		"atop":       {Icon: nerdfont.Graph, Color: colors.Yellow, Emoji: emoji.Microscope},
		"bash":       {Icon: nerdfont.Shell, Color: colors.Normal, Emoji: emoji.TopHat},
		"bat":        {Icon: nerdfont.Animal, Color: colors.Magenta, Emoji: emoji.Bat},
		"cargo":      {Icon: nerdfont.Rust, Color: colors.Red, Emoji: emoji.Crab},
		"cat":        {Icon: nerdfont.Animal, Color: colors.Red, Emoji: emoji.Cat},
		"clx":        {Icon: nerdfont.Yc, Color: colors.Orange, Emoji: emoji.Newspaper},
		"curl":       {Icon: nerdfont.Globe, Color: colors.Blue, Emoji: emoji.Globe},
		"docker":     {Icon: nerdfont.Docker, Color: colors.Blue, Emoji: emoji.Whale},
		"duplicate":  {Icon: nerdfont.Duplicate, Color: colors.Yellow, Emoji: emoji.Leaves},
		"emacs":      {Icon: nerdfont.Emacs, Color: colors.Magenta, Emoji: emoji.Pen},
		"exa":        {Icon: nerdfont.Directories, Color: colors.Yellow, Emoji: emoji.Folder},
		"fd":         {Icon: nerdfont.Search, Color: colors.Cyan, Emoji: emoji.MagnifyingGlass},
		"find":       {Icon: nerdfont.Search, Color: colors.Cyan, Emoji: emoji.MagnifyingGlass},
		"fish":       {Icon: nerdfont.Shell, Color: colors.Normal, Emoji: emoji.TropicalFish},
		"fzf":        {Icon: nerdfont.Search, Color: colors.Cyan, Emoji: emoji.MagnifyingGlass},
		"gh":         {Icon: nerdfont.Github, Color: colors.Blue, Emoji: emoji.TanabataTree},
		"git":        {Icon: nerdfont.Git, Color: colors.Red, Emoji: emoji.Wood},
		"glow":       {Icon: nerdfont.Markdown, Color: colors.Magenta, Emoji: emoji.Star},
		"go":         {Icon: nerdfont.Golang, Color: colors.Cyan, Emoji: emoji.HamsterFace},
		"grep":       {Icon: nerdfont.Search, Color: colors.Cyan, Emoji: emoji.MagnifyingGlass},
		"htop":       {Icon: nerdfont.Graph, Color: colors.Yellow, Emoji: emoji.Microscope},
		"http":       {Icon: nerdfont.Globe, Color: colors.Blue, Emoji: emoji.Globe},
		"java":       {Icon: nerdfont.Java, Color: colors.Red, Emoji: emoji.Coffee},
		"lazygit":    {Icon: nerdfont.Git, Color: colors.Red, Emoji: emoji.TanabataTree},
		"less":       {Icon: nerdfont.Text, Color: colors.Magenta, Emoji: emoji.BookRed},
		"lf":         {Icon: nerdfont.Directories, Color: colors.Yellow, Emoji: emoji.Folder},
		"ls":         {Icon: nerdfont.Directories, Color: colors.Yellow, Emoji: emoji.Folder},
		"lua":        {Icon: nerdfont.Lua, Color: colors.Blue, Emoji: emoji.Moon},
		"lynx":       {Icon: nerdfont.Globe, Color: colors.Blue, Emoji: emoji.Globe},
		"man":        {Icon: nerdfont.Book, Color: colors.Magenta, Emoji: emoji.BookBlue},
		"more":       {Icon: nerdfont.Text, Color: colors.Magenta, Emoji: emoji.BookRed},
		"mv":         {Icon: nerdfont.Move, Color: colors.Yellow, Emoji: emoji.MovingTruck},
		"nano":       {Icon: nerdfont.Pencil, Color: colors.Magenta, Emoji: emoji.Pen},
		"nnn":        {Icon: nerdfont.Directories, Color: colors.Yellow, Emoji: emoji.Folder},
		"node":       {Icon: nerdfont.Javascript, Color: colors.Yellow, Emoji: emoji.Globe},
		"npm":        {Icon: nerdfont.Package, Color: colors.Red, Emoji: emoji.Package},
		"nvim":       {Icon: nerdfont.Vim, Color: colors.Green, Emoji: emoji.Pen},
		"pico":       {Icon: nerdfont.Pencil, Color: colors.Magenta, Emoji: emoji.Pen},
		"ping":       {Icon: nerdfont.Globe, Color: colors.Blue, Emoji: emoji.Globe},
		"ranger":     {Icon: nerdfont.Directories, Color: colors.Yellow, Emoji: emoji.Folder},
		"rg":         {Icon: nerdfont.Search, Color: colors.Cyan, Emoji: emoji.MagnifyingGlass},
		"rm":         {Icon: nerdfont.Trash, Color: colors.Red, Emoji: emoji.CrossMark},
		"rsync":      {Icon: nerdfont.Sync, Color: colors.Red, Emoji: emoji.AnticlockwiseArrows},
		"ruby":       {Icon: nerdfont.Ruby, Color: colors.Red, Emoji: emoji.DiamondsSuit},
		"rustc":      {Icon: nerdfont.Rust, Color: colors.Red, Emoji: emoji.Crab},
		"safe-rm":    {Icon: nerdfont.Trash, Color: colors.Red, Emoji: emoji.CrossMark},
		"scp":        {Icon: nerdfont.Computers, Color: colors.Cyan, Emoji: emoji.Computer},
		"sleep":      {Icon: nerdfont.Hourglass, Color: colors.Cyan, Emoji: emoji.Zzz},
		"spin":       {Icon: nerdfont.Down, Color: colors.Cyan, Emoji: emoji.Spin},
		"ssh":        {Icon: nerdfont.Computers, Color: colors.Cyan, Emoji: emoji.Key},
		"sudo":       {Icon: nerdfont.Warning, Color: colors.Red, Emoji: emoji.RedExclamationMark},
		"tail":       {Icon: nerdfont.Down, Color: colors.Cyan, Emoji: emoji.DownPointingTriangle},
		"tig":        {Icon: nerdfont.Git, Color: colors.Red, Emoji: emoji.TanabataTree},
		"tmux":       {Icon: nerdfont.Tmux, Color: colors.Green, Emoji: emoji.NutAndBold},
		"top":        {Icon: nerdfont.Graph, Color: colors.Yellow, Emoji: emoji.Microscope},
		"vi":         {Icon: nerdfont.Vim, Color: colors.Green, Emoji: emoji.Pen},
		"vim":        {Icon: nerdfont.Vim, Color: colors.Green, Emoji: emoji.Pen},
		"w3m":        {Icon: nerdfont.Globe, Color: colors.Blue, Emoji: emoji.Globe},
		"wget":       {Icon: nerdfont.Globe, Color: colors.Blue, Emoji: emoji.Globe},
		"youtube-dl": {Icon: nerdfont.YouTube, Color: colors.Red, Emoji: emoji.FilmFrames},
		"zsh":        {Icon: nerdfont.Shell, Color: colors.Normal, Emoji: emoji.TopHat},
	}
}
