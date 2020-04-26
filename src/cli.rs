use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "movine", about = "the simple migration manager")]
pub enum Opt {
    #[structopt(name = "status")]
    /// Get the status of migrations (applied, unapplied, mismatched).
     Status {
        #[structopt(short = "v", long = "verbose")]
        /// Enable verbose logging
        debug: bool,
    },

    #[structopt(name = "up")]
    Up {
        #[structopt(short = "n", long = "number")]
        /// Number of up or down migrations to run.
        number: Option<usize>,

        #[structopt(short = "p", long = "plan")]
        /// Do a dry run and show the migration plan.
        show_plan: bool,

        #[structopt(short = "v", long = "verbose")]
        /// Enable verbose logging
        debug: bool,
    },

    #[structopt(name = "down")]
    Down {
        #[structopt(short = "n", long = "number")]
        /// Number of up or down migrations to run.
        number: Option<usize>,

        #[structopt(short = "p", long = "plan")]
        /// Do a dry run and show the migration plan.
        show_plan: bool,

        #[structopt(short = "i", long = "ignore-divergent")]
        /// Ignore any divergent migrations.
        ignore_divergent: bool,

        #[structopt(short = "v", long = "verbose")]
        /// Enable verbose logging
        debug: bool,
    },

    #[structopt(name = "fix")]
    Fix {
        #[structopt(short = "p", long = "plan")]
        /// Do a dry run and show the migration plan.
        show_plan: bool,

        #[structopt(short = "v", long = "verbose")]
        /// Enable verbose logging
        debug: bool,
    },

    #[structopt(name = "redo")]
    Redo {
        #[structopt(short = "n", long = "number")]
        /// Number of up or down migrations to run.
        number: Option<usize>,

        #[structopt(short = "p", long = "plan")]
        /// Do a dry run and show the migration plan.
        show_plan: bool,

        #[structopt(short = "v", long = "verbose")]
        /// Enable verbose logging
        debug: bool,
    },

    #[structopt(name = "custom")]
    /// [unimplemented]
    Custom {
        #[structopt(short = "p", long = "plan")]
        /// Do a dry run and show the migration plan.
        show_plan: bool,

        plan: Vec<String>,

        #[structopt(short = "v", long = "verbose")]
        /// Enable verbose logging
        debug: bool,
    },

    #[structopt(name = "generate")]
    /// Generate a migration with a given name.
    Generate { 
        name: String,

        #[structopt(short = "v", long = "verbose")]
        /// Enable verbose logging
        debug: bool,
    },

    #[structopt(name = "init")]
    /// Initialize the database and the local migration directory.
    Init {
        #[structopt(short = "v", long = "verbose")]
        /// Enable verbose logging
        debug: bool,
    },
}
