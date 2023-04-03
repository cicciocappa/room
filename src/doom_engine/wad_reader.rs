use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Read;
use std::io::SeekFrom;

use winit::platform::unix::x11::ffi::SubstructureNotifyMask;

use super::data_types::*;

pub struct WadReader {
    pub directory: Vec<Directory>,
    header: Header,
    file: BufReader<File>,
}

struct Header {
    wad_type: [u8; 4],
    lump_count: u32,
    init_offset: u32,
}

impl WadReader {
    pub fn new(path: &str) -> Self {
        let f = File::open(path).unwrap();
        let mut file = BufReader::new(f);
        let header = WadReader::read_header(&mut file);
        let directory = WadReader::read_directory(&mut file, &header);
        WadReader {
            file,
            header,
            directory,
        }
    }

    fn read_header(file: &mut BufReader<File>) -> Header {
        let mut buffer: [u8; 4] = [0; 4];
        file.read(&mut buffer).unwrap();
        let mut b: [u8; 4] = [0; 4];
        file.read(&mut b).unwrap();
        let lump_count = get_u32(&b);
        file.read(&mut b).unwrap();
        let init_offset = get_u32(&b);
        Header {
            wad_type: buffer,
            lump_count,
            init_offset,
        }
    }

    fn read_directory(file: &mut BufReader<File>, header: &Header) -> Vec<Directory> {
        println!(
            "{:?} {} {}",
            header.wad_type, header.lump_count, header.init_offset
        );
        let mut directory = Vec::new();
        file.seek(SeekFrom::Start(header.init_offset as u64))
            .unwrap();

        for _ in 0..header.lump_count as usize {
            let mut b: [u8; 4] = [0; 4];
            file.read(&mut b).unwrap();
            let lump_offset = get_u32(&b);
            file.read(&mut b).unwrap();
            let lump_size = get_u32(&b);
            let mut lump_name: [u8; 8] = [0; 8];
            file.read(&mut lump_name).unwrap();
            directory.push(Directory {
                lump_offset,
                lump_size,
                lump_name,
            });
        }
        directory
    }

    pub fn read_vertex(&mut self, lump_index: usize) -> Vec<Vertex> {
        let count = self.directory[lump_index].lump_size / 4;
        let offset = self.directory[lump_index].lump_offset as u64;
        let mut b: [u8; 4] = [0; 4];
        self.file.seek(SeekFrom::Start(offset)).unwrap();
        let mut v = Vec::new();
        for _ in 0..count {
            self.file.read(&mut b).unwrap();

            //let (x, y) = get_i16_tuple(&b);
            //println!("{x} {y}");
            let x = get_i16(&b[0..2]);
            let y = get_i16(&b[2..4]);
            v.push(Vertex { x, y });
        }

        v
    }
    pub fn read_linedef(&mut self, lump_index: usize) -> Vec<Linedef> {
        let count = self.directory[lump_index].lump_size / 14;
        let offset = self.directory[lump_index].lump_offset as u64;
        let mut b: [u8; 14] = [0; 14];

        self.file.seek(SeekFrom::Start(offset)).unwrap();
        let mut v = Vec::new();
        for _ in 0..count {
            self.file.read(&mut b).unwrap();

            let start_vertex_id = get_u16(&b[0..2]);
            let end_vertex_id = get_u16(&b[2..4]);
            let flags = get_u16(&b[4..6]);
            let line_type = get_u16(&b[6..8]);
            let sector_tag = get_u16(&b[8..10]);
            let front_sidedef_id = get_u16(&b[10..12]);
            let back_sidedef_id = get_u16(&b[12..14]);
            v.push(Linedef {
                start_vertex_id,
                end_vertex_id,
                flags,
                line_type,
                sector_tag,
                front_sidedef_id,
                back_sidedef_id,
            });
        }
        v
    }

