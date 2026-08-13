The `file` source skips redundant fingerprinting for already-watched files when
`fingerprint.strategy` is `device_and_inode`, reducing discovery disk I/O on large
stable file sets. Checksum fingerprinting continues to run every glob cycle so
in-place rotations such as `copytruncate` are still detected.

authors: vparfonov
