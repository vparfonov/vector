#[cfg(not(feature = "bibytes"))]
mod decimal {
    use crate::human_bytes;

    #[test]
    fn nothing() {
        assert_eq!(human_bytes(0_u32).as_ref(), "0 B");
    }

    #[test]
    fn bytes() {
        assert_eq!(human_bytes(550_u32).as_ref(), "550 B");
    }
    #[test]
    fn kilobytes() {
        assert_eq!(human_bytes(563_200_u32).as_ref(), "550 KB");
    }

    #[test]
    fn megabytes() {
        assert_eq!(human_bytes(681_574_400_u32).as_ref(), "650 MB");
    }

    #[test]
    fn gigabytes() {
        assert_eq!(
            human_bytes(16_428_249_907_u64 as f64).as_ref(),
            "15.3 GB"
        );
    }

    #[test]
    fn terabytes() {
        // Hacky, I know, but easier to write ;)
        let terabyte: u64 = 2_u64.pow(40);
        assert_eq!(
            human_bytes(((terabyte * 2) + (terabyte / 2)) as f64).as_ref(),
            "2.5 TB"
        );
    }
}

#[cfg(feature = "bibytes")]
mod bibytes {
    use crate::human_bytes;

    #[test]
    fn nothing() {
        assert_eq!(human_bytes(0_u32).as_ref(), "0 B");
    }

    #[test]
    fn bytes() {
        assert_eq!(human_bytes(550_u32).as_ref(), "550 B");
    }
    #[test]
    fn kibibytes() {
        assert_eq!(human_bytes(563_200_u32).as_ref(), "550 KiB");
    }

    #[test]
    fn mebibytes() {
        assert_eq!(human_bytes(681_574_400_u32).as_ref(), "650 MiB");
    }

    #[test]
    fn gibibytes() {
        assert_eq!(
            human_bytes(16_428_249_907_u64 as f64).as_ref(),
            "15.3 GiB"
        );
    }

    #[test]
    fn tebibytes() {
        // Hacky, I know, but easier to write ;)
        let tebibyte: u64 = 2_u64.pow(40);
        assert_eq!(
            human_bytes(((tebibyte * 2) + (tebibyte / 2)) as f64).as_ref(),
            "2.5 TiB"
        );
    }
}
