use std::{env, fs, path::PathBuf};

use ini::Ini;


struct Settings{
    grub:bool,
    initrid:bool,
    yes:bool,
    force:bool,
}
fn main(){
    let mut args: Vec<String> = env::args().skip(1).collect();
    assert!(args.len()>=1);
    println!("{:?}",args);
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
    let mut settings=Settings{grub:false,initrid:false,yes:false,force:false};
    let conf= Ini::load_from_file("config.ini").unwrap();
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
        if a=="force"||a=="f"{
            settings.force=true;
        }
    }
    
    let targets=conf.get("targets").unwrap().split(',').map(|x| x.to_string()).collect::<Vec<String>>();
    let current=conf.get("current").unwrap().to_string();
    let postexec=conf.get("postexec");
    let grub_command=conf.get("grub_command");
    assert!(postexec.is_some() && grub_command.is_some());
    let reboot:String;
    if conf.get("reboot").is_some(){
        reboot=conf.get("reboot").unwrap().to_string();
    }else {
        reboot="/sbin/reboot".to_string();
    }
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
    if !settings.force && current==target{
        eprint!("already on:{}, use -f to ignore",current);
        std::process::exit(1);
    }
    let files=conf.get("files").unwrap().split(',').map(|x| PathBuf::from(x)).collect::<Vec<PathBuf>>(); 
    println!("{}",target);
    for p in files{
        if !p.exists(){
            panic!("{}: doesn't exist",p.to_string_lossy());
        }
        if !p.is_file(){
            panic!("{}: is not a file",p.to_string_lossy());
        }
        let mut contet=fs::read_to_string(&p).expect(&format!("failed to read:{}",p.to_string_lossy()));
        
        for mut z in contet.as_mut().lines().map(|x| x.to_string()){
            let pos=z.find("#rebootrs");
            if pos.is_none(){
                continue;
            }
            // println!("{}",z.chars().skip(pos.unwrap()+"#rebootrs-".len()).collect::<String>().trim());
            if z.chars().skip(pos.unwrap()+"#rebootrs-".len()).collect::<String>().trim()==target{
                if z.starts_with('#'){
                    z=z.chars().skip(1).collect::<String>();
                }

            }else {
                if !z.starts_with('#'){
                    let mut temp:String='#'.to_string();
                    temp.push_str(z.as_str());
                    z=temp;
                }
               
            }
            println!("{}",z);
        }
    }


   
    

}