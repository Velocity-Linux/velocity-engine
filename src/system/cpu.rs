use std::path::PathBuf;

pub struct SystemUtils;

impl SystemUtils {
    pub fn read_file(path: &str) -> Option<String> {
        std::fs::read_to_string(path).ok()
    }

    pub fn read_first_line(path: &str) -> Option<String> {
        Self::read_file(path).and_then(|s| s.lines().next().map(|s| s.to_string()))
    }

    pub fn write_file(path: &str, content: &str) -> Result<(), String> {
        std::fs::write(path, content).map_err(|e| format!("Cannot write {}: {}", path, e))
    }

    pub fn file_exists(path: &str) -> bool {
        PathBuf::from(path).exists()
    }

    pub fn run_command(cmd: &str, args: &[&str]) -> Result<String, String> {
        debug!("Running command: {} {:?}", cmd, args);
        let output = std::process::Command::new(cmd)
            .args(args)
            .output()
            .map_err(|e| format!("Failed to execute {}: {}", cmd, e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Command failed: {}: {}", cmd, stderr));
        }

        String::from_utf8(output.stdout)
            .map(|s| s.trim().to_string())
            .map_err(|e| format!("Invalid UTF-8 output: {}", e))
    }

    pub fn get_process_list() -> Vec<(u32, String)> {
        let mut processes = Vec::new();
        if let Ok(entries) = std::fs::read_dir("/proc") {
            for entry in entries.flatten() {
                if let Ok(pid) = entry.file_name().to_string_lossy().parse::<u32>() {
                    if let Ok(cmdline) = std::fs::read(format!("/proc/{}/cmdline", pid)) {
                        let cmdline_str = String::from_utf8_lossy(&cmdline);
                        let process_name = cmdline_str
                            .split('\0')
                            .next()
                            .and_then(|s| s.rsplit('/').next().map(|s| s.to_string()))
                            .unwrap_or_default();
                        if !process_name.is_empty() {
                            processes.push((pid, process_name));
                        }
                    }
                }
            }
        }
        processes
    }

    pub fn is_process_running(name: &str) -> bool {
        let processes = Self::get_process_list();
        processes.iter().any(|(_, proc_name)| proc_name == name)
    }

    pub fn find_pids_by_name(name: &str) -> Vec<u32> {
        let processes = Self::get_process_list();
        processes
            .iter()
            .filter(|(_, proc_name)| proc_name == name)
            .map(|(pid, _)| *pid)
            .collect()
    }

    pub fn get_current_cpu_governor() -> Option<String> {
        let governors_path = "/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor";
        Self::read_first_line(governors_path)
    }

    pub fn set_cpu_governor(governor: &str) -> Result<(), String> {
        let pattern = "/sys/devices/system/cpu/cpu*/cpufreq/scaling_governor";
        if let Ok(paths) = glob::glob(pattern) {
            for path in paths.flatten() {
                if let Err(e) = Self::write_file(path.to_str().unwrap(), governor) {
                    warn!("Cannot set governor on {}: {}", path.display(), e);
                }
            }
            Ok(())
        } else {
            Err("Cannot find CPU frequency paths".to_string())
        }
    }

    pub fn get_available_governors() -> Vec<String> {
        let path = "/sys/devices/system/cpu/cpu0/cpufreq/scaling_available_governors";
        Self::read_file(&path)
            .map(|s| s.split_whitespace().map(|s| s.to_string()).collect())
            .unwrap_or_default()
    }

    pub fn get_cpu_affinity(pid: u32) -> Vec<usize> {
        let path = format!("/proc/{}/status", pid);
        if let Ok(content) = std::fs::read_to_string(&path) {
            for line in content.lines() {
                if line.starts_with("Cpus_allowed_list:") {
                    let parts: Vec<&str> = line.split(':').collect();
                    if parts.len() == 2 {
                        return Self::parse_cpu_list(parts[1].trim());
                    }
                }
            }
        }
        Vec::new()
    }

    pub fn set_cpu_affinity(pid: u32, cpus: &[usize]) -> Result<(), String> {
        let mut mask: u64 = 0;
        for cpu in cpus {
            if *cpu < 64 {
                mask |= 1u64 << cpu;
            }
        }

        unsafe {
            let res = libc::sched_setaffinity(
                pid,
                std::mem::size_of::<u64>(),
                &mask as *const u64 as *const libc::c_void,
            );
            if res != 0 {
                return Err(format!(
                    "sched_setaffinity failed: {}",
                    std::io::Error::last_os_error()
                ));
            }
        }
        debug!("Set CPU affinity for PID {} to {:?}", pid, cpus);
        Ok(())
    }

    pub fn set_process_priority(pid: u32, priority: i32) -> Result<(), String> {
        unsafe {
            let res = libc::setpriority(libc::PRIO_PROCESS, pid, priority);
            if res != 0 {
                return Err(format!(
                    "setpriority failed: {}",
                    std::io::Error::last_os_error()
                ));
            }
        }
        debug!("Set priority for PID {} to {}", pid, priority);
        Ok(())
    }

    pub fn set_io_priority(pid: u32, class: &str, priority: i32) -> Result<(), String> {
        let ioprio_data = match class {
            "realtime" => ((2 << 13) | (priority.clamp(0, 7))),
            "best-effort" => ((1 << 13) | (priority.clamp(0, 7))),
            "idle" => ((3 << 13) | 0),
            _ => return Err(format!("Unknown I/O priority class: {}", class)),
        };

        unsafe {
            let res = libc::syscall(libc::SYS_ioprio_set, 1, pid, ioprio_data);
            if res != 0 {
                return Err(format!(
                    "ioprio_set failed: {}",
                    std::io::Error::last_os_error()
                ));
            }
        }
        debug!("Set I/O priority for PID {} to {}", pid, class);
        Ok(())
    }

    fn parse_cpu_list(s: &str) -> Vec<usize> {
        let mut cpus = Vec::new();
        for part in s.split(',') {
            if part.contains('-') {
                let range: Vec<&str> = part.split('-').collect();
                if range.len() == 2 {
                    if let (Ok(start), Ok(end)) =
                        (range[0].parse::<usize>(), range[1].parse::<usize>())
                    {
                        cpus.extend(start..=end);
                    }
                }
            } else if let Ok(cpu) = part.parse::<usize>() {
                cpus.push(cpu);
            }
        }
        cpus
    }
}
