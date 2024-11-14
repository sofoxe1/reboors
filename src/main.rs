use std::{env, fs, io::stdin, path::PathBuf, process::Command};
use ini::Ini;


struct Settings{
    grub:bool,
    initrid:bool,
    yes:bool,
}
fn main(){
    let mut args: Vec<String> = env::args().skip(1).collect();
    assert!(args.len()>=1);
    let mut target=args.swap_remove(0);
    if target.starts_with('-'){
        panic!("first argument can't start with '-'");
    }
    let mut args_s:Vec<char>=Vec::new();
    let mut args_l:Vec<String>=Vec::new();
    for arg in args{
        if arg.starts_with("--"){
            args_l.push(arg.chars().skip(2).collect());
        }else if arg.starts_with('-') {
            args_s.extend(arg.chars().skip(1));            
        }

    }
    let mut settings=Settings{grub:false,initrid:false,yes:false};
    let conf= Ini::load_from_file("/etc/rebootrs/config.ini").unwrap();
    let conf=&conf["General"];
   
    let mut args=args_l.into_iter().chain(args_s.into_iter().map(|x| x.to_string())).collect::<Vec<String>>();
    let conf_args=conf.get("args");
    if conf_args.is_some(){
        args.extend(conf.get("args").unwrap().split(',').map(|x| x.to_string()).collect::<Vec<String>>());
    }
    
    for a in args{
        if a=="grub"||a=="g"{
            settings.grub=true;
        }if a=="initrid"||a=="i"{
            settings.initrid=true;
        }if a=="yes"||a=="y"{
            settings.yes=true;
        }
    }
    
    let targets=conf.get("targets").unwrap().split(',').map(|x| x.to_string()).collect::<Vec<String>>();
    let postexec=conf.get("postexec");
    let grub_command=conf.get("grub_command").unwrap();
    let initrd_command=conf.get("initrd_command").unwrap();
    let reboot=conf.get("reboot").or(Some("/sbin/reboot")).unwrap();
    let mut hit=None;
    if !targets.contains(&target.to_string()){
        for t in targets{
            if t.starts_with(&target){
                if hit.is_some(){
                    panic!("use longer abbreviation");
                }
                hit=Some(t.clone());
            }
        }
        if hit.is_none(){
            panic!("target not specified");
        }
        target=hit.unwrap();
    }
    let files=conf.get("files").unwrap().split(',').map(|x| PathBuf::from(x)).collect::<Vec<PathBuf>>(); 
    for p in files{
        if !p.exists(){
            panic!("{}: doesn't exist",p.to_string_lossy());
        }
        if !p.is_file(){
            panic!("{}: is not a file",p.to_string_lossy());
        }
        let mut contet=fs::read_to_string(&p).expect(&format!("failed to read:{}",p.to_string_lossy()));
        let mut write=String::new();
        for z in contet.as_mut().lines().map(|x| x.to_string()){
            let pos=z.find("#rebootrs");
            if pos.is_none(){
                write.push_str(&z);
                write.push('\n');
                continue;
            }
            if z.chars().skip(pos.unwrap()+"#rebootrs-".len()).collect::<String>().trim()==target{
                if z.starts_with('#'){
                    write.push_str(&z.chars().skip(1).collect::<String>());
                }else {
                    write.push_str(&z);
                }

            }else {
                if !z.starts_with('#'){
                    write.push('#');
                    write.push_str(&z);
                }else {
                    write.push_str(&z);
                }
               
            }
            write.push('\n');
            
          
        }
        fs::write(p,write).unwrap();
    }
    if settings.grub{
        let t = Command::new(grub_command.split(" ").take(1).collect::<String>()).args(grub_command.split(" ").skip(1).collect::<Vec<&str>>()).spawn();
        t.unwrap().wait().unwrap();
    }
    if settings.initrid{
        let t = Command::new(initrd_command.split(" ").take(1).collect::<String>()).args(initrd_command.split(" ").skip(1).collect::<Vec<&str>>()).spawn();
        t.unwrap().wait().unwrap();
    }
    if postexec.is_some(){
        let postexec=postexec.unwrap();
        let t = Command::new(postexec.split(" ").take(1).collect::<String>()).args(postexec.split(" ").skip(1).collect::<Vec<&str>>()).spawn();
        t.unwrap().wait().unwrap();
    }
    if !settings.yes{
        println!("press ENTER to reboot or ctrl+c to cancel");
        stdin().read_line(&mut String::new()).unwrap();
        println!("rebooting");
    }
    let _ =Command::new(reboot.split(" ").take(1).collect::<String>()).args(reboot.split(" ").skip(1).collect::<Vec<&str>>()).spawn();
}