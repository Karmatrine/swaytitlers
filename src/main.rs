use std::env;
use std::process::exit;
use swayipc::Node;
use swayipc::Output;
use swayipc::{Connection, Event, EventType, WorkspaceChange};
use tokio::signal::ctrl_c;

fn find_focused_window(node: &Node) -> Option<&Node> {
    if node.focused && node.node_type != swayipc::NodeType::Workspace {
        return Some(node);
    }
    for child in &node.nodes {
        if let Some(focused) = find_focused_window(child) {
            return Some(focused);
        }
    }
    for child in &node.floating_nodes {
        if let Some(focused) = find_focused_window(child) {
            return Some(focused);
        }
    }
    None
}

async fn handle_window_change(connection: &mut Connection, output: &str) {
    let outputs: Vec<Output> = connection.get_outputs().unwrap();
    if let Some(focused_output) = outputs.into_iter().find(|o| o.focused) {
        if focused_output.name == output {
            let tree: Node = connection.get_tree().unwrap();
            if let Some(focused_window) = find_focused_window(&tree) {
                println!(
                    "{}",
                    focused_window.name.as_ref().unwrap_or(&"".to_string())
                );
            } else {
                println!();
            }
        } else {
            println!();
        }
    } else {
        println!();
    }
}

async fn handle_binding_change(connection: &mut Connection, event: &Event, output: &str) {
    if let Event::Binding(binding_event) = event {
        let command = &binding_event.binding.command;
        if command == "kill" || command.contains("move") {
            handle_window_change(connection, output).await;
        }
    }
}

async fn handle_workspace_change(connection: &mut Connection, event: &Event, output: &str) {
    if let Event::Workspace(workspace_event) = event {
        if workspace_event.change == WorkspaceChange::Init
            || workspace_event.change == WorkspaceChange::Empty
        {
            handle_window_change(connection, output).await;
        }
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: Please provide the output name as a command-line argument.");
        exit(1);
    }
    let output = &args[1];

    let connection = Connection::new().unwrap();
    let mut subs = connection
        .subscribe([EventType::Workspace, EventType::Binding, EventType::Window])
        .unwrap();

    let window_change_fut = async {
        let mut connection = Connection::new().unwrap();
        for event in subs.by_ref() {
            match event {
                Ok(event) => match &event {
                    Event::Window(_) => handle_window_change(&mut connection, output).await,
                    Event::Binding(_) => {
                        handle_binding_change(&mut connection, &event, output).await
                    }
                    Event::Workspace(_) => {
                        handle_workspace_change(&mut connection, &event, output).await
                    }
                    _ => (),
                },
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                }
            }
        }
    };

    let signal_fut = async {
        ctrl_c().await.unwrap();
        println!("Cleaning up and exiting...");
        exit(0);
    };

    tokio::select! {
        _ = window_change_fut => {},
        _ = signal_fut => {},
    }
}
