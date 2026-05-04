use crate::error::AppError;

pub fn install() -> Result<(), AppError> {
    let exe = std::env::current_exe()?;
    let exe_str = exe.display().to_string();

    #[cfg(target_os = "windows")]
    {
        let startup = dirs::data_dir()
            .ok_or_else(|| AppError::Args("cannot find AppData directory".into()))?
            .join("Microsoft\\Windows\\Start Menu\\Programs\\Startup");

        let vbs_path = startup.join("imgclip-watch.vbs");
        let vbs_content = format!(
            "Set WshShell = CreateObject(\"WScript.Shell\")\n\
             WshShell.Run \"\"\"{exe_str}\"\" --watch --quiet\", 0, False\n"
        );
        std::fs::write(&vbs_path, vbs_content)?;
        eprintln!("imgclip: installed auto-start ({})", vbs_path.display());
    }

    #[cfg(target_os = "linux")]
    {
        let autostart = dirs::config_dir()
            .ok_or_else(|| AppError::Args("cannot find config directory".into()))?
            .join("autostart");
        std::fs::create_dir_all(&autostart)?;

        let desktop = autostart.join("imgclip.desktop");
        let content = format!(
            "[Desktop Entry]\n\
             Type=Application\n\
             Name=imgclip\n\
             Exec=\"{exe_str}\" --watch --quiet\n\
             Hidden=false\n\
             NoDisplay=false\n\
             X-GNOME-Autostart-enabled=true\n"
        );
        std::fs::write(&desktop, content)?;
        eprintln!("imgclip: installed auto-start ({})", desktop.display());
    }

    #[cfg(target_os = "macos")]
    {
        let agents = dirs::home_dir()
            .ok_or_else(|| AppError::Args("cannot find home directory".into()))?
            .join("Library/LaunchAgents");
        std::fs::create_dir_all(&agents)?;

        let plist = agents.join("com.imgclip.watch.plist");
        let content = format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
             <!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \
             \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n\
             <plist version=\"1.0\"><dict>\n\
             <key>Label</key><string>com.imgclip.watch</string>\n\
             <key>ProgramArguments</key><array>\n\
             <string>{exe_str}</string>\n\
             <string>--watch</string>\n\
             <string>--quiet</string>\n\
             </array>\n\
             <key>RunAtLoad</key><true/>\n\
             </dict></plist>\n"
        );
        std::fs::write(&plist, content)?;
        eprintln!("imgclip: installed auto-start ({})", plist.display());
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        let _ = exe_str;
        return Err(AppError::Args(
            "auto-start not supported on this platform".into(),
        ));
    }

    Ok(())
}

pub fn uninstall() -> Result<(), AppError> {
    let removed = remove_entry();
    if removed {
        eprintln!("imgclip: removed auto-start");
    } else {
        eprintln!("imgclip: auto-start not found (already removed)");
    }
    Ok(())
}

fn remove_entry() -> bool {
    #[cfg(target_os = "windows")]
    {
        if let Some(path) = dirs::data_dir()
            .map(|d| d.join("Microsoft\\Windows\\Start Menu\\Programs\\Startup\\imgclip-watch.vbs"))
        {
            if path.exists() {
                let _ = std::fs::remove_file(&path);
                return true;
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        if let Some(path) = dirs::config_dir().map(|d| d.join("autostart/imgclip.desktop")) {
            if path.exists() {
                let _ = std::fs::remove_file(&path);
                return true;
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Some(path) =
            dirs::home_dir().map(|d| d.join("Library/LaunchAgents/com.imgclip.watch.plist"))
        {
            if path.exists() {
                let _ = std::fs::remove_file(&path);
                return true;
            }
        }
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {}

    false
}
