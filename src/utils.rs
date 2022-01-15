use std::collections::HashMap;
use once_cell::sync::Lazy;

pub const SERVICES: Lazy<HashMap<u32, &str>> = Lazy::new(|| {
    let hash = HashMap::from([
        (21, "FTP"),
        (990, "FTPS"),
        (22, "SSH"),
        (23, "Telnet"),
        (53, "DNS"),
        (25, "SMTP"),
        (587, "SMTP (SSL)"),
        (110, "POP"),
        (995, "POP (SSL)"),
        (143, "IMAP"),
        (993, "IMAP (SSL)"),
        (67, "DHCP"),
        (123, "NTP"),
        (80, "HTTP"),
        (8080, "HTTP"),
        (443, "HTTPS"),
        (194, "IRC"),
        (445, "SMB"),
        (5060, "SIP"),
        (3306, "MySQL"),
        (5432, "PostgreSQL"),
        (27017, "MongoDB"),
        (6379, "Redis"),
        (2082, "cPanel"),
        (6000, "X11"),
        (5672, "AMQP"),
        (389, "LDAP"),
        (636, "LDAPS"),
        (9987, "TeamSpeak 3"),
        (666, "Doom"),
        (25565, "Minecraft")
    ]);
    hash
});