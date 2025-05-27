#[derive(Debug)]
pub struct ElfFile {
    pub header:ElfHeader,
}

#[derive(Debug)]
pub struct ElfHeader {
    // https://github.com/torvalds/linux/blob/master/include/uapi/linux/elf.h#L234
    pub ident:ElfIdentifier, // magic number and other info
}

#[derive(Debug)]
pub struct ElfIdentifier {
    
}


