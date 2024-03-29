use std::process::{Command,exit};
use names::Generator;

fn update_commit_push(){
    let add_command=Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("failed to execute git add command");

    if !add_command.status.success(){
        eprintln!("Error: Failed to add files to the git-repo");
        exit(1);
    }

    let commit_command=Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(name_generator())
        .output()
        .expect("failed to execute git commit command");

    if !commit_command.status.success(){
        eprintln!("Error: Failed to commit files to the git-repo");
        exit(1);
    }

    let push_command=Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("master")
        .output()
        .expect("failed to execute git push successfully");

    if !push_command.status.success(){
        eprintln!("Error: Failed to push files to git-repo");
        exit(1);
    }
    println!("Successfully added,committed, and pushed all changes");
}

fn name_generator() -> String{
    let mut generator=Generator::default();
    generator.next().unwrap()

}

fn main(){
    update_commit_push();
}