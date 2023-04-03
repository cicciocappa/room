use super::data_types::*;
use super::wad_reader::WadReader;

const THINGS: usize = 1;
const LINEDEFS: usize = 2;
const SIDEDEFS: usize = 3;
const VERTEXES: usize = 4;
const SEGS: usize = 5;
const SSECTORS: usize = 6;
const NODES: usize = 7;
const SECTORS: usize = 8;
const REJECT: usize = 9;
const BLOCKMAP: usize = 10;

pub struct WadData {
    reader: WadReader,
    map_index: usize,
    pub vertexes: Vec<Vertex>,
    pub linedefs: Vec<Linedef>,
    pub nodes: Vec<Node>,
    pub sub_sectors: Vec<SubSector>,
    pub segments: Vec<Seg>,
    pub things: Vec<Thing>,
}

impl WadData {
    pub fn new(path: &str, map_name: &str) -> Self {
        let mut reader = WadReader::new(path);
        let map_index = reader
            .directory
            .iter()
            .position(|d| std::str::from_utf8(&d.lump_name).unwrap() == map_name)
            .unwrap();
        
        let vertexes = reader.read_vertex(map_index + VERTEXES);
        let linedefs = reader.read_linedef(map_index + LINEDEFS);
        let nodes=reader.read_node(map_index+NODES);
        let sub_sectors=reader.read_subsector(map_index+SSECTORS);
        let segments = reader.read_segment(map_index+SEGS);
        let things = reader.read_thing(map_index+THINGS);

        
         
        //let map_index = 0;
        WadData {
            reader,
            map_index,
           
            vertexes,
            linedefs,
            nodes,
            sub_sectors,
            segments,
            things,
        }
    }
}
