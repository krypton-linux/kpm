use alpm::Alpm;
use tr::tr;


pub fn sync_db(){
    let Ok(alpm) = Alpm::new("/", "/var/lib/pacman") else{
        eprintln!("{}", tr!("Failed to initialize ALPM"));
        std::process::exit(1);
    };

    
    alpm.register_syncdb("core", alpm::SigLevel::DATABASE).unwrap_or_else(|e| {
        eprintln!("{}", tr!("Failed to register csyncdb:"));
        eprintln!("{}", tr!("{}", e));
        std::process::exit(1);
    });



    alpm.register_syncdb("extra", alpm::SigLevel::DATABASE).unwrap_or_else(|e| {
        eprintln!("{}", tr!("Failed to register syncdb:"));
        eprintln!("{}", tr!("{}", e));
        std::process::exit(1);
    });



}

