use alpm::Alpm;


pub fn sync_db(){
    let Ok(alpm) = Alpm::new("/", "/var/lib/pacman") else{
        eprintln!("Failed to initialize ALPM");
        std::process::exit(1);
    };

    
    alpm.register_syncdb("core", alpm::SigLevel::DATABASE).unwrap_or_else(|e| {
        eprintln!("Failed to register csyncdb:");
        eprintln!("{e}");
        std::process::exit(1);
    });



    alpm.register_syncdb("extra", alpm::SigLevel::DATABASE).unwrap_or_else(|e| {
        eprintln!("Failed to register syncdb:");
        eprintln!("{e}");
        std::process::exit(1);
    });



}

