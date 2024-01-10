mod config_file;
mod console_ui;
mod file_storage;
use config_file::Config;
use copypasta::{ClipboardContext, ClipboardProvider};
use file_storage::FileStorage;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{collections::HashMap, io::ErrorKind};
use zeroize::Zeroize;

type Pwds = HashMap<String, String>;

#[tokio::main]
async fn main() {
    let password = console_ui::input_string("Master password: ").await;
    let storage = FileStorage {
        file_path: Config::get_storage_path().await.pwd_file_path,
        pwd: password,
    };
    let mc = new_magic_crypt!(&storage.pwd, 256);
    let loaded_pwds_str_res = storage.load_pwds().await;
    let mut pwds_update = false;
    let mut pwds: Pwds = {
        match loaded_pwds_str_res {
            Ok(pwds_str) => {
                let mut decrypred = mc
                    .decrypt_base64_to_string(pwds_str)
                    .expect("Password storage is damaged, not possible to decrypt");
                let pwds_res: serde_json::Result<Pwds> = serde_json::from_str(&decrypred);
                decrypred.zeroize();
                match pwds_res {
                    Ok(pwds) => pwds,
                    Err(e) => {
                        panic!("Error while parsing decrypted data: {:?}", e)
                    }
                }
            }
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    println!("No pwd file, it will be created later");
                    HashMap::new()
                } else {
                    panic!("Unknown error while trying to access pwd file: {:?}", e);
                }
            }
        }
    };

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {
        print!(">>> ");
        std::io::stdout().flush().expect(":((");
        let input = console_ui::input_string("").await;
        match input.as_str().trim() {
            "list" => {
                let mut i = 1;
                for key  in pwds.keys() {
                    println!("{i}. {key}");
                    i += 1;
                }
                if i == 1 {
                    println!("No saved passwords yet.\nYou can add them using `add` command");
                }
            }
            "exit" => {
                break;
            }
            "add" => {
                let name = loop {
                    let n = console_ui::input_string("Name: ").await.trim().to_string();
                    if pwds.contains_key(&n) {
                        println!("Such password name already exists");
                    } else {
                        break n;
                    }
                };
                let password = rpassword::prompt_password("Password: ").unwrap();
                pwds.insert(name, password);
                pwds_update = true;
            }
            "get" => {
                let key = loop {
                    let n = console_ui::input_string("Pwd name: ")
                        .await
                        .trim()
                        .to_string();
                    if !pwds.contains_key(&n) {
                        println!("No such password name");
                    } else {
                        break n;
                    }
                };
                let pwd = pwds.get(&key).expect("We checked such key exists");
                let mut ctx = ClipboardContext::new().expect("Can't access clipboard");
                ctx.set_contents(pwd.to_owned()).unwrap();
                println!("Password was copied to clipboard");
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
            _ => {
                println!(
                    "Usage:
    `list` - list all resources for which you have saved password
    `add`  - add new password
    `get`  - select password, copy it to clipboard
    `exit` - sync and exit"
                )
            }
        }
    }
    if pwds_update {
        let mut pwds =
            serde_json::to_string(&pwds).expect("Failed to se passwords somehow");
        let data = mc.encrypt_str_to_base64(&pwds);
        if let Err(e) = storage.save_pwds(&data).await {
            eprint!("Failed to save passwords: {:?}", e);
        };
        pwds.zeroize();
    }
    for (mut key, mut pwd) in pwds {
        key.zeroize();
        pwd.zeroize();
    }
}
