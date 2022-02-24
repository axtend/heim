use heim_common::prelude::*;
use heim_common::units::time;
use heim_host as host;

#[heim_derive::test]
async fn smoke_platform() {
    let platform = host::platform().await;
    let platform = platform.unwrap();
    println!("Platform");
    println!("  system = {}", platform.system());
    println!("  release = {}", platform.release());
    println!("  version = {}", platform.version());
    println!("  architecture = {}", platform.architecture());
    println!("  hostname = {}", platform.hostname());

    #[cfg(target_os = "windows")]
    {
        use heim_host::os::windows::PlatformExt;

        println!("  domain = {}", platform.domain());
    }
}

#[heim_derive::test]
async fn smoke_uptime() {
    let uptime = host::uptime().await;

    assert!(uptime.unwrap().get::<time::second>() > 0.0);
}

#[heim_derive::test]
async fn smoke_boot_time() {
    let boot_time = host::boot_time().await;

    assert!(boot_time.unwrap().get::<time::second>() > 0.0);
}

#[heim_derive::test]
async fn smoke_users() {
    let users = host::users().await.unwrap();
    ::futures::pin_mut!(users);
    while let Some(user) = users.next().await {
        let user = user.unwrap();

        let _ = user.username();

        #[cfg(target_os = "linux")]
        {
            use heim_host::os::linux::UserExt;

            let _ = user.pid();
            let _ = user.terminal();
            let _ = user.id();
            let _ = user.hostname();
            let _ = user.address();
            let _ = user.session_id();
        }

        #[cfg(target_os = "macos")]
        {
            use heim_host::os::macos::UserExt;

            let _ = user.pid();
            let _ = user.terminal();
            let _ = user.id();
            let _ = user.hostname();
        }

        #[cfg(target_os = "windows")]
        {
            use heim_host::os::windows::UserExt;

            let _ = user.domain();
            let _ = user.address();
        }
    }
}
