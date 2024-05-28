use std::{
    fs::File,
    io::{self, BufReader, Read, Seek},
    mem::{size_of, swap},
    path::Path,
};

use byteorder::{LittleEndian, ReadBytesExt};

use serde::{Deserialize, Serialize};

pub enum LanguageType {
    English,
    Korean,
    French,
    German,
    Spanish,
    Chinese,
}
pub enum FourCC {
    // Offset 0
    FccAtex = 0x58455441,
    FccAttx = 0x58545441,
    FccAtec = 0x43455441,
    FccAtep = 0x50455441,
    FccAteu = 0x55455441,
    FccAtet = 0x54455441,
    Fcc3dcx = 0x58434433,
    FccDxt = 0x00545844,
    FccDds = 0x20534444,
    FccStrs = 0x73727473,
    FccAsnd = 0x646e7361,
    FccRiff = 0x46464952, // resource interchange file format
    FccTtf = 0x00000100, // files with this signature seems to be ttf but it is Embedded OpenType fonts with ttf header
    FccOggS = 0x5367674f,
    FccArap = 0x50415241, // relate to temp folder name of CoherentUI
    FccCtex = 0x58455443, // DXT5 compressed texture, custom format.

    // Texture codec
    FccDxt1 = 0x31545844,
    FccDxt2 = 0x32545844,
    FccDxt3 = 0x33545844,
    FccDxt4 = 0x34545844,
    FccDxt5 = 0x35545844,
    FccDxtn = 0x4e545844,
    FccDxtl = 0x4c545844,
    FccDxta = 0x41545844,
    FccR32f = 0x00000072,

    // RIFF FourCC
    FccWebp = 0x50424557,

    // PF FourCC
    FccArmf = 0x464d5241,
    FccAsndPf = 0x444e5341,
    FccAbnk = 0x4b4e4241,
    FccAbix = 0x58494241,
    FccAmsp = 0x50534d41,
    FccCdhs = 0x53484443,
    FccCinp = 0x504e4943,
    FccCntc = 0x63746e63,
    FccModl = 0x4c444f4d,
    FccGeom = 0x4d4f4547,
    FccDeps = 0x53504544,
    FccEula = 0x616c7565,
    FccHvkC = 0x436b7668,
    FccLocl = 0x6c636f6c, // store config, e-mail and hashed password
    FccMapc = 0x6370616d,
    FccMpsd = 0x6473706d,
    FccPimg = 0x474d4950,
    FccAmat = 0x54414d41,
    FccAnic = 0x63696e61,
    FccEmoc = 0x636f6d65,
    FccPrlt = 0x746c7270,
    FccCmpc = 0x63706d63,
    FccTxtm = 0x6d747874,
    FccTxtV = 0x56747874,
    FccTxtv = 0x76747874,
    FccPng = 0x474e5089,
    FccCmaC = 0x43616d63,
    FccMMet = 0x74654d6d,
    FccAfnt = 0x544e4641,

    // Not quite FourCC
    FccMz = 0x5a4d, // Executable or Dynamic Link Library
    FccPf = 0x4650,
    FccMp3 = 0xfbff, // MPEG-1 Layer 3
    FccJpeg = 0xffd8ff,
    FccId3 = 0x334449,   // MP3 with an ID3v2 container
    FccBink2 = 0x32424b, // Bink 2 video
    FccUtf8 = 0xbfbbef,  // UTF-8 encoding
}

pub enum AnetFileType {
    AnftUnknown, //< Unknown format.

    // Texture types
    AnftTextureStart, //< Values in between this and ANFT_TextureEnd are texture types.
    AnftAtex,         //< ATEX texture, generic use.
    AnftAttx,         //< ATTX texture, used for terrain (in GW1).
    AnftAtec,         //< ATEC texture, unknown use.
    AnftAtep,         //< ATEP texture, used for maps.
    AnftAteu,         //< ATEU texture, used for UI.
    AnftAtet,         //< ATET texture, unknown use.
    AnftCtex,         //< CTEX texture, unknown use.
    AnftDds,          //< DDS texture, not an ANet specific format.
    AnftJpeg,         //< JPEG Image, not an ANet specific format.
    AnftWebp,         //< WebP Image, not an ANet specific format.
    AnftPng,          //< PNG Image, not an ANet specific format.
    AnftTextureEnd,   //< Values in between this and ANFT_TextureStart are texture types.

