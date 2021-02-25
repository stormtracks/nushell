/*
pub const BAT_LANGUAGES: &[&str] = &[
    "as",
    "csv",
    "tsv",
    "applescript",
    "script editor",
    "s",
    "S",
    "adoc",
    "asciidoc",
    "asc",
    "asa",
    "yasm",
    "nasm",
    "asm",
    "inc",
    "mac",
    "awk",
    "bat",
    "cmd",
    "bib",
    "sh",
    "bash",
    "zsh",
    ".bash_aliases",
    ".bash_completions",
    ".bash_functions",
    ".bash_login",
    ".bash_logout",
    ".bash_profile",
    ".bash_variables",
    ".bashrc",
    ".profile",
    ".textmate_init",
    ".zshrc",
    "PKGBUILD",
    ".ebuild",
    ".eclass",
    "c",
    "h",
    "cs",
    "csx",
    "cpp",
    "cc",
    "cp",
    "cxx",
    "c++",
    "C",
    "h",
    "hh",
    "hpp",
    "hxx",
    "h++",
    "inl",
    "ipp",
    "cabal",
    "clj",
    "cljc",
    "cljs",
    "edn",
    "CMakeLists.txt",
    "cmake",
    "h.in",
    "hh.in",
    "hpp.in",
    "hxx.in",
    "h++.in",
    "CMakeCache.txt",
    "cr",
    "css",
    "css.erb",
    "css.liquid",
    "d",
    "di",
    "dart",
    "diff",
    "patch",
    "Dockerfile",
    "dockerfile",
    "ex",
    "exs",
    "elm",
    "erl",
    "hrl",
    "Emakefile",
    "emakefile",
    "fs",
    "fsi",
    "fsx",
    "fs",
    "fsi",
    "fsx",
    "fish",
    "attributes",
    "gitattributes",
    ".gitattributes",
    "COMMIT_EDITMSG",
    "MERGE_MSG",
    "TAG_EDITMSG",
    "gitconfig",
    ".gitconfig",
    ".gitmodules",
    "exclude",
    "gitignore",
    ".gitignore",
    ".git",
    "gitlog",
    "git-rebase-todo",
    "go",
    "dot",
    "DOT",
    "gv",
    "groovy",
    "gvy",
    "gradle",
    "Jenkinsfile",
    "hs",
    "hs",
    "hsc",
    "show-nonprintable",
    "html",
    "htm",
    "shtml",
    "xhtml",
    "asp",
    "html.eex",
    "yaws",
    "rails",
    "rhtml",
    "erb",
    "html.erb",
    "adp",
    "twig",
    "html.twig",
    "ini",
    "INI",
    "INF",
    "reg",
    "REG",
    "lng",
    "cfg",
    "CFG",
    "desktop",
    "url",
    "URL",
    ".editorconfig",
    ".hgrc",
    "hgrc",
    "java",
    "bsh",
    "properties",
    "jsp",
    "js",
    "htc",
    "js",
    "jsx",
    "babel",
    "es6",
    "js.erb",
    "json",
    "sublime-settings",
    "sublime-menu",
    "sublime-keymap",
    "sublime-mousemap",
    "sublime-theme",
    "sublime-build",
    "sublime-project",
    "sublime-completions",
    "sublime-commands",
    "sublime-macro",
    "sublime-color-scheme",
    "ipynb",
    "Pipfile.lock",
    "jsonnet",
    "libsonnet",
    "libjsonnet",
    "jl",
    "kt",
    "kts",
    "tex",
    "ltx",
    "less",
    "css.less",
    "lisp",
    "cl",
    "clisp",
    "l",
    "mud",
    "el",
    "scm",
    "ss",
    "lsp",
    "fasl",
    "lhs",
    "lua",
    "make",
    "GNUmakefile",
    "makefile",
    "Makefile",
    "makefile.am",
    "Makefile.am",
    "makefile.in",
    "Makefile.in",
    "OCamlMakefile",
    "mak",
    "mk",
    "md",
    "mdown",
    "markdown",
    "markdn",
    "matlab",
    "build",
    "nix",
    "m",
    "h",
    "mm",
    "M",
    "h",
    "ml",
    "mli",
    "mll",
    "mly",
    "pas",
    "p",
    "dpr",
    "pl",
    "pm",
    "pod",
    "t",
    "PL",
    "php",
    "php3",
    "php4",
    "php5",
    "php7",
    "phps",
    "phpt",
    "phtml",
    "txt",
    "ps1",
    "psm1",
    "psd1",
    "proto",
    "protodevel",
    "pb.txt",
    "proto.text",
    "textpb",
    "pbtxt",
    "prototxt",
    "pp",
    "epp",
    "purs",
    "py",
    "py3",
    "pyw",
    "pyi",
    "pyx",
    "pyx.in",
    "pxd",
    "pxd.in",
    "pxi",
    "pxi.in",
    "rpy",
    "cpy",
    "SConstruct",
    "Sconstruct",
    "sconstruct",
    "SConscript",
    "gyp",
    "gypi",
    "Snakefile",
    "wscript",
    "R",
    "r",
    "s",
    "S",
    "Rprofile",
    "rd",
    "re",
    "rst",
    "rest",
    "robot",
    "rb",
    "Appfile",
    "Appraisals",
    "Berksfile",
    "Brewfile",
    "capfile",
    "cgi",
    "Cheffile",
    "config.ru",
    "Deliverfile",
    "Fastfile",
    "fcgi",
    "Gemfile",
    "gemspec",
    "Guardfile",
    "irbrc",
    "jbuilder",
    "Podfile",
    "podspec",
    "prawn",
    "rabl",
    "rake",
    "Rakefile",
    "Rantfile",
    "rbx",
    "rjs",
    "ruby.rail",
    "Scanfile",
    "simplecov",
    "Snapfile",
    "thor",
    "Thorfile",
    "Vagrantfile",
    "haml",
    "sass",
    "rxml",
    "builder",
    "rs",
    "scala",
    "sbt",
    "sql",
    "ddl",
    "dml",
    "erbsql",
    "sql.erb",
    "swift",
    "log",
    "tcl",
    "tf",
    "tfvars",
    "hcl",
    "sty",
    "cls",
    "textile",
    "toml",
    "tml",
    "Cargo.lock",
    "Gopkg.lock",
    "Pipfile",
    "ts",
    "tsx",
    "varlink",
    "vim",
    ".vimrc",
    "xml",
    "xsd",
    "xslt",
    "tld",
    "dtml",
    "rss",
    "opml",
    "svg",
    "yaml",
    "yml",
    "sublime-syntax",
];
*/
