macro_rules! bbi {
    ($index:literal) => {
        match BoundingBoxIndex::from_u32($index) {
            Some(index) => index,
            None => panic!("invalid bounding box index"),
        }
    }
}

const ANCHORS: &[Anchor] = &[
    Anchor {
        position: Point { x: 949, y: 1590 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1024, y: 1590 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 949, y: 1650 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1024, y: 1650 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 960, y: 1620 },
        bounding_box: bbi!(0),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 849, y: 1570 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 924, y: 1570 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 849, y: 1650 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 924, y: 1650 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 860, y: 1600 },
        bounding_box: bbi!(1),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 860, y: 1620 },
        bounding_box: bbi!(1),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 914, y: 1610 },
        bounding_box: bbi!(1),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 609, y: 370 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 684, y: 370 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 609, y: 450 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 684, y: 450 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 620, y: 400 },
        bounding_box: bbi!(2),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 620, y: 420 },
        bounding_box: bbi!(2),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 674, y: 410 },
        bounding_box: bbi!(2),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 249, y: 70 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 70 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 249, y: 150 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 150 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 260, y: 100 },
        bounding_box: bbi!(3),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 260, y: 120 },
        bounding_box: bbi!(3),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 314, y: 110 },
        bounding_box: bbi!(3),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 129, y: -10 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: -10 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 129, y: 70 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 70 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 140, y: 20 },
        bounding_box: bbi!(4),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 140, y: 40 },
        bounding_box: bbi!(4),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 194, y: 30 },
        bounding_box: bbi!(4),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 10 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 10 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 90 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 90 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 40 },
        bounding_box: bbi!(5),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 60 },
        bounding_box: bbi!(5),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 50 },
        bounding_box: bbi!(5),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 129, y: 130 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 130 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 129, y: 210 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 210 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 140, y: 160 },
        bounding_box: bbi!(6),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 140, y: 180 },
        bounding_box: bbi!(6),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 194, y: 170 },
        bounding_box: bbi!(6),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 90 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 90 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 170 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 170 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 120 },
        bounding_box: bbi!(7),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 140 },
        bounding_box: bbi!(7),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 130 },
        bounding_box: bbi!(7),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 170 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 170 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 250 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 250 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 200 },
        bounding_box: bbi!(8),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 220 },
        bounding_box: bbi!(8),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 210 },
        bounding_box: bbi!(8),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 489, y: 670 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 564, y: 670 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 489, y: 750 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 564, y: 750 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 500, y: 700 },
        bounding_box: bbi!(9),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 500, y: 720 },
        bounding_box: bbi!(9),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 554, y: 710 },
        bounding_box: bbi!(9),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 369, y: 430 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 444, y: 430 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 369, y: 510 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 444, y: 510 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 380, y: 460 },
        bounding_box: bbi!(10),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 380, y: 480 },
        bounding_box: bbi!(10),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 434, y: 470 },
        bounding_box: bbi!(10),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 249, y: 310 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 310 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 249, y: 390 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 390 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 260, y: 340 },
        bounding_box: bbi!(11),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 260, y: 360 },
        bounding_box: bbi!(11),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 314, y: 350 },
        bounding_box: bbi!(11),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 250 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 250 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 330 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 330 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 280 },
        bounding_box: bbi!(12),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 300 },
        bounding_box: bbi!(12),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 290 },
        bounding_box: bbi!(12),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 129, y: 370 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 370 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 129, y: 450 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 450 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 140, y: 400 },
        bounding_box: bbi!(13),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 140, y: 420 },
        bounding_box: bbi!(13),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 194, y: 410 },
        bounding_box: bbi!(13),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 330 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 330 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 410 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 410 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 360 },
        bounding_box: bbi!(14),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 380 },
        bounding_box: bbi!(14),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 370 },
        bounding_box: bbi!(14),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 410 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 410 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 490 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 490 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 440 },
        bounding_box: bbi!(15),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 460 },
        bounding_box: bbi!(15),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 450 },
        bounding_box: bbi!(15),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 249, y: 550 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 550 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 249, y: 630 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 630 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 260, y: 580 },
        bounding_box: bbi!(16),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 260, y: 600 },
        bounding_box: bbi!(16),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 314, y: 590 },
        bounding_box: bbi!(16),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 490 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 490 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 570 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 570 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 520 },
        bounding_box: bbi!(17),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 540 },
        bounding_box: bbi!(17),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 530 },
        bounding_box: bbi!(17),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 129, y: 610 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 610 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 129, y: 690 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 690 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 140, y: 640 },
        bounding_box: bbi!(18),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 140, y: 660 },
        bounding_box: bbi!(18),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 194, y: 650 },
        bounding_box: bbi!(18),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 570 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 570 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 650 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 650 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 600 },
        bounding_box: bbi!(19),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 620 },
        bounding_box: bbi!(19),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 610 },
        bounding_box: bbi!(19),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 650 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 650 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 730 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 730 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 680 },
        bounding_box: bbi!(20),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 700 },
        bounding_box: bbi!(20),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 690 },
        bounding_box: bbi!(20),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 369, y: 910 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 444, y: 910 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 369, y: 990 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 444, y: 990 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 380, y: 940 },
        bounding_box: bbi!(21),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 380, y: 960 },
        bounding_box: bbi!(21),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 434, y: 950 },
        bounding_box: bbi!(21),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 249, y: 790 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 790 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 249, y: 870 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 870 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 260, y: 820 },
        bounding_box: bbi!(22),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 260, y: 840 },
        bounding_box: bbi!(22),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 314, y: 830 },
        bounding_box: bbi!(22),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 730 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 730 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 810 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 810 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 760 },
        bounding_box: bbi!(23),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 780 },
        bounding_box: bbi!(23),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 770 },
        bounding_box: bbi!(23),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 129, y: 850 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 850 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 129, y: 930 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 930 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 140, y: 880 },
        bounding_box: bbi!(24),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 140, y: 900 },
        bounding_box: bbi!(24),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 194, y: 890 },
        bounding_box: bbi!(24),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 810 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 810 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 890 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 890 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 840 },
        bounding_box: bbi!(25),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 860 },
        bounding_box: bbi!(25),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 850 },
        bounding_box: bbi!(25),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 890 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 890 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 970 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 970 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 920 },
        bounding_box: bbi!(26),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 940 },
        bounding_box: bbi!(26),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 930 },
        bounding_box: bbi!(26),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 249, y: 1030 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 1030 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 249, y: 1110 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 1110 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 260, y: 1060 },
        bounding_box: bbi!(27),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 260, y: 1080 },
        bounding_box: bbi!(27),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 314, y: 1070 },
        bounding_box: bbi!(27),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 970 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 970 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 1050 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 1050 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 1000 },
        bounding_box: bbi!(28),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 1020 },
        bounding_box: bbi!(28),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 1010 },
        bounding_box: bbi!(28),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 129, y: 1090 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 1090 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 129, y: 1170 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 1170 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 140, y: 1120 },
        bounding_box: bbi!(29),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 140, y: 1140 },
        bounding_box: bbi!(29),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 194, y: 1130 },
        bounding_box: bbi!(29),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 1050 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 1050 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 1130 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 1130 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 1080 },
        bounding_box: bbi!(30),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 1100 },
        bounding_box: bbi!(30),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 1090 },
        bounding_box: bbi!(30),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 1130 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 1130 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 1210 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 1210 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 1160 },
        bounding_box: bbi!(31),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 1180 },
        bounding_box: bbi!(31),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 1170 },
        bounding_box: bbi!(31),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 729, y: 2770 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 804, y: 2770 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 729, y: 2850 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 804, y: 2850 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 740, y: 2800 },
        bounding_box: bbi!(32),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 740, y: 2820 },
        bounding_box: bbi!(32),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 794, y: 2810 },
        bounding_box: bbi!(32),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 609, y: 1990 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 684, y: 1990 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 609, y: 2070 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 684, y: 2070 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 620, y: 2020 },
        bounding_box: bbi!(33),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 620, y: 2040 },
        bounding_box: bbi!(33),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 674, y: 2030 },
        bounding_box: bbi!(33),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 489, y: 1590 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 564, y: 1590 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 489, y: 1670 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 564, y: 1670 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 500, y: 1620 },
        bounding_box: bbi!(34),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 500, y: 1640 },
        bounding_box: bbi!(34),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 554, y: 1630 },
        bounding_box: bbi!(34),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 369, y: 1390 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 444, y: 1390 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 369, y: 1470 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 444, y: 1470 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 380, y: 1420 },
        bounding_box: bbi!(35),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 380, y: 1440 },
        bounding_box: bbi!(35),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 434, y: 1430 },
        bounding_box: bbi!(35),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 249, y: 1270 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 1270 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 249, y: 1350 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 1350 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 260, y: 1300 },
        bounding_box: bbi!(36),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 260, y: 1320 },
        bounding_box: bbi!(36),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 314, y: 1310 },
        bounding_box: bbi!(36),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 1210 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 1210 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 1290 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 1290 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 1240 },
        bounding_box: bbi!(37),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 1260 },
        bounding_box: bbi!(37),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 1250 },
        bounding_box: bbi!(37),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 129, y: 1330 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 1330 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 129, y: 1410 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 1410 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 140, y: 1360 },
        bounding_box: bbi!(38),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 140, y: 1380 },
        bounding_box: bbi!(38),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 194, y: 1370 },
        bounding_box: bbi!(38),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 1290 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 1290 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 1370 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 1370 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 1320 },
        bounding_box: bbi!(39),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 1340 },
        bounding_box: bbi!(39),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 1330 },
        bounding_box: bbi!(39),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 1370 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 1370 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 1450 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 1450 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 1400 },
        bounding_box: bbi!(40),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 1420 },
        bounding_box: bbi!(40),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 1410 },
        bounding_box: bbi!(40),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 249, y: 1490 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 1490 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 249, y: 1570 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 1570 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 260, y: 1520 },
        bounding_box: bbi!(41),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 260, y: 1540 },
        bounding_box: bbi!(41),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 314, y: 1530 },
        bounding_box: bbi!(41),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 129, y: 1530 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 1530 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 129, y: 1610 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 1610 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 140, y: 1560 },
        bounding_box: bbi!(42),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 140, y: 1580 },
        bounding_box: bbi!(42),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 194, y: 1570 },
        bounding_box: bbi!(42),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 1490 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 1490 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 1570 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 1570 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 1520 },
        bounding_box: bbi!(43),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 1540 },
        bounding_box: bbi!(43),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 1530 },
        bounding_box: bbi!(43),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 1570 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 1570 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 1650 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 1650 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 1600 },
        bounding_box: bbi!(44),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 1620 },
        bounding_box: bbi!(44),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 1610 },
        bounding_box: bbi!(44),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 369, y: 1790 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 444, y: 1790 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 369, y: 1870 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 444, y: 1870 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 380, y: 1820 },
        bounding_box: bbi!(45),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 380, y: 1840 },
        bounding_box: bbi!(45),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 434, y: 1830 },
        bounding_box: bbi!(45),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 249, y: 1690 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 1690 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 249, y: 1770 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 1770 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 260, y: 1720 },
        bounding_box: bbi!(46),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 260, y: 1740 },
        bounding_box: bbi!(46),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 314, y: 1730 },
        bounding_box: bbi!(46),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 129, y: 1730 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 1730 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 129, y: 1810 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 1810 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 140, y: 1760 },
        bounding_box: bbi!(47),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 140, y: 1780 },
        bounding_box: bbi!(47),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 194, y: 1770 },
        bounding_box: bbi!(47),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 1690 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 1690 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 1770 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 1770 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 1720 },
        bounding_box: bbi!(48),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 1740 },
        bounding_box: bbi!(48),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 1730 },
        bounding_box: bbi!(48),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 1770 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 1770 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 1850 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 1850 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 1800 },
        bounding_box: bbi!(49),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 1820 },
        bounding_box: bbi!(49),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 1810 },
        bounding_box: bbi!(49),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 249, y: 1890 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 1890 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 249, y: 1970 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 1970 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 260, y: 1920 },
        bounding_box: bbi!(50),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 260, y: 1940 },
        bounding_box: bbi!(50),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 314, y: 1930 },
        bounding_box: bbi!(50),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 129, y: 1930 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 1930 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 129, y: 2010 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 2010 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 140, y: 1960 },
        bounding_box: bbi!(51),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 140, y: 1980 },
        bounding_box: bbi!(51),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 194, y: 1970 },
        bounding_box: bbi!(51),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 1890 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 1890 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 1970 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 1970 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 1920 },
        bounding_box: bbi!(52),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 1940 },
        bounding_box: bbi!(52),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 1930 },
        bounding_box: bbi!(52),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 1970 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 1970 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 2050 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 2050 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 2000 },
        bounding_box: bbi!(53),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 2020 },
        bounding_box: bbi!(53),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 2010 },
        bounding_box: bbi!(53),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 489, y: 2390 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 564, y: 2390 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 489, y: 2470 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 564, y: 2470 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 500, y: 2420 },
        bounding_box: bbi!(54),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 500, y: 2440 },
        bounding_box: bbi!(54),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 554, y: 2430 },
        bounding_box: bbi!(54),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 369, y: 2190 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 444, y: 2190 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 369, y: 2270 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 444, y: 2270 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 380, y: 2220 },
        bounding_box: bbi!(55),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 380, y: 2240 },
        bounding_box: bbi!(55),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 434, y: 2230 },
        bounding_box: bbi!(55),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 249, y: 2090 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 2090 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 249, y: 2170 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 2170 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 260, y: 2120 },
        bounding_box: bbi!(56),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 260, y: 2140 },
        bounding_box: bbi!(56),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 314, y: 2130 },
        bounding_box: bbi!(56),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 129, y: 2130 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 2130 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 129, y: 2210 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 2210 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 140, y: 2160 },
        bounding_box: bbi!(57),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 140, y: 2180 },
        bounding_box: bbi!(57),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 194, y: 2170 },
        bounding_box: bbi!(57),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 2090 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 2090 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 2170 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 2170 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 2120 },
        bounding_box: bbi!(58),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 2140 },
        bounding_box: bbi!(58),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 2130 },
        bounding_box: bbi!(58),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 2170 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 2170 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 2250 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 2250 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 2200 },
        bounding_box: bbi!(59),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 2220 },
        bounding_box: bbi!(59),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 2210 },
        bounding_box: bbi!(59),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 249, y: 2290 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 2290 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 249, y: 2370 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 2370 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 260, y: 2320 },
        bounding_box: bbi!(60),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 260, y: 2340 },
        bounding_box: bbi!(60),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 314, y: 2330 },
        bounding_box: bbi!(60),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 129, y: 2330 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 2330 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 129, y: 2410 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 2410 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 140, y: 2360 },
        bounding_box: bbi!(61),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 140, y: 2380 },
        bounding_box: bbi!(61),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 194, y: 2370 },
        bounding_box: bbi!(61),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 2290 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 2290 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 2370 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 2370 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 2320 },
        bounding_box: bbi!(62),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 2340 },
        bounding_box: bbi!(62),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 2330 },
        bounding_box: bbi!(62),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 2370 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 2370 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 2450 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 2450 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 2400 },
        bounding_box: bbi!(63),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 2420 },
        bounding_box: bbi!(63),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 2410 },
        bounding_box: bbi!(63),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 369, y: 2590 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 444, y: 2590 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 369, y: 2670 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 444, y: 2670 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 380, y: 2620 },
        bounding_box: bbi!(64),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 380, y: 2640 },
        bounding_box: bbi!(64),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 434, y: 2630 },
        bounding_box: bbi!(64),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 249, y: 2490 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 2490 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 249, y: 2570 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 2570 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 260, y: 2520 },
        bounding_box: bbi!(65),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 260, y: 2540 },
        bounding_box: bbi!(65),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 314, y: 2530 },
        bounding_box: bbi!(65),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 129, y: 2530 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 2530 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 129, y: 2610 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 2610 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 140, y: 2560 },
        bounding_box: bbi!(66),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 140, y: 2580 },
        bounding_box: bbi!(66),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 194, y: 2570 },
        bounding_box: bbi!(66),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 2490 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 2490 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 2570 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 2570 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 2520 },
        bounding_box: bbi!(67),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 2540 },
        bounding_box: bbi!(67),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 2530 },
        bounding_box: bbi!(67),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 2570 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 2570 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 2650 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 2650 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 2600 },
        bounding_box: bbi!(68),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 2620 },
        bounding_box: bbi!(68),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 2610 },
        bounding_box: bbi!(68),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 249, y: 2690 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 2690 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 249, y: 2770 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 2770 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 260, y: 2720 },
        bounding_box: bbi!(69),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 260, y: 2740 },
        bounding_box: bbi!(69),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 314, y: 2730 },
        bounding_box: bbi!(69),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 129, y: 2730 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 2730 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 129, y: 2810 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 2810 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 140, y: 2760 },
        bounding_box: bbi!(70),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 140, y: 2780 },
        bounding_box: bbi!(70),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 194, y: 2770 },
        bounding_box: bbi!(70),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 2690 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 2690 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 2770 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 2770 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 2720 },
        bounding_box: bbi!(71),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 2740 },
        bounding_box: bbi!(71),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 2730 },
        bounding_box: bbi!(71),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 2770 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 2770 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 2850 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 2850 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 2800 },
        bounding_box: bbi!(72),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 2820 },
        bounding_box: bbi!(72),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 2810 },
        bounding_box: bbi!(72),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 609, y: 3530 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 684, y: 3530 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 609, y: 3610 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 684, y: 3610 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 620, y: 3560 },
        bounding_box: bbi!(73),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 620, y: 3580 },
        bounding_box: bbi!(73),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 674, y: 3570 },
        bounding_box: bbi!(73),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 489, y: 3190 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 564, y: 3190 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 489, y: 3270 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 564, y: 3270 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 500, y: 3220 },
        bounding_box: bbi!(74),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 500, y: 3240 },
        bounding_box: bbi!(74),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 554, y: 3230 },
        bounding_box: bbi!(74),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 369, y: 2990 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 444, y: 2990 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 369, y: 3070 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 444, y: 3070 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 380, y: 3020 },
        bounding_box: bbi!(75),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 380, y: 3040 },
        bounding_box: bbi!(75),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 434, y: 3030 },
        bounding_box: bbi!(75),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 249, y: 2890 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 2890 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 249, y: 2970 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 2970 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 260, y: 2920 },
        bounding_box: bbi!(76),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 260, y: 2940 },
        bounding_box: bbi!(76),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 314, y: 2930 },
        bounding_box: bbi!(76),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 129, y: 2930 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 2930 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 129, y: 3010 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 3010 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 140, y: 2960 },
        bounding_box: bbi!(77),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 140, y: 2980 },
        bounding_box: bbi!(77),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 194, y: 2970 },
        bounding_box: bbi!(77),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 2890 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 2890 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 2970 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 2970 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 2920 },
        bounding_box: bbi!(78),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 2940 },
        bounding_box: bbi!(78),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 2930 },
        bounding_box: bbi!(78),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 2970 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 2970 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 3050 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 3050 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 3000 },
        bounding_box: bbi!(79),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 3020 },
        bounding_box: bbi!(79),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 3010 },
        bounding_box: bbi!(79),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 249, y: 3090 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 3090 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 249, y: 3170 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 3170 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 260, y: 3120 },
        bounding_box: bbi!(80),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 260, y: 3140 },
        bounding_box: bbi!(80),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 314, y: 3130 },
        bounding_box: bbi!(80),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 129, y: 3130 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 3130 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 129, y: 3210 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 3210 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 140, y: 3160 },
        bounding_box: bbi!(81),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 140, y: 3180 },
        bounding_box: bbi!(81),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 194, y: 3170 },
        bounding_box: bbi!(81),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 3090 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 3090 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 3170 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 3170 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 3120 },
        bounding_box: bbi!(82),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 3140 },
        bounding_box: bbi!(82),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 3130 },
        bounding_box: bbi!(82),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 3170 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 3170 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 3250 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 3250 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 3200 },
        bounding_box: bbi!(83),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 3220 },
        bounding_box: bbi!(83),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 3210 },
        bounding_box: bbi!(83),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 249, y: 3370 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 3370 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 249, y: 3450 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 3450 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 260, y: 3400 },
        bounding_box: bbi!(84),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 260, y: 3420 },
        bounding_box: bbi!(84),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 314, y: 3410 },
        bounding_box: bbi!(84),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 129, y: 3290 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 3290 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 129, y: 3370 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 3370 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 140, y: 3320 },
        bounding_box: bbi!(85),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 140, y: 3340 },
        bounding_box: bbi!(85),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 194, y: 3330 },
        bounding_box: bbi!(85),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 3250 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 3250 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 3330 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 3330 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 3280 },
        bounding_box: bbi!(86),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 3300 },
        bounding_box: bbi!(86),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 3290 },
        bounding_box: bbi!(86),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 3330 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 3330 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 3410 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 3410 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 3360 },
        bounding_box: bbi!(87),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 3380 },
        bounding_box: bbi!(87),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 3370 },
        bounding_box: bbi!(87),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 129, y: 3450 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 3450 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 129, y: 3530 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 3530 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 140, y: 3480 },
        bounding_box: bbi!(88),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 140, y: 3500 },
        bounding_box: bbi!(88),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 194, y: 3490 },
        bounding_box: bbi!(88),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 3410 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 3410 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 3490 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 3490 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 3440 },
        bounding_box: bbi!(89),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 3460 },
        bounding_box: bbi!(89),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 3450 },
        bounding_box: bbi!(89),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 3490 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 3490 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 3570 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 3570 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 3520 },
        bounding_box: bbi!(90),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 3540 },
        bounding_box: bbi!(90),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 3530 },
        bounding_box: bbi!(90),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 369, y: 3850 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 444, y: 3850 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 369, y: 3930 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 444, y: 3930 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 380, y: 3880 },
        bounding_box: bbi!(91),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 380, y: 3900 },
        bounding_box: bbi!(91),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 434, y: 3890 },
        bounding_box: bbi!(91),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 249, y: 3690 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 3690 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 249, y: 3770 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 3770 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 260, y: 3720 },
        bounding_box: bbi!(92),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 260, y: 3740 },
        bounding_box: bbi!(92),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 314, y: 3730 },
        bounding_box: bbi!(92),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 129, y: 3610 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 3610 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 129, y: 3690 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 3690 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 140, y: 3640 },
        bounding_box: bbi!(93),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 140, y: 3660 },
        bounding_box: bbi!(93),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 194, y: 3650 },
        bounding_box: bbi!(93),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 3570 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 3570 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 3650 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 3650 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 3600 },
        bounding_box: bbi!(94),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 3620 },
        bounding_box: bbi!(94),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 3610 },
        bounding_box: bbi!(94),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 3650 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 3650 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 3730 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 3730 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 3680 },
        bounding_box: bbi!(95),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 3700 },
        bounding_box: bbi!(95),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 3690 },
        bounding_box: bbi!(95),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 129, y: 3770 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 3770 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 129, y: 3850 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 3850 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 140, y: 3800 },
        bounding_box: bbi!(96),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 140, y: 3820 },
        bounding_box: bbi!(96),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 194, y: 3810 },
        bounding_box: bbi!(96),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 3730 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 3730 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 3810 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 3810 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 3760 },
        bounding_box: bbi!(97),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 3780 },
        bounding_box: bbi!(97),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 3770 },
        bounding_box: bbi!(97),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 3810 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 3810 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 3890 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 3890 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 3840 },
        bounding_box: bbi!(98),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 3860 },
        bounding_box: bbi!(98),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 3850 },
        bounding_box: bbi!(98),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 249, y: 4010 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 4010 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 249, y: 4090 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 4090 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 260, y: 4040 },
        bounding_box: bbi!(99),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 260, y: 4060 },
        bounding_box: bbi!(99),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 314, y: 4050 },
        bounding_box: bbi!(99),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 129, y: 3930 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 3930 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 129, y: 4010 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 4010 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 140, y: 3960 },
        bounding_box: bbi!(100),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 140, y: 3980 },
        bounding_box: bbi!(100),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 194, y: 3970 },
        bounding_box: bbi!(100),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 3890 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 3890 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 3970 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 3970 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 3920 },
        bounding_box: bbi!(101),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 3940 },
        bounding_box: bbi!(101),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 3930 },
        bounding_box: bbi!(101),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 3970 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 3970 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 4050 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 4050 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 4000 },
        bounding_box: bbi!(102),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 4020 },
        bounding_box: bbi!(102),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 4010 },
        bounding_box: bbi!(102),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 129, y: 4090 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 4090 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 129, y: 4170 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 4170 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 140, y: 4120 },
        bounding_box: bbi!(103),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 140, y: 4140 },
        bounding_box: bbi!(103),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 194, y: 4130 },
        bounding_box: bbi!(103),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 4050 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 4050 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 4130 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 4130 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 4080 },
        bounding_box: bbi!(104),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 4100 },
        bounding_box: bbi!(104),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 4090 },
        bounding_box: bbi!(104),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 4130 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 4130 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 4210 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 4210 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 4160 },
        bounding_box: bbi!(105),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 4180 },
        bounding_box: bbi!(105),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 4170 },
        bounding_box: bbi!(105),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 949, y: 4610 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1024, y: 4610 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 949, y: 4670 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1024, y: 4670 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 960, y: 4640 },
        bounding_box: bbi!(106),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 489, y: 4590 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 564, y: 4590 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 489, y: 4670 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 564, y: 4670 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 500, y: 4620 },
        bounding_box: bbi!(107),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 500, y: 4640 },
        bounding_box: bbi!(107),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 554, y: 4630 },
        bounding_box: bbi!(107),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 369, y: 4350 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 444, y: 4350 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 369, y: 4430 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 444, y: 4430 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 380, y: 4380 },
        bounding_box: bbi!(108),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 380, y: 4400 },
        bounding_box: bbi!(108),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 434, y: 4390 },
        bounding_box: bbi!(108),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 129, y: 4250 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 4250 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 129, y: 4330 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 4330 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 140, y: 4280 },
        bounding_box: bbi!(109),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 140, y: 4300 },
        bounding_box: bbi!(109),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 194, y: 4290 },
        bounding_box: bbi!(109),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 4210 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 4210 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 4290 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 4290 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 4240 },
        bounding_box: bbi!(110),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 4260 },
        bounding_box: bbi!(110),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 4250 },
        bounding_box: bbi!(110),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 4290 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 4290 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 4370 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 4370 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 4320 },
        bounding_box: bbi!(111),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 4340 },
        bounding_box: bbi!(111),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 4330 },
        bounding_box: bbi!(111),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 249, y: 4450 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 4450 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 249, y: 4530 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 4530 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 260, y: 4480 },
        bounding_box: bbi!(112),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 260, y: 4500 },
        bounding_box: bbi!(112),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 314, y: 4490 },
        bounding_box: bbi!(112),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 129, y: 4390 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 4390 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 129, y: 4470 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 4470 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 140, y: 4420 },
        bounding_box: bbi!(113),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 140, y: 4440 },
        bounding_box: bbi!(113),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 194, y: 4430 },
        bounding_box: bbi!(113),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 4410 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 4410 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 4490 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 4490 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 4440 },
        bounding_box: bbi!(114),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 4460 },
        bounding_box: bbi!(114),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 4450 },
        bounding_box: bbi!(114),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 129, y: 4510 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 4510 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 129, y: 4590 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 4590 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 140, y: 4540 },
        bounding_box: bbi!(115),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 140, y: 4560 },
        bounding_box: bbi!(115),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 194, y: 4550 },
        bounding_box: bbi!(115),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 4530 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 4530 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 4610 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 4610 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 4560 },
        bounding_box: bbi!(116),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 4580 },
        bounding_box: bbi!(116),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 4570 },
        bounding_box: bbi!(116),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 369, y: 4810 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 444, y: 4810 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 369, y: 4890 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 444, y: 4890 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 380, y: 4840 },
        bounding_box: bbi!(117),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 380, y: 4860 },
        bounding_box: bbi!(117),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 434, y: 4850 },
        bounding_box: bbi!(117),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 249, y: 4670 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 4670 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 249, y: 4750 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 4750 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 260, y: 4700 },
        bounding_box: bbi!(118),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 260, y: 4720 },
        bounding_box: bbi!(118),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 314, y: 4710 },
        bounding_box: bbi!(118),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 4610 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 4610 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 4690 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 4690 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 4640 },
        bounding_box: bbi!(119),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 4660 },
        bounding_box: bbi!(119),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 4650 },
        bounding_box: bbi!(119),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 129, y: 4730 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 4730 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 129, y: 4810 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 4810 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 140, y: 4760 },
        bounding_box: bbi!(120),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 140, y: 4780 },
        bounding_box: bbi!(120),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 194, y: 4770 },
        bounding_box: bbi!(120),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 4690 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 4690 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 4770 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 4770 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 4720 },
        bounding_box: bbi!(121),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 4740 },
        bounding_box: bbi!(121),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 4730 },
        bounding_box: bbi!(121),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 4770 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 4770 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 4850 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 4850 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 4800 },
        bounding_box: bbi!(122),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 4820 },
        bounding_box: bbi!(122),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 4810 },
        bounding_box: bbi!(122),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 249, y: 4930 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 4930 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 249, y: 5010 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 324, y: 5010 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 260, y: 4960 },
        bounding_box: bbi!(123),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 260, y: 4980 },
        bounding_box: bbi!(123),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 314, y: 4970 },
        bounding_box: bbi!(123),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 129, y: 4870 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 4870 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 129, y: 4950 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 4950 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 140, y: 4900 },
        bounding_box: bbi!(124),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 140, y: 4920 },
        bounding_box: bbi!(124),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 194, y: 4910 },
        bounding_box: bbi!(124),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 4890 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 4890 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 4970 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 4970 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 4920 },
        bounding_box: bbi!(125),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 4940 },
        bounding_box: bbi!(125),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 4930 },
        bounding_box: bbi!(125),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 129, y: 4990 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 4990 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 129, y: 5070 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 204, y: 5070 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 140, y: 5020 },
        bounding_box: bbi!(126),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 140, y: 5040 },
        bounding_box: bbi!(126),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 194, y: 5030 },
        bounding_box: bbi!(126),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 9, y: 5010 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 5010 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 9, y: 5090 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 84, y: 5090 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 20, y: 5040 },
        bounding_box: bbi!(127),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 20, y: 5060 },
        bounding_box: bbi!(127),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 74, y: 5050 },
        bounding_box: bbi!(127),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: -624, y: -370 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -549, y: -370 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -624, y: -310 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -549, y: -310 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -560, y: -340 },
        bounding_box: bbi!(128),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: -624, y: -130 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -549, y: -130 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -624, y: -70 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -549, y: -70 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -560, y: -100 },
        bounding_box: bbi!(129),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: -510, y: -130 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -435, y: -130 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -510, y: -70 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -435, y: -70 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -500, y: -100 },
        bounding_box: bbi!(130),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: -446, y: -100 },
        bounding_box: bbi!(130),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: -624, y: -190 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -549, y: -190 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -624, y: -130 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -549, y: -130 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -560, y: -160 },
        bounding_box: bbi!(131),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: -624, y: -250 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -549, y: -250 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -624, y: -190 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -549, y: -190 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -560, y: -220 },
        bounding_box: bbi!(132),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: -624, y: -310 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -549, y: -310 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -624, y: -250 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -549, y: -250 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -560, y: -280 },
        bounding_box: bbi!(133),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: -624, y: -430 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -549, y: -430 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -624, y: -370 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -549, y: -370 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -560, y: -400 },
        bounding_box: bbi!(134),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: -624, y: -490 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -549, y: -490 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -624, y: -430 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -549, y: -430 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -560, y: -460 },
        bounding_box: bbi!(135),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: -510, y: -190 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -435, y: -190 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -510, y: -130 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -435, y: -130 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -500, y: -160 },
        bounding_box: bbi!(136),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: -446, y: -160 },
        bounding_box: bbi!(136),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: -510, y: -250 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -435, y: -250 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -510, y: -190 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -435, y: -190 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -500, y: -220 },
        bounding_box: bbi!(137),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: -446, y: -220 },
        bounding_box: bbi!(137),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: -510, y: -310 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -435, y: -310 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -510, y: -250 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -435, y: -250 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -500, y: -280 },
        bounding_box: bbi!(138),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: -446, y: -280 },
        bounding_box: bbi!(138),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: -510, y: -370 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -435, y: -370 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -510, y: -310 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -435, y: -310 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -500, y: -340 },
        bounding_box: bbi!(139),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: -446, y: -340 },
        bounding_box: bbi!(139),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: -510, y: -430 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -435, y: -430 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -510, y: -370 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -435, y: -370 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -500, y: -400 },
        bounding_box: bbi!(140),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: -446, y: -400 },
        bounding_box: bbi!(140),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: -510, y: -490 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -435, y: -490 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -510, y: -430 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -435, y: -430 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -500, y: -460 },
        bounding_box: bbi!(141),
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: -446, y: -460 },
        bounding_box: bbi!(141),
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: -380, y: 0 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -20, y: 2560 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -400, y: 520 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -400, y: 3080 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -340, y: 1040 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -400, y: 3600 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -100, y: 1560 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -320, y: 4120 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -380, y: 2080 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -340, y: 40 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -80, y: 2600 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -340, y: 560 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -320, y: 3120 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -40, y: 1080 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -40, y: 3640 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -280, y: 1600 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -40, y: 4160 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -40, y: 2120 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -280, y: 4680 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -40, y: 80 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -200, y: 2640 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -20, y: 600 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -20, y: 3160 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -80, y: 1120 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -100, y: 3680 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -540, y: -100 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -160, y: 1640 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -100, y: 2160 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -80, y: 120 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -400, y: 2680 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -80, y: 640 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -260, y: 1160 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -200, y: 3720 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -380, y: 1680 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -400, y: 4240 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -260, y: 2200 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -540, y: -160 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -280, y: 160 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -320, y: 2720 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -280, y: 680 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -220, y: 1200 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -340, y: 1720 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -340, y: 4280 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -220, y: 2240 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -160, y: 200 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -40, y: 2760 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -160, y: 720 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -380, y: 3280 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -380, y: 1240 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -320, y: 3800 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -540, y: -220 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -20, y: 1760 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -20, y: 4320 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -400, y: 2280 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -280, y: 4840 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -220, y: 240 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -280, y: 2800 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -380, y: 760 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -40, y: 3320 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -320, y: 1280 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -40, y: 3840 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -80, y: 1800 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -80, y: 4360 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -20, y: 2320 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -540, y: -280 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -380, y: 280 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -340, y: 800 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -100, y: 3360 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -20, y: 1320 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -140, y: 3880 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -140, y: 1840 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -400, y: 4400 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -100, y: 2360 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -40, y: 4920 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -20, y: 320 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -380, y: 2880 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -20, y: 840 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -140, y: 3400 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -280, y: 1360 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -400, y: 3920 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -540, y: -340 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -400, y: 1880 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -40, y: 4440 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -260, y: 2400 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -280, y: 4960 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -100, y: 360 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -320, y: 2920 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -80, y: 880 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -400, y: 3440 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -160, y: 1400 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -320, y: 3960 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -340, y: 1920 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -100, y: 4480 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -220, y: 2440 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -540, y: -400 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -280, y: 400 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -40, y: 2960 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -260, y: 920 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -20, y: 3480 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -220, y: 1440 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -20, y: 4000 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -40, y: 1960 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -400, y: 4520 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -400, y: 2480 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -160, y: 440 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -260, y: 3000 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -220, y: 960 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -100, y: 3520 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -400, y: 1480 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -540, y: -460 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -80, y: 2000 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -320, y: 4560 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -340, y: 2520 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -220, y: 480 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -220, y: 3040 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -400, y: 1000 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -140, y: 3560 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -40, y: 1520 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -400, y: 4080 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -140, y: 2040 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: -40, y: 4600 },
        bounding_box: BoundingBoxIndex::INVALID,
        connect_directions: Directions::ALL,
    },
];

