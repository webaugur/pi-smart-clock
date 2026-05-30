use crate::drivers::platform::Platform;

pub async fn safe_flash_with_rollback<P: Platform>(platform: &mut P, new_firmware: &[u8]) {
    println!("🔄 Backing up current firmware...");
    platform.flash_backup_current().await;

    println!("🔄 Flashing new firmware...");
    platform.flash_write(0x100000, new_firmware).await;

    println!("🔄 Setting verification flag...");
    platform.set_boot_flag(0xA5A5A5A5).await;

    println!("🔄 Rebooting into verification mode...");
    platform.reboot();
}