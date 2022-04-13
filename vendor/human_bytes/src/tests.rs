#[cfg(not(feature = "bibytes"))]
mod decimal {
    use crate::human_bytes;

    #[test]
    fn nothing() {
        assert_eq!(human_bytes(0_u32), "0 B".to_string());
    }

    #[test]
    fn bytes() {
        assert_eq!(human_bytes(550_u32), "550 B".to_string());
    }
    #[test]
    fn kilobytes() {
        assert_eq!(human_bytes(563_200_u32), "550 KB".to_string());
    }

    #[test]
    fn megabytes() {
        assert_eq!(human_bytes(681_574_400_u32), "650 MB".to_string());
    }

    #[test]
    fn gigabytes() {
        assert_eq!(
            human_bytes(16_428_249_907_u64 as f64),
            "15.3 GB".to_string()
        );
    }

    #[test]
    fn terabytes() {
        // Hacky, I know, but easier to write ;)
        let terabyte: u64 = 2_u64.pow(40);
        assert_eq!(
            human_bytes(((terabyte * 2) + (terabyte / 2)) as f64),
            "2.5 TB"
        );
    }
}

#[cfg(feature = "bibytes")]
mod bibytes {
    use crate::human_bytes;

    #[test]
    fn nothing() {
        assert_eq!(human_bytes(0_u32), "0 B".to_string());
    }

    #[test]
    fn bytes() {
        assert_eq!(human_bytes(550_u32), "550 B".to_string());
    }
    #[test]
    fn kibibytes() {
        assert_eq!(human_bytes(563_200_u32), "550 KiB".to_string());
    }

    #[test]
    fn mebibytes() {
        assert_eq!(human_bytes(681_574_400_u32), "650 MiB".to_string());
    }

    #[test]
    fn gibibytes() {
        assert_eq!(
            human_bytes(16_428_249_907_u64 as f64),
            "15.3 GiB".to_string()
        );
    }

    #[test]
    fn tebibytes() {
        // Hacky, I know, but easier to write ;)
        let tebibyte: u64 = 2_u64.pow(40);
        assert_eq!(
            human_bytes(((tebibyte * 2) + (tebibyte / 2)) as f64),
            "2.5 TiB"
        );
    }
}
