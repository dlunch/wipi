// ELF header constants
const EI_CLASS: usize = 4;
const EI_DATA: usize = 5;
const EI_OSABI: usize = 7;
const EI_ABIVERSION: usize = 8;
const E_MACHINE_OFFSET: usize = 18;
const E_ENTRY_OFFSET: usize = 24;
const E_FLAGS_OFFSET: usize = 36;
const E_PHOFF_OFFSET: usize = 28;
const E_PHENTSIZE_OFFSET: usize = 42;
const E_PHNUM_OFFSET: usize = 44;

// ELF constants
const ELFCLASS32: u8 = 1;
const ELFDATA2LSB: u8 = 1;
const EM_ARM: u16 = 40;

pub fn fix_lgt_elf(source: &[u8]) -> anyhow::Result<Vec<u8>> {
    anyhow::ensure!(source.len() >= 52, "Input too small to be a valid ELF file");
    anyhow::ensure!(&source[0..4] == b"\x7fELF", "Not a valid ELF file");
    anyhow::ensure!(
        source[EI_CLASS] == ELFCLASS32,
        "Only 32-bit ELF files are supported"
    );
    anyhow::ensure!(
        source[EI_DATA] == ELFDATA2LSB,
        "Only little-endian ELF files are supported"
    );

    let machine_bytes = &source[E_MACHINE_OFFSET..E_MACHINE_OFFSET + 2];
    let machine = u16::from_le_bytes([machine_bytes[0], machine_bytes[1]]);
    anyhow::ensure!(
        machine == EM_ARM,
        "Only ARM architecture ELF files are supported (found machine type: {})",
        machine
    );

    let mut result = source.to_vec();

    result[EI_OSABI] = 97;
    result[EI_ABIVERSION] = 0;

    let flags: u32 = 0x206;
    result[E_FLAGS_OFFSET..E_FLAGS_OFFSET + 4].copy_from_slice(&flags.to_le_bytes());

    // Clear LSB of entry point to force ARM mode (not Thumb mode)
    let entry_bytes = &result[E_ENTRY_OFFSET..E_ENTRY_OFFSET + 4];
    let mut entry = u32::from_le_bytes([
        entry_bytes[0],
        entry_bytes[1],
        entry_bytes[2],
        entry_bytes[3],
    ]);
    entry &= !1; // Clear the LSB
    result[E_ENTRY_OFFSET..E_ENTRY_OFFSET + 4].copy_from_slice(&entry.to_le_bytes());

    // Remove program headers
    result[E_PHOFF_OFFSET..E_PHOFF_OFFSET + 4].copy_from_slice(&0u32.to_le_bytes());
    result[E_PHENTSIZE_OFFSET..E_PHENTSIZE_OFFSET + 2].copy_from_slice(&0u16.to_le_bytes());
    result[E_PHNUM_OFFSET..E_PHNUM_OFFSET + 2].copy_from_slice(&0u16.to_le_bytes());

    Ok(result)
}

#[cfg(test)]
mod tests {
    use object::{
        Architecture, BinaryFormat, Endianness, File, Object as ObjectTrait,
        write::{Object, Symbol, SymbolSection},
    };

    use super::*;

    fn create_test_elf() -> Vec<u8> {
        let mut obj = Object::new(BinaryFormat::Elf, Architecture::Arm, Endianness::Little);

        let text_section =
            obj.add_section(vec![], b".text".to_vec(), object::write::SectionKind::Text);
        obj.append_section_data(text_section, &[0x00, 0x00, 0xa0, 0xe1], 4);

        let data_section =
            obj.add_section(vec![], b".data".to_vec(), object::write::SectionKind::Data);
        obj.append_section_data(data_section, &[0x42, 0x00, 0x00, 0x00], 4);

        obj.add_symbol(Symbol {
            name: b"_start".to_vec(),
            value: 0,
            size: 4,
            kind: object::write::SymbolKind::Text,
            scope: object::write::SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(text_section),
            flags: object::write::SymbolFlags::None,
        });

        let mut elf_data = obj.write().expect("Failed to write ELF");

        // Simulate program headers in a linked executable
        if elf_data.len() >= 52 {
            let phoff: u32 = 52;
            elf_data[E_PHOFF_OFFSET..E_PHOFF_OFFSET + 4].copy_from_slice(&phoff.to_le_bytes());

            let phentsize: u16 = 32;
            elf_data[E_PHENTSIZE_OFFSET..E_PHENTSIZE_OFFSET + 2]
                .copy_from_slice(&phentsize.to_le_bytes());

            let phnum: u16 = 1;
            elf_data[E_PHNUM_OFFSET..E_PHNUM_OFFSET + 2].copy_from_slice(&phnum.to_le_bytes());

            // Set entry point with LSB set (Thumb mode) to test that it gets cleared
            let entry_with_thumb: u32 = 0x1001; // Example address with LSB set
            elf_data[E_ENTRY_OFFSET..E_ENTRY_OFFSET + 4]
                .copy_from_slice(&entry_with_thumb.to_le_bytes());
        }

        elf_data
    }