const BOUNDING_BOXES: &[BoundingBox] = &[
    BoundingBox {
        center: Point { x: 987, y: 1620 },
        half_width: 36,
        half_height: 29,
    },
    BoundingBox {
        center: Point { x: 887, y: 1610 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 647, y: 410 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 287, y: 110 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 167, y: 30 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 50 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 167, y: 170 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 130 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 210 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 527, y: 710 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 407, y: 470 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 287, y: 350 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 290 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 167, y: 410 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 370 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 450 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 287, y: 590 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 530 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 167, y: 650 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 610 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 690 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 407, y: 950 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 287, y: 830 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 770 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 167, y: 890 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 850 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 930 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 287, y: 1070 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 1010 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 167, y: 1130 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 1090 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 1170 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 767, y: 2810 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 647, y: 2030 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 527, y: 1630 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 407, y: 1430 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 287, y: 1310 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 1250 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 167, y: 1370 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 1330 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 1410 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 287, y: 1530 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 167, y: 1570 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 1530 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 1610 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 407, y: 1830 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 287, y: 1730 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 167, y: 1770 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 1730 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 1810 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 287, y: 1930 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 167, y: 1970 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 1930 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 2010 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 527, y: 2430 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 407, y: 2230 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 287, y: 2130 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 167, y: 2170 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 2130 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 2210 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 287, y: 2330 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 167, y: 2370 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 2330 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 2410 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 407, y: 2630 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 287, y: 2530 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 167, y: 2570 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 2530 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 2610 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 287, y: 2730 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 167, y: 2770 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 2730 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 2810 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 647, y: 3570 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 527, y: 3230 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 407, y: 3030 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 287, y: 2930 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 167, y: 2970 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 2930 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 3010 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 287, y: 3130 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 167, y: 3170 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 3130 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 3210 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 287, y: 3410 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 167, y: 3330 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 3290 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 3370 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 167, y: 3490 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 3450 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 3530 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 407, y: 3890 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 287, y: 3730 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 167, y: 3650 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 3610 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 3690 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 167, y: 3810 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 3770 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 3850 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 287, y: 4050 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 167, y: 3970 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 3930 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 4010 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 167, y: 4130 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 4090 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 4170 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 987, y: 4640 },
        half_width: 36,
        half_height: 29,
    },
    BoundingBox {
        center: Point { x: 527, y: 4630 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 407, y: 4390 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 167, y: 4290 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 4250 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 4330 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 287, y: 4490 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 167, y: 4430 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 4450 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 167, y: 4550 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 4570 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 407, y: 4850 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 287, y: 4710 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 4650 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 167, y: 4770 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 4730 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 4810 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 287, y: 4970 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 167, y: 4910 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 4930 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 167, y: 5030 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 47, y: 5050 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: -587, y: -340 },
        half_width: 36,
        half_height: 29,
    },
    BoundingBox {
        center: Point { x: -587, y: -100 },
        half_width: 36,
        half_height: 29,
    },
    BoundingBox {
        center: Point { x: -473, y: -100 },
        half_width: 36,
        half_height: 29,
    },
    BoundingBox {
        center: Point { x: -587, y: -160 },
        half_width: 36,
        half_height: 29,
    },
    BoundingBox {
        center: Point { x: -587, y: -220 },
        half_width: 36,
        half_height: 29,
    },
    BoundingBox {
        center: Point { x: -587, y: -280 },
        half_width: 36,
        half_height: 29,
    },
    BoundingBox {
        center: Point { x: -587, y: -400 },
        half_width: 36,
        half_height: 29,
    },
    BoundingBox {
        center: Point { x: -587, y: -460 },
        half_width: 36,
        half_height: 29,
    },
    BoundingBox {
        center: Point { x: -473, y: -160 },
        half_width: 36,
        half_height: 29,
    },
    BoundingBox {
        center: Point { x: -473, y: -220 },
        half_width: 36,
        half_height: 29,
    },
    BoundingBox {
        center: Point { x: -473, y: -280 },
        half_width: 36,
        half_height: 29,
    },
    BoundingBox {
        center: Point { x: -473, y: -340 },
        half_width: 36,
        half_height: 29,
    },
    BoundingBox {
        center: Point { x: -473, y: -400 },
        half_width: 36,
        half_height: 29,
    },
    BoundingBox {
        center: Point { x: -473, y: -460 },
        half_width: 36,
        half_height: 29,
    },
];