    // Sound
    AnftSoundStart, //< Values in between this and ANFT_SoundEnd are sound types.
    AnftSound,      //< Sound file of unknown type.
    AnftAsndMp3,    //< asnd MP3 format file.
    AnftAsndOgg,    //< asnd OGG format file.
    AnftPackedMp3,  //< PF packed MP3 file.
    AnftPackedOgg,  //< PF packed Ogg file.
    AnftOgg,        //< Uncompressed Ogg file.
    AnftMp3,        //< Uncompressed MP3 file.
    AnftSoundEnd,   //< Values in between this and ANFT_SoundStart are sound types.

    // RIFF
    AnftRiff, //< Resource Interchange File Format container.

    // PF
    AnftPf,                        //< PF file of unknown type.
    AnftManifest,                  //< Manifest file.
    AnftTextPackManifest,          //< TextPack Manifest file.
    AnftTextPackVariant,           //< TextPack Variant file.
    AnftTextPackVoices,            //< TextPack Voices file.
    AnftBank,                      //< Soundbank file, contains other files.
    AnftBankIndex,                 //< Soundbank files index
    AnftModel,                     //< Model file.
    AnftModelCollisionManifest,    //< Model collision manifest file.
    AnftDependencyTable,           //< Dependency table.
    AnftEula,                      //< EULA file.
    AnftGameContent,               //< Game content file.
    AnftGameContentPortalManifest, //< Game content portal Manifest file.
    AnftMapCollision,              //< Map collision properties.
    AnftMapParam,                  //< Map file.
    AnftMapShadow,                 //< Map shadow file.
    AnftMapMetadata,               //< Map metadata file.
    AnftPagedImageTable,           //< Paged Image Table file.
    AnftMaterial,                  //< Compiled DirectX 9 shader.
    AnftComposite,                 //< Composite data.
    AnftCinematic,                 //< Cinematic data.
    AnftAnimSequences,             //< Animation Sequences data.
    AnftEmoteAnimation,            //< Emote animation data.
    AnftAudioScript,               //< Audio script file.
    AnftShaderCache,               //< Shader cache file.
    AnftConfig,                    //< Configuration file.

    // Binary
    AnftBinary, //< Binary file of unknown type.
    AnftDll,    //< DLL file.
    AnftExe,    //< Executable file.

    // Misc
    AnftStringFile,     //< Strings file.
    AnftFontFile,       //< Font file.
    AnftBitmapFontFile, //< Bitmap font file.
    AnftBink2video,     //< Bink2 video file.
    AnftArap,
    AnftUtf8, //< UTF-8 encoding.
    AnftText, //< Text file.
}

pub enum AnetCompressionFlags {
    AncfUncompressed = 0, //< File is uncompressed.
    AncfCompressed = 8,   //< File is compressed.
}
pub enum AnetMftEntryFlags {
    AnmefNone = 0,  //< No flags set.
    AnmefInUse = 1, //< Entry is in use.
}