    #[test]
    fn test_fix_lgt_elf_comprehensive() {
        let original_elf = create_test_elf();

        // Verify original ELF has program headers
        let original_phoff_bytes = &original_elf[E_PHOFF_OFFSET..E_PHOFF_OFFSET + 4];
        let original_phoff = u32::from_le_bytes([
            original_phoff_bytes[0],
            original_phoff_bytes[1],
            original_phoff_bytes[2],
            original_phoff_bytes[3],
        ]);
        assert_ne!(
            original_phoff, 0,
            "Original ELF should have program header offset set"
        );

        let original_phnum_bytes = &original_elf[E_PHNUM_OFFSET..E_PHNUM_OFFSET + 2];
        let original_phnum = u16::from_le_bytes([original_phnum_bytes[0], original_phnum_bytes[1]]);
        assert_ne!(
            original_phnum, 0,
            "Original ELF should have program header count > 0"
        );

        let original_phentsize_bytes = &original_elf[E_PHENTSIZE_OFFSET..E_PHENTSIZE_OFFSET + 2];
        let original_phentsize =
            u16::from_le_bytes([original_phentsize_bytes[0], original_phentsize_bytes[1]]);
        assert_ne!(
            original_phentsize, 0,
            "Original ELF should have program header entry size > 0"
        );

        let fixed_elf = fix_lgt_elf(&original_elf).expect("Failed to fix ELF");

        // Check transformations
        assert_eq!(fixed_elf[EI_OSABI], 97, "OSABI should be set to 97");
        assert_eq!(
            fixed_elf[EI_ABIVERSION], 0,
            "ABI version should be set to 0"
        );

        let flags_bytes = &fixed_elf[E_FLAGS_OFFSET..E_FLAGS_OFFSET + 4];
        let flags = u32::from_le_bytes([
            flags_bytes[0],
            flags_bytes[1],
            flags_bytes[2],
            flags_bytes[3],
        ]);
        assert_eq!(flags, 0x206, "Flags should be set to 0x206");

        // Check that entry point LSB is cleared (ARM mode, not Thumb mode)
        let entry_bytes = &fixed_elf[E_ENTRY_OFFSET..E_ENTRY_OFFSET + 4];
        let entry = u32::from_le_bytes([
            entry_bytes[0],
            entry_bytes[1],
            entry_bytes[2],
            entry_bytes[3],
        ]);
        assert_eq!(
            entry & 1,
            0,
            "Entry point LSB should be cleared to force ARM mode"
        );
        assert_eq!(
            entry, 0x1000,
            "Entry point should be 0x1000 (0x1001 with LSB cleared)"
        );

        let phoff_bytes = &fixed_elf[E_PHOFF_OFFSET..E_PHOFF_OFFSET + 4];
        let phoff = u32::from_le_bytes([
            phoff_bytes[0],
            phoff_bytes[1],
            phoff_bytes[2],
            phoff_bytes[3],
        ]);
        assert_eq!(phoff, 0, "Program header offset should be 0");

        let phentsize_bytes = &fixed_elf[E_PHENTSIZE_OFFSET..E_PHENTSIZE_OFFSET + 2];
        let phentsize = u16::from_le_bytes([phentsize_bytes[0], phentsize_bytes[1]]);
        assert_eq!(phentsize, 0, "Program header entry size should be 0");

        let phnum_bytes = &fixed_elf[E_PHNUM_OFFSET..E_PHNUM_OFFSET + 2];
        let phnum = u16::from_le_bytes([phnum_bytes[0], phnum_bytes[1]]);
        assert_eq!(phnum, 0, "Program header count should be 0");

        // Verify sections are preserved
        let fixed_file = File::parse(&*fixed_elf).expect("Failed to parse fixed ELF");

        assert!(
            fixed_file.section_by_name(".text").is_some(),
            "Fixed ELF should have .text section"
        );
        assert!(
            fixed_file.section_by_name(".data").is_some(),
            "Fixed ELF should have .data section"
        );
        assert_eq!(
            fixed_file.architecture(),
            Architecture::Arm,
            "Architecture should be ARM"
        );

        // Test error cases
        let small_input = vec![0u8; 10];
        let result = fix_lgt_elf(&small_input);
        assert!(result.is_err(), "Should fail with too small input");

        let mut invalid_elf = vec![0u8; 60];
        invalid_elf[0..4].copy_from_slice(b"NOTF");
        let result = fix_lgt_elf(&invalid_elf);
        assert!(result.is_err(), "Should fail with invalid ELF magic");

        let mut elf64 = vec![0u8; 60];
        elf64[0..4].copy_from_slice(b"\x7fELF");
        elf64[EI_CLASS] = 2; // ELFCLASS64
        elf64[EI_DATA] = ELFDATA2LSB;
        elf64[E_MACHINE_OFFSET..E_MACHINE_OFFSET + 2].copy_from_slice(&EM_ARM.to_le_bytes());
        let result = fix_lgt_elf(&elf64);
        assert!(result.is_err(), "Should fail with 64-bit ELF");

        let mut elf_be = vec![0u8; 60];
        elf_be[0..4].copy_from_slice(b"\x7fELF");
        elf_be[EI_CLASS] = ELFCLASS32;
        elf_be[EI_DATA] = 2; // ELFDATA2MSB
        elf_be[E_MACHINE_OFFSET..E_MACHINE_OFFSET + 2].copy_from_slice(&EM_ARM.to_le_bytes());
        let result = fix_lgt_elf(&elf_be);
        assert!(result.is_err(), "Should fail with big-endian ELF");

        let mut elf_x86 = vec![0u8; 60];
        elf_x86[0..4].copy_from_slice(b"\x7fELF");
        elf_x86[EI_CLASS] = ELFCLASS32;
        elf_x86[EI_DATA] = ELFDATA2LSB;
        elf_x86[E_MACHINE_OFFSET..E_MACHINE_OFFSET + 2].copy_from_slice(&3u16.to_le_bytes()); // EM_386
        let result = fix_lgt_elf(&elf_x86);
        assert!(result.is_err(), "Should fail with non-ARM architecture");
    }
}
