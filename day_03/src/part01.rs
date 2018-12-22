use crate::claims;

pub fn get_overlapping_fabric(passed_claims: &std::vec::Vec<claims::Claim>) -> usize {

    // Detect total fabric dimensions
    let mut width = 0;
    let mut height = 0;

    for claim in passed_claims {
        let top_x = claim.offset_x + claim.width;
        if top_x > width {
            width = top_x;
        }

        let top_y = claim.offset_y + claim.height;
        if top_y > height {
            height = top_y;
        }
    }

    let mut grid_raw = vec![0; width * height];

    // Write all claims to indicate overlapping
    for claim in passed_claims {
        for x in claim.offset_x..claim.offset_x+claim.width {
            for y in claim.offset_y..claim.offset_y+claim.height {
                grid_raw[x + y*width] += 1;
            }
        }
    }

    let overlapping = grid_raw.iter().filter(|&n| *n > 1).count();

    overlapping
}