pub enum AnetFlexibleVertexFormat {
    AnfvfPosition = 0x00000001, //< 12 bytes. Position as three 32-bit floats in the order x, y, z.
    AnfvfWeights = 0x00000002,  //< 4 bytes. Contains bone weights.
    AnfvfGroup = 0x00000004,    //< 4 bytes. Related to bone weights.
    AnfvfNormal = 0x00000008,   //< 12 bytes. Normal as three 32-bit floats in the order x, y, z.
    AnfvfColor = 0x00000010,    //< 4 bytes. Vertex color.
    AnfvfTangent = 0x00000020,  //< 12 bytes. Tangent as three 32-bit floats in the order x, y, z.
    AnfvfBitangent = 0x00000040, //< 12 bytes. Bitangent as three 32-bit floats in the order x, y, z.
    AnfvfTangentFrame = 0x00000080, //< 12 bytes.
    AnfvfUv32mask = 0x0000ff00, //< 8 bytes for each set bit. Contains UV-coords as two 32-bit floats in the order u, v.
    AnfvfUv16mask = 0x00ff0000, //< 4 bytes for each set bit. Contains UV-coords as two 16-bit floats in the order u, v.
    AnfvfUnknown1 = 0x01000000, //< 48 bytes. Unknown data.
    AnfvfUnknown2 = 0x02000000, //< 4 bytes. Unknown data.
    AnfvfUnknown3 = 0x04000000, //< 4 bytes. Unknown data.
    AnfvfUnknown4 = 0x08000000, //< 16 bytes. Unknown data.
    AnfvfPositionCompressed = 0x10000000, //< 6 bytes. Position as three 16-bit floats in the order x, y, z.
    AnfvfUnknown5 = 0x20000000,           //< 12 bytes. Unknown data.
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AnetDatHeader {
    pub version: u8,
    pub identifier: Vec<u8>,
    pub header_size: u32,
    pub unknown_field: u32,
    pub chunk_size: u32,
    pub crc: u32,
    pub unknown_field_2: u32,
    pub mft_offset: u64,
    pub mft_size: u32,
    pub flags: u32,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AnetMftHeader {
    pub identifier: Vec<u8>,
    pub unknown_field: u64,
    pub num_entries: u32,
    pub unknown_field_2: u64,
}
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AnetMftEntry {
    pub offset: u64,
    pub size: u32,
    pub compression_flag: u16,
    pub entry_flag: u16,
    pub counter: u32,
    pub crc: u32,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AnetIdEntry {
    pub file_id: u32,
    pub base_id: u32,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AnetFileReference {
    pub parts: Vec<u8>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AnetAtexHeader {
    pub identifier: Vec<u8>,
    pub identifier_integer: u32,
    pub format: u8,
    pub format_integer: u32,
    pub width: u16,
    pub height: u16,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AnetPfHeader {
    pub identifier: Vec<u8>,
    pub unknown_field: u16,
    pub unknown_field_2: u16,
    pub file_type: Vec<u8>,
    pub file_type_integer: u32,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AnetPfChunkHeader {
    pub chunk_type: Vec<u8>,
    pub chunk_type_integer: u32,
    pub chunk_data_size: u32,
    pub chunk_version: u16,
    pub chunk_header_size: u16,
    pub offset_table_offset: u32,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AnetModelMaterialPermutations {
    pub token: u64,
    pub material_count: u32,
    pub material_offset: i32,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AnetModelMaterialData {
    pub token: u64,
    pub material_id: u32,
    pub material_file_offset: i32,
    pub material_flags: u32,
    pub sort_order: u32,
    pub texture_count: u32,
    pub texture_offset: i32,
    pub constants_count: u32,
    pub constants_offset: i32,
    pub mat_const_links_count: u32,
    pub mat_const_links_offset: u32,
    pub uv_trans_links_count: u32,
    pub uv_trans_links_offset: u32,
    pub tex_transforms4_count: u32,
    pub tex_transforms4_offset: u32,
    pub tex_coord_count: u8,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AnetModelTextureReference {
    pub offset_to_file_reference: i32,
    pub texture_flags: u32,
    pub token: u64,
    pub blit_id: u64,
    pub uv_anim_id: u32,
    pub uv_ps_input_index: u8,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AnetArchive {
    pub dat_header: AnetDatHeader,
    pub mft_header: AnetMftHeader,
    pub mft_data: Vec<AnetMftEntry>,
    pub mft_index_data: Vec<AnetIdEntry>,
}
const DAT_MAGIC_NUMBER: usize = 3;
const MFT_MAGIC_NUMBER: usize = 4;
const MFT_ENTRY_INDEX_NUM: usize = 1;

impl AnetArchive {
    pub fn load_from_file<P: AsRef<Path>>(file_path: P) -> io::Result<Self> {
        // Check if the file extension is '.dat'
        let file_path_str = file_path.as_ref().to_str().unwrap();
        if !file_path_str.to_lowercase().ends_with(".dat") {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid file extension. Expected '.dat'.",
            ));
        }

        // Open the file and create a buffered reader.
        let file = std::fs::File::open(file_path)?;
        let mut buf_reader = BufReader::new(file);

        // Delegate to load_from_reader for further processing.
        Self::load_from_reader(&mut buf_reader)
    }

    fn load_from_reader<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let mut gw2_dat_data = AnetArchive::default();
        gw2_dat_data.read_header(reader)?;
        gw2_dat_data.read_mft_header(reader)?;
        gw2_dat_data.read_mft_data(reader)?;
        gw2_dat_data.read_mft_index(reader)?;
        Ok(gw2_dat_data)
    }

    fn read_header<R: Read + Seek>(&mut self, file: &mut R) -> io::Result<&mut Self> {
        self.dat_header.version = file.read_u8()?;
        let mut magic = [0; DAT_MAGIC_NUMBER];
        file.read_exact(&mut magic)?;
        self.dat_header.identifier = Vec::from(magic);
        self.dat_header.header_size = file.read_u32::<LittleEndian>()?;
        self.dat_header.unknown_field = file.read_u32::<LittleEndian>()?;
        self.dat_header.chunk_size = file.read_u32::<LittleEndian>()?;
        self.dat_header.crc = file.read_u32::<LittleEndian>()?;
        self.dat_header.unknown_field_2 = file.read_u32::<LittleEndian>()?;
        self.dat_header.mft_offset = file.read_u64::<LittleEndian>()?;
        self.dat_header.mft_size = file.read_u32::<LittleEndian>()?;
        self.dat_header.flags = file.read_u32::<LittleEndian>()?;
        let check_magic = [0x41, 0x4e, 0x1a];
        if self.dat_header.identifier != check_magic {
            panic!("Not an GW2 DAT file: invalid header magic");
        }
        Ok(self)
    }
    fn read_mft_header<R: Read + Seek>(&mut self, file: &mut R) -> io::Result<&mut Self> {
        file.seek(std::io::SeekFrom::Start(self.dat_header.mft_offset as u64))?;

        let mut magic = [0; MFT_MAGIC_NUMBER];
        file.read_exact(&mut magic)?;
        self.mft_header.identifier = Vec::from(magic);
        self.mft_header.unknown_field = file.read_u64::<LittleEndian>()?;
        self.mft_header.num_entries = file.read_u32::<LittleEndian>()?;
        self.mft_header.unknown_field_2 = file.read_u64::<LittleEndian>()?;
        Ok(self)
    }

    fn read_mft_data<R: Read + Seek>(&mut self, file: &mut R) -> io::Result<&mut Self> {
        for _ in 0..self.mft_header.num_entries {
            let mut mft_data = AnetMftEntry::default();
            mft_data.offset = file.read_u64::<LittleEndian>()?;
            mft_data.size = file.read_u32::<LittleEndian>()?;
            mft_data.compression_flag = file.read_u16::<LittleEndian>()?;
            mft_data.entry_flag = file.read_u16::<LittleEndian>()?;
            mft_data.counter = file.read_u32::<LittleEndian>()?;
            mft_data.crc = file.read_u32::<LittleEndian>()?;
            self.mft_data.push(mft_data);
        }
        Ok(self)
    }
    fn read_mft_index<R: Read + Seek>(&mut self, file: &mut R) -> io::Result<&mut Self> {
        let num_file_id_entries = self.mft_data.get(MFT_ENTRY_INDEX_NUM).unwrap().size as usize
            / size_of::<AnetIdEntry>() as usize;
        file.seek(std::io::SeekFrom::Start(
            self.mft_data.get(MFT_ENTRY_INDEX_NUM).unwrap().offset as u64,
        ))?;
        let mut file_id_table: Vec<AnetIdEntry> = Vec::default();
        for _ in 0..num_file_id_entries {
            file_id_table.push(AnetIdEntry {
                file_id: file.read_u32::<LittleEndian>()?,
                base_id: file.read_u32::<LittleEndian>()?,
            });
        }

        for _ in 0..self.mft_data.len() {
            self.mft_index_data.push(AnetIdEntry {
                file_id: 0,
                base_id: 0,
            });
        }

        for i in 0..num_file_id_entries {
            let entry_index = file_id_table.get(i).unwrap().base_id as usize;
            let entry = &mut self.mft_index_data[entry_index];
            if entry.base_id == 0 {
                entry.base_id = file_id_table.get(i).unwrap().file_id;
            } else if entry.file_id == 0 {
                entry.file_id = file_id_table.get(i).unwrap().file_id;
            }

            if entry.base_id > 0 && entry.file_id > 0 {
                if entry.base_id > entry.file_id {
                    swap(&mut entry.base_id, &mut entry.file_id);
                }
            }
        }

        Ok(self)
    }

    pub fn get_mft_data<P: AsRef<Path>>(
        &mut self,
        file_path: P,
        index: usize,
    ) -> io::Result<Vec<u8>> {
        // Check if the file extension is '.dat'
        let file_path_str = file_path.as_ref().to_str().unwrap();
        if !file_path_str.to_lowercase().ends_with(".dat") {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid file extension. Expected '.dat'.",
            ));
        }

        // Open the file and create a buffered reader.
        let file = std::fs::File::open(file_path)?;
        let mut buf_reader = BufReader::new(file);

        let mft_table = &self.mft_data[index];

        // Call mft_read_data to read the compressed data
        let data = Self::mft_read_data(&mut buf_reader, mft_table.offset, mft_table.size);
        Ok(data)
    }
    fn mft_read_data(file: &mut BufReader<File>, offset: u64, length: u32) -> Vec<u8> {
        file.seek(std::io::SeekFrom::Start(offset as u64)).unwrap();
        let mut data = vec![0; length as usize];
        file.read_exact(&mut data).unwrap();
        data
    }
}