    pub fn read_node(&mut self, lump_index: usize) -> Vec<Node> {
        let count = self.directory[lump_index].lump_size / 14;
        let offset = self.directory[lump_index].lump_offset as u64;
        let mut b: [u8; 28] = [0; 28];

        self.file.seek(SeekFrom::Start(offset)).unwrap();
        let mut v = Vec::new();
        for _ in 0..count {
            self.file.read(&mut b).unwrap();

            let x_partition = get_i16(&b[0..2]);
            let y_partition = get_i16(&b[2..4]);
            let dx_partition = get_i16(&b[4..6]);
            let dy_partition = get_i16(&b[6..8]);
            let bbox_front = BBox {
                top: get_i16(&b[8..10]),
                bottom: get_i16(&b[10..12]),
                left: get_i16(&b[12..14]),
                right: get_i16(&b[14..16]),
            };
            let bbox_back = BBox {
                top: get_i16(&b[16..18]),
                bottom: get_i16(&b[18..20]),
                left: get_i16(&b[20..22]),
                right: get_i16(&b[22..24]),
            };

            let front_child_id = get_u16(&b[24..26]);
            let back_child_id = get_u16(&b[26..28]);

            v.push(Node {
                x_partition,
                y_partition,
                dx_partition,
                dy_partition,
                bbox_front,
                bbox_back,
                front_child_id,
                back_child_id,
            });
        }
        v
    }

    pub fn read_subsector(&mut self, lump_index: usize) -> Vec<SubSector> {
        let count = self.directory[lump_index].lump_size / 4;
        let offset = self.directory[lump_index].lump_offset as u64;
        let mut b: [u8; 4] = [0; 4];
        self.file.seek(SeekFrom::Start(offset)).unwrap();
        let mut v = Vec::new();
        for _ in 0..count {
            self.file.read(&mut b).unwrap();

            //let (x, y) = get_i16_tuple(&b);
            //println!("{x} {y}");
            let seg_count = get_u16(&b[0..2]);
            let first_seg_id = get_u16(&b[2..4]);
            v.push(SubSector {
                seg_count,
                first_seg_id,
            });
        }

        v
    }
    pub fn read_segment(&mut self, lump_index: usize) -> Vec<Seg> {
        let count = self.directory[lump_index].lump_size / 12;
        let offset = self.directory[lump_index].lump_offset as u64;
        let mut b: [u8; 12] = [0; 12];

        self.file.seek(SeekFrom::Start(offset)).unwrap();
        let mut v = Vec::new();
        for _ in 0..count {
            self.file.read(&mut b).unwrap();

            let start_vertex_id = get_u16(&b[0..2]);
            let end_vertex_id = get_u16(&b[2..4]);
            let angle = get_i16(&b[4..6]);
            let linedef_id = get_u16(&b[6..8]);
            let direction = get_u16(&b[8..10]);
            let offset = get_i16(&b[10..12]);
            v.push(Seg {
                start_vertex_id,
                end_vertex_id,
                angle,
                linedef_id,
                direction,
                offset,
              
            });
        }
        v
    }

    pub fn read_thing(&mut self, lump_index: usize) -> Vec<Thing> {
        let count = self.directory[lump_index].lump_size / 10;
        let offset = self.directory[lump_index].lump_offset as u64;
        let mut b: [u8; 10] = [0; 10];

        self.file.seek(SeekFrom::Start(offset)).unwrap();
        let mut v = Vec::new();
        for _ in 0..count {
            self.file.read(&mut b).unwrap();
            let pos = (get_i16(&b[0..2]),get_i16(&b[2..4]));
            let angle = get_i16(&b[4..6]);
            let ttype = get_u16(&b[6..8]);
            let flags = get_u16(&b[8..10]);
            v.push(Thing {
                pos,
                angle,
                ttype,
                flags,
                            
            });
        }
        v
    }
}

fn get_u32(b: &[u8; 4]) -> u32 {
    b[0] as u32 + b[1] as u32 * 256 + b[2] as u32 * 256 * 256 + b[3] as u32 * 256 * 256 * 256
}

fn get_u16(b: &[u8]) -> u16 {
    b[0] as u16 + 256 * b[1] as u16
}

fn get_i16(b: &[u8]) -> i16 {
    let n = b[0] as i32 + b[1] as i32 * 256;
    if n < 32768 {
        n as i16
    } else {
        (n - 65536) as i16
    }
}

pub struct Directory {
    pub lump_offset: u32,
    pub lump_size: u32,
    pub lump_name: [u8; 8],
}
