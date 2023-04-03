pub struct Vertex {
    pub x: i16,
    pub y: i16,
}

pub struct Linedef {
    pub start_vertex_id: u16,
    pub end_vertex_id: u16,
    pub flags: u16,
    pub line_type: u16,
    pub sector_tag: u16,
    pub front_sidedef_id: u16,
    pub back_sidedef_id: u16,
}

pub struct Thing {
    pub pos: (i16,i16),
    pub angle: i16,
    pub ttype: u16,
    pub flags: u16,
}

pub struct Seg {
    pub start_vertex_id: u16,
    pub end_vertex_id: u16,
    pub angle: i16,
    pub linedef_id: u16,
    pub direction: u16,
    pub offset: i16,
}

pub struct SubSector {
    pub seg_count: u16,
    pub first_seg_id: u16,
}

pub struct Node {
   
    pub x_partition: i16,
    pub y_partition: i16,
    pub dx_partition: i16,
    pub dy_partition: i16,
    pub bbox_front: BBox,
    pub bbox_back: BBox,
    pub front_child_id: u16,
    pub back_child_id: u16,
}

 pub struct BBox {
    pub top: i16,
    pub bottom: i16,
    pub left: i16,
    pub right: i16,
}
