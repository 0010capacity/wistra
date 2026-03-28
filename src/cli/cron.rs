use anyhow::{Context, Result, bail};
use std::process::Command;

/// Generate cron line for wistra
pub fn generate_cron_line(hour: u8, minute: u8, no_git: bool) -> String {
    let git_flag = if no_git { " --no-git" } else { "" };
    format!("{} {} * * * wistra run --quiet --no-confirm{}", minute, hour, git_flag)
}

/// Parse time string (HH:MM format)
pub fn parse_time(time: &str) -> Result<(u8, u8)> {
    let parts: Vec<&str> = time.split(':').collect();
    if parts.len() != 2 {
        bail!("Time must be in HH:MM format (e.g., 14:30)");
    }

    let hour: u8 = parts[0]
        .parse()
        .context("Invalid hour")?;
    let minute: u8 = parts[1]
        .parse()
        .context("Invalid minute")?;

    if hour > 23 {
        bail!("Hour must be between 0 and 23");
    }
    if minute > 59 {
        bail!("Minute must be between 0 and 59");
    }

    Ok((hour, minute))
}

/// Show cron line for the given time
pub fn show_cron(hour: u8, minute: u8, no_git: bool) {
    println!("📝 Add this line to your crontab (crontab -e):");
    println!("    {}", generate_cron_line(hour, minute, no_git));
}

/// Install cron job using crontab
pub fn install_cron(hour: u8, minute: u8, no_git: bool) -> Result<()> {
    let new_line = generate_cron_line(hour, minute, no_git);

    // Get current crontab
    let current_crontab = Command::new("crontab")
        .arg("-l")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default();

    // Check if wistra cron already exists
    let lines: Vec<&str> = current_crontab.lines().collect();
    let mut updated_lines: Vec<String> = lines.iter().map(|s| s.to_string()).collect();
    let mut found = false;

    for line in &mut updated_lines {
        if line.contains("wistra run") {
            *line = new_line.clone();
            found = true;
            break;
        }
    }

    if !found {
        updated_lines.push(new_line.clone());
    }

    // Create temporary crontab content
    let new_crontab = updated_lines.join("\n");
    let temp_file = std::env::temp_dir().join("wistra_crontab");

    std::fs::write(&temp_file, &new_crontab)
        .context("Failed to write temporary crontab file")?;

    // Install new crontab
    let status = Command::new("crontab")
        .arg(&temp_file)
        .status()
        .context("Failed to run crontab command")?;

    if !status.success() {
        bail!("Failed to install crontab");
    }

    // Clean up temp file
    let _ = std::fs::remove_file(&temp_file);

    if found {
        println!("✅ Updated existing cron job: {}", new_line);
    } else {
        println!("✅ Added new cron job: {}", new_line);
    }

    Ok(())
}

/// Remove wistra cron job from crontab
pub fn remove_cron() -> Result<()> {
    // Get current crontab
    let output = Command::new("crontab")
        .arg("-l")
        .output()
        .context("Failed to read crontab")?;

    if !output.status.success() {
        println!("No crontab configured.");
        return Ok(());
    }

    let current_crontab = String::from_utf8(output.stdout)
        .context("Invalid crontab content")?;

    // Remove wistra lines
    let lines: Vec<&str> = current_crontab.lines().collect();
    let updated_lines: Vec<&str> = lines
        .iter()
        .filter(|line| !line.contains("wistra run"))
        .copied()
        .collect();

    if updated_lines.len() == lines.len() {
        println!("No wistra cron job found.");
        return Ok(());
    }

    // Create temporary crontab content
    let new_crontab = updated_lines.join("\n");
    let temp_file = std::env::temp_dir().join("wistra_crontab");

    std::fs::write(&temp_file, &new_crontab)
        .context("Failed to write temporary crontab file")?;

    // Install new crontab
    let status = Command::new("crontab")
        .arg(&temp_file)
        .status()
        .context("Failed to run crontab command")?;

    if !status.success() {
        bail!("Failed to update crontab");
    }

    // Clean up temp file
    let _ = std::fs::remove_file(&temp_file);

    println!("✅ Removed wistra cron job.");

    Ok(())
}
