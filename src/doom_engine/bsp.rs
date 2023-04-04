use super::Player;
use super::data_types::*;

const SUB_SECTOR_IDENTIFIER:u16 = 0x8000;
pub struct BSP {
    pub root_node_id: usize,
}

impl BSP {
    pub fn is_on_back_side(p:&Player, n:&Node)-> bool {
        let dx = p.pos.0 - n.x_partition;
        let dy = p.pos.1 - n.y_partition;
        dx * n.dy_partition -dy*n.dx_partition <= 0 
    }

    pub fn render_sub_sector(sub_sector_id:u16){
        println!("rendo {sub_sector_id}");
    }

    pub fn render_bsp_node(player: &Player, nodes: &Vec<Node>, node_id:u16) {
        if node_id >= SUB_SECTOR_IDENTIFIER {
            let sub_sector_id = node_id - SUB_SECTOR_IDENTIFIER;
            BSP::render_sub_sector(sub_sector_id);
            return;
        }

        let node = &(*nodes)[node_id as usize];

        let is_on_back = BSP::is_on_back_side(player, &node);
        if is_on_back {
            BSP::render_bsp_node(player, nodes, node.back_child_id);
            BSP::render_bsp_node(player, nodes, node.front_child_id);
        }
        else {
            BSP::render_bsp_node(player, nodes, node.front_child_id);
            BSP::render_bsp_node(player, nodes, node.back_child_id);
        }
    }
}