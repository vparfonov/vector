use crc64fast_nvme::Digest;
/// Generates CRC-64/NVME checksums, using SIMD-accelerated
/// carryless-multiplication, from a file on disk.
use std::env;
use std::fs;
use std::process::ExitCode;

const CRC_NVME: crc::Algorithm<u64> = crc::Algorithm {
    width: 64,
    poly: 0xAD93D23594C93659,
    init: 0xFFFFFFFFFFFFFFFF,
    refin: true,
    refout: true,
    xorout: 0xFFFFFFFFFFFFFFFF,
    check: 0xae8b14860a799888,
    residue: 0x0000000000000000,
};

fn calculate_crc_64_simd_from_file(file: &str) -> u64 {
    let mut c = Digest::new();

    c.write(std::fs::read(file).unwrap().as_slice());

    c.sum64()
}

fn calculate_crc_64_validate_from_file(file: &str) -> u64 {
    let crc = crc::Crc::<u64>::new(&CRC_NVME);

    let mut digest = crc.digest();

    digest.update(std::fs::read(file).unwrap().as_slice());

    digest.finalize()
}

fn calculate_crc_64_simd_from_string(input: &str) -> u64 {
    let mut c = Digest::new();

    c.write(input.as_bytes());

    c.sum64()
}

fn calculate_crc_64_validate_from_string(input: &str) -> u64 {
    let crc = crc::Crc::<u64>::new(&CRC_NVME);

    let mut digest = crc.digest();

    digest.update(input.as_bytes());

    digest.finalize()
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Usage: crc_64_nvm_checksum [--inputType] [inputString] [--validate-slow]");
        println!("Example for a file: crc_64_nvm_checksum --file /path/to/file");
        println!("Example for a string: crc_64_nvm_checksum --string 123456789");
        println!("Optionally including '--validate-slow' in the argument list will skip SIMD calculation, typically just for testing.");

        return ExitCode::from(1);
    }

    let input_type = &args[1];

    if "--file" == input_type {
        let file = &args[2];

        if fs::metadata(file).is_err() {
            println!("Couldn't open file {}", file);

            return ExitCode::from(1);
        }

        if args.len() == 3 {
            println!("{}", calculate_crc_64_simd_from_file(file));

            return ExitCode::from(0);
        }

        if args.len() == 4 && "--validate-slow" == &args[3] {
            println!("{}", calculate_crc_64_validate_from_file(file));

            return ExitCode::from(0);
        }
    }

    if "--string" == input_type {
        let input = &args[2];

        if args.len() == 3 {
            println!("{}", calculate_crc_64_simd_from_string(input));

            return ExitCode::from(0);
        }

        if args.len() == 4 && "--validate-slow" == &args[3] {
            println!("{}", calculate_crc_64_validate_from_string(input));

            return ExitCode::from(0);
        }
    }

    println!("An error occurred, likely due to bad command-line arguments.");

    ExitCode::from(1)
}
