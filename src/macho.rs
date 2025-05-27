use nom::number::Endianness;

// magic numbers
// https://github.com/apple/darwin-xnu/blob/main/EXTERNAL_HEADERS/mach-o/loader.h#L65
// https://github.com/apple/darwin-xnu/blob/main/EXTERNAL_HEADERS/mach-o/loader.h#L84
pub const MAGIC: u32 = 0xfeed_face; // magic field - 32 bit
pub const CIGAM: u32 = 0xcdfa_edfe; // reverse -- little endian (need to swap bytes)

pub const MAGIC64: u32 = 0xfeed_facf; // magic field - 64 bit
pub const CIGAM64: u32 = 0xcffa_edfe; // reverse -- little endian (need to swap bytes)

// cpu arc types
// https://github.com/apple/darwin-xnu/blob/main/osfmk/mach/machine.h#L130
pub const CPU_ARCH_MASK: u32 = 0xff000000;
pub const CPU_ARCH_ABI64: u32 = 0x01000000; // 64 bit ABI
pub const CPU_ARCH_ABI64_32: u32 = 0x02000000;

// cpu types
// https://github.com/apple/darwin-xnu/blob/main/osfmk/mach/machine.h#L138
pub const CPU_TYPE_X86: u32 = 7;
pub const CPU_TYPE_X86_64: u32 = CPU_TYPE_X86 | CPU_ARCH_ABI64;

pub const CPU_TYPE_ARM: u32 = 12;
pub const CPU_TYPE_ARM64: u32 = CPU_TYPE_ARM | CPU_ARCH_ABI64;

#[derive(Debug)]
pub struct MachFile {
    // https://github.com/aidansteele/osx-abi-macho-file-format-reference
    pub header: MachHeader,
    pub commands: Vec<SegmentCommand>,
    pub segments: Vec<Segment>,
}

impl MachFile {
    pub fn parse(input: &[u8]) -> Result<MachFile, &str> {
        let commands = Vec::new();
        let segments = Vec::new();

        match MachHeader::parse(input) {
            Ok(header) => Ok(MachFile {
                header,
                commands,
                segments,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn str(&self, file: &str) -> String {
        format!("{}:\n{}", file, self.header.str())
    }
}

#[derive(Debug)]
pub struct MachHeader {
    // https://github.com/apple/darwin-xnu/blob/main/EXTERNAL_HEADERS/mach-o/loader.h#L72
    pub magic: u32,
    pub endianness: Endianness,
    pub cpu_type: u32,
    pub cpu_sub_type: u32,
    pub file_type: u32,
    pub ncmds: u32,        // number of load commands
    pub size_of_cmds: u32, // the size of all the load commands
    pub flag: u32,
}

fn parse(input: &[u8]) -> Result<u32, &str> {
    if input.len() != 4 {
        return Err("input should be of size 4");
    }
    match u32::from_str_radix(
        format!("{:x}{:x}{:x}{:x}", input[0], input[1], input[2], input[3]).as_str(),
        16,
    ) {
        Ok(v) => Ok(v),
        Err(e) => {
            eprintln!("format error {}", e);
            Err("error parsing input")
        }
    }
}

impl MachHeader {
    pub fn parse(input: &[u8]) -> Result<MachHeader, &str> {
        let magic_no = parse(&input[0..4]);
        let cpu_type = parse(&input[4..8])?;
        let cpu_sub_type = parse(&input[8..12])?;
        let file_type = parse(&input[12..16])?;
        let ncmds = parse(&input[16..20])?;
        let size_of_cmds = parse(&input[20..24])?;
        let flag = parse(&input[24..28])?;
        if let Ok(MAGIC) = magic_no {
            let magic = MAGIC;
            let endianness = Endianness::Big;
            Ok(MachHeader {
                magic,
                endianness,
                cpu_type,
                cpu_sub_type,
                file_type,
                ncmds,
                size_of_cmds,
                flag,
            })
        } else if let Ok(MAGIC64) = magic_no {
            let magic = MAGIC64;
            let endianness = Endianness::Big;
            Ok(MachHeader {
                magic,
                endianness,
                cpu_type,
                cpu_sub_type,
                file_type,
                ncmds,
                size_of_cmds,
                flag,
            })
        } else if let Ok(CIGAM) = magic_no {
            let magic = CIGAM;
            let endianness = Endianness::Little;
            Ok(MachHeader {
                magic,
                endianness,
                cpu_type,
                cpu_sub_type,
                file_type,
                ncmds,
                size_of_cmds,
                flag,
            })
        } else if let Ok(CIGAM64) = magic_no {
            let magic = CIGAM64;
            let endianness = Endianness::Little;
            Ok(MachHeader {
                magic,
                endianness,
                cpu_type,
                cpu_sub_type,
                file_type,
                ncmds,
                size_of_cmds,
                flag,
            })
        } else {
            Err("not a matching matcho header")
        }
    }

    pub fn str(&self) -> String {
        let mut buf: Vec<String> = Vec::new();
        buf.push("match header".to_string());

        let mut endian_str: &str = "";
        if self.endianness == Endianness::Little {
            endian_str = "little";
        } else if self.endianness == Endianness::Big {
            endian_str = "big";
        } else {
            endian_str = "native";
        }

        let endian = format!("endianness: {}", endian_str);
        buf.push(endian);

        let header = format!("magic: 0x{:x}", self.magic);
        buf.push(header);

        let cpu = format!(
            "cpu_type: 0x{:x}, sub_type: 0x{:x}",
            self.cpu_type, self.cpu_sub_type
        );
        buf.push(cpu);

        let other = format!(
            "file_type: 0x{:x}, ncmds: 0x{:x}, size_of_cmds: 0x{:x}, flag: 0x{:x}",
            self.file_type, self.ncmds, self.size_of_cmds, self.flag
        );
        buf.push(other);

        buf.join("\n")
    }
}

#[derive(Debug)]
pub struct SegmentCommand {
    // https://github.com/apple/darwin-xnu/blob/main/EXTERNAL_HEADERS/mach-o/loader.h#L355
}

#[derive(Debug)]
pub struct Segment {
    pub sections: Vec<Section>,
}

#[derive(Debug)]
pub struct Section {}

#[derive(Debug)]
pub struct MachoIdentifier {}

#[derive(Debug)]
pub struct MachoCommand {}
