use std::path::PathBuf;

use clap::{command, value_parser, Arg, ArgAction, Command, ValueHint};

use crate::progress::spinner::Spinner;

// For single thread mode set this variable on your command line:
// export RAYON_NUM_THREADS=1

pub fn build_cli() -> Command {
    command!()
        .name("Dust")
        .about("Like du but more intuitive")
        .trailing_var_arg(true)
        .arg(
            Arg::new("depth")
                .short('d')
                .long("depth")
                .num_args(1)
                .value_parser(value_parser!(usize))
                .help("Depth to show")
        )
        .arg(
            Arg::new("number_of_lines")
                .short('n')
                .long("number-of-lines")
                .num_args(1)
                .value_parser(value_parser!(usize))
                .help("Number of lines of output to show. (Default is terminal_height - 10)")
        )
        .arg(
            Arg::new("display_full_paths")
                .short('p')
                .long("full-paths")
                .action(ArgAction::SetTrue)
                .help("Subdirectories will not have their path shortened"),
        )
        .arg(
            Arg::new("ignore_directory")
                .short('X')
                .long("ignore-directory")
                .num_args(1)
                .action(ArgAction::Append)
                .value_parser(value_parser!(PathBuf))
                .help("Exclude any file or directory with this name"),
        )
         .arg(
            Arg::new("dereference_links")
                .short('L')
                .long("dereference-links")
                .action(ArgAction::SetTrue)
                .help("dereference sym links - Treat sym links as directories and go into them"),
        )
        .arg(
            Arg::new("limit_filesystem")
                .short('x')
                .long("limit-filesystem")
                .action(ArgAction::SetTrue)
                .help("Only count the files and directories on the same filesystem as the supplied directory"),
        )
        .arg(
            Arg::new("display_apparent_size")
                .short('s')
                .long("apparent-size")
                .action(ArgAction::SetTrue)
                .help("Use file length instead of blocks"),
        )
        .arg(
            Arg::new("reverse")
                .short('r')
                .long("reverse")
                .action(ArgAction::SetTrue)
                .help("Print tree upside down (biggest highest)"),
        )
        .arg(
            Arg::new("no_colors")
                .short('c')
                .long("no-colors")
                .action(ArgAction::SetTrue)
                .help("No colors will be printed (Useful for commands like: watch)"),
        )
        .arg(
            Arg::new("no_bars")
                .short('b')
                .long("no-percent-bars")
                .action(ArgAction::SetTrue)
                .help("No percent bars or percentages will be displayed"),
        )
        .arg(
            Arg::new("min_size")
                .short('z')
                .long("min-size")
                .action(ArgAction::Set)
                .num_args(1)
                .help("Minimum size file to include in output"),
        )
        .arg(
            Arg::new("screen_reader")
                .short('R')
                .long("screen-reader")
                .action(ArgAction::SetTrue)
                .help("For screen readers. Removes bars. Adds new column: depth level (May want to use -p too for full path)"),
        )
        .arg(
            Arg::new("skip_total")
                .long("skip-total")
                .action(ArgAction::SetTrue)
                .help("No total row will be displayed"),
        )
        .arg(
            Arg::new("by_filecount")
                .short('f')
                .long("filecount")
                .action(ArgAction::SetTrue)
                .help("Directory 'size' is number of child files/dirs not disk size"),
        )
        .arg(
            Arg::new("ignore_hidden")
                .short('i') // Do not use 'h' this is used by 'help'
                .long("ignore_hidden")
                .action(ArgAction::SetTrue)
                .help("Do not display hidden files"),
        )
        .arg(
            Arg::new("invert_filter")
                .short('v')
                .long("invert-filter")
                .num_args(1)
                .action(ArgAction::Append)
                .conflicts_with("filter")
                .conflicts_with("types")
                .help("Exclude filepaths matching this regex. To ignore png files type: -v \"\\.png$\" "),
        )
        .arg(
            Arg::new("filter")
                .short('e')
                .long("filter")
                .num_args(1)
                .action(ArgAction::Append)
                .conflicts_with("types")
                .help("Only include filepaths matching this regex. For png files type: -e \"\\.png$\" "),
        )
        .arg(
            Arg::new("types")
                .short('t')
                .long("file_types")
                .action(ArgAction::SetTrue)
                .conflicts_with("depth")
                .conflicts_with("only_dir")
                .help("show only these file types"),
        )
        .arg(
            Arg::new("width")
                .short('w')
                .long("terminal_width")
                .num_args(1)
                .value_parser(value_parser!(usize))
                .help("Specify width of output overriding the auto detection of terminal width"),
        )
        .arg(
            Arg::new("iso")
                .short('H')
                .long("si")
                .action(ArgAction::SetTrue)
                .help("print sizes in powers of 1000 (e.g., 1.1G)")
        )
        .arg(
            Arg::new("disable_progress")
                .short('P')
                .long("no-progress")
                .action(ArgAction::SetTrue)
                .help("Disable the progress indication."),
        )
        .arg(
            Arg::new("spinner")
                .short('S')
                .long("spinner")
                .num_args(1)
                .value_parser(value_parser!(Spinner))
        )
        .arg(
            Arg::new("only_dir")
                .short('D')
                .long("only-dir")
                .action(ArgAction::SetTrue)
                .conflicts_with("only_file")
                .conflicts_with("types")
                .help("Only directories will be displayed."),
        )
        .arg(
            Arg::new("only_file")
                .short('F')
                .long("only-file")
                .action(ArgAction::SetTrue)
                .conflicts_with("only_dir")
                .help("Only files will be displayed. (Finds your largest files)"),
        )
        .arg(
            Arg::new("inputs")
                .action(ArgAction::Append)
                .value_hint(ValueHint::AnyPath)
        )
}
