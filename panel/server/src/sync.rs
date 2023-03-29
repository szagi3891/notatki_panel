use tokio::sync::Notify;
use tokio::time::sleep;
use std::sync::Arc;
use std::time::Duration;
use crate::utils::{SpawnOwner, spawn_and_wait};
use tokio::process::Command;
use tokio::time::timeout;

fn convert_to_lines(data: Vec<u8>) -> Vec<String> {
    let data = String::from_utf8(data).unwrap();
    let result: Vec<String> = data.lines().map(String::from).collect();
    result
}

struct Executor {
    git_sync: String,
}

impl Executor {
    fn new(git_sync: String) -> Executor {
        Executor {
            git_sync,
        }
    }

    async fn exec_command_inner(&self, command: &mut Command, ignore_error: bool) -> Vec<String> {
        command.current_dir(&self.git_sync);

        let command_text = format!("{:?}", command);
        let output = match timeout(Duration::from_secs(7), command.output()).await {
            Ok(Ok(data)) => data,
            Ok(Err(err)) => {
                panic!("Niepowodzenie0 ==> {} ==> {}", command_text, err);
            },
            Err(_err) => {
                panic!("timeout");
            }
        };

        if !ignore_error {
            if !output.status.success() {
                println!("status code -> {:?}", output.status.code());
                panic!("Niepowodzenie1 ==> {}", command_text);
            }
        
            let stderr = convert_to_lines(output.stderr);
            if stderr.len() > 0 {
                println!("stderr -> {:?}", stderr);
                panic!("Niepowodzenie2 ==> {}", command_text);
            }
        }

        convert_to_lines(output.stdout)
    }

    async fn exec_command(&self, command: &mut Command) -> Vec<String> {
        self.exec_command_inner(command, false).await
    }

    async fn exec_command_ignore_error(&self, command: &mut Command) -> Vec<String> {
        self.exec_command_inner(command, true).await
    }
}

fn get_exact_one(mut response: Vec<String>) -> String {
    let line = response.pop().unwrap();
    assert_eq!(response.len(), 0);
    line
}

async fn get_current_branch(exec: &Executor) -> String {
    let response = exec.exec_command(Command::new("git").arg("branch").arg("--show-current")).await;
    get_exact_one(response)
}

async fn get_local_commit(exec: &Executor, branch: &String) -> String {
    let response = exec.exec_command(Command::new("git")
        .arg("log")
        .arg("-1")
        .arg(format!("origin/{branch}"))
        .arg(r#"--pretty=format:"%H""#)
    ).await;

    get_exact_one(response)
}

async fn get_remote_commit(exec: &Executor, branch: &String) -> String {
    let response = exec.exec_command(Command::new("git")
        .arg("log")
        .arg("-1")
        .arg(branch)
        .arg(r#"--pretty=format:"%H""#)
    ).await;

    get_exact_one(response)
}

async fn has_commit_synchronized(exec: &Executor, branch: &String) -> bool {
    let local = get_local_commit(exec, branch).await;
    let remote = get_remote_commit(exec, branch).await;

    local == remote
}


pub async fn start_sync(notify: Arc<Notify>, git_sync: String) -> SpawnOwner {    
    SpawnOwner::new(async move {
        loop {
            spawn_and_wait({
                let notify = notify.clone();
                let git_sync = git_sync.clone();

                notify.notify_one();

                async move {
                    loop {
                        notify.notified().await;
                        sync(git_sync.clone()).await;
                    }
                }
            }).await;

            sleep(Duration::from_millis(5000)).await;

            log::info!("Restart sync process ...");
        }
    })
}


async fn sync(git_sync: String) {
    let exec = Executor::new(git_sync);

    let current_branch = get_current_branch(&exec).await;
    log::info!("Start sync {} ...", current_branch);



    log::info!("Try git pull origin");
    exec.exec_command_ignore_error(
        Command::new("git")
        .arg("pull")
        .arg("origin")
        .arg(&current_branch)
    ).await;

    exec.exec_command_ignore_error(
        Command::new("git")
        .arg("merge")
        .arg("--abort")
    ).await;

    if has_commit_synchronized(&exec, &current_branch).await {
        log::info!("Sync ok...");
        return;
    }


    log::info!("Try git rebase and push");

    exec.exec_command_ignore_error(
        Command::new("git")
        .arg("rebase")
        .arg(format!("origin/{}", &current_branch))
    ).await;

    exec.exec_command_ignore_error(
        Command::new("git")
        .arg("rebase")
        .arg("--abort")
    ).await;

    exec.exec_command_ignore_error(
        Command::new("git")
        .arg("push")
        .arg("origin")
        .arg(format!("{}:{}", &current_branch, &current_branch))
    ).await;

    if has_commit_synchronized(&exec, &current_branch).await {
        log::info!("Sync ok...");
        return;
    }


    panic!("Dalsze kroki synchronizacyjne");
}