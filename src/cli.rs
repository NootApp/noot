use std::process::exit;
use clap::Parser;

/// Helper struct to allow the user to pass args to the instance
#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    /// Allows the user to skip the default splash screen
    #[arg(short, long, default_value_t = false)]
    pub skip_splash: bool,
    
    /// Specify the workspace to load
    #[arg(short, long)]
    pub load_workspace: Option<String>,

    /// Command to list the workspaces in a table and then exit
    #[arg(long, default_value_t)]
    pub list_workspaces: bool
}




impl Args {
    pub fn process(&self) {
        if self.list_workspaces {
            Args::list_workspaces();
            exit(0);
        }
    } 

    pub fn list_workspaces() {
        let store = crate::storage::process::ProcessStorageManager::new();

        let workspaces = store.list_workspaces();

        if workspaces.len() == 0 {
            println!("No workspaces found. Launch noot to generate a workspace");
        }

        let mut builder = tabled::builder::Builder::default();

        builder.push_record(["Id", "Name", "Disk Path", "Last Accessed"]);

        for workspace in &workspaces {
            builder.push_record([
                workspace.id.clone(),
                workspace.name.clone(),
                workspace.disk_path.clone(),
                workspace.last_accessed.to_rfc2822()
            ]);
        }

        let mut table = builder.build();

        table.with(tabled::settings::Style::modern_rounded());

        println!("{}", table);
        println!("To open any of these workspaces immediately on startup, use the workspace ID in the command below:");
        println!("noot --load-workspace <Id>");
        println!("Eg: noot --load-workspace {}", workspaces.first().unwrap().id);
    }
}
