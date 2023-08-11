pub use crate::icon::emojis;
pub use crate::icon::nerdfonts;

use crate::color::Color;
use crate::icon::Icon;
use phf::phf_map;

static DEFAULT_MAPPINGS: phf::Map<&'static str, (&'static str, Color, &'static str)> = phf_map! {
    "Python" => (nerdfonts::PYTHON, Color::Yellow, emojis::SNAKE),
    "[tmux]" => (nerdfonts::TMUX, Color::Green, emojis::NUT_AND_BOLD),
    "ack" => (nerdfonts::SEARCH, Color::Cyan, emojis::MAGNIFYING_GLASS),
    "atop" => (nerdfonts::GRAPH, Color::Yellow, emojis::MICROSCOPE),
    "awk" => (nerdfonts::FILE, Color::Magenta, emojis::BOOK_RED),
    "bat" => (nerdfonts::ANIMAL, Color::Magenta, emojis::BAT),
    "cargo" => (nerdfonts::PACKAGE, Color::Red, emojis::PACKAGE),
    "cat" => (nerdfonts::ANIMAL, Color::Red, emojis::CAT),
    "chgrp" => (nerdfonts::USER, Color::Green, emojis::USER),
    "chmod" => (nerdfonts::USER, Color::Red, emojis::USER),
    "chown" => (nerdfonts::USER, Color::Green, emojis::USER),
    "clx" => (nerdfonts::YC, Color::Orange, emojis::NEWSPAPER),
    "curl" => (nerdfonts::GLOBE, Color::Blue, emojis::GLOBE),
    "cut" => (nerdfonts::FILE, Color::Magenta, emojis::BOOK_RED),
    "df" => (nerdfonts::PIE_CHART, Color::Yellow, emojis::BAR_CHART),
    "diff" => (nerdfonts::DUPLICATE, Color::Red, emojis::LEAVES),
    "docker" => (nerdfonts::DOCKER, Color::Blue, emojis::WHALE),
    "du" => (nerdfonts::PIE_CHART, Color::Yellow, emojis::BAR_CHART),
    "duplicate" => (nerdfonts::DUPLICATE, Color::Yellow, emojis::LEAVES),
    "emacs" => (nerdfonts::EMACS, Color::Magenta, emojis::TEXT_EDIT),
    "exa" => (nerdfonts::DIRECTORIES, Color::Yellow, emojis::FOLDER),
    "fd" => (nerdfonts::SEARCH, Color::Cyan, emojis::MAGNIFYING_GLASS),
    "find" => (nerdfonts::SEARCH, Color::Cyan, emojis::MAGNIFYING_GLASS),
    "free" => (nerdfonts::AREA_CHART, Color::Yellow, emojis::BRAIN),
    "fzf" => (nerdfonts::SEARCH, Color::Cyan, emojis::MAGNIFYING_GLASS),
    "gh" => (nerdfonts::GITHUB, Color::Blue, emojis::TANABATA_TREE),
    "git" => (nerdfonts::GIT, Color::Red, emojis::WOOD),
    "glow" => (nerdfonts::MARKDOWN, Color::Magenta, emojis::STAR),
    "go" => (nerdfonts::GOLANG, Color::Cyan, emojis::HAMSTER_FACE),
    "grep" => (nerdfonts::SEARCH, Color::Cyan, emojis::MAGNIFYING_GLASS),
    "head" => (nerdfonts::FILE, Color::Magenta, emojis::BOOK_RED),
    "htop" => (nerdfonts::GRAPH, Color::Yellow, emojis::MICROSCOPE),
    "http" => (nerdfonts::GLOBE, Color::Blue, emojis::GLOBE),
    "java" => (nerdfonts::JAVA, Color::Red, emojis::COFFEE),
    "lazygit" => (nerdfonts::GIT, Color::Red, emojis::TANABATA_TREE),
    "less" => (nerdfonts::FILE, Color::Magenta, emojis::BOOK_RED),
    "lf" => (nerdfonts::DIRECTORIES, Color::Yellow, emojis::FOLDER),
    "ln" => (nerdfonts::FILE, Color::Cyan, emojis::FOLDER),
    "ls" => (nerdfonts::DIRECTORIES, Color::Yellow, emojis::FOLDER),
    "lua" => (nerdfonts::LUA, Color::Blue, emojis::MOON),
    "lynx" => (nerdfonts::GLOBE, Color::Blue, emojis::GLOBE),
    "make" => (nerdfonts::TOOLS, Color::Yellow, emojis::TOOLS),
    "man" => (nerdfonts::BOOK, Color::Magenta, emojis::BOOK_BLUE),
    "mkdir" => (nerdfonts::DIRECTORIES, Color::Yellow, emojis::FOLDER),
    "more" => (nerdfonts::FILE, Color::Magenta, emojis::BOOK_RED),
    "mv" => (nerdfonts::MOVE, Color::Yellow, emojis::MOVING_TRUCK),
    "nano" => (nerdfonts::PENCIL, Color::Magenta, emojis::TEXT_EDIT),
    "netstat" => (nerdfonts::GLOBE, Color::Blue, emojis::GLOBE),
    "nnn" => (nerdfonts::DIRECTORIES, Color::Yellow, emojis::FOLDER),
    "node" => (nerdfonts::HEXAGON, Color::Green, emojis::HEXAGON),
    "nvim" => (nerdfonts::VI, Color::Green, emojis::TEXT_EDIT),
    "pico" => (nerdfonts::PENCIL, Color::Magenta, emojis::TEXT_EDIT),
    "ping" => (nerdfonts::GLOBE, Color::Blue, emojis::GLOBE),
    "ps" => (nerdfonts::GRAPH, Color::Green, emojis::BAR_CHART),
    "pwd" => (nerdfonts::DIRECTORIES, Color::Yellow, emojis::FOLDER),
    "ranger" => (nerdfonts::DIRECTORIES, Color::Yellow, emojis::FOLDER),
    "rg" => (nerdfonts::SEARCH, Color::Cyan, emojis::MAGNIFYING_GLASS),
    "rm" => (nerdfonts::TRASH, Color::Red, emojis::CROSS_MARK),
    "rsync" => (nerdfonts::SYNC, Color::Red, emojis::ANTICLOCKWISE_ARROWS),
    "ruby" => (nerdfonts::RUBY, Color::Red, emojis::DIAMONDS_SUIT),
    "rustc" => (nerdfonts::RUST, Color::Red, emojis::CRAB),
    "scp" => (nerdfonts::COMPUTERS, Color::Cyan, emojis::COMPUTER),
    "sed" => (nerdfonts::FILE, Color::Magenta, emojis::BOOK_RED),
    "sk" => (nerdfonts::SEARCH, Color::Cyan, emojis::MAGNIFYING_GLASS),
    "sleep" => (nerdfonts::HOURGLASS, Color::Cyan, emojis::ZZZ),
    "sort" => (nerdfonts::SORT, Color::Cyan, emojis::BAR_CHART),
    "spin" => (nerdfonts::DOWN, Color::Cyan, emojis::SPIN),
    "ssh" => (nerdfonts::COMPUTERS, Color::Yellow, emojis::KEY),
    "strace" => (nerdfonts::SEARCH, Color::Magenta, emojis::MICROSCOPE),
    "sudo" => (nerdfonts::WARNING, Color::Red, emojis::RED_EXCLAMATION_MARK),
    "tail" => (nerdfonts::DOWN, Color::Green, emojis::DOWN_POINTING_TRIANGLE),
    "tar" => (nerdfonts::ZIP, Color::Orange, emojis::COMPRESSED),
    "top" => (nerdfonts::GRAPH, Color::Yellow, emojis::MICROSCOPE),
    "touch" => (nerdfonts::FILE, Color::Magenta, emojis::FOLDER),
    "uniq" => (nerdfonts::DUPLICATE, Color::Yellow, emojis::LEAVES),
    "unzip" => (nerdfonts::ZIP, Color::Yellow, emojis::COMPRESSED),
    "vi" => (nerdfonts::VI, Color::Green, emojis::TEXT_EDIT),
    "vim" => (nerdfonts::VI, Color::Green, emojis::TEXT_EDIT),
    "w3m" => (nerdfonts::GLOBE, Color::Blue, emojis::GLOBE),
    "wget" => (nerdfonts::GLOBE, Color::Blue, emojis::GLOBE),
    "who" => (nerdfonts::USER, Color::Green, emojis::USER),
    "whoami" => (nerdfonts::USER, Color::Green, emojis::USER),
    "xargs" => (nerdfonts::TOOLS, Color::Yellow, emojis::TOOLS),
    "yes" => (nerdfonts::CHECK, Color::Green, emojis::CHECK),
    "zip" => (nerdfonts::ZIP, Color::Yellow, emojis::COMPRESSED),
};

pub fn get_icon(command: &str) -> Option<Icon> {
    DEFAULT_MAPPINGS
        .get(command)
        .map(|(nerdfont, color, emojis)| {
            Icon::new(nerdfont.to_string(), color.clone(), emojis.to_string())
        })
}
